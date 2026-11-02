# Hush ilerleme

## Şu an
- Kullanıcının son yönlendirmesi: faz sonunda belgeleyip yeni sohbete geç; bu oturum yeni kapsam açmadan devir için kapatılıyor.
- P0 tamamlandı. Sonraki sohbetin hedefi yalnız P1'in native kapak/restart kabulünü kapatmak.
- P2–P6 kod hazırlıkları korunuyor; kabul kapıları tamamlanmış sayılmıyor.
- Ayrıntılı devir: [HANDOFF.md](HANDOFF.md). Test kanıtları: [TEST-REPORT.md](TEST-REPORT.md).
- Dal: `feat/local-library`; güncel checkpoint için `git log -1`.

## Aşamalar
- [x] P0 — masaüstü iskeleti (GitHub bağlantı engeli belgeli)
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
- GitHub CLI kurulu değil; bağlı GitHub aracı/remote yok. 2026-11-02 resmî CLI sürüm bilgisini indirme izni kullanıcı tarafından reddedildi. GitHub işi açık; yerel fazları engellemiyor.

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

### 2026-11-02 — devam
- Oturumdan kalan değişiklikler korundu. `npm run check` ve mevcut 11 Rust testi yeniden geçti.
- P3: seanslar, quotes/tracks/screenshots, koleksiyonlar ve filtreler eklendi. Kalıcı silme açık onay ekranında.
- P4: günlük kayıt/draft ve saat dilimli takvim eklendi; yeni kurtarma testleri çalıştırılıyor.
- P0/P1 native temel doğrulandı; tam kabul için native kapak picker birleşik akışı ve production .app açılışı son build'de yeniden denenecek.

### 2026-11-02 — faz devri
- Kullanıcı faz bazında yeni sohbetlere geçmek istedi. P0 kabulü kapatıldı; P1–P7 tamamlandı işaretlenmedi.
- P4 yerel genel arama, günlük taslağı kurtarma ve saat dilimli takvim bağlandı.
- P5 katalog önizleme, albüm parçalarının ilk kayıtta atomik yazılması, Keychain bağlantı ekranı ve HTTPS iletişim adresi doğrulaması eklendi.
- P6 manifest/hash doğrulamalı ZIP, SQLite online backup, staged restore, global storage gate, süreli token ve ayar ekranı eklendi. Tam kabul/test matrisi hâlâ açık.
- `npm run check` geçti: strict TypeScript, lint, 4 UI testi ve production web build.
- `cargo test --locked --manifest-path src-tauri/Cargo.toml`: 19 Rust testi geçti.
- `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`, fmt check ve git diff --check geçti.
- ZIP 9.2.1 crates.io üzerinden doğrulandı; yalnız deflate desteği açıldı. rustix 1.1.6 boş disk kontrolü için mevcut kilitli sürümden kullanıldı.
- Native test sandbox dışında geçti: iki bağımsız süreçte 2 + 2 test; kitap kalıcılığı ve günlük autosave kurtarma. İlk sandbox WebDriver timeout'u tekrar koşusunda giderildi. Kapsam TEST-REPORT.md içinde.
- Yeni sohbetin kesin başlangıç noktası HANDOFF.md içinde: P1 native kapak + orijinal dosyayı taşıma + yeniden açılış.
