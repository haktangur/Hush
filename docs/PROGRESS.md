# Hush ilerleme

## Şu an
- Aktif aşama: P7 — otomatik kalite, paketleme ve yerel teslim tamam; fiziksel VoiceOver/klavye/DMG kabulü ve GitHub bağlantısı açık.
- Son çalışan commit: bu oturumun P7 commit'i (`feat/local-library`); önceki `ecead2a`.
- Sonraki somut iş: kullanıcı manuel kabul listesini (TEST-REPORT → Manuel kabul) çalıştırıp sonucu bildirsin; GitHub için kullanıcı yeni açık yetki verirse private depo + push + CI.
- Ayrıntılı devir: [HANDOFF.md](HANDOFF.md). Kanıt/yöntem/sınırlar: [TEST-REPORT.md](TEST-REPORT.md).

## Aşamalar
- [x] P0 — masaüstü iskeleti (GitHub bağlantı engeli belgeli)
- [x] P1 — kalıcı veri
- [x] P2 — oda
- [x] P3 — koleksiyonlar
- [x] P4 — günlük/takvim/arama
- [x] P5 — kataloglar (canlı sağlayıcı sınırları raporda)
- [x] P6 — yedek/kurtarma
- [ ] P7 — test/GitHub/teslim (otomatik kontroller, paket ve yerel commit tamam; manuel VoiceOver/DMG kabulü ve GitHub açık)

## Doğrulananlar
- 2026-11-01: AGENTS.md, CODEX_BASLANGIC.md, README, docs/01–08 ve contracts okundu.
- macOS 26.6.2 / arm64; Node 24.18.0, npm 11.16.0, rustc 1.97.1, cargo 1.97.1; Xcode Command Line Tools mevcut.
- Tauri CLI kararlı sürümü npm üzerinden 2.12.4 olarak doğrulandı.

## Engeller
- (Çözüldü 2026-11-08) GitHub: kullanıcı public `https://github.com/haktangur/Hush` deposunu verdi; `main` ve `feat/local-library` push edildi. Commit e-postaları kullanıcının isteğiyle GitHub noreply adresine çevrildi (D24).

## Karar değişiklikleri
- D15: faz sonunda durma, GitHub CLI indirmesini yinelememe ve Finder/System Events otomasyon iznini yeniden istememe; kullanıcı talimatları için docs/08 karar kaydına bak.

### P0 doğrulama notları
- `npm run check`: TypeScript, ESLint, 1 UI testi, Vite production build geçti.
- `cargo test --locked`: 1 bootstrap envelope testi geçti.
- `cargo clippy --locked --all-targets -- -D warnings`: geçti.
- `npm audit`: minimatch kaynaklı 12 moderate geliştirme aracı bulgusu; uygulamanın runtime bağımlılıkları etkilenmiyor. Çözüm incelemesi açık.
- P1 domain/storage/repository dosyaları hazırlanıyor; henüz runtime'a bağlanmadı, test edilmiş sayılmaz.

### P1 — 2026-11-01
- Son temel commit: `b0629ff`; aktif dal `feat/local-library`.
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
- Başlangıç ağacı temizdi (`d616b6a`); mevcut P2–P6 hazırlıkları korundu. P2 çalışmasına geçilmedi.
- Native görsel seçici ana Hush penceresine bağlandı ve pencere seçim öncesinde görünür/odaklı hale getirildi. Kullanıcının ilk denemede göremediği seçici düzeltmeden sonra kullanıldı.
- Test-only asset scope yalnız aktif generation/assets dizinine açıldı; production kapsamı ve migration değişmedi.
- 04:44–04:45 TRT: kullanıcı gerçek native seçiciden sentetik PNG seçti; WDIO gerçek IPC ile kitabı kaydetti, orijinali taşıdı ve iki bağımsız süreçte 2 + 2 test geçti. Yeniden açılışta kapak decode/görünürlük, kitap/not/37/200, kimlikler ve image/thumb SHA-256 eşitliği doğrulandı. IPC mock yok.
- `npm run check` (4 UI), `cargo test --locked` (19 Rust), clippy, fmt ve diff kontrolü geçti. P1 sayfa/hata metni/migration/kind ölçütleri bu testlerle doğrulandı.
- Tekrar: `npm run test:native:p1` (native seçici adımı kullanıcı tarafından, diğer adımlar otomatik). `npm run test:native` etkileşimsiz temel smoke olarak korundu.
- Kanıt: `artifacts/native-p1-reopened.png`; ayrıntılı geçici veri yolu ve hashler TEST-REPORT içinde. Uygulama çıktısı test-only `src-tauri/target/native/debug/hush`; final release değildir.
- GitHub/CLI indirme denenmedi. Finder/System Events izni reddedilince AppleScript otomasyonu kaldırıldı; yeniden izin istenmedi.

### 2026-11-02 — P2 kabulü kapatıldı
- Başlangıç ağacı temizdi (`2bd3b41`). Mevcut P3–P6 hazırlıkları ve migration'lar korundu; yeni faz geliştirmesine geçilmedi.
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
- Başlangıç ağacı temizdi (`8b86373`); kullanıcı değişikliği yoktu. Mevcut P4–P6 hazırlıkları, bağımlılıklar ve migration'lar korundu. P4'e geçilmedi.
- Elle aynı başlık uyarısı, mevcut kaydı aç / ayrı kayda devam et; kompakt liste; albüm HTTPS bağlantı düğmeleri ve Rust sistem açıcı; screenshot uygulama içi büyütme tamamlandı.
- Eser mutation'larında item/room/list/collection/calendar cache yenileniyor. Silinmiş eser sorgusu eski kapak verisini taşımıyor; restore kapağı geri getiriyor. Sıralama ilk sayfaya dönüyor, virgülle çoklu etiket filtresi çalışıyor.
- 14:19 TRT: `npm run test:native -- --collections` gerçek macOS Tauri/WebKit/SQLite ile ilk süreçte 5 test; ikinci bağımsız süreçte 1 restart testi geçti (4 ilk-süreç testi skipped).
- İkişer oyun/albüm/kitap UI CRUD + trash/restore, seans CRUD, alıntı/parça düzenleme ve silme, screenshot ilişki/görüntüleme/düzenleme/silme, tür/revision hataları, koleksiyon üyeliği/silme, duplicate UX, filtre/sıralama/60+1 sayfalama doğrulandı. Üç türde ek purge kayıtlarıyla alt kayıt ve slot/üyelik FK temizliği doğrulandı.
- Restart sonrası altı tam eser DTO'su, tüm alt kayıt listeleri, üç kalan koleksiyon ve poster slotları birebir eşit; kapaklar decode oluyor. Toplam 68 aktif sentetik eser; üç purge kaydı yok.
- Native veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-keOHNx`; evidence ve screenshot'lar Git dışında. Screenshot/kapak görsel fixture'ı yalnız ayrı DB'ye kuruldu; eser ve alt kayıt işlemleri gerçek IPC, mock yok. Native picker P1'deki önceki kabul kapsamıyla sınırlı, P3'te tekrar edilmedi.
- `npm run check` (4 UI), 20 Rust testi, clippy, fmt ve diff kontrolü geçti. P2 regresyonu 3 + 1 geçti (2 skipped); temel native smoke 2 + 2 geçti (14:21 TRT); ayrıntı TEST-REPORT'ta.
- GitHub CLI/Finder/System Events izni yeniden istenmedi. Yeni final `.app/.dmg`, GitHub URL/CI/draft release yok. P4 başlangıç metni `docs/NEXT_CHAT_PROMPT.md` içinde.

### 2026-11-03 — P4 kabulü kapatıldı
- Başlangıç ağacı temizdi (`925eb4b`). P0–P3 ve mevcut P5–P6 hazırlıkları korundu; P5 çalışmasına geçilmedi.
- Takvim saat dilimi seçimi ve timezone içeren query key; günlük/arama/gün boş durumları ve filtre temizleme tamamlandı. Genel arama günlük sonucunu UUID filtresiyle açar; aynı başlıkta yanlış kayıt göstermez. Günlük paneli kapanınca arama seçimi temizlenir.
- Gerçek macOS Tauri/WebKit/SQLite: ilk süreç 4 test, ayrı restart süreci 1 test geçti. Sürece özgü 1 + 4 test bilinçli skipped. İki günlük UI oluştur/düzenle/trash/restore; altı mood ve boş değer; eser bağları; gerçek conflict'te metin koruma; autosave/restart/kurtarma doğrulandı.
- İstanbul/Los Angeles/UTC için Ekim–Kasım–Aralık 2026 gün/ay sınırları, DST tekrar saati, günlük gününün sabitliği, üç türün süreleri, eklenen eser ve günlüklü gün sayıları; seans silme ve trash ebeveyninin toplamdan çıkması geçti.
- Cmd+K, I/ı ve İ/i çiftlerinin ayrımı, `%_` literal, üretici/not/etiket/günlük gövdesi, boş sonuç ve doğru detaya geçiş doğrulandı. Rust HTTP gönderim sayacı ve WebView HTTP resource listesi sıfır; kaynak incelemesi tek reqwest gönderim noktasını doğruladı. Ağ denetimi yalnız e2e feature'da, içerik/sır kaydetmez; mock yok.
- P4 tekrar: `npm run test:native -- --journal` / `npm run test:native:p4`. Ayrıntılı veri yolları, koşu ve sınırlamalar TEST-REPORT'ta.
- `npm run check` (4 UI), 20 Rust testi, clippy, fmt ve diff kontrolü geçti. Migration/bağımlılık/lockfile değişmedi. Test verisi ve screenshot'lar Git dışında.
- GitHub CLI/Finder/System Events izni yeniden istenmedi. Yeni final `.app/.dmg` veya GitHub/CI sonucu yok; test binary `src-tauri/target/native/debug/hush`. P5 promptu NEXT_CHAT_PROMPT.md'de.

- P4 sonrası temel native regresyonu 2026-11-03 01:25 TRT’de 2 + 2 geçti; ayrı `hush-native-2L3LZH` dizini. P4 son kabul verisi `hush-native-O7nPqz` (01:23–01:24 TRT).

### 2026-11-03 — P5 katalog kabulü
- Ortak provider kuyruğu, 429/503/header ve body timeout için en fazla iki retry, jitter, HTTP-date/saniye Retry-After ve sonraki istekleri de kapsayan cooldown tamamlandı. Uzun cooldown arayüzü bekletmeden saniye bilgisiyle döner. 400/401/403/404/500 tekrar edilmez; IGDB 401 token yenilemesi ayrı ve bir defalıktır.
- OL/MB/IGDB arama ve detay cache'i; kapak cache'i ve 1 saatlik negatif kapak cache'i; 100 MiB LRU ve dosya sayısı sınırı. Test cache ve Keychain namespace'i üretimden ayrıldı. Bağımlılık/lockfile/migration değişmedi.
- Taze metadata farkı, eski/yeni alan ve kapak önizlemesi, seçili alanları revision/epoch token'ıyla atomik uygulama. Puan/not/durum/ilerleme/tarih/etiket/favori ve alt kayıtlar korunur. Aynı baskı mevcut kayda yönlendirilir; SQL uniqueness son savunma olarak kalır.
- Native sentetik P5: 4 + 1 passed; ilk süreçte canlı/restart 2 skipped, ikinci süreçte 5 skipped. Son veri `hush-native-I2JwFM`. Albüm kapağı gerçek decode/import ve offline görünürlükten geçti. IPC mock yok.
- Ayrı canlı OL koşusu: `hush-native-4nR1RX`, 4 + 1 passed (1 + 4 skipped); cache kullanmadan 20 sonuç, baskı/kapak/önizleme/kayıt, UI seçili başlık refresh ve offline restart. Bu canlı kanıt son duplicate UI ekinden önce alındı; duplicate son fixture koşusunda ayrıca geçti.
- Gerçek macOS Keychain'e benzersiz sentetik değer yaz/oku/sil geçti. Gerçek IGDB bağlantısı ve Keychain OS izin reddi senaryosu denenmedi; düz metin fallback yok. Canlı MB/CAA için adres verilmedi, uydurulmadı.
- P6 promptu NEXT_CHAT_PROMPT.md'de; P6 kodlamasına geçilmedi. Güncel test sonuçları TEST-REPORT'ta.

- P5 son doğrulama: `npm run check` (4 UI), 28 Rust testi, clippy/fmt/diff kontrolü geçti. Final paket/GitHub/CI/notarization hâlâ P7 sınırıdır.

### 2026-11-03 — P6 yedek/kurtarma kabulü
- Başlangıç ağacı `b853be4` üzerinde temizdi. Bağımlılık/lockfile/migration değiştirilmedi; üretim verisine dokunulmadı.
- Gerçek Tauri/WebKit/Rust/SQLite ile altı eser (her türde aktif ve trash), tüm alt kayıtlar, provenance, etiketler, koleksiyon, iki günlük, taslak, üç slot ve ayarlar karşılaştırıldı. 19 tablonun tüm satırları ve beş asset'in image/thumb hash'leri birebir eşit.
- UI export, restore özeti, iptal, açık değiştirme onayı, cache temizliği ve oda kapakları geçti. Dosya seçim sonucu yalnız e2e derlemesinde izole fixture adıyla sağlanır; native picker yeniden test edilmedi. IPC mock yok.
- 19 bozuk arşiv reddi; aktif pointer ve tüm kayıt/görseller değişmedi. Duplicate ZIP adı, eksik manifest dosyasının hata kodu ve ayar/slot doğrulaması düzeltildi.
- Generation rename öncesi, rename sonrası/pointer öncesi ve pointer sonrası gerçek SIGKILL; üç bağımsız yeniden açılışta doğru tam veri. Eski generation korunur. Disk-dolu üç noktada kontrollü StorageFull enjeksiyonudur; disk fiziksel olarak doldurulmadı.
- Önizleme yeni seçim/iptal/hata ile iptal edilir; tek kullanımlık/süreli token kontrolü, stage yeniden doğrulaması ve dosya/dizin sync korunur. Restore UI geçmiş günlük seçimini de temizler.
- Sentetik Keychain sırrı arşivde yok; restore değeri değiştirmedi; test sonunda silindi. Gerçek kullanıcı sırrı okunmadı.
- `npm run check` (4 React), 31 Rust testi; fmt/clippy/diff ve temel native regresyon sonuçları TEST-REPORT'ta.
- Yeni final `.app/.dmg` yok. GitHub remote/URL/CI yok; reddedilen GitHub CLI/Finder/System Events izinleri yeniden istenmedi. P7 promptu NEXT_CHAT_PROMPT.md içinde.

- P6 son temel native regresyonu 23:24 TRT: 2 + 2 geçti; ayrı `hush-native-hMaDU7`. Clippy `--all-targets -- -D warnings`, fmt ve diff kontrolü geçti.

### 2026-11-04 — P7 kalite ve yerel teslim çalışması (devir durumu)
- P7 için native yaşam döngüsü eklendi: pencere kapatma/Cmd+W pencereyi gizler; Cmd+Q, açık diyalog varsa formu koruyup kullanıcıyı Kaydet/Vazgeç akışına yönlendirir. Uygulama yeniden açıldığında pencereyi öne getirir. `app_finish_close` yalnız ana pencere capability'sindedir.
- İlk açılışta atlanabilir Türkçe gizlilik açıklaması, Türkçe native Hush/Düzen/Pencere menüsü, Hakkında metni ve Cmd+N bağlam kısayolu eklendi. Cmd+N odada yeni kitap, açık koleksiyon/günlükte bağlama uygun yeni kayıt açar; açık formu terk etmez.
- `npm run test:native -- --quality`: gerçek Tauri pencere close olayı sırasında kaydedilmemiş kitap başlığının korunduğunu, Cmd+S ile kaydedildiğini ve bağımsız ikinci süreçte kaydın bulunduğunu doğruladı. Native menüdeki hazır macOS Quit öğesinin doğrudan `terminate:` çağrısı yaptığı manuel testte keşfedildi; özel Cmd+Q/Cmd+W eylemleriyle düzeltildi. Bu düzeltmeden sonra otomatik lifecycle birim testleri eklendi; kullanıcı yeniden manuel onay vermedi.
- `npm run test:native -- --performance`: ayrı geçici generation'da 5.000 eser ve 10.000 seansla gerçek IPC/Rust/SQLite/WebKit koştu. Son ölçüm: yerel arama p95 3 ms, panel ilk çizim p95 33 ms, kare aralığı p95 18 ms; minimize edilince sahne animasyonları durdu. Release-optimizasyonlu e2e binary kullanıldı; production paketin soğuk açılış ölçümü değildir.
- P3–P6 regresyon paketi `npm run test:native -- --regressions` geçti: temel smoke 2+2, collections 5+1, journal 4+1, catalog 4+1 ve P6 tam restore/SIGKILL matrisi. P6 kasıtlı SIGKILL süreçlerindeki WebDriver bağlantı hataları beklenen sonuçtur; üç bağımsız verify süreçleri geçti.
- `npm run build:mac` Apple Silicon ad-hoc imzalı `Hush.app`, DMG ve app ZIP üretti. `codesign --verify --deep --strict`, `hdiutil verify`, DMG'yi salt-okunur bağlayıp içerideki app'in imza kontrolü ve binary SHA-256 eşitliği geçti. `npm run verify:release` arm64/ad-hoc, üretim bağımlılık grafiği ve bilinen test işaretleri/sentetik veri/sır taramasını geçti.
- Yerel CI (`.github/workflows/ci.yml`) ve yalnız manuel tetiklenen draft-release workflow'u hazırlandı; remote yok olduğu için GitHub CI/release hiç çalıştırılmadı. GitHub CLI indirme, Finder/System Events izni yeniden istenmedi.
- Kullanıcı manuel kontrolde ilk iki adıma kadar ilerledi; Cmd+Q uygulamayı kapattı ve tekrar açamadı. Bu hata yukarıdaki özel native menü eylemleriyle düzeltildi. Düzeltme sonrası aynı sentetik test veri dizininde yeni pencere açıldı, ancak kullanıcıdan tekrar manuel sonuç gelmedi. VoiceOver ayrıntılı kabulü, fiziksel Cmd+W/Cmd+Q/Dock yeniden açma ve menü kontrolü bu nedenle **açık** kalır.

### 2026-11-07 — P7 son doğrulama (Claude devri)
- Devir belgeleri okundu, P7 değişiklikleri korunarak gözden geçirildi. P0–P6 mimarisinde değişiklik gerekmedi.
- Bulunan sorun: devir notu `npm run check` geçti diyordu, ama lifecycle Vitest testi başarısızdı. Test yeniden yazıldı (7 Vitest geçti).
- Bulunan sorun: açık her panel Cmd+Q/Cmd+W'yi engelliyordu. D22 ile koruma yalnız kaydedilmemiş form/işlem kaydına (`src/app/unsavedWork.ts`) bağlandı; altı form kayda girer; yalnız en üst diyalog uyarıyı okur; Rust çıkış onayı tipli `QuitApproved` state'inde.
- `npm run test:native -- --quality` gerçek native close ile kirli formu korudu, temiz panelde pencereyi gizleyip geri getirdi; 1+1 geçti.
- Performans: arama p95 3 ms, panel ilk çizim p95 32 ms, kare p95 18 ms, minimize duraklama (e2e release profili).
- `--regressions` artık oda ve kalite paketlerini de çalıştırır; smoke/oda/koleksiyon/günlük/katalog/yedek-SIGKILL/kalite hepsi geçti.
- `cargo test` 31 (sandbox dışında), fmt, clippy, diff kontrolü geçti.
- `npm run build:mac` + `npm run verify:release` geçti; DMG içindeki app imzası ve SHA eşitliği doğrulandı. Release denetimine e2e capability/identifier işaretleri eklendi; DMG staging kopyası artık silinir.
- Manuel VoiceOver/klavye/Dock/DMG kabulü kullanıcıdan bekleniyor. GitHub CLI/Finder/System Events izni yeniden istenmedi; remote/CI/draft release yok.

### 2026-11-07 — Arayüz düzeni (kullanıcı geri bildirimi)
- Kullanıcı arayüzü özensiz buldu (kutu boyutları farklı, boşluklar tutarsız). Ortak tasarım ölçüsü uygulandı: tüm kontroller 40 px (`--control-h`), select/metin/tarih/sayı alanları aynı yükseklik, sayı okları kaldırıldı, select oku CSS ile çizildi, onay kutusu/kaydırıcı tema rengi, tek odak halkası; panel/form/kart boşlukları 4·8·12·16·20·24·28 ölçeğinde.
- Filtreler eşit sütunlu ızgara; kart ızgarası sabit genişlik; detayda durum rozeti ve etiket çipleri; formlarda kapak önizlemesi ve yapışkan Kaydet alt çubuğu; arama sonuçları satır listesi; poster seçici ızgarası; Ayarlar'da yapışkan Kaydet; alt gezinme düğmeleri eşit genişlik; sahne kenarları yumuşatıldı.
- Yeni `npm run test:native -- --visual`: sentetik kayıtları gerçek IPC ile oluşturup 19 ekranı 1440×900 ve 1100×720'de `artifacts/ui/` altına çeker, taşma raporu yazar (taşma yok). Screenshot'lar incelendi.
- `npm run check` (7), `--regressions` (tümü geçti), `build:mac` + `verify:release` geçti.

### 2026-11-08 — Şartname boşlukları ve arayüz pürüzleri
- Kullanıcı manuel testlerini sona bıraktı. Şartname taraması: boş durum ipuçları (docs/01), panel giriş hareketi (docs/03), Cmd+S'in tüm formlarda çalışması (docs/03) eksikti; tamamlandı. Boş tarih alanlarının bugünün tarihini göstermesi giderildi.
- Görsel tura yeni assert'ler eklendi; tam regresyon 17 süreç geçti; `build:mac` + `verify:release` geçti.
- `npm audit --omit=dev` temiz; geliştirme araçlarındaki bulgular release'e girmez, major yükseltme gerektirir (yükseltilmedi).
- Kalan: kullanıcı destekli manuel kabul (VoiceOver/klavye/Dock/DMG) ve GitHub (yeni açık yetki gerekli).
