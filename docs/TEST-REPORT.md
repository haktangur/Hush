# Hush test raporu — P4 kabulü ve faz devri

2026-11-03 · macOS 26.6.2 · Apple Silicon arm64 · Node 24.18.0 · Rust 1.97.1.
Ürün geliştirme aşamasında; tam kabul veya final teslim raporu değildir.

## Son geçen kontroller

| Kontrol | Sonuç ve kapsam |
|---|---|
| `npm run check` | Geçti: TypeScript strict, ESLint, 4 React testi, Vite production build |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | 20 test geçti; lib/main/doc test koşuları hatasız |
| `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` | Geçti |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | Geçti |
| `git diff --check` | Geçti |
| `npm run test:native` (sandbox dışında) | 2026-11-03 01:25 TRT: iki bağımsız süreçte 2 + 2 temel regresyon geçti |
| `npm run test:native -- --journal` | 2026-11-03 01:23–01:24 TRT: 4 + 1 geçti (1 + 4 skipped) |
| `npm run test:native -- --collections` | 14:19 TRT: P3 ilk süreç 5, restart 1 geçti (4 skipped) |
| `npm run test:native -- --room` | 14:19 TRT: P2 regresyon 3 + 1 geçti (2 skipped) |

## Otomatik test kapsamı

React: bootstrap hatasının başarı gibi gösterilmemesi, kaydetme hatasında kişisel notun korunması, sayfa aşımının IPC öncesi reddi, dirty formdan geri dönüş.

Rust: DTO, kitap validasyonu/kind değişmezliği, revision/aynı UUID retry/trash, FK hatasında atomiklik, Türkçe ve literal LIKE araması, migration sözleşmesi/ileri schema/missing pointer, görsel doğrulama ve orijinal dosyadan bağımsız kapak kopyası, alt kayıt revision/kind ve koleksiyon silmenin eseri koruması, günlük/draft/reopen/takvim, provider temel mapping/URL, iletişim URL doğrulaması, backup yol kısıtı/bozuk arşivin aktif veriyi koruması/draft round-trip ve eski generation'ın korunması.

Backup'ın üç testi tüm veri türleriyle eşdeğerlik, disk dolması, ZIP saldırı matrisi veya pointer kesinti testleri yerine geçmez.

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

P0–P4 kapıları tamamlandı. Sıradaki kabul P5 otomatik katalog; aşağıdaki genel matris açık kalır. P2 oda/klavye/hareket alt kapsamı aşağıda ayrıntılıdır; A19 VoiceOver ve tam A01–A22 matrisini kapatmaz.

A01–A22 tam matris henüz tamam değil. Özellikle katalog canlı/hata/refresh akışları, tam backup restore/kesinti, VoiceOver/klavye, 5.000 eser/10.000 seans performansı, DMG'den açılış ve production binary'de test bileşeni yokluğu doğrulanmadı.

GitHub remote/CI sonucu/draft release yok. GitHub CLI indirme izni kullanıcı tarafından reddedildi. Final `.dmg` yok; notarization yapılmadı. npm geliştirme bağımlılıklarına ait önceki audit bulgularının çözüm incelemesi açık; bu oturumda temiz audit iddiası yok.

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
