# Hush — sohbet devri

Tarih: 2026-11-03. Çalışma dizini: `/Users/haktan/Desktop/Hush`.

## Kullanıcının son kararı

Geliştirme faz bazında sürdürülecek. Bu oturum yalnız P5 otomatik katalog kabulünü tamamladı. **P0–P5 tamamlandı; sonraki sohbet yalnız P6’yı ele almalı.** P6–P7 hazırlıklarını silme veya tamamlandı varsayma. Faz bitiminde belgeleri ve sonraki sohbet promptunu güncelleyip yerel commit oluştur, sonra dur.

Finder/System Events otomasyon izni kullanıcı tarafından reddedildi. AppleScript denemesi kaldırıldı; izin tekrar istenmedi. Native kapak seçimini kullanıcı yaptı; kalan kabul adımları gerçek IPC ile otomatik tamamlandı. Aynı otomasyon iznini veya GitHub CLI indirmesini yeniden isteme.

## Faz durumu

Planın kaynağı `docs/06-UYGULAMA-PLANI.md` (P0–P7).

- **P0 tamamlandı:** Tauri 2/React/TypeScript/Rust iskeleti, SQLite temeli, gerçek IPC, araç zinciri ve kilit dosyaları, yerel Git, CI başlangıcı. Apple Silicon ad-hoc `.app` üretildi ve açıldı. P0 kabulü remote yerine belgelenmiş bağlantı engeline izin veriyor; GitHub tamamlanmış değildir.
- **P1 tamamlandı:** temel kitap/veri testlerine ek olarak gerçek native seçici → kapak import → kaynak dosyayı taşı → yeni süreçte aç → kitap/kapak görünürlüğü ve hash eşitliği 2026-11-02 04:44–04:45 TRT’de geçti. Seçim kullanıcı tarafından, kayıt/taşıma/restart/assertion WDIO ile; mock IPC yok.
- **P2 tamamlandı:** 2026-11-02 04:57 TRT gerçek Tauri/WebKit ile sekiz hotspot iki boyutta, menü/kısayol, focus/Escape, azaltılmış hareket, minimize, boş/dolu oda ve ışık/poster/plak restart kabulü geçti. VoiceOver ve OS hareket tercihini değiştirme testi yapılmadı; P7 kapsamı açık.
- **P3 tamamlandı:** 2026-11-02 14:19 TRT, gerçek Tauri/WebKit ile ikişer oyun/albüm/kitap UI CRUD/trash/restore; alt kayıtlar, tür/revision kontrolleri, koleksiyon silme, filtre/sıralama/sayfalama, duplicate ve üç türde purge/cascade doğrulandı. İlk süreç 5, bağımsız restart 1 test geçti. Görsel fixture ve picker kapsamı TEST-REPORT’ta açıkça ayrıldı.
- **P4 tamamlandı:** 2026-11-03 gerçek Tauri/WebKit/SQLite ile aynı güne çoklu günlük CRUD/trash/restore, altı mood/boş, ilişkiler, conflict metin koruma, autosave/restart/kurtarma; üç timezone gün/ay/DST toplamları; Cmd+K, Türkçe/literal arama ve sıfır HTTP kabulü. İlk süreç 4, restart 1 test geçti (1 + 4 skipped).
- **P5 tamamlandı:** limit/retry/Retry-After/timeout/cache; OL/MB baskı önizleme, duplicate yönlendirme, gerçek kayıt/offline restart; seçili metadata diff ve kişisel alan koruması. Canlı Open Library doğrulandı. MusicBrainz/CAA sentetik cache ile doğrulandı; adres yokken canlı bağlantı denenmedi. Keychain sentetik round-trip geçti; gerçek IGDB OAuth denenmedi.
- **P6 hazırlığı var:** `.hushbackup` export/inspect/restore, snapshot, boyut/hash/şema/görsel kontrolleri, storage gate, süreli token, pointer değişimi, ayar ekranı. Üç temel Rust testi var. Tam round-trip, arşiv saldırı matrisi ve commit kesinti testleri eksik. Üretim güvenilirliği onaylanmış sayılmamalı.
- **P7 açık:** final production `.app/.dmg`, kurulum, tam kabul, performans, GitHub push/CI/draft release.

## Önce okunacaklar

AGENTS.md, CODEX_BASLANGIC.md, README.md, docs/01–08, docs/PROGRESS.md, docs/TEST-REPORT.md ve bu dosya. Ardından `git status --short`, `git log -3 --oneline` ve mevcut kodu incele. Migration 0001 sözleşmeyle aynı; değiştirme.

## Sonraki sohbetin somut işi — yalnız P6

Tam prompt `NEXT_CHAT_PROMPT.md` içinde. Mevcut backup export/inspect/restore ve generation pointer protokolünü tamamla; tüm veri türleriyle round-trip, arşiv saldırı/bozulma/sınır matrisi ve pointer commit öncesi/sonrası kesinti kabulünü gerçek macOS uygulamasında yap. P5'i baştan yazma; canlı MusicBrainz/IGDB sınırlarını geçmiş sayma.

P5 tekrar: `npm run test:native -- --catalog` (veya `npm run test:native:p5`). Sentetik cache, gerçek Rust/SQLite/IPC; native 4 + 1 passed, 2 + 5 skipped. Son veri `hush-native-I2JwFM`, `catalog-evidence.json`. Canlı OL için isteğe bağlı `--catalog-live`; son canlı kanıt `hush-native-4nR1RX`, `live-open-library.json` ve `live-open-library-flow.json`. İki koşuyu karıştırma.

P5 uygulama notları:
- Metadata `services/metadata.rs`: allowlist, on dakikalık token, revision ve storage epoch kontrolü. Yalnız seçilen katalog alanları değişir; parçalar/seanslar/alıntılar/screenshot ilişkileri değişmez. Yeni parça metadata refresh'i sunulmaz; baskı parçaları ilk kayıtta alınır.
- Provider `body` tek HTTP send noktasıdır. Timeout (header/body), 429, 503 en fazla iki retry; diğer HTTP hataları retry yapmaz. 401 IGDB'de tek token yenilemesine ayrılır. HTTP-date/saniye Retry-After ortak kuyruğu erteler; >15 saniye bekleme saniye bilgisiyle erken döner.
- Arama 24 saat, detay/pozitif kapak 7 gün; negatif kapak 1 saat, disk cache 100 MiB LRU. Refresh TTL=0 ile taze metadata ister. Cache dosya adları hash'tir. E2E cache root test veri dizinidir.
- Keychain configure önce token doğrular; başarısız OAuth mevcut kayıtları değiştirmez. Secret giriş alanı sonuçta temizlenir; id yazma hatasında önceki secret geri konur. Test servis adı ayrı namespace'tir; gerçek Keychain testi rastgele kayıt oluşturup siler. Gerçek IGDB/izin reddi testi yapılmadı.
- `tests/fixtures/CATALOG-FIXTURES.md` yöntemi açıklar. Offline marker yalnız e2e build'de HTTP çıkışını reddeder. Release binary denetimi P7'dedir.

P4 tekrar: `npm run test:native -- --journal` / `npm run test:native:p4`. İlk süreç 4 passed + 1 skipped, bağımsız restart 1 passed + 4 skipped. Güncel kanıt yolu TEST-REPORT'ta. Fixture eser/seansları gerçek IPC ile oluşturulur; günlük CRUD ve taslak UI'dır. Gerçek Application Support verisi kullanılmaz.

P4 düzeltmeleri: Calendar panelinde IANA timezone seçimi ve cache anahtarı; günlük/arama/gün boş mesajları; filtre temizleme; genel arama günlük sonucunu opsiyonel UUID journal_list filtresiyle açma ve kapanışta seçimi temizleme. `providers/mod.rs` tek HTTP send noktasında e2e-only içeriksiz sayaç var; P5 yeni gönderim noktası eklerse bu gözlem kapsamını korumalı. Son test kaynak değişikliği, sabit Kasım 2026 fixture'ına ay gezinmesini ve eklenen eser sayısını gerçek createdAt/timezone'a göre hesaplayarak koşuyu takvim tarihinden bağımsızlaştırdı.


P3 tekrar: `npm run test:native -- --collections` veya `npm run test:native:p3`. Son veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-keOHNx`; `collections-evidence.json` altı tam eser/alt kayıt snapshot'ı, slotlar, üç koleksiyon, toplam ve purge ID'lerini tutar. İlk süreç 5, ikinci süreç 1 test (4 skipped). Yerel görseller `artifacts/p3-*.png`.

P3 düzeltmeleri: Library mutation'ları `item` cache'ini de invalidate eder; Room SlotImage hata durumunda eski kapağı göstermez. Liste sıralaması offset'i sıfırlar, çoklu etiket virgülle ayrılır, grid/kompakt liste seçilebilir. Elle aynı başlıkta uyarı ve ayrı kayda devam akışı vardır. Screenshot uygulama içi modalda açılır. Albüm HTTPS düğmeleri tipli istemci → `external_open` → Rust doğrulaması → sistem açıcı yolunu kullanır. Test image fixture'ı dışında DB'ye doğrudan kayıt eklenmez; screenshot bağlama ve kapak atama gerçek IPC'dir. Picker tekrar edilmedi; P1 kanıtı korunur.

P2 tekrar: `npm run test:native:p2` (`npm run test:native -- --room` aynı işi yapar). Gerçek IPC; fixture asset dosyaları/manifesti yalnız yeni test dizinine kuruluyor. İlk süreç 3 test, ikinci süreç 1 test; iki ilk-açılış testi ikinci süreçte bilinçli skipped. Yeni P2 ekranları `artifacts/p2-*.png`. P2 kabul verisi: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-9RDHcK`.

Ortak Dialog: `src/components/Dialog.tsx`; Radix focus trap korunur, Trigger kullanılmayan oda/menü açılışlarında odak döner. Yeni paneller aynı wrapper'ı kullanmalı. Cmd+7/8 poster, Cmd+9 plak ataması, Cmd+L lamba; paneller açıkken global gezinme kısayolları formu terk ettirmez.

WDIO embedded 1.5.1 klavye olaylarını WebView'de sentezler; Space/Enter'ın native button varsayılan aktivasyonunu uygulamaz. Test için üretim butonuna sahte handler eklenmedi; Cmd kısayolları gerçek handler üzerinden doğrulanır. Select seçimleri WebView DOM change olayıyla, kaydetme ve kalıcılık gerçek IPC ile test edilir. Bu manuel OS klavye/VoiceOver kabulü değildir.

Tekrarlanabilir P1 testi: `npm run test:native:p1`; runner’ın yazdırdığı PNG yolunu native seçicide Cmd+Shift+G ile kullanıcı seçer (5 dakika bekleme). Otomasyon izni/AppleScript kullanılmaz. Varsayılan `npm run test:native` kullanıcı etkileşimi beklemez. İki komut da test-only debug binary kullanır.

## Teknik devir notları

- Tipli tek IPC istemcisi `src/infra/tauri/client.ts`. Komutlar `src-tauri/src/commands/mod.rs`; izin kayıtları build.rs + lib.rs + capabilities/main.json.
- Depolama root/active.json → storage/UUID/library.sqlite3 ve assets. Pointer kaybolunca DB sıfırlanmıyor.
- `StorageGate` tüm normal komutlarda read, export/restore'da write alıyor; async katalog işlemleri de gate kapsamına alındı. Yeni mutasyonlar bu kuralı izlemeli.
- Native test build hedefi ayrı: `src-tauri/target/native`. Unit/release hedefiyle karıştırma; aynı hedefte eşzamanlı build önceki oturumda rustdoc çakışması oluşturmuştu.
- WDIO 9 + Tauri servis/plugin 1.5.1. WDIO 10 uyumsuzdu; yükseltme.
- `scripts/test-native.mjs` geçici veri dizini açıp iki bağımsız uygulama süreciyle test çalıştırır. `--cover` P1 native kapak kabulünü de açar; mevcut günlük taslağı regresyonu korunur. Backup UI testi değildir. E2E asset scope yalnız aktif generation/assets dizinine eklenir; production scope genişletilmedi.
- ScreenCaptureKit -3811 nedeniyle CUA ekran incelemesi çalışmadı. WDIO WebView screenshot çalışıyor; artifacts/ altı Git dışı.
- UI metinleri src/i18n/tr.ts içinde. Test/plugin import production frontend'de mode=e2e koşuluyla elenir; final release binary denetimi henüz yapılmadı.
- Katalog duplicate sorgusu repository katmanına taşındı; settings.rs içinde kalan SQL ayrı teknik borçtur.
- Keychain durumundaki “kayıtlı” ifadesi o anda canlı token doğrulaması demek değildir. Kullanıcıdan sohbet içinde anahtar isteme. MusicBrainz iletişim adresi uydurma.
- Backup doğrulaması genişletilmeli: tüm veri türleri ve asset eşdeğerliği, kötü arşiv fixture'ları, pointer commit öncesi/sonrası fault injection, kurtarma ekranı. Mevcut üç test tam P6 kabulü değildir.
- Dirty native pencere kapatma/Cmd+Q, normal native menüler, onboarding ve performans kabul işleri açık. Grid/list, manual duplicate UX, albüm linkleri ve screenshot büyütme P3’te tamamlandı; katalog duplicate/refresh kabulü P5’te tamamlandı.

## GitHub ve çıktılar

Dal: `feat/local-library`. Devir checkpoint'i için güncel `git log`u kontrol et.

Remote yok, `gh` kurulu değil, bağlı GitHub aracı yok. 2026-11-02'de GitHub CLI resmî sürüm bilgisini curl ile indirme izni **kullanıcı tarafından reddedildi**; indirme yapılmadı. Yeni sohbette aynı işlemi otomatik tekrar isteme veya başka araçla dolaşma. Kullanıcı GitHub bağlantısını yeniden yetkilendirirse ilerle. Yerel faz çalışması bunu beklemiyor.

Eski P0 çıktısı: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`. Bu güncel özellikleri içeren final paket değildir. DMG, GitHub URL, CI sonucu ve draft release henüz yok. Apple notarization yapılmadı.
