# Hush test raporu — P1 kabulü ve faz devri

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

P1 kapısı tamamlandı. Sıradaki kabul P2 oda/gezinme; aşağıdaki genel matris açık kalır.

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
