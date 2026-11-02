# Hush — sohbet devri

Tarih: 2026-11-02. Çalışma dizini: `/Users/haktan/Desktop/Hush`.

## Kullanıcının son kararı

Geliştirme faz bazında sürdürülecek. İlk fazın ardından belgeler doldurulup yeni sohbete geçilecek. Bu nedenle bu oturumda yeni kapsam açılması durduruldu; mevcut kod kontrol edilerek devrediliyor. Yeni sohbet **P1 kabul kapısını** tamamlamalı, sonra belgeleyip kullanıcıya sonraki sohbet promptunu vermeli. P2–P7'ye ait hazır kodu silme veya yeniden başlatma.

## Faz durumu

Planın kaynağı `docs/06-UYGULAMA-PLANI.md` (P0–P7).

- **P0 tamamlandı:** Tauri 2/React/TypeScript/Rust iskeleti, SQLite temeli, gerçek IPC, araç zinciri ve kilit dosyaları, yerel Git, CI başlangıcı. Apple Silicon ad-hoc `.app` üretildi ve açıldı. P0 kabulü remote yerine belgelenmiş bağlantı engeline izin veriyor; GitHub tamamlanmış değildir.
- **P1 kabulü açık:** kitap kaydetme, revision, trash/restore ve yeniden açılış doğrulandı. Yerel kapak kopyası Rust testinden geçti. Native dosya seçiciyle kapak ekle → orijinali taşı → uygulamayı yeniden aç → kapak görünür birleşik testi eksik.
- **P2 hazırlığı var:** katmanlı SVG oda, sekiz hotspot, ışık/hareket/poster ayarları; iki pencere boyutunda ekran görüntüsü/sığma kontrolü. Tüm hotspot/klavye/focus/VoiceOver ve restart ayar matrisi tamam değil.
- **P3 hazırlığı var:** tüm türler, seans/alıntı/parça/screenshot, etiket ve koleksiyonlar. Her tür için iki kayıtla tam native CRUD ve silme matrisi eksik.
- **P4 hazırlığı var:** günlük, autosave taslağı, takvim, yerel arama. Native taslak yeniden açılış testi mevcut. Tam kabul matrisi eksik.
- **P5 hazırlığı var:** OL/MB/IGDB Rust adaptörleri, önizleme, ilk parçalarla atomik albüm kaydı, Keychain ayarı. Canlı/provider hata testleri ve metadata refresh diff tamam değil.
- **P6 hazırlığı var:** `.hushbackup` export/inspect/restore, snapshot, boyut/hash/şema/görsel kontrolleri, storage gate, süreli token, pointer değişimi, ayar ekranı. Üç temel Rust testi var. Tam round-trip, arşiv saldırı matrisi ve commit kesinti testleri eksik. Üretim güvenilirliği onaylanmış sayılmamalı.
- **P7 açık:** final production `.app/.dmg`, kurulum, tam kabul, performans, GitHub push/CI/draft release.

## Önce okunacaklar

AGENTS.md, CODEX_BASLANGIC.md, README.md, docs/01–08, docs/PROGRESS.md, docs/TEST-REPORT.md ve bu dosya. Ardından `git status --short`, `git log -3 --oneline` ve mevcut kodu incele. Migration 0001 sözleşmeyle aynı; değiştirme.

## Sonraki sohbetin somut işi — yalnız P1 kapısı

1. Mevcut doğrulama sonuçlarını TEST-REPORT'tan al; yeni değişiklik gerektirmedikçe tüm testleri tekrar tekrar çalıştırma.
2. Native kapak seçme/kopyalama/görüntüleme/restart akışını sentetik PNG ile tamamla. Test verisi gerçek Application Support'a yazılmamalı.
3. Test veri dizini `HUSH_TEST_DATA_DIR` yalnız `e2e` feature içinde. Asset protocol production scope `$APPDATA/storage/*/assets/**/*`; geçici test dizini için test-only dar scope gerekebilir. Production scope'u genişletme.
4. Birleşik native akışta hata çıkarsa düzelt. Gerçek UI testini mock IPC ile değiştirme.
5. P1 kabulünü belgelerle kapat, anlamlı checkpoint commit oluştur. P2 için yeni sohbet başlangıç promptu ver ve faz sonunda dur.

## Teknik devir notları

- Tipli tek IPC istemcisi `src/infra/tauri/client.ts`. Komutlar `src-tauri/src/commands/mod.rs`; izin kayıtları build.rs + lib.rs + capabilities/main.json.
- Depolama root/active.json → storage/UUID/library.sqlite3 ve assets. Pointer kaybolunca DB sıfırlanmıyor.
- `StorageGate` tüm normal komutlarda read, export/restore'da write alıyor; async katalog işlemleri de gate kapsamına alındı. Yeni mutasyonlar bu kuralı izlemeli.
- Native test build hedefi ayrı: `src-tauri/target/native`. Unit/release hedefiyle karıştırma; aynı hedefte eşzamanlı build önceki oturumda rustdoc çakışması oluşturmuştu.
- WDIO 9 + Tauri servis/plugin 1.5.1. WDIO 10 uyumsuzdu; yükseltme.
- `scripts/test-native.mjs` geçici veri dizini açıp iki bağımsız uygulama süreciyle test çalıştırır. Mevcut native test kapsamı kitap ve günlük taslağıdır; backup UI testi değildir.
- ScreenCaptureKit -3811 nedeniyle CUA ekran incelemesi çalışmadı. WDIO WebView screenshot çalışıyor; artifacts/ altı Git dışı.
- UI metinleri src/i18n/tr.ts içinde. Test/plugin import production frontend'de mode=e2e koşuluyla elenir; final release binary denetimi henüz yapılmadı.
- `services/settings.rs` ve catalog annotate içinde kalan SQL, repository katmanına taşınmalı; bunu P1 testini gereksiz büyütmeden sonraki fazlarda ele al.
- Katalog Retry-After/timeout/negatif kapak cache ve metadata refresh eksikleri var. Keychain durumundaki “kayıtlı” ifadesi canlı token doğrulaması demek değil. Kullanıcıdan sohbet içinde anahtar isteme.
- Backup doğrulaması genişletilmeli: tüm veri türleri ve asset eşdeğerliği, kötü arşiv fixture'ları, pointer commit öncesi/sonrası fault injection, kurtarma ekranı. Mevcut üç test tam P6 kabulü değildir.
- Dirty native pencere kapatma/Cmd+Q, normal native menüler, onboarding, grid/list, manual duplicate UX, albüm linkleri, screenshot büyütme ve performans kabul işleri açık.

## GitHub ve çıktılar

Dal: `feat/local-library`. Devir checkpoint'i için güncel `git log`u kontrol et.

Remote yok, `gh` kurulu değil, bağlı GitHub aracı yok. 2026-11-02'de GitHub CLI resmî sürüm bilgisini curl ile indirme izni **kullanıcı tarafından reddedildi**; indirme yapılmadı. Yeni sohbette aynı işlemi otomatik tekrar isteme veya başka araçla dolaşma. Kullanıcı GitHub bağlantısını yeniden yetkilendirirse ilerle. Yerel P1 çalışması bunu beklemiyor.

Eski P0 çıktısı: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`. Bu güncel özellikleri içeren final paket değildir. DMG, GitHub URL, CI sonucu ve draft release henüz yok. Apple notarization yapılmadı.
