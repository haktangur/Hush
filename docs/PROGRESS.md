# Hush ilerleme

## Şu an
- Kullanıcının son yönlendirmesi: yalnız P4 kabulünü tamamla, belge/yerel commit ve P5 başlangıç promptu sonrası dur.
- P0–P4 tamamlandı. Sonraki sohbetin hedefi yalnız P5 otomatik katalog kabulü.
- P5–P6 kod hazırlıkları korunuyor; bu fazların kabul kapıları tamamlanmış sayılmıyor.
- Ayrıntılı devir: [HANDOFF.md](HANDOFF.md). Test kanıtları: [TEST-REPORT.md](TEST-REPORT.md).
- Dal: `feat/local-library`; güncel checkpoint için `git log -1`.

## Aşamalar
- [x] P0 — masaüstü iskeleti (GitHub bağlantı engeli belgeli)
- [x] P1 — kalıcı veri
- [x] P2 — oda
- [x] P3 — koleksiyonlar
- [x] P4 — günlük/takvim/arama
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

### 2026-11-02 — P2 kabulü kapatıldı
- Başlangıç ağacı temizdi (`84e989c`). Mevcut P3–P6 hazırlıkları ve migration'lar korundu; yeni faz geliştirmesine geçilmedi.
- Posterler ayarlar menüsüne eklendi; atanmış eser panelden açılabiliyor. Cmd+7/8 posterler, Cmd+9 plak ataması, Cmd+L lamba; mevcut Cmd+2–6 panel yollarıyla aynı state kullanılıyor.
- Ortak Dialog bileşeni tetikleyen düğmeye odağı geri veriyor; panelden başka panele geçişte oda düğmesi yedek hedef olarak korunuyor.
- Dekoratif plak dönüşü, panel/minimize/blur durumunda duraklama ve azaltılmış harekette sabit görünüm tamamlandı. Ölçekli hotspot etiketleri minimum 12 CSS px.
- 04:57 TRT: gerçek macOS Tauri/WebKit P2 testi ilk süreçte 3, yeniden açılışta 1 test geçti (ilk sürece özgü 2 test ikinci süreçte skipped). Sekiz hotspot 1440×900 ve 1100×720, menü/kısayol, odak/Escape, hata formu, üç palet, azaltılmış hareket ve minimize doğrulandı.
- İki farklı poster ve plak kapağı ile Quiet Dawn/lamba kapalı/yağmur kapalı/neon 27/azaltılmış hareket/etiketler kapalı restart sonrası aynı. Görseller yerel asset protokolünde decode edildi. Sentetik fixture asset'leri ayrı geçici DB'ye kuruldu; eser/ayar/slot işlemleri gerçek IPC. Mock yok.
- Boş/dolu oda, iki boyut, üç palet, hata formu ve kitap paneli screenshot'ları görsel olarak incelendi; `artifacts/p2-*.png` Git dışında.
- `npm run check` (4 UI), 19 Rust testi, clippy/fmt ve diff kontrolü geçti. Ortak Dialog değişikliği sonrası mevcut native kitap/günlük restart regresyonu 2 + 2 geçti.
- Yeniden çalıştırma: `npm run test:native:p2` veya mevcut izinli `npm run test:native -- --room`. Veri ve kanıt yolu TEST-REPORT.md'de.
- VoiceOver, işletim sistemi hareket tercihini değiştirerek manuel kontrol ve final paket/CI kapsamı P7'de açık; bu P2 sonucu onları geçmiş saymaz. GitHub CLI/Finder/System Events izni yeniden istenmedi.

### 2026-11-02 — P3 kabulü kapatıldı
- Başlangıç ağacı temizdi (`5af7e13`); kullanıcı değişikliği yoktu. Mevcut P4–P6 hazırlıkları, bağımlılıklar ve migration'lar korundu. P4'e geçilmedi.
- Elle aynı başlık uyarısı, mevcut kaydı aç / ayrı kayda devam et; kompakt liste; albüm HTTPS bağlantı düğmeleri ve Rust sistem açıcı; screenshot uygulama içi büyütme tamamlandı.
- Eser mutation'larında item/room/list/collection/calendar cache yenileniyor. Silinmiş eser sorgusu eski kapak verisini taşımıyor; restore kapağı geri getiriyor. Sıralama ilk sayfaya dönüyor, virgülle çoklu etiket filtresi çalışıyor.
- 14:19 TRT: `npm run test:native -- --collections` gerçek macOS Tauri/WebKit/SQLite ile ilk süreçte 5 test; ikinci bağımsız süreçte 1 restart testi geçti (4 ilk-süreç testi skipped).
- İkişer oyun/albüm/kitap UI CRUD + trash/restore, seans CRUD, alıntı/parça düzenleme ve silme, screenshot ilişki/görüntüleme/düzenleme/silme, tür/revision hataları, koleksiyon üyeliği/silme, duplicate UX, filtre/sıralama/60+1 sayfalama doğrulandı. Üç türde ek purge kayıtlarıyla alt kayıt ve slot/üyelik FK temizliği doğrulandı.
- Restart sonrası altı tam eser DTO'su, tüm alt kayıt listeleri, üç kalan koleksiyon ve poster slotları birebir eşit; kapaklar decode oluyor. Toplam 68 aktif sentetik eser; üç purge kaydı yok.
- Native veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-keOHNx`; evidence ve screenshot'lar Git dışında. Screenshot/kapak görsel fixture'ı yalnız ayrı DB'ye kuruldu; eser ve alt kayıt işlemleri gerçek IPC, mock yok. Native picker P1'deki önceki kabul kapsamıyla sınırlı, P3'te tekrar edilmedi.
- `npm run check` (4 UI), 20 Rust testi, clippy, fmt ve diff kontrolü geçti. P2 regresyonu 3 + 1 geçti (2 skipped); temel native smoke 2 + 2 geçti (14:21 TRT); ayrıntı TEST-REPORT'ta.
- GitHub CLI/Finder/System Events izni yeniden istenmedi. Yeni final `.app/.dmg`, GitHub URL/CI/draft release yok. P4 başlangıç metni `docs/NEXT_CHAT_PROMPT.md` içinde.

### 2026-11-03 — P4 kabulü kapatıldı
- Başlangıç ağacı temizdi (`f1d2b73`). P0–P3 ve mevcut P5–P6 hazırlıkları korundu; P5 çalışmasına geçilmedi.
- Takvim saat dilimi seçimi ve timezone içeren query key; günlük/arama/gün boş durumları ve filtre temizleme tamamlandı. Genel arama günlük sonucunu UUID filtresiyle açar; aynı başlıkta yanlış kayıt göstermez. Günlük paneli kapanınca arama seçimi temizlenir.
- Gerçek macOS Tauri/WebKit/SQLite: ilk süreç 4 test, ayrı restart süreci 1 test geçti. Sürece özgü 1 + 4 test bilinçli skipped. İki günlük UI oluştur/düzenle/trash/restore; altı mood ve boş değer; eser bağları; gerçek conflict'te metin koruma; autosave/restart/kurtarma doğrulandı.
- İstanbul/Los Angeles/UTC için Ekim–Kasım–Aralık 2026 gün/ay sınırları, DST tekrar saati, günlük gününün sabitliği, üç türün süreleri, eklenen eser ve günlüklü gün sayıları; seans silme ve trash ebeveyninin toplamdan çıkması geçti.
- Cmd+K, I/ı ve İ/i çiftlerinin ayrımı, `%_` literal, üretici/not/etiket/günlük gövdesi, boş sonuç ve doğru detaya geçiş doğrulandı. Rust HTTP gönderim sayacı ve WebView HTTP resource listesi sıfır; kaynak incelemesi tek reqwest gönderim noktasını doğruladı. Ağ denetimi yalnız e2e feature'da, içerik/sır kaydetmez; mock yok.
- P4 tekrar: `npm run test:native -- --journal` / `npm run test:native:p4`. Ayrıntılı veri yolları, koşu ve sınırlamalar TEST-REPORT'ta.
- `npm run check` (4 UI), 20 Rust testi, clippy, fmt ve diff kontrolü geçti. Migration/bağımlılık/lockfile değişmedi. Test verisi ve screenshot'lar Git dışında.
- GitHub CLI/Finder/System Events izni yeniden istenmedi. Yeni final `.app/.dmg` veya GitHub/CI sonucu yok; test binary `src-tauri/target/native/debug/hush`. P5 promptu NEXT_CHAT_PROMPT.md'de.

- P4 sonrası temel native regresyonu 2026-11-03 01:25 TRT’de 2 + 2 geçti; ayrı `hush-native-2L3LZH` dizini. P4 son kabul verisi `hush-native-O7nPqz` (01:23–01:24 TRT).
