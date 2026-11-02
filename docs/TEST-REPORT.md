# Hush test raporu — P2 kabulü ve faz devri

2026-11-02 · macOS 26.6.2 · Apple Silicon arm64 · Node 24.18.0 · Rust 1.97.1.
Ürün geliştirme aşamasında; tam kabul veya final teslim raporu değildir.

## Son geçen kontroller

| Kontrol | Sonuç ve kapsam |
|---|---|
| `npm run check` | Geçti: TypeScript strict, ESLint, 4 React testi, Vite production build |
| `cargo test --locked --manifest-path src-tauri/Cargo.toml` | 19 test geçti; lib/main/doc test koşuları hatasız |
| `cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` | Geçti |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | Geçti |
| `git diff --check` | Geçti |
| `npm run test:native` (sandbox dışında) | Gerçek Tauri/WebKit ile iki bağımsız süreçte 2 + 2 test geçti |

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

P0–P2 kapıları tamamlandı. Sıradaki kabul P3 yerel koleksiyonlar; aşağıdaki genel matris açık kalır. P2 oda/klavye/hareket alt kapsamı aşağıda ayrıntılıdır; A19 VoiceOver ve tam A01–A22 matrisini kapatmaz.

A01–A22 tam matris henüz tamam değil. Özellikle tüm türlerin native CRUD'ı, katalog canlı/hata/refresh akışları, tam backup restore/kesinti, VoiceOver/klavye, 5.000 eser/10.000 seans performansı, DMG'den açılış ve production binary'de test bileşeni yokluğu doğrulanmadı.

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
