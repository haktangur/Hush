# Hush test raporu — geliştirme sürüyor

Ortam: macOS 26.6.2 / Apple Silicon; 2026-11-01.

## Geçen otomatik kontroller
- P0: npm run check; 1 UI testi; Rust bootstrap testi; clippy.
- P1: npm run check; 4 UI testi (bootstrap hata, kayıt hatasında not koruma, sayfa aşımı, kirli formdan dönüş).
- P1: 10 Rust testi: SQLite reopen, aynı UUID retry, revision conflict, trash/restore, item-kind immutability, FK başarısızlığında atomiklik, Türkçe ve literal LIKE arama, DTO roundtrip, migration sözleşmesi/ileri schema/missing pointer, geçersiz görsel, kaynak görsel silindikten sonra kapak kopyası.
- P1: cargo clippy --locked --all-targets -- -D warnings geçti.

## Native / görsel
- P0 Apple Silicon Hush.app üretildi, ad-hoc imzalandı. `open` başarılı, uygulama çalışan uygulamalar listesinde görüldü.
- Ekran inceleme aracı iki kez macOS ScreenCaptureKit -3811 hatası verdi. Native IPC, görsel kalite veya VoiceOver geçti diye raporlanmıyor.
- WDIO embedded test altyapısı hazırlanıyor. Üretimden ayrı e2e Cargo feature, config, frontend output ve geçici veri dizini.

## Henüz tamamlanmayan kabul
A01–A22 tam matris henüz tamamlanmadı. P1 native akışı ve P2–P7 ürün kapsamı geliştirme aşamasında. DMG henüz oluşturulmadı. GitHub bağlantısı doğrulanmadı, CI çalışmadı. Notarization yapılmadı.

## Native güncelleme
WDIO 10/servis 1.5.1 uyumsuzluğu WDIO 9 sürümlerine sabitlenerek çözüldü. İlk test gerçek UI formuyla kitap kaydetti; ikinci bağımsız süreç aynı geçici veri dizininde kitabı gerçek IPC'den okudu. Başlık, not ve 37/200 ilerleme eşleşti. İki run hatasız geçti. Kaynak dosyadan bağımsız kapak testi halen Rust katmanında; native picker/reopen birleşik testi henüz yapılmadı.
