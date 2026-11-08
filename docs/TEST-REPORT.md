# Hush test raporu — P7 son doğrulama

2026-11-07 · macOS 26.6.2 · Apple Silicon arm64 · Node 24.18.0 · Rust 1.97.1 · WebKit 605.1.15.
Otomatik sonuçlar aşağıdadır. Fiziksel VoiceOver/klavye ve DMG'den gerçek açılış kullanıcı destekli manuel kabuldür; sonucu gelmeyen adım geçti sayılmaz.

## P7 son doğrulama — 2026-11-07 (Claude devri)

Devirde bulunan iki sorun düzeltildi (ayrıntı: docs/08 D22):

1. Devir notu `npm run check`'in 6 testle geçtiğini bildiriyordu; gerçekte `lifecycle.test.ts` başarısızdı (modül düzeyindeki menü önbelleği ikinci testte `Menu.new` çağrısını boş bırakıyordu). Test yeniden yazıldı.
2. Açık her panel Cmd+Q/Cmd+W'yi engelliyordu; salt okunur panel açıkken Cmd+Q hiçbir şey yapmıyordu. Koruma artık yalnız kaydedilmemiş form/işlem kaydına (`src/app/unsavedWork.ts`) bakar.

| Kontrol | Gerçek sonuç (2026-11-07) |
|---|---|
| `npm run check` | Geçti: strict TypeScript, ESLint, **7** Vitest testi (lifecycle 3), production Vite build. |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | Sandbox dışında **31** test geçti. Sandbox içinde loopback HTTP fixture'ı `OFFLINE` döndürdüğü için `http_status_retry_and_body_limits` başarısız oluyor; ortam kısıtıdır, kod hatası değildir. |
| `cargo fmt --check`, `cargo clippy --locked --all-targets -D warnings`, `git diff --check` | Geçti. |
| `npm run test:native -- --quality` | 1 + 1 geçti (`hush-native-j0uAKw`): gerçek native close olayı kirli kitap formunu korudu, uyarı göründü; Cmd+S kaydetti; temiz panel açıkken aynı close pencereyi gizledi, `show` geri getirdi ve panel duruyordu; ikinci süreç kaydı ve onboarding tamamlandı bilgisini gördü. |
| `npm run test:native -- --performance` | Geçti (`hush-native-2DACDh/performance.json`): 5.000 eser / 10.000 seans; yerel arama p95 **3 ms**, panel ilk çizim p95 **32 ms**, otomasyon dahil panel p95 34 ms, kare aralığı p95 **18 ms** (≈55 fps; 60 fps hedefinin p95'te biraz altında), küçültülünce sahne animasyonları `paused`; süreç RSS ≈142 MB. Release-optimizasyonlu **e2e** binary'dir; production soğuk açılış ölçümü değildir. |
| `npm run test:native -- --regressions` | Geçti (exit 0). Smoke 2+2; oda (P2) 3+1 (2 skipped); koleksiyonlar 5+1 (4 skipped); günlük/takvim/arama 4+1 (1+4 skipped); katalog fixture 4+1 (2+5 skipped); P6 round-trip + 19 bozuk arşiv + üç gerçek SIGKILL/verify (`hush-native-DXaYev`); kalite 1+1. Oda ve kalite paketleri bu turda regresyon listesine eklendi. |
| `npm run build:mac` | Geçti: arm64 `Hush.app`, `Hush-0.1.0-aarch64.dmg`, app ZIP, `SHA256SUMS.txt`; ad-hoc imza; `hdiutil verify` VALID. DMG staging kopyası artık build sonunda siliniyor. |
| DMG içeriği | Salt-okunur bağlandı: `Hush.app`, Applications bağlantısı, kurulum yönergesi, lisans envanteri. İçerideki app için `codesign --verify --deep --strict` geçti; çalıştırılabilir SHA-256 build app ile aynı; `LSMinimumSystemVersion` 14.0; identifier `app.hush.desktop`. |
| `npm run verify:release` | Geçti: arm64, ad-hoc, production dependency graph'ta WDIO yok; `HUSH_TEST_DATA_DIR`, e2e capability/identifier, backup selector/fault, mock ve sır desenleri yok. |

SHA-256 (son paket, 2026-11-08): DMG `8ffef62fe078e616ec1afb4d71d8d72e7f7270bd29cbaba378382c3fbf5f3620`, app ZIP `604469a7f1ba49571b5231bdc22fd260af02106f5f8a245f1954b69a21d0bc47`.

**Şartname boşlukları (2026-11-08):** docs/01 ilk açılış boş durum ipuçları ("İlk oyununu ekle", "Bir albüm bul", "Kitap ekle") ilgili eşyada; docs/03 panel giriş hareketi (200 ms fade + 8 px kayma, azaltılmış harekette yok); Cmd+S artık alt çubuklu her düzenleme formunu ve Ayarlar'ı kaydeder; boş tarih alanları WebKit'in "bugün" yer tutucusunu göstermez. Görsel tur bunları assert eder (üç ipucu boş kütüphanede var, kayıttan sonra yok; Cmd+S ile koleksiyon kaydı; tarih alanı boşluk işareti). İlk CSS keyframe denemesi, örtülü test penceresinde WebKit zaman çizelgesi durduğu için paneli opaklık 0'da bıraktı ve koleksiyon regresyonu yakaladı; giriş hareketi yalnız sayfa görünürken çalışan Web Animations çağrısına çevrildi. Ardından görsel tur ve tam `--regressions` (17 süreç) geçti.

**npm audit:** `npm audit --omit=dev` 0 bulgu (üretim bağımlılıkları temiz). Geliştirme ağacında 31 bulgu (17 orta, 14 yüksek) WDIO test araçları ve typescript-eslint zincirindedir; release paketinde yoktur (`verify:release`). Çözümleri major sürüm yükseltmesi gerektirdiği için AGENTS.md gereği görev sırasında yükseltilmedi.

**Hover hatası (kullanıcı bildirdi):** Ortak `button:hover` dolgusu oda eşyalarının şeffaf hover stilini eziyordu; eşyanın üstüne gelince opak gri kutu çiziliyordu. `.hotspot:hover:not(:disabled)` ile düzeltildi. Görsel tur artık bütün `:hover` kurallarını eşit özgüllükte bir sınıfa kopyalayıp sekiz eşyada arka plan saydamlığını (alfa < 0,1) ve çerçeveyi assert eder; gömülü WebKit sürücüsü gerçek `:hover` üretmediği için imleç hareketi tek başına kanıt sayılmaz.

**Test izolasyonu hatası:** Açık bir `--manual` test penceresi WebDriver portu 4445'i tuttuğunda WDIO yeni derlenen uygulama yerine o pencereyi sürüyordu; bu oturumda üç görsel tur koşusu kullanıcının sentetik test penceresine bağlandı (gerçek veri etkilenmedi). Bu koşular kanıt sayılmadı. `scripts/test-native.mjs` artık port doluysa çalışmayı reddeder. Ardından temiz koşuda görsel tur, `npm run check` (7) ve tam `--regressions` (17 süreç geçti) yeniden çalıştırıldı.

**Arayüz düzeni (D23):** `npm run test:native -- --visual` 19 ekranı 1440×900 ve 1100×720'de çekti, yatay taşma raporu boş; screenshot'lar elle incelendi. Düzen sonrası `npm run check` (7 test), tam `--regressions` (smoke, oda, koleksiyon, günlük, katalog, yedek/SIGKILL, kalite) ve `build:mac` + `verify:release` yeniden geçti.

### A01–A22 değerlendirmesi

Kanıt türü: **N** gerçek Tauri/WebKit/Rust/SQLite native otomasyon (sentetik geçici veri, IPC mock yok), **R** Rust testi, **V** Vitest, **K** kullanıcı destekli/manuel.

| ID | Durum | Kanıt |
|---|---|---|
| A01 | Kısmi | N: her koşu boş sentetik dizinde, offline işaretiyle açılır; elle kayıt çalışır. Production paketin temiz kurulumu K ile açık. |
| A02 | Kısmi | N (oda, 2026-11-07): sekiz hotspot, menü/kısayollar, odak dönüşü, Escape. VoiceOver seslendirmesi K ile açık. |
| A03 | Geçti | N: koleksiyon restart süreci altı tam eser DTO'su ve kapakları birebir buldu. |
| A04 | Geçti (2026-11-02) | N+K: native seçiciden kapak, orijinal taşındıktan sonra yeniden açılışta aynı SHA. Bu turda tekrarlanmadı. |
| A05 | Geçti | V + R + N: aşım/negatif sayfa reddi, not korunur. |
| A06 | Geçti | N (günlük): seans ekle/sil, üç saat dilimi, ay sınırları ve toplamlar. |
| A07 | Geçti | N (koleksiyon): parça/alıntı/screenshot ilişkisi restart sonrası aynı. |
| A08 | Geçti | N (smoke): günlük taslağı, süreç sonlandırma, kurtarma. |
| A09 | Geçti | N (koleksiyon): trash/restore/purge, slot/üyelik FK temizliği, poster fallback. |
| A10 | Geçti | N (günlük): I/ı/İ/i, `%_` literal; Rust HTTP sayacı ve WebView ağ listesi sıfır. |
| A11 | Kısmi | Canlı Open Library 2026-11-03 geçti (bu turda tekrarlanmadı). MusicBrainz/CAA yalnız sentetik cache; canlı iletişim adresi olmadığından canlı kabul yok. |
| A12 | Geçti | N (katalog): IGDB bağlı değilken elle oyun ekleme ve ayar yönlendirmesi. |
| A13 | Geçti | R: timeout/429/503/4xx/5xx, sınırlı retry, gövde limiti; N: bozuk kapak. Gerçek IGDB anahtar reddi denenmedi. |
| A14 | Geçti | N (katalog): refresh kişisel not/puan/ilerlemeyi korur. |
| A15 | Geçti | N (P6): 19 tablo ve asset hash eşdeğerliği; sentetik Keychain sırrı arşivde yok. |
| A16 | Geçti | N: 19 bozuk arşiv; disk dolması kontrollü StorageFull enjeksiyonu (fiziksel disk doldurulmadı). |
| A17 | Geçti | N: üç noktada gerçek SIGKILL + bağımsız verify. |
| A18 | Geçti | R + N: eski revision conflict, form metni korunur. |
| A19 | Kısmi | N: 1440×900 ve 1100×720 sığma, uygulama içi azaltılmış hareket, minimize/blur duraklama. VoiceOver ve macOS sistem hareket tercihi K ile açık. |
| A20 | Geçti (hedefe göre) | N: yukarıdaki ölçüm; arama ve panel hedefleri sağlandı, kare p95 18 ms. Production soğuk açılış ölçülmedi. |
| A21 | Kısmi | DMG doğrulama ve içerideki imza geçti. Kullanıcının gerçek Application Support verisine dokunmamak için production app'i ajan açmadı; DMG'den sürükle-aç K ile açık. |
| A22 | Geçti | `verify:release` + binary taraması: test eklentisi, e2e capability, mock, sır, kişisel veri yok. |

### Manuel kabul (kullanıcıdan bekleniyor)

Önceki manuel denemede Cmd+Q kirli formda uygulamayı kapatmıştı (başarısız). Özel menü eylemleri ve D22 kuralı sonrasında kullanıcıdan yeni sonuç gelmedi. Açık adımlar: VoiceOver'ın hotspot ve Kitaplar → Ekle alan adlarını okuması; kirli formda Cmd+Q/Cmd+W'nin durması ve uyarının okunması; Escape → Geri; temiz durumda Cmd+W → Dock'tan yeniden aç → Cmd+Q; Türkçe Hush/Düzen/Pencere menüleri; DMG'den kurulum ve açılış.

### Bilinen sınırlar

Dock menüsündeki "Çık" ve sistem kapatma `terminate:` ile çıkar; kaydedilmemiş form metnini korumaz (D22). Paket ad-hoc imzalıdır, notarize değildir. GitHub remote/URL, CI koşusu ve draft release yoktur; workflow dosyaları yalnız yerel hazırlıktır. Canlı MusicBrainz/CAA ve gerçek IGDB OAuth kabulü yapılmadı. npm geliştirme bağımlılığı audit bulgularının incelemesi açık.

## P7 güncellemesi — 2026-11-04 (Codex; geçmiş kayıt)

> Not (2026-11-07): Bu bölümdeki `npm run check` "6 test geçti" satırı doğrulanamadı; devir anında lifecycle testi başarısızdı. Güncel sonuçlar yukarıdadır.

| Kontrol | Gerçek sonuç |
|---|---|
| `npm run check` | Geçti: strict TypeScript, ESLint, **6** Vitest testi, production Vite build. P7 lifecycle/Cmd+N değişikliklerinden sonra çalıştırıldı. |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | P7 başlangıcında sandbox dışında **31** Rust testi geçti. P7 Rust lifecycle değişikliği sonrasında yeniden çalıştırılacak. |
| `npm run test:native -- --quality` | Geçti: gerçek native close request açık formdaki metni korudu; Cmd+S kaydetti; ikinci Tauri süreci kaydı gördü. |
| `npm run test:native -- --performance` | Geçti: release-optimizasyonlu e2e binary, ayrı sentetik generation, 5.000 eser / 10.000 seans. Arama p95 **3 ms**, panel ilk çizim p95 **33 ms**, frame p95 **18 ms**; minimize animasyonları durdurdu. Production cold-start ölçümü değildir. |
| `npm run test:native -- --regressions` | Geçti: smoke 2+2; collections 5+1; journal 4+1; catalog 4+1; P6 round-trip, 19 bozuk arşiv ve üç SIGKILL/verify sınırı. |
| `npm run build:mac` | Geçti: arm64 `Hush.app`, DMG, app ZIP ve SHA256SUMS üretildi; ad-hoc imza. |
| DMG doğrulaması | `hdiutil verify` geçti. Salt-okunur bağlanmış DMG'deki app için `codesign --verify --deep --strict` geçti; binary SHA-256 build app ile eşitti. Finder'dan görsel yükleme/sürükle-bırak iddiası değildir. |
| `npm run verify:release` | Geçti: arm64/ad-hoc, production dependency graphında WDIO yok; bilinen e2e marker, test fixture verisi ve sır deseni bulunmadı. |

### P7 manuel kabul durumu

Kullanıcı ayrı sentetik/offline Hush penceresinde ilk iki VoiceOver/klavye adımını denediğini bildirdi; VoiceOver'ın kontrol ve alan adlarını okumasına ilişkin ayrıntılı sonuç paylaşılmadı. Üçüncü adımda, kaydedilmemiş form açıkken Cmd+Q uygulamayı doğrudan kapattı ve kullanıcı yeniden açamadı. Bu **başarısız sonuç** olarak kaydedildi.

Neden, macOS predefined Quit menü öğesinin sistem `terminate:` yolunu çağırmasıydı. Native menu, açık diyalogda `hush-close-blocked` bildiren ve boş ekranda yalnız `app_finish_close` komutunu çağıran özel Cmd+Q/Cmd+W eylemleriyle değiştirildi. Vitest lifecycle testleri açık diyaloğun korunmasını ve native kapanışın ancak diyalog kapanınca çağrılmasını doğrular. Aynı test root'u korunarak yeni pencere açıldı; kullanıcıdan düzeltme sonrası yeni manuel sonuç gelmedi.

Bu nedenle aşağıdakiler **açık kabul** olarak kalır: fiziksel Cmd+Q/Cmd+W, Dock'tan yeniden açma, Escape → Geri, Türkçe Hush/Düzen/Pencere menu kontrolleri ve VoiceOver'ın hotspot/form alan adlarını seslendirmesi. Otomatik WDIO klavye olayları bunların yerine geçmez.

### P7 paket ve üretim sınırı

Paketler Git dışı hedef dizinindedir:

- `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`
- `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/Hush-0.1.0-aarch64.dmg`
- `src-tauri/target/aarch64-apple-darwin/release/bundle/Hush-0.1.0-aarch64.app.zip`

Paket ad-hoc imzalıdır; Apple notarization yapılmadı. GitHub remote/URL, CI run sonucu ve draft release yoktur. Yerel CI/release workflow dosyaları yalnız hazırlıktır. Canlı MusicBrainz/CAA ve gerçek IGDB OAuth kabulü yapılmadı. Son P7 kaynak değişikliklerinden sonra final build ve denetim tekrar çalıştırılmalıdır.

## Son geçen kontroller

| Kontrol | Sonuç ve kapsam |
|---|---|
| `npm run check` | Geçti: TypeScript strict, ESLint, 4 React testi, Vite production build |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | 31 test geçti; lib/main/doc test koşuları hatasız |
| `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` | Geçti |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | Geçti |
| `git diff --check` | Geçti |
| `npm run test:native -- --backup` | 2026-11-03: 5 başarılı süreç, 3 kasıtlı SIGKILL + bağımsız verify; runner exit 0 |
| `npm run test:native -- --catalog` | 2026-11-03: 4 + 1 geçti (2 + 5 skipped); canlı OL ayrı koşuda doğrulandı |
| `npm run test:native` (sandbox dışında) | 2026-11-03 23:24 TRT: iki bağımsız süreçte 2 + 2 temel regresyon geçti |
| `npm run test:native -- --journal` | 2026-11-03 01:23–01:24 TRT: 4 + 1 geçti (1 + 4 skipped) |
| `npm run test:native -- --collections` | 14:19 TRT: P3 ilk süreç 5, restart 1 geçti (4 skipped) |
| `npm run test:native -- --room` | 14:19 TRT: P2 regresyon 3 + 1 geçti (2 skipped) |

## Otomatik test kapsamı

React: bootstrap hatasının başarı gibi gösterilmemesi, kaydetme hatasında kişisel notun korunması, sayfa aşımının IPC öncesi reddi, dirty formdan geri dönüş.

Rust: DTO, kitap validasyonu/kind değişmezliği, revision/aynı UUID retry/trash, FK hatasında atomiklik, Türkçe ve literal LIKE araması, migration sözleşmesi/ileri schema/missing pointer, görsel doğrulama ve orijinal dosyadan bağımsız kapak kopyası, alt kayıt revision/kind ve koleksiyon silmenin eseri koruması, günlük/draft/reopen/takvim, provider temel mapping/URL, iletişim URL doğrulaması, backup yol kısıtı/bozuk arşivin aktif veriyi koruması/draft round-trip ve eski generation'ın korunması.

P6 tam eşdeğerlik, bozuk arşiv ve SIGKILL kabulü bu raporun son bölümündedir; önceki üç temel testin kapsamı genişletildi.

## Native son koşu — 2026-11-02 04:34 TRT

Mock kullanılmadı. Sentetik geçici veri dizini ve ayrı `src-tauri/target/native` derleme hedefi kullanıldı.

1. Gerçek UI ile sentetik kitap kaydedildi; not ve 37/200 okuma ilerlemesi IPC'den doğrulandı.
2. Günlük taslağı yazıldı; autosave bildirimi beklendi.
3. Uygulama süreci sonlandırılıp aynı test diziniyle yeni süreç açıldı. Kitap verisi korundu.
4. Taslak kurtarıldı, kaydedildi; kalıcı günlük oluştu ve geçici draft temizlendi.
5. İlk süreçte 1440×900 ve 1100×720 boyutlarında sahnenin alana sığması assert edildi ve screenshot alındı. Bu, VoiceOver veya bütün hotspot/focus kabulü değildir.

Sandbox içindeki ilk deneme WebDriver port 4445 hazır olma timeout'u verdi. Sandbox dışındaki tekrar iki süreçte de başarılı oldu; kalıcı ürün hatası olarak bırakılmadı.

## Artifact ve görsel kanıtlar

- `artifacts/room-1440.png`, `artifacts/room-1100.png`, `artifacts/native-p1.png`: sentetik, yerel ve Git dışında.
- P0 Apple Silicon ad-hoc `.app` önceki koşuda üretildi, `open` başarılı oldu ve çalışan uygulama listesinde görüldü.
- P0 app yolu: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`. Güncel özelliklerin final çıktısı değildir.
- ScreenCaptureKit -3811 nedeniyle CUA görüntüleme başarısızdı; WDIO screenshot yöntemi çalıştı. Manuel VoiceOver testi yapılmış sayılmıyor.

## Açık kabul koşulları

P0–P6 kapıları tamamlandı. P7'de iki pencere boyutu, oda/hotspot/focus/reduced-motion regresyonu önceki P2 native koşusunda; 5.000 eser/10.000 seans ve production binary test bileşeni denetimi yukarıdaki P7 koşularında doğrulandı. A01–A22'nin otomatik kanıtı tamamlayıcıdır; fiziksel VoiceOver/klavye ve düzeltme sonrası Cmd+Q/Cmd+W/Dock kabulü açık kaldığından tam matris kapatılmamıştır.

Final `.dmg` üretildi ve içeriğindeki uygulamanın imzası doğrulandı; notarization yapılmadı. GitHub remote/CI sonucu/draft release yoktur. GitHub CLI indirme izni kullanıcı tarafından reddedildi. Canlı MB/CAA ile gerçek IGDB OAuth test edilmedi. npm geliştirme bağımlılıklarına ait önceki audit bulgularının çözüm incelemesi açık; temiz audit iddiası yoktur.

## P1 birleşik native kabul — 2026-11-02 04:44–04:45 TRT

**Geçti.** Gerçek Tauri debug uygulaması ve WebKit kullanıldı; IPC, picker veya depolama mock edilmedi. Dosya seçimi kullanıcı tarafından yapıldığı için bu koşu **kullanıcı destekli native testtir**, tamamen otomatik olarak raporlanmaz.

- UI’dan “Native sentetik kitap”, 37/200 ve “Yeniden açılışta korunmalı.” notu girildi.
- Kullanıcı gerçek macOS seçicisinden sentetik `source.png` dosyasını seçti (projedeki özgün Hush ikonu kopyası; kişisel görsel değil).
- Rust kapak importu ve UI kaydı tamamlandı; WebView’de img görünürlüğü, complete ve naturalWidth/naturalHeight > 0 doğrulandı.
- Orijinal PNG `source.png.moved` olarak taşındı; eski yolun yokluğu assert edildi.
- Birinci uygulama süreci kapandı; aynı izole dizinle ikinci süreç açıldı. Kitap UI’dan tekrar açıldı, kapak yeniden decode edildi ve görüntülendi.
- Kitap ID, kapak ID, not, sayfa ilerlemesi ve image/thumb SHA-256 değerleri eşleşti. İki süreçte 2 + 2 test geçti; mevcut günlük taslağı kurtarma regresyonu da geçti.
- Yeniden açılış screenshot’ı görsel olarak incelendi: kapak, kitap başlığı, not ve 37/200 görünür. Bu P2 görsel/erişilebilirlik kabulü değildir.

Yerel/Git dışı kanıt:
- `artifacts/native-p1-reopened.png`
- Veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-n7hNZt`
- Aynı dizinde `cover-evidence.json`, taşınmış kaynak ve `storage/<generation>/assets/` dosyaları. Gerçek Application Support verisi kullanılmadı.
- Ana görsel SHA-256: `20398e4ffc1c42511d653207d4a79c300a1a8e46f9edc0f542a7a4447b1bfdb5`
- Önizleme SHA-256: `e80bef8db52a5e5ecb1a0b6e79a4e64507d8ca2a0003b7114fae6df5130aa291`

Düzeltmeler: native chooser ana pencereye bağlandı; seçim öncesi show/focus uygulandı. Geçici test görsellerinin asset protocol üzerinden gösterilmesi için yalnız `e2e` feature’da aktif assets dizinine scope eklendi. Production kapsamı ve migration değişmedi.

İlk otomasyon denemesi System Events -1743 izin reddiyle başarısız oldu. Kullanıcının reddi korundu, AppleScript kaldırıldı. Sonraki penceresiz seçim beklemesi iptal edildi; parent/focus düzeltmesinden sonraki koşu kullanıcı seçimiyle geçti. Bunlar gizlenmiş başarılı test sayılmadı.

Koşu sırasında komut `npm run test:native` idi; aynı kapak dalı sonrasında isteğe bağlı `npm run test:native:p1` komutuna ayrıldı. Bu komut seçici için beş dakika bekler; yolu konsola ve `artifacts/native-cover-source.txt` dosyasına yazar. `npm run test:native` temel etkileşimsiz smoke olarak korunur.

Bu oturumda `npm run check` (4 React testi), 19 Rust testi, clippy ve fmt geçti; native picker düzeltmesinden sonra Rust kontrolleri yeniden geçti. Yeni final `.app/.dmg` üretilmedi; test binary’si `src-tauri/target/native/debug/hush`. Notarization/GitHub/CI/draft release sınırları değişmedi.

Test ayrımından sonra `npm run test:native` 04:45 TRT’de yeniden çalıştırıldı: kullanıcı etkileşimi olmadan iki bağımsız süreçte 2 + 2 test geçti.

## P2 oda/gezinme kabulü — 2026-11-02 04:57 TRT

**Geçti.** `npm run test:native -- --room` (eşdeğeri `npm run test:native:p2`) gerçek macOS Tauri debug uygulamasını, embedded WebKit sürücüsünü ve SQLite'ı kullandı. IPC mock yok. Native test binary: `src-tauri/target/native/debug/hush`; tarayıcı demosu veya final release değildir.

İlk süreç: **3 test geçti**. Ayrı ikinci süreç: **1 restart testi geçti**, ilk açılışa özgü **2 test bilinçli atlandı**. Yeniden açılışta yalnız gerçekleştirilen kontrol başarılı sayılır.

| P2 kontrolü | Gerçek sonuç |
|---|---|
| Boş oda | Gerçek `library_list.total=0`; demo eser eklenmedi; özgün mobilya/şehir/varsayılan posterler tamam |
| İki boyut | 1440×900 ve 1100×720; sahne contain, sekiz hedef görünür, hedef alanları ≥44 CSS px |
| Hotspot/menü/kısayol | Kitap, oyun, albüm, günlük, takvim ve iki poster aynı panel başlıklarına ulaşıyor; lamba click ve Cmd+L gerçek ayarı değiştiriyor |
| Odak/Escape | Hotspot paneli kapanınca aynı düğmeye dönüş; Shift+Tab modal içinde kalıyor; kirli form Escape ile onay açıyor, Geri metni koruyor, At yalnız formu kapatıyor |
| Poster/plak menüsü | Ayarlar → Sol/Sağ poster/Plak standı; posterden atanan kitabın gerçek detayı açılıyor |
| Işık | Cozy Night, Violet Rain ve Quiet Dawn UI kaydı ve görsel kontrolü geçti |
| Hareket | Tam modda loop'lar çalışıyor; panel açılınca duraklıyor, kapanınca sürüyor; gerçek minimize komutu sonrası duraklıyor; Azalt modunda sahne animation listesi boş |
| Hata/okunabilirlik | Minimum boyutta geçersiz 37/20 sayfa kaydı reddediliyor, not korunuyor; hata formu ve uzun Türkçe başlıklı kitap paneli görsel olarak incelendi |
| Restart | Tüm settings snapshot/revision ve üç slot kaynağı birebir eşit; Quiet Dawn, neon 27, lamba/yağmur/etiketler kapalı, hareket reduced; iki farklı poster ve plak kapağı decode edilip görünüyor; kitaplık gerçek kitabı açıyor |

Görsel inceleme: 1440 ve 1100 boş/dolu sahnelerde duvar/pencere/mobilya/poster hizası, temas gölgeleri ve ışık yönü tutarlı; hedefler kırpılmıyor. Minimum boyutta etiket fontu en az 12 CSS px olacak şekilde düzeltildi. Hata formu scroll ediliyor; kitap panelindeki uzun başlık okunuyor. Kapaksız atamanın varsayılan poster görünümü önceki başarılı P2 koşusunda ayrıca görüldü; final koşu iki farklı gerçek kapakla yapıldı.

### Test ayrımı ve yerel kanıt

Veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-9RDHcK`. Aynı dizinde `room-evidence.json`, `active.json`, ayrı generation DB ve sentetik assets var. Gerçek Application Support verisine dokunulmadı.

İki sentetik WebP fixture ve asset manifest satırı test runner tarafından yalnız yeni geçici DB'ye kurulur. Eser oluşturma gerçek `library_save`; poster/plak ataması UI → gerçek `room_slot_set`; ayar kaydı UI → gerçek `settings_save`. Fixture kurulumu yeni bir picker/import kabulü olarak sunulmaz. Fixture kaynak/lisansı `tests/fixtures/ROOM-FIXTURES.md` içindedir.

WDIO embedded 1.5.1, klavye olaylarını WebView'de sentezler. Select kontrolleri driver'ın option click sınırlaması nedeniyle DOM `change` olayıyla seçilir; gerçek kaydetme/SQLite/restart yolu değiştirilmez. Driver Space/Enter varsayılan button aktivasyonunu sağlamadığı için bu yol geçmiş sayılmadı; kaynak kodu incelendi, semantik HTML button korundu ve Cmd+L yolu doğrulandı. Manuel VoiceOver, fiziksel klavye ile tüm OS kontrolleri ve sistem hareket tercihini değiştirme testi yapılmadı. Sistem reduced-motion CSS kuralı mevcut; bu oturumdaki otomatik hareket kanıtı uygulamadaki Azalt seçeneğidir.

Yerel/Git dışı screenshot'lar:
- `artifacts/p2-empty-1440.png`, `p2-empty-1100.png`
- `artifacts/p2-filled-saved-1440.png`, `p2-filled-saved-1100.png`
- `artifacts/p2-filled-reopened-1440.png`, `p2-filled-reopened-1100.png`
- `artifacts/p2-cozy_night.png`, `p2-violet_rain.png`, `p2-quiet_dawn.png`
- `artifacts/p2-form-error-1100.png`, `p2-book-panel.png`

İlk sandbox koşusu WebDriver port 4445 timeout'u verdi; mevcut native test izniyle sandbox dışındaki koşu çalıştı. Ara koşulardaki test selector/asset placeholder sayısı/resize bekleme/select ve Space aktivasyon varsayımları düzeltildi; başarısız koşular başarı olarak raporlanmadı. Finder/System Events veya GitHub CLI izni yeniden istenmedi.

### Son regresyon ve sınırlar

- `npm run check`: strict TypeScript, lint, **4 UI testi**, production frontend build geçti (04:57 TRT).
- `cargo test --locked --manifest-path src-tauri/Cargo.toml`: **19 test** geçti.
- clippy `--all-targets -- -D warnings`, fmt ve `git diff --check` geçti.
- Ortak Dialog değişikliği sonrası `npm run test:native`: **2 + 2** mevcut kitap/günlük restart testi geçti (04:57 TRT; ayrı `hush-native-sYObfE` dizini). P1 kullanıcı destekli kapak picker testi yeniden çalıştırılmadı; önceki geçerli kanıt korundu.
- Bağımlılıklar, lockfile'lar ve migration'lar değiştirilmedi. Yeni production test komutu/override eklenmedi; test fixture'ları frontend kaynaklarına alınmadı.
- Yeni final `.app/.dmg` üretilmedi. Önceki P0 `.app` güncel final sürüm değildir. GitHub URL/remote/CI/draft release yok; notarization yapılmadı. P3–P7 ve tam A01–A22 kabulü açık.

## P3 yerel koleksiyon kabulü — 2026-11-02 14:19 TRT

**Geçti.** `npm run test:native -- --collections` (eşdeğeri `npm run test:native:p3`), gerçek macOS Tauri debug uygulaması, WebKit 605.1.15 ve gerçek Rust/SQLite komutlarını kullandı. Native IPC mock edilmedi. İlk süreç **5 test geçti**; sonlandırılıp aynı veriyle açılan bağımsız ikinci süreçte **1 restart testi geçti**, ilk sürece özgü **4 test bilinçli skipped**.

| Kontrol | Sonuç ve kanıt |
|---|---|
| Altı ana eser | UI’dan ikişer oyun/albüm/kitap oluşturuldu; puan, favori, kişisel not, etiket ve durum düzenlendi. Her biri UI’dan trash ve restore işleminden geçti. Tam DTO’lar restart öncesi/sonrası eşit. |
| Tür alanları | Kitap 37/200 → Okudum ile 200/200; oyun %42 → Bitirdim ile %100 ve tamamlanma tarihi; dönüş notu korundu. Albüm iki HTTPS bağlantısını sakladı. |
| Seans | Her ana eserde UI ekle 25 dk → düzenle 45 dk → sil 0 dk → tekrar ekle 35 dk; kendi revision’ı ve ebeveyn değiştirilemezliği gerçek IPC ile kontrol edildi. |
| Alıntı/parça | Her kitap/albümde UI ekle/düzenle/sil/tekrar ekle; alıntı sayfası/yorum ve parça süresi/favorisi/yorumu kalıcı. |
| Screenshot | Her oyun için sentetik yerel asset gerçek children-replace IPC ile bağlandı; UI’dan açıklama düzenlendi, modalda gerçek asset decode edildi, kaldırıldı; tekrar gerçek IPC ile bağlandı ve restart eşitliği doğrulandı. |
| Tür/revision | Her türde yanlış alt kayıt türü VALIDATION, eski ebeveyn revision’ı CONFLICT; eski seans revision’ıyla update/delete ve farklı ebeveyne taşıma CONFLICT. Reddedilen işlemlerden sonra alt kayıtlar değişmedi. Trash ebeveyninde alt kayıt/seans ekleme NOT_FOUND. |
| Duplicate | Aynı elle girilen başlık kaydetmeden uyarıyor. Kaydı aç doğru detayı açtı. Ayrı kayda devam → Kaydet ikinci ID’yi oluşturdu; test bu baskıyı sonra farklı başlıkla düzenledi. Canlı katalog duplicate kabulü P5’tir. |
| Koleksiyon | UI oluştur, ad/açıklama düzenle, farklı türlerden üye ekle/çıkar, sil. Silinen koleksiyonun altı ana eseri korunuyor. Purge sonrası üç ayrı koleksiyonda yaşayan eser üyeliği korundu. |
| Filtre/liste | Durum + favori + koleksiyon + Türkçe çoklu etiket `istanbul, ışık`; başlık araması, boş sonuç, kompakt liste; puan/başlık/eklenme sıralaması. 61 ek sentetik kitapla 60+1 sayfalama ve Önceki/Sonraki; sıralama değişince ilk sayfaya dönüş. |
| Poster/cache | Altı eserin her biri kapaklı olarak sol postere UI’dan atandı. Aynı süreçte trash → 0 dinamik kapak/varsayılan poster; restore → decode edilmiş kapak. Yeniden açılış beklenmeden güncellendi. |
| Purge | Ek oyun, albüm ve kitap için UI onayını iptal etme, sonra onaylama. Aktif eser purge reddi; trash sonrası purge başarılı. Slot default, session/track/quote/screenshot/collection FK bağlantıları yok; yaşayan üye korunuyor. SQLite salt-okunur sorgularıyla cascade doğrulandı. |
| Dış bağlantı | Spotify ve Apple Music düğmeleri gerçek UI’dan aktive edildi, arayüzde hata oluşmadı; Rust doğrulaması `file:` açmayı reddetti. Dış sitelerin içerik/oturum/oynatma durumu kabul kapsamı değildir. |
| Restart | Altı eserin tüm DTO’ları ve extras listeleri, üç kalan koleksiyon, slotlar birebir eşit. 68 aktif sentetik eser; üç purge ID’si NOT_FOUND. Altı detay UI’dan açılıp not/kapak görüntülendi. |

### Ayrım, kanıt ve düzeltmeler

- Son P3 veri dizini: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-keOHNx`. `collections-evidence.json` snapshot’ları aynı dizindedir. Gerçek Application Support verisi kullanılmadı.
- Görsel fixture `tests/fixtures/room-album.webp` (önceki özgün/sentetik fixture) yalnız test DB’sine asset manifesti ve dosyalarıyla kuruldu. Ana eser/alt kayıt/slot/koleksiyon mutasyonları gerçek IPC’ye gider. Kapak atamaları ve screenshot ilk/tekrar bağlamaları doğrudan gerçek IPC; altı ana eserin oluştur/düzenle/trash/restore işlemleri UI’dır.
- **Native picker/import P3’te tekrar edilmedi.** P1’deki gerçek seçici/import/kopya/restart kanıtı korunur; bu fixture kurulumu yeni picker veya screenshot kaynak-boyut dönüşümü testi diye sunulmaz. Finder/System Events otomasyonu denenmedi veya yeniden istenmedi.
- Select seçimleri önceki P2 yaklaşımıyla WebView DOM change olayı kullanır. Kaydetme, doğrulama, depolama ve restart gerçektir; fiziksel OS klavye veya VoiceOver kabulü değildir.
- Git dışı görseller: `artifacts/p3-screenshot.png`, `p3-pagination.png`, `p3-trash-fallback.png`, `p3-restored-poster.png`, `p3-saved.png`, `p3-verified.png`, `p3-reopened.png`. Screenshot modalı, liste ve trash/restore poster çifti görsel olarak incelendi; kırpılmış modal veya kaybolmuş poster hedefi yok.
- Düzeltmeler: `item` cache invalidation; hatalı item sorgusunda stale kapak gizleme; sıralamada offset sıfırlama; virgüllü etiket filtresi; manual duplicate uyarısı; grid/list; uygulama içi screenshot görüntüleyici; albüm dış bağlantıları ve dar `external_open` komutu.
- İlk sandbox denemesi embedded WebDriver port 4445 timeout’u verdi; mevcut izinli sandbox dışı native koşu çalıştı. Ara koşularda testin aria-label/metin, menü konumu ve sayfa dışında kalan kitabı aramadan seçme varsayımları düzeltildi. Bu başarısız koşular geçmiş sayılmadı. Son koşu tüm P3 senaryolarını sıfırdan temiz geçici dizinde tamamladı.

### Son kontroller ve gerçek sınırlar

- `npm run check`: TypeScript strict, ESLint, **4 React testi**, production frontend build geçti (14:18 TRT).
- `cargo test --locked --manifest-path src-tauri/Cargo.toml`: **20 Rust testi** geçti; yeni test dış URL doğrulamasını kapsar.
- `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `git diff --check` geçti.
- P2 regresyonu `npm run test:native -- --room`: **3 + 1** geçti (restart’ta **2 skipped**), 14:19 TRT; ayrı `hush-native-VrPGVa` veri dizini. SlotImage değişikliği sonrası iki poster/plak ve mevcut gezinme/hareket kabulü korunuyor.
- Temel `npm run test:native` regresyonu: **2 + 2** kitap/günlük taslak testi geçti, 14:21 TRT; ayrı `hush-native-N6jUHv` dizini. Bu mevcut smoke testi P4 kabulü olarak işaretlenmedi.
- Test binary: `src-tauri/target/native/debug/hush`. Yeni final `.app/.dmg` üretilmedi. Önceki P0 `.app` güncel teslim değildir. Production frontend build başarılı; final release binary’de test bileşeni yokluğu P7’de doğrulanacak.
- Bağımlılık/lockfile/migration değişmedi; test verisi veya ekran görüntüsü Git’e eklenmedi. Native test komutları/override mevcut e2e feature/config ile sınırlı.
- GitHub remote/URL/CI/draft release yok; reddedilen CLI indirmesi yeniden istenmedi. Notarization yok. P4–P7 kabulü açık; bu oturum P4 çalışmasına geçmez.

## P4 günlük/takvim/yerel arama kabulü — 2026-11-03 01:23–01:24 TRT

**Geçti.** `npm run test:native -- --journal` (eşdeğeri `npm run test:native:p4`) gerçek macOS Tauri debug uygulamasını, WebKit 605.1.15 ve Rust/SQLite'ı kullandı. İlk süreç **4 passed + 1 skipped**, uygulama süreci sonlandırıldıktan sonra aynı izole veriyle ikinci süreç **1 passed + 4 skipped**. Skipped testler diğer sürece özgüdür; yapılmamış adımlar başarılı sayılmadı. P5'e geçilmedi.

| Kontrol | Sonuç |
|---|---|
| Aynı gün çoklu kayıt | 2026-11-01 gününe iki günlük UI'dan eklendi; metin ve eser bağları düzenlendi. İkisi de UI trash → Ayarlar / Silinen günlükler → geri al yolundan geçti. |
| Ruh hâli/ilişkiler | Altı mood ve boş mood UI ile kaydedilip gerçek IPC'den doğrulandı. Her iki günlükte iki eser ID'si ve salt gün korundu; restart snapshot eşitliği geçti. |
| Hata/metin koruma | Açık editör sırasında gerçek IPC ile revision ilerletildi. UI Kaydet conflict verdi, form metni aynen kaldı; Vazgeç / Değişiklikleri at gerçek draft temizliğini yaptı. |
| Autosave/restart | Başlık, gün, melancholy mood, son metin ve albüm ilişkisi 750 ms autosave sonrası draft_get ile eşleşti. Kaydetmeden süreç sonlandırıldı; yeni süreç Taslağı sürdür ile tüm alanları kurtardı. Kaydet sonrası draft null ve üçüncü kalıcı günlük oluştu. |
| Gün/ay/saat dilimi | İstanbul, Los Angeles ve UTC; Ekim, Kasım, Aralık 2026. Günlük daima 2026-11-01. 31 Ekim 22:00Z, 1 Kasım 07:30Z ve 1 Aralık 01:00Z seansları üç tür için doğru gün/aya ayrıldı. |
| DST ve toplamlar | Los Angeles tekrar saatindeki 1 Kasım 08:30Z ve 09:30Z oyun seansları ayrı ayrı sayıldı. Kasım oyun/kitap/albüm dakikaları İstanbul **40/30/30**, Los Angeles **60/50/50**, UTC **30/20/20**. Günlüklü gün **1**; eklenen eser **3**. Önceki/sonraki ay toplamları ve restart snapshot'ları eşit. |
| Takvim UI | Üç timezone seçimi gerçek calendar_month komutuna gider. Ay gezinmesi, gün olayları, 30 Kasım Los Angeles'ta üç adet 30 dk seans ve boş gün mesajı doğrulandı. Geçersiz IANA zone VALIDATION verdi. |
| Toplamdan çıkarma | Oyun trash iken seansları aylık toplamdan çıktı; restore geri getirdi. Ek 7 dk seans toplamı 30→37 yaptı, gerçek session_delete sonrası 30'a döndü. |
| Arama | Cmd+K gerçek uygulama handler'ını açtı. I/ı ve İ/i çiftleri dört izole tek-harf eserle ayrı eşleşti. Üretici, kişisel not, etiket, günlük başlığı/gövdesi ve `%_` literal sonuçları doğrulandı; trash günlükler sonuçlardan çıktı. |
| Sonuç/boş durum | Genel aramadan doğru eser detayı; aynı başlıklı iki farklı günlüğün seçilen UUID'si açıldı. Günlük filtresinde boş sonuç ve Filtreyi temizle; yerel aramada boş sonuç mesajı geçti. |
| Ağ | Yerel arama boyunca Rust HTTP gönderim sayısı **0**, WebView HTTP(S) resource listesi **[]**. IPC mock, sahte yanıt veya ağ bloklama kullanılmadı. |

### Kanıt, yöntem ve sınırlar

- Son P4 veri dizini: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-O7nPqz`. `journal-evidence.json` günlük DTO'ları, eserler, dokuz timezone/ay snapshot'ı ve son taslağı tutar. `search-network-evidence.json` ve boş `http-attempts.txt` ağ kanıtıdır. Gerçek Application Support verisine dokunulmadı.
- Üç fixture eser, sınır seansları, gerçek revision çakışması ve ek arama fixture'ları doğrudan **gerçek IPC** ile kuruldu. Ana günlük CRUD/trash/restore, mood/link düzenlemeleri, taslak ve arama/takvim kullanıcı akışları UI'dır. DB'ye doğrudan fixture SQL yazılmadı.
- WDIO runner ilk uygulama sürecini SIGTERM ile kapatıp yeni süreç başlatır; yalnız sayfa reload değildir. Bu koşu SIGKILL/power-loss testi olarak raporlanmaz. Son autosave tamamlanmadan kapanan son 750 ms'nin kurtarıldığı iddia edilmez.
- Select değişiklikleri mevcut WDIO yaklaşımıyla DOM change olayı kullanır. Cmd+K WebView klavye sentezidir; fiziksel OS klavye/VoiceOver testi değildir.
- Kaynak incelemesinde tek Rust `reqwest` gönderim noktası `Catalog::body` bulundu. Bu noktada yalnız e2e feature'da, geçici veri dizinine içeriksiz `send` işareti yazılır; URL/arama metni/token kaydedilmez. WebView tarafı Resource Timing ile gözlendi. Bu, uygulama katmanındaki ağ kanıtıdır; tüm makine için packet capture iddiası değildir. Release derlemesinde sayaç dalı yok; final binary/DMG denetimi P7'de kalır.
- Yerel/Git dışı görseller: `artifacts/p4-journals.png`, `p4-calendar.png`, `p4-search.png`, `p4-draft.png`, `p4-reopened.png`. Takvim, literal arama ve kurtarılmış günlük ekranları görsel olarak incelendi; uzun takvim/günlük içeriği panel içinde scroll edilir.
- İlk 2026-11-02 koşusunda test, IŞIK etiketi taşıyan albümün IŞIK aramasına gelmemesini yanlış bekledi. Fixture beklentisi düzeltildi; ürünün doğru etiket eşleşmesi korunuyor. Sonraki 2026-11-02 14:29 koşusu ve 2026-11-03 son koşusu geçti. İlk hata başarılı test olarak sayılmadı.
- Ürün düzeltmeleri: timezone seçimi/cache anahtarı, açık boş/yükleme durumları, günlük filtresini temizleme, arama sonucunu UUID ile daraltma ve kapanışta seçimi temizleme. Opsiyonel ID filtre SQL'i parametrelidir; yeni migration veya IPC yetkisi yok.

### Kontroller ve teslim sınırı

- `npm run check`: strict TypeScript, ESLint, **4 React testi**, production frontend build geçti (2026-11-03 01:22 TRT).
- `cargo test --locked --manifest-path src-tauri/Cargo.toml`: **20 Rust testi**, main/doc test koşuları geçti.
- `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`, fmt ve `git diff --check` geçti.
- Bağımlılık, lockfile ve migration değişmedi. Test verisi/screenshot'lar Git dışında. Yeni test komutu yalnız geliştirme runner'ındadır; production frontend'e test import'u eklenmedi.
- Çıktı: `src-tauri/target/native/debug/hush` test binary'si. Yeni final `.app/.dmg` üretilmedi; eski P0 `.app` güncel teslim değildir. GitHub URL/remote/CI/draft release yok, notarization yapılmadı. Reddedilen CLI/Finder/System Events izinleri yeniden istenmedi.
- P5–P7 ve tam A01–A22 matrisi açık. Bu P4 sonucu canlı katalog, backup kesintisi, final release, performans veya VoiceOver kabulünü kapatmaz.

P4 sonrası temel native regresyonu: `npm run test:native`, **2 + 2 passed**, 2026-11-03 01:25 TRT. Ayrı veri dizini `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-2L3LZH`. Kitap/not/37 sayfa kalıcılığı ve mevcut taslak kurtarma korundu; kapak seçici tekrar çalıştırılmadı.

## P5 otomatik katalog kabulü — 2026-11-03

**P5 kabulü tamamlandı; canlı sağlayıcı sınırları aşağıdadır.** P6 uygulanmadı. Başlangıç ağacı e27b042 üzerinde temizdi. Migration ve bağımlılık sürümleri/lockfile'lar değiştirilmedi.

### Gerçek macOS, sentetik provider cache

`npm run test:native -- --catalog`: WebKit 605.1.15, gerçek Tauri debug uygulaması, Rust adaptörleri ve SQLite. Son koşu **4 passed + 2 skipped**, bağımsız ikinci süreç **1 passed + 5 skipped**. Canlı test varsayılan koşuda kapalı; diğer skipped testler karşı sürece özgü. Veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-I2JwFM`.

- Kitap/albüm UI araması → Önbellekten etiketi → baskı seçimi → düzenlenebilir önizleme. Önizlemeden önce DB tür sayısı sıfır; Kaydet sonrası gerçek kayıt var. Puan 8 ve kişisel not, kitapta 37/240 sayfa saklandı.
- Albüm seçili baskısının parçası atomik kayıtta oluştu; 123456 ms → 123 sn. Artist-credit joinphrase korundu. Sentetik kapak cache'den gerçek image decode/import ile yerel AssetStore'a geçti.
- Aynı provider ID başka UUID ile save edilince CONFLICT; aynı kitap baskısını UI'da tekrar seçme mevcut kaydı açar, ikinci form/kayıt oluşturmaz.
- CAA negatif kapak cache'i ve bozuk görsel ayrı `catalog_prepare` çağrılarında warning + kapaksız draft döndürdü; kayıt sayısı değişmedi. Bunlar canlı CAA 404/bozuk yanıt iddiası değildir.
- E2E izole Keychain namespace'inde IGDB `not_configured`; boş credentials VALIDATION. UI'da elle oyun kaydı çalıştı. Gerçek kullanıcı Keychain girdileri okunmadı/değiştirilmedi.
- İkinci bağımsız süreçte kitap/albüm DTO ve extras snapshot'ları birebir aynı. Kapak `<img>` complete/naturalWidth kontrolü geçti. HTTP çıkışı e2e-only marker ile kapalı; sayaç yeniden açılışta artmadı. İşletim sistemi Wi-Fi'sini kapatma testi değildir.

Fixture kaynakları `tests/native/catalog-fixtures.mjs` ve `tests/fixtures/CATALOG-FIXTURES.md`. Cache JSON'ları sentetik; IPC mock yok, kalıcı kayıtlar UI → gerçek IPC → Rust → SQLite yoluyla oluşturuldu. Select değerleri önceki fazlardaki gibi WebView DOM change olayıyla seçildi; fiziksel OS klavye/VoiceOver iddiası değildir.

### Ayrı canlı Open Library kanıtı

`npm run test:native -- --catalog --catalog-live`, veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-4nR1RX`. Bu koşu duplicate UI testinin son ekinden öncedir: **4 passed + 1 skipped**, restart **1 passed + 4 skipped**. Ayrı `live-open-library.json`: `ok=true`, `fromCache=false`, `count=20`. `live-open-library-flow.json`: search/edition/preview/save/refresh/cover tümü true.

“The Left Hand of Darkness” için canlı arama, baskı listesi, seçilen baskı/author/kapak; UI önizleme, kişisel test notu ve 7 puanla kaydetme geçti. Sonrasında gerçek IPC ile yalnız yerel başlık değiştirildi; UI “Katalog bilgisini yenile” taze metadata getirdi, başlık farkı seçilip uygulandı. Not, puan, durum ve ilerleme korundu. Ayrı offline süreçte canlı kaydın DTO/extras eşitliği ve yerel kapağın decode edilmesi doğrulandı. Canlı metadata/kapaklar yalnız geçici dizinde; Git'te yok.

MusicBrainz/CAA canlı çağrısı **yapılmadı**: kullanıcıya User-Agent iletişim adresinin amacı açıklandı fakat doğrulanmış adres verilmedi. Adres uydurulmadı. Albüm kabulü açıkça sentetik cache/gerçek native persistence kabulüdür. Gerçek IGDB OAuth/API bağlantısı **denenmedi**, gerçek credentials kullanılmadı.

### Rust ve güvenlik kapsamı

- HTTP fixture testleri gerçek reqwest ile yalnız loopback TCP'ye gider: header timeout ve body timeout üç denemede TIMEOUT; 429/503 iki retry ardından başarı; kalıcı 429 üç denemede RATE_LIMITED. 400/401/403/404/500 bir kez denenir. Body boyut aşımı reddedilir.
- Saniye ve HTTP-date Retry-After parse; sonraki album isteği 60 saniyelik aynı cooldown'a takılır ve bekleme süresini döndürür; book kuyruğu bağımsız kalır. Üretim limitleri OL/MB 1100 ms, IGDB 500 ms; connect 5 sn, metadata 15 sn, kapak 20 sn.
- Cache TTL/boyut okuma sınırı, malformed/eksik JSON mapping ve publication date yıl ayrıştırması. 60+60 MiB sparse cache dosyası ve 20 MiB incoming bütçeyle eski dosya LRU'dan çıkarılır, yeni korunur; disk önbelleği 100 MiB sınırında kalır.
- Üç türde gerçek SQLite metadata save testi kişisel puan/not/status/tarih/tag ve tür alanları ile seansları korur; kitap alıntısı ve albüm favorili/yorumlu parçası tam extras snapshot olarak korunur. Eski revision CONFLICT. Kitap 37 sayfa → toplam 20 reddedilir; toplam 400 seçilirse mevcut 37 korunur. Screenshot silme/yazma refresh yolunda yok; oyun testinde screenshot listesi boştu, dolu screenshot kabulü P3 kanıtında kalır.
- Gerçek macOS Keychain'e benzersiz test servisinde rastgele sentetik değer yazma/okuma/silme geçti. Değer sohbete/log/DB/Git'e yazılmadı. Bu OAuth doğrulaması, gerçek token yenileme veya OS izin reddi testi değildir. Token single-flight kilidi kaynak incelemesinde korundu; yeni credentials doğrulanmadan eskiler değiştirilmez. Düz metin fallback yok.
- Metadata token'ı 10 dakika, revision ve generation epoch ile bağlıdır; apply'da sadece alan allowlist'i kabul edilir. Kişisel alan veya alt kayıt istemciden metadata patch'i olarak yazılamaz. SQL transaction ağ isteğinden ayrıdır. Parçaların otomatik refresh'i sunulmaz; ilk import ve manuel parça düzenleme korunur.

### Hatalı ara koşular, görseller ve teslim sınırı

İlk sandbox native denemesi ilerlemedi; sonlandırılıp mevcut izinli sandbox dışı yol kullanıldı. İlk Rust HTTP fixture denemesinde sandbox loopback bind'i reddetti; izinli koşu geçti. Bir canlı UI testinde baskı listesi yüklenmeden disabled önizleme düğmesine basılması 120 sn test timeout'u verdi; test yükleme sonunu bekleyecek biçimde düzeltildi, sonraki canlı koşular geçti. Bu hatalı koşular başarılı sayılmadı.

Git dışı görseller: `artifacts/p5-book-saved.png`, `p5-album-saved.png`, `p5-book-offline.png`, `p5-album-offline.png`, `p5-live-refresh.png`. Canlı kapak ve kişisel test notunun görünür olduğu detay görsel olarak incelendi. Son sentetik koşu aynı book/albüm dosya adlarını yeniler; canlı görsel ayrı isimdedir.

Çıktı `src-tauri/target/native/debug/hush` test binary'sidir. Yeni final `.app/.dmg` üretilmedi; eski P0 paketi güncel final değildir. GitHub remote/URL, CI sonucu, draft release ve notarization yok. Reddedilen GitHub CLI indirmesi/Finder/System Events izinleri yeniden istenmedi. Release'te test bileşeni yokluğunun tam binary/DMG denetimi P7'de kalır; production frontend build'de test import'u elenir. P6/P7 ve tam A01–A22 kabulü açık.

P5 son kontroller: `npm run check` (TypeScript strict, ESLint, 4 React testi, production frontend build), **28 Rust testi**, main/doc testler, clippy `--all-targets -- -D warnings`, fmt ve `git diff --check` geçti. Son Rust çalışması LRU/Retry-After ve metadata epoch korumasından sonradır. Native son koşudan sonra eklenen epoch koşulu kaynak/derleme/Rust regresyonuyla kontrol edildi; live UI tekrarına yeni bir sonuç yazılmadı.

## P6 yedekleme/kurtarma kabulü — 2026-11-03 23:22–23:24 TRT

**İstenen P6 kabul senaryoları geçti. P7 başlatılmadı.** Gerçek macOS Tauri debug uygulaması, WebKit 605.1.15, gerçek IPC/Rust/SQLite kullanıldı. Başlangıç `b00355b` üzerinde temizdi; kullanıcı verisi, migration, bağımlılık sürümü ve lockfile değiştirilmedi.

### Native tam eşdeğerlik

`npm run test:native -- --backup` (`test:native:p6` aynı runner): **5 başarılı süreç**, ayrıca aşağıdaki **3 kasıtlı SIGKILL süreci**. Her kesintiden sonra yeni uygulama süreci açılır; yalnız WebView reload değildir. Son runner exit code **0**.

Veri dizini: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-DO5yep`. `expected.json` tam snapshot, `changed.json` yedekten sonraki farklı veri, `backup-run.json` faz/çıkış/PID/nokta/tarih kanıtıdır. `roundtrip-passed`, `reopen-passed`, üç `verify-*-passed` dosyası assertion tamamlanınca yazılır. Arşivler ve kanıtlar Git dışındadır.

| Veri | Karşılaştırılan içerik |
|---|---|
| Eserler | İkişer oyun/albüm/kitap; her türde biri aktif, biri trash. Tüm ortak/türe özgü alanlar, ID/revision/tarihler, puan/not/favori/ilerleme |
| Alt kayıtlar | Altı seans, iki favorili/yorumlu parça, iki alıntı, iki screenshot bağlantısı |
| İlişkiler | İki provenance, iki Türkçe etiket/12 ilişki, üç türü içeren koleksiyon/üç üyelik |
| Günlük | Aktif ve trash iki günlük, altı eser bağlantısı, bir tam günlük taslağı |
| Oda/ayar | Eser posteri, bağımsız görsel posteri, plak standı; tüm ayar JSON'u/revision/tarih |
| Dosyalar | Beş asset'in 10 image/thumb dosyası, göreli yollar ve SHA-256 değerleri |

**19 tablonun her biri doludur; tüm satırların tüm sütunları ve 10 dosyanın hash'leri birebir eşittir.** Sadece DTO veya toplam kayıt sayısı kıyası yapılmadı. Snapshot SQLite salt-okunur bağlantıyla alınır; DB dosyasının byte hash'i WAL/snapshot düzeni nedeniyle eşdeğerlik ölçütü değildir.

UI'dan export → dosya başarı bildirimi; restore önizlemesinde 6 eser/2 günlük/5 görsel → Vazgeç (değişiklik yok) → yeniden incele/açık “Bu yedeği geri yükle” onayı → oda açılması geçti. Önizleme aktif veriyi değiştirmez. Restore sonrasında farklı neon/taslak değişikliği geri alınır, storage epoch değişir, eski DB hâlâ vardır; query/asset cache yeniden yüklenir ve üç dinamik oda görseli decode olur. Bağımsız restart aynı satır/hash eşdeğerliğini tekrar doğrular.

### Bozuk arşiv ve hata davranışı

19 fixture ayrı ayrı gerçek `backup_inspect` komutundan reddedildi. Her ret sonrası pointer ve tam snapshot değişmez:

- ZIP olmayan/kesilmiş arşiv;
- `../`, mutlak yol, backslash ve beklenmeyen dosya;
- manifest hash/boyut uyuşmazlığı, eksik dosya, yeniden hash'lenmiş bozuk WebP;
- ileri SQLite schema (`UNSUPPORTED_SCHEMA`);
- yeniden hash'lenmiş FK ihlali, eksik book details, geçersiz ayar JSON değeri, bozuk draft JSON, ek trigger, yanlış türde plak slotu;
- duplicate ZIP adı, symlink ve aşırı DB boyutu bildiren header.

Diğer fixture'lar `INVALID_BACKUP` döndürdü. Bozuk arşivin Türkçe hata mesajı UI'dan da doğrulandı. Header boyut fixture'ı header tutarlılığı/boyut reddidir; 256 MiB veya 2 GiB gerçek dosya oluşturulmadı. 20.000 entry/2 GiB limitlerinin tüm sınır değerleri native olarak tarandı iddiası yok.

Export son yazma/persist öncesi, inspect alan kontrolü ve restore rename öncesinde e2e-only `StorageFull` enjeksiyonu **DISK_FULL** döndürdü. Aktif veri/pointer ve önceki başarılı yedeğin byte hash'i korundu. Disk fiziksel olarak doldurulmadı; bu kontrollü hata yolu testidir. Rust testi ayrıca eksik referanslı görselde export'un eski hedefi koruduğunu ve sahiplenilmemiş import'un yedeğe alınmadığını doğrular.

Dosya seçimini iptal etme null döndürür. Yeni seçim/iptal önceki inspection token'ını iptal eder. Başarılı token ikinci kullanımda `EXPIRED_BACKUP` verir; 15 dakikalık süre aşımı ve sonradan bozulan stage Rust testinde pointer'ı korur.

### Gerçek kesinti matrisi

| SIGKILL noktası | Yeni süreçte sonuç |
|---|---|
| Staging → generation rename öncesi | Eski pointer; yedekten sonra değiştirilmiş eski verinin tüm satır/hash'leri aynı |
| Generation rename sonrası, pointer commit öncesi | Eski pointer ve tam eski veri; hazırlanmış yeni dizin aktif olmadı |
| Pointer commit sonrası, yeni bağlantı açılmadan önce | Yeni pointer ve yedeğin tüm satır/hash'leri aynı; eski generation mevcut |

Rust yalnız e2e derlemesinde durable PID/nokta işareti yazar ve park eder. Node runner o PID'ye **SIGKILL** gönderir. Crash WDIO koşularındaki `fetch failed`/`ECONNREFUSED` ve nonzero exit beklenir; **normal başarılı test olarak sayılmaz**. Runner yalnız işaret + gerçekleşen kill + ardından bağımsız başarılı verify birlikte varsa kabul eder. Fiziksel güç kesintisi/donanım cache testi değildir. Üç verify ardından normal restore da tekrar başarılıdır; karışık DB/görsel generation oluşmadı.

### Sır ve test ayrımı

- Rust testinde ayrı rastgele macOS Keychain service'ine sentetik değer yazıldı; export'un tüm açılmış girdileri bu değer için tarandı, yoktu. Restore sonrasında Keychain değeri aynıydı; test girdisi silindi. Gerçek kullanıcı Keychain girdileri okunmadı/değiştirilmedi; sır log/DB/Git'e yazılmadı.
- Native arşiv yalnız `manifest.json`, `library.sqlite3`, 10 izinli asset dosyası içerir. Test root'undaki sentetik dışlama işareti arşivde yoktur. Katalog cache/log/Keychain dosyaları export kodunun girdisi değildir.
- Asset fixture dosyaları ve manifest satırları yalnız yeni test DB'sine kuruldu. Diğer tüm kayıt/ilişki/ayar/draft yazmaları gerçek uygulama IPC'sidir. Salt-okunur snapshot karşılaştırması IPC mock değildir.
- Native picker otomasyonu yapılmadı. Sadece e2e feature'da seçim sonucu geçici test dizinindeki tek bileşenli fixture adıyla sağlanır; üretim komutu keyfi path almaz. P1'in kullanıcı destekli picker kanıtı korunur; P6'yı yeni picker testi diye sunmuyoruz.
- Yeni selector/fault/disk hata dalları `cfg(feature="e2e")` altında; frontend'de mock veya test import'u eklenmedi. Tam release binary/DMG testi P7'dedir.

### Düzeltmeler, görseller ve kalan sınırlar

ZIP 9.2.1'in duplicate girdileri tekilleştirmesi nedeniyle eski döngü tek başına yeterli değildi; `has_duplicate_names` ve local/central header tutarlılığı kontrolü eklendi. Eksik manifest dosyası IO yerine INVALID_BACKUP verir. Restore ayar aralıklarını/katalog iletişim biçimini ve plak slotunu doğrular; dosya/dizinler pointer öncesi sync edilir. Filesystem StorageFull doğru hata koduna dönüşür. Yeni inspect önceki token/önizlemeyi temizler; restore günlük seçimini de sıfırlar. E2E asset scope yeni generation'a açılır.

Görseller `artifacts/p6-preview.png`, `p6-rejected.png`, `p6-restored.png`, `p6-reopen.png`, `p6-verify-*.png`; Git dışında. Önizleme özeti/uyarı ve restore sonrası üç kapak görsel olarak incelendi. Ayarlar paneli scroll edilir; onay düğmeleri scroll ile erişilir ve UI testi tarafından tıklanır.

İlk sandbox native çalışması ilerlemedi ve sonlandırıldı; mevcut izinli sandbox dışı native yol kullanıldı. Rust sandbox loopback/Keychain erişimini engelledi; dışarıdaki tam koşu geçti. Ara koşularda testin trash öğeyi yeni koleksiyona ekleme varsayımı düzeltildi; eksik dosya hata kodu ve duplicate açığı ürün kodunda giderildi. Önceki başarısız koşular geçmiş sayılmadı. Son native koşuda bazı WebDriver refresh adımları otomatik retry öncesi 30 sn `execute/sync` timeout uyarısı verdi; tüm ilgili snapshot/görsel assertion'ları ve runner sonuçları geçti. Bu uygulama açılış performans ölçümü değildir.

Active pointer haricen silinir/bozulursa mevcut davranış hata/retry ekranıdır; otomatik boş DB yaratılmaz. Sağlam pointer'lı crash recovery yukarıda kanıtlandı. Haricen pointer kaybında özel generation seçicisi/otomatik kurtarma uygulanmadı; generation dosyaları korunur. Eski generation temizliği yapılmaz. Bu sınırlar güvenli export/restore kabulünü değiştirmez, tam kurtarma yönetimi UI'sı iddiası değildir.

Son kontroller: `npm run check` (strict TypeScript, ESLint, **4 React testi**, production web build) ve **31 Rust testi** (main/doc dahil) geçti. Format/clippy/diff ve temel native regresyonun son sonuçları aşağıdadır.

Çıktı `src-tauri/target/native/debug/hush` test binary'sidir. Yeni final `.app/.dmg` üretilmedi; eski P0 paketi güncel teslim değildir. GitHub URL/remote/CI/draft release yok; notarization yok. Reddedilen GitHub CLI/Finder/System Events izinleri tekrar istenmedi. P7 başlangıç promptu NEXT_CHAT_PROMPT.md'dedir.

P6 son regresyonu: `npm run test:native`, 2026-11-03 **23:24 TRT**, iki bağımsız süreçte **2 + 2 passed**; `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-hMaDU7`. Kitap/not/ilerleme ve günlük autosave/restart korundu. `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `git diff --check` geçti. Test verisi, arşiv ve görseller Git'e eklenmedi.
