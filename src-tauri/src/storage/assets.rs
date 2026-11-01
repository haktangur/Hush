use crate::{
    domain::{now, valid_id, AppError, Result},
    storage::Storage,
};
use image::{ImageDecoder, ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::{Cursor, Write},
    path::Path,
};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    pub relative_path: String,
    pub thumbnail_path: String,
    pub sha256: String,
    pub mime_type: String,
    pub byte_size: u64,
    pub width: u32,
    pub height: u32,
    pub created_at: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedAsset {
    pub id: String,
    pub path: String,
    pub thumbnail_path: String,
    pub width: u32,
    pub height: u32,
}
pub fn import(s: &Storage, path: &Path, purpose: &str) -> Result<Asset> {
    if !matches!(purpose, "cover" | "screenshot" | "poster") {
        return Err(AppError::new("VALIDATION"));
    }
    let f = std::fs::File::open(path)?;
    if f.metadata()?.len() > 20 * 1024 * 1024 {
        return Err(AppError::new("VALIDATION"));
    }
    use std::io::Read;
    let mut bytes = Vec::new();
    f.take(20 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > 20 * 1024 * 1024 {
        return Err(AppError::new("VALIDATION"));
    }
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| AppError::new("VALIDATION"))?;
    if !matches!(
        reader.format(),
        Some(ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP)
    ) {
        return Err(AppError::new("VALIDATION"));
    }
    let mut decoder = reader
        .into_decoder()
        .map_err(|_| AppError::new("VALIDATION"))?;
    let (w, h) = decoder.dimensions();
    if w == 0 || h == 0 || u64::from(w) * u64::from(h) > 25_000_000 {
        return Err(AppError::new("VALIDATION"));
    }
    let orientation = decoder
        .orientation()
        .map_err(|_| AppError::new("VALIDATION"))?;
    let mut decoded =
        image::DynamicImage::from_decoder(decoder).map_err(|_| AppError::new("VALIDATION"))?;
    decoded.apply_orientation(orientation);
    let bound = if purpose == "screenshot" { 2560 } else { 1600 };
    let decoded = decoded
        .resize(bound, bound, image::imageops::FilterType::Lanczos3)
        .to_rgba8();
    let image = image::DynamicImage::ImageRgba8(decoded);
    let thumb = image.thumbnail(360, 360);
    let id = uuid::Uuid::new_v4().to_string();
    let assets = s.generation.join("assets");
    std::fs::create_dir_all(&assets)?;
    let stage = assets.join(format!(".import-{id}"));
    std::fs::create_dir(&stage)?;
    let result = (|| {
        let encode = |im: &image::DynamicImage, name: &str| -> Result<Vec<u8>> {
            let mut bytes = Cursor::new(Vec::new());
            im.write_to(&mut bytes, ImageFormat::WebP)
                .map_err(|_| AppError::new("IO"))?;
            let bytes = bytes.into_inner();
            let mut f = std::fs::File::create(stage.join(name))?;
            f.write_all(&bytes)?;
            f.sync_all()?;
            Ok(bytes)
        };
        let bytes = encode(&image, "image.webp")?;
        encode(&thumb, "thumb.webp")?;
        std::fs::rename(&stage, assets.join(&id))?;
        let asset = Asset {
            id: id.clone(),
            relative_path: format!("assets/{id}/image.webp"),
            thumbnail_path: format!("assets/{id}/thumb.webp"),
            sha256: Sha256::digest(&bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect(),
            mime_type: "image/webp".into(),
            byte_size: bytes.len() as u64,
            width: image.width(),
            height: image.height(),
            created_at: now(),
        };
        let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
        c.execute("INSERT INTO assets(id,relative_path,thumbnail_path,sha256,mime_type,byte_size,width,height,created_at) VALUES (?,?,?,?,?,?,?,?,?)",rusqlite::params![asset.id,asset.relative_path,asset.thumbnail_path,asset.sha256,asset.mime_type,asset.byte_size as i64,asset.width,asset.height,asset.created_at])?;
        Ok(asset)
    })();
    if stage.exists() {
        let _ = std::fs::remove_dir_all(stage);
    }
    result
}
pub fn resolve(s: &Storage, ids: &[String]) -> Result<Vec<ResolvedAsset>> {
    if ids.len() > 100 {
        return Err(AppError::new("VALIDATION"));
    }
    let c = s.connection.lock().map_err(|_| AppError::new("INTERNAL"))?;
    let mut result = Vec::new();
    for id in ids {
        valid_id(id)?;
        let (path, thumb, w, h): (String, String, u32, u32) = c.query_row(
            "SELECT relative_path,thumbnail_path,width,height FROM assets WHERE id=?",
            [id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;
        if path != format!("assets/{id}/image.webp") || thumb != format!("assets/{id}/thumb.webp") {
            return Err(AppError::new("IO"));
        }
        result.push(ResolvedAsset {
            id: id.clone(),
            path: s.generation.join(path).to_string_lossy().into_owned(),
            thumbnail_path: s.generation.join(thumb).to_string_lossy().into_owned(),
            width: w,
            height: h,
        });
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imported_cover_survives_source_removal() {
        let d = tempfile::tempdir().unwrap();
        let s = Storage::open(&d.path().join("data")).unwrap();
        let path = d.path().join("source.png");
        image::RgbImage::new(800, 1000).save(&path).unwrap();
        let a = import(&s, &path, "cover").unwrap();
        std::fs::remove_file(path).unwrap();
        let resolved = resolve(&s, &[a.id]).unwrap();
        assert!(Path::new(&resolved[0].path).exists());
        assert_eq!(
            image::open(&resolved[0].thumbnail_path).unwrap().height(),
            360
        );
    }
    #[test]
    fn rejects_invalid_image() {
        let d = tempfile::tempdir().unwrap();
        let s = Storage::open(&d.path().join("data")).unwrap();
        let p = d.path().join("x.svg");
        std::fs::write(&p, b"<svg/>").unwrap();
        assert!(import(&s, &p, "cover").is_err());
    }
}
