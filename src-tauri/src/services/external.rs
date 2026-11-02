use crate::domain::{AppError, Result};
fn validate(value: &str) -> Result<url::Url> {
    let url = url::Url::parse(value).map_err(|_| AppError::new("VALIDATION"))?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(AppError::new("VALIDATION"));
    }
    Ok(url)
}
pub fn open(value: &str) -> Result<()> {
    let url = validate(value)?;
    let status = std::process::Command::new("/usr/bin/open")
        .arg(url.as_str())
        .status()
        .map_err(|_| AppError::new("IO"))?;
    if !status.success() {
        return Err(AppError::new("IO"));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_https_without_credentials_reaches_the_system_opener() {
        assert!(validate("https://open.spotify.com/").is_ok());
        for value in [
            "file:///tmp/test",
            "javascript:alert(1)",
            "http://example.org",
            "https://user:secret@example.org",
            "--help",
        ] {
            assert!(validate(value).is_err());
        }
    }
}
