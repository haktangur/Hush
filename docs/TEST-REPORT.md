# Hush test raporu — faz devri

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

Öncelik P1: native kapak picker → orijinal dosyayı taşı → yeniden aç → kapağı görüntüle birleşik testi.

A01–A22 tam matris henüz tamam değil. Özellikle tüm türlerin native CRUD'ı, katalog canlı/hata/refresh akışları, tam backup restore/kesinti, VoiceOver/klavye, 5.000 eser/10.000 seans performansı, DMG'den açılış ve production binary'de test bileşeni yokluğu doğrulanmadı.

GitHub remote/CI sonucu/draft release yok. GitHub CLI indirme izni kullanıcı tarafından reddedildi. Final `.dmg` yok; notarization yapılmadı. npm geliştirme bağımlılıklarına ait önceki audit bulgularının çözüm incelemesi açık; bu oturumda temiz audit iddiası yok.
