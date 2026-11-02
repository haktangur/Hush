# Hush — sohbet devri

Tarih: 2026-11-02. Çalışma dizini: `/Users/haktan/Desktop/Hush`.

## Kullanıcının son kararı

Geliştirme faz bazında sürdürülecek. Bu oturum yalnız P2 oda/gezinme kabulünü tamamladı. **P0, P1 ve P2 tamamlandı; sonraki sohbet yalnız P3’ü ele almalı.** P3–P7 hazırlıklarını silme veya tamamlandı varsayma. Faz bitiminde belgeleri ve sonraki sohbet promptunu güncelleyip yerel commit oluştur, sonra dur.

Finder/System Events otomasyon izni kullanıcı tarafından reddedildi. AppleScript denemesi kaldırıldı; izin tekrar istenmedi. Native kapak seçimini kullanıcı yaptı; kalan kabul adımları gerçek IPC ile otomatik tamamlandı. Aynı otomasyon iznini veya GitHub CLI indirmesini yeniden isteme.

## Faz durumu

Planın kaynağı `docs/06-UYGULAMA-PLANI.md` (P0–P7).

- **P0 tamamlandı:** Tauri 2/React/TypeScript/Rust iskeleti, SQLite temeli, gerçek IPC, araç zinciri ve kilit dosyaları, yerel Git, CI başlangıcı. Apple Silicon ad-hoc `.app` üretildi ve açıldı. P0 kabulü remote yerine belgelenmiş bağlantı engeline izin veriyor; GitHub tamamlanmış değildir.
- **P1 tamamlandı:** temel kitap/veri testlerine ek olarak gerçek native seçici → kapak import → kaynak dosyayı taşı → yeni süreçte aç → kitap/kapak görünürlüğü ve hash eşitliği 2026-11-02 04:44–04:45 TRT’de geçti. Seçim kullanıcı tarafından, kayıt/taşıma/restart/assertion WDIO ile; mock IPC yok.
- **P2 tamamlandı:** 2026-11-02 04:57 TRT gerçek Tauri/WebKit ile sekiz hotspot iki boyutta, menü/kısayol, focus/Escape, azaltılmış hareket, minimize, boş/dolu oda ve ışık/poster/plak restart kabulü geçti. VoiceOver ve OS hareket tercihini değiştirme testi yapılmadı; P7 kapsamı açık.
- **P3 hazırlığı var:** tüm türler, seans/alıntı/parça/screenshot, etiket ve koleksiyonlar. Her tür için iki kayıtla tam native CRUD ve silme matrisi eksik.
- **P4 hazırlığı var:** günlük, autosave taslağı, takvim, yerel arama. Native taslak yeniden açılış testi mevcut. Tam kabul matrisi eksik.
- **P5 hazırlığı var:** OL/MB/IGDB Rust adaptörleri, önizleme, ilk parçalarla atomik albüm kaydı, Keychain ayarı. Canlı/provider hata testleri ve metadata refresh diff tamam değil.
- **P6 hazırlığı var:** `.hushbackup` export/inspect/restore, snapshot, boyut/hash/şema/görsel kontrolleri, storage gate, süreli token, pointer değişimi, ayar ekranı. Üç temel Rust testi var. Tam round-trip, arşiv saldırı matrisi ve commit kesinti testleri eksik. Üretim güvenilirliği onaylanmış sayılmamalı.
- **P7 açık:** final production `.app/.dmg`, kurulum, tam kabul, performans, GitHub push/CI/draft release.

## Önce okunacaklar

AGENTS.md, CODEX_BASLANGIC.md, README.md, docs/01–08, docs/PROGRESS.md, docs/TEST-REPORT.md ve bu dosya. Ardından `git status --short`, `git log -3 --oneline` ve mevcut kodu incele. Migration 0001 sözleşmeyle aynı; değiştirme.

## Sonraki sohbetin somut işi — yalnız P3

1. `docs/06-UYGULAMA-PLANI.md` P3 kabulünü mevcut Library/Activity/Collections, Rust servis ve repository koduyla karşılaştır. P0–P2'yi yeniden başlatma.
2. Her türden en az iki sentetik kaydı ayrı test dizininde gerçek macOS UI/IPC ile oluştur, düzenle, sil, geri al; restart kalıcılığını doğrula. Mock IPC kullanma.
3. Oyun ilerlemesi/seans/screenshot, albüm parça/favori/not/dış bağlantı, kitap ilerleme/alıntı, etiket/koleksiyon/filtre/sıralama/sayfalama/duplicate UX eksiklerini tamamla. Görünür düğmeler gerçek işlem yapsın.
4. Alt kayıt tür/revision denetimleri, koleksiyon silmenin eserleri koruması, eser trash/restore/purge ile poster fallback ve kapak önbelleğinin yenilenmesini test et. P2 SlotImage item cache'in trash/restore sonrası invalidation ihtiyacını özellikle denetle.
5. P3 tamamlanınca PROGRESS/TEST-REPORT/HANDOFF ve NEXT_CHAT_PROMPT dosyalarını güncelle, yerel commit oluştur, P4 başlangıç promptuyla dur. P4'e geçme.

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
- `services/settings.rs` ve catalog annotate içinde kalan SQL, repository katmanına taşınmalı; bunu P1 testini gereksiz büyütmeden sonraki fazlarda ele al.
- Katalog Retry-After/timeout/negatif kapak cache ve metadata refresh eksikleri var. Keychain durumundaki “kayıtlı” ifadesi canlı token doğrulaması demek değil. Kullanıcıdan sohbet içinde anahtar isteme.
- Backup doğrulaması genişletilmeli: tüm veri türleri ve asset eşdeğerliği, kötü arşiv fixture'ları, pointer commit öncesi/sonrası fault injection, kurtarma ekranı. Mevcut üç test tam P6 kabulü değildir.
- Dirty native pencere kapatma/Cmd+Q, normal native menüler, onboarding, grid/list, manual duplicate UX, albüm linkleri, screenshot büyütme ve performans kabul işleri açık.

## GitHub ve çıktılar

Dal: `feat/local-library`. Devir checkpoint'i için güncel `git log`u kontrol et.

Remote yok, `gh` kurulu değil, bağlı GitHub aracı yok. 2026-11-02'de GitHub CLI resmî sürüm bilgisini curl ile indirme izni **kullanıcı tarafından reddedildi**; indirme yapılmadı. Yeni sohbette aynı işlemi otomatik tekrar isteme veya başka araçla dolaşma. Kullanıcı GitHub bağlantısını yeniden yetkilendirirse ilerle. Yerel faz çalışması bunu beklemiyor.

Eski P0 çıktısı: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`. Bu güncel özellikleri içeren final paket değildir. DMG, GitHub URL, CI sonucu ve draft release henüz yok. Apple notarization yapılmadı.
