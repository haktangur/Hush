# Hush ilerleme

## Şu an
- Aktif aşama: P0 — masaüstü iskeleti.
- Son çalışan commit: Henüz yok; başlangıç klasörü Git deposu değildi.
- Sonraki somut iş: Tauri iskeleti, bootstrap IPC ve erken Apple Silicon uygulama derlemesi.

## Aşamalar
- [ ] P0 — masaüstü iskeleti
- [ ] P1 — kalıcı veri
- [ ] P2 — oda
- [ ] P3 — koleksiyonlar
- [ ] P4 — günlük/takvim/arama
- [ ] P5 — kataloglar
- [ ] P6 — yedek/kurtarma
- [ ] P7 — test/GitHub/teslim

## Doğrulananlar
- 2026-11-01: AGENTS.md, CODEX_BASLANGIC.md, README, docs/01–08 ve contracts okundu.
- macOS 26.6.2 / arm64; Node 24.18.0, npm 11.16.0, rustc 1.97.1, cargo 1.97.1; Xcode Command Line Tools mevcut.
- Tauri CLI kararlı sürümü npm üzerinden 2.12.4 olarak doğrulandı.

## Engeller
- GitHub CLI kurulu değil; mevcut bağlı GitHub aracı bulunmadı. Bağlantı seçenekleri araştırılıyor; yerel geliştirme sürüyor.

## Karar değişiklikleri
- Henüz yok.

### P0 doğrulama notları
- `npm run check`: TypeScript, ESLint, 1 UI testi, Vite production build geçti.
- `cargo test --locked`: 1 bootstrap envelope testi geçti.
- `cargo clippy --locked --all-targets -- -D warnings`: geçti.
- `npm audit`: minimatch kaynaklı 12 moderate geliştirme aracı bulgusu; uygulamanın runtime bağımlılıkları etkilenmiyor. Çözüm incelemesi açık.
- P1 domain/storage/repository dosyaları hazırlanıyor; henüz runtime'a bağlanmadı, test edilmiş sayılmaz.

### P1 — 2026-11-01
- Son temel commit: `3d121fb`; aktif dal `feat/local-library`.
- Application Support/generation pointer, SQLite migration, tipli DTO ve servis/repository katmanları eklendi.
- Üç eser türü için temel form/liste/detay, revision, trash/restore, Türkçe arama ve native kapak importu bağlandı.
- 10 Rust testi, 4 UI testi, TypeScript, lint, frontend build ve clippy geçti.
- Native doğrulama için test-only WDIO kuruluyor. P0/P1 kabul kapıları henüz tamamlandı işaretlenmedi.
- ScreenCaptureKit -3811 araç hatası nedeniyle görsel inceleme yapılamadı; otomatik native yol deneniyor.
- WDIO 9 ile native kayıt ve süreç yeniden başlatma testleri geçti: gerçek IPC ile kitap 37/200 ve not korunuyor. Mock yok.
- P2 özgün SVG oda, kalıcı ışık/hareket ayarları ve poster kaynak seçimi eklendi. Günlük/takvim ve kalan navigasyon henüz bağlı değil; P2 tamamlanmadı.
- Native screenshot yolu çalışıyor; ScreenCaptureKit engeli WDIO'nun kendi WebView screenshot'ında yok. İki boyutta sığma assertion'ı ekleniyor.
