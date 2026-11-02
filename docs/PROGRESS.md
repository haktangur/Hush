# Hush ilerleme

## Şu an
- Kullanıcının son yönlendirmesi: yalnız P1 kabulünü tamamla, belge/yerel commit sonrası dur; P2 yeni sohbette.
- P0 ve P1 tamamlandı. Sonraki sohbetin hedefi yalnız P2 oda/gezinme kabulü.
- P2–P6 kod hazırlıkları korunuyor; kabul kapıları tamamlanmış sayılmıyor.
- Ayrıntılı devir: [HANDOFF.md](HANDOFF.md). Test kanıtları: [TEST-REPORT.md](TEST-REPORT.md).
- Dal: `feat/local-library`; güncel checkpoint için `git log -1`.

## Aşamalar
- [x] P0 — masaüstü iskeleti (GitHub bağlantı engeli belgeli)
- [x] P1 — kalıcı veri
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
- D15: faz sonunda durma, GitHub CLI indirmesini yinelememe ve Finder/System Events otomasyon iznini yeniden istememe; kullanıcı talimatları için docs/08 karar kaydına bak.

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

### 2026-11-02 — P1 kabulü kapatıldı
- Başlangıç ağacı temizdi (`fb29a9a`); mevcut P2–P6 hazırlıkları korundu. P2 çalışmasına geçilmedi.
- Native görsel seçici ana Hush penceresine bağlandı ve pencere seçim öncesinde görünür/odaklı hale getirildi. Kullanıcının ilk denemede göremediği seçici düzeltmeden sonra kullanıldı.
- Test-only asset scope yalnız aktif generation/assets dizinine açıldı; production kapsamı ve migration değişmedi.
- 04:44–04:45 TRT: kullanıcı gerçek native seçiciden sentetik PNG seçti; WDIO gerçek IPC ile kitabı kaydetti, orijinali taşıdı ve iki bağımsız süreçte 2 + 2 test geçti. Yeniden açılışta kapak decode/görünürlük, kitap/not/37/200, kimlikler ve image/thumb SHA-256 eşitliği doğrulandı. IPC mock yok.
- `npm run check` (4 UI), `cargo test --locked` (19 Rust), clippy, fmt ve diff kontrolü geçti. P1 sayfa/hata metni/migration/kind ölçütleri bu testlerle doğrulandı.
- Tekrar: `npm run test:native:p1` (native seçici adımı kullanıcı tarafından, diğer adımlar otomatik). `npm run test:native` etkileşimsiz temel smoke olarak korundu.
- Kanıt: `artifacts/native-p1-reopened.png`; ayrıntılı geçici veri yolu ve hashler TEST-REPORT içinde. Uygulama çıktısı test-only `src-tauri/target/native/debug/hush`; final release değildir.
- GitHub/CLI indirme denenmedi. Finder/System Events izni reddedilince AppleScript otomasyonu kaldırıldı; yeniden izin istenmedi.
