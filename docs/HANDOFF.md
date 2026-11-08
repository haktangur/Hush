# Hush — sohbet devri

2026-11-07 · `<proje kökü>` · `feat/local-library` ·## Son talimat ve durum

P0–P6 tamam. P7 otomatik kalite, paketleme ve yerel commit 2026-11-07'de tamamlandı (Claude). Açık kalanlar: kullanıcı destekli VoiceOver/klavye/Dock/DMG kabulü ve GitHub (remote/push/CI/draft release). Güncel sonuçlar ve A01–A22 tablosu: [TEST-REPORT.md](TEST-REPORT.md).

## P7 güncel kanıt

- Paketler (Git dışı): `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`, `.../dmg/Hush-0.1.0-aarch64.dmg`, `.../Hush-0.1.0-aarch64.app.zip`, `.../SHA256SUMS.txt`. Ad-hoc imza; notarize değil.
- Kapanış kuralı (D22): yalnız `useUnsavedWork` kaydına giren kirli form/süren işlem Cmd+Q/Cmd+W'yi durdurur. Yeni bir form eklenirse aynı hook'u kullanmalıdır; aksi hâlde Cmd+Q o formun metnini korumaz. Dock "Çık"/sistem kapatma korunmaz (tao `applicationShouldTerminate:` uygulamıyor).
- Tekrar komutları: `npm run check`; sandbox dışında `cargo test --locked --manifest-path src-tauri/Cargo.toml`; `npm run test:native -- --quality|--performance|--regressions`; `npm run build:mac`; `npm run verify:release`.
- Sandbox içinde Rust loopback HTTP testi ve WDIO portu çalışmaz; native/HTTP testlerini sandbox dışında koştur.
- Manuel oturum: `npm run test:native -- --manual` ayrı sentetik/offline test penceresi açar; kullanıcının gerçek verisine dokunmaz. Bu pencere WebDriver portu 4445'i tutar; açıkken diğer native testler bilerek çalışmayı reddeder (önce pencereyi kapat).
- Görsel QA: `npm run test:native -- --visual` 20 ekranı iki boyutta `artifacts/ui/` altına çeker; taşma ve oda hover saydamlığını assert eder.
- Türkçe kurulum: [KURULUM-VE-KULLANIM.md](KURULUM-VE-KULLANIM.md). Sürüm notu: [RELEASE-NOTES.md](RELEASE-NOTES.md). Lisans envanteri: `THIRD-PARTY-NOTICES.md`.
- `.github/workflows/ci.yml` ve `release.yml` yerel hazırlıktır; hiç koşmadı. Reddedilen GitHub CLI indirmesi/Finder/System Events izinleri yeniden istenmez; GitHub için kullanıcıdan yeni açık yetki gerekir.

NEXT_CHAT_PROMPT.md).

## P6 kanıtı ve teknik notlar

- Tekrar: `npm run test:native -- --backup` / `npm run test:native:p6`. Gerçek Tauri/WebKit, Rust, SQLite; IPC mock yok.
- Son veri: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-DO5yep`. `expected.json`, `changed.json`, `backup-run.json`, `*-passed`, sentetik `.hushbackup` dosyaları Git dışında.
- 19 tablonun tüm satırları ve beş asset'in image/thumb SHA-256 değerleri karşılaştırılır. Tüm türler, seanslar, tracks/quotes/screenshots, trash, provenance, etiketler, koleksiyon, günlük/ilişki/taslak, slotlar ve ayarlar doludur.
- Beş başarılı native süreç: roundtrip, reopen ve üç SIGKILL sonrası verify. Üç crash süreci beklenen WDIO bağlantı hatasıyla biter; runner durable PID/nokta işareti olmadan hatayı kabul etmez. Her verify eski/yeni tam veriyi kontrol eder.
- 19 bozuk arşiv ve üç StorageFull enjeksiyonu aktif veri/pointer'ı korur. Gerçek disk doldurulmadı; fiziksel elektrik kesintisi testi yok.
- Dosya seçici sonucu e2e-only fixture dosyasından gelir. Picker UI otomasyonu yapılmadı. Üretim komutu keyfi path almaz. Asset fixture kurulumu dışında veriler gerçek IPC ile yazılır.
- `backup.rs`: duplicate/header kontrolü, hash/manifest/file allowlist, SQLite integrity/FK/schema, item/journal/draft/settings/slot ve görsel doğrulaması; stage revalidation, directory sync, pointer commit. Eski generation'lar tutulur.
- Yeni inspect önceki token'ı siler; başarısız/iptal edilmiş seçim eski önizlemeyi kullanamaz. Restore token tek kullanımlık ve 15 dakika geçerli. Restore UI query cache ve günlük/item seçimlerini temizler.
- Sadece e2e derlemesinde selector, fault marker, StorageFull enjeksiyonu ve yeni generation asset scope'u vardır. Release binary/DMG yokluk denetimi P7'de zorunlu.
- Keychain testi ayrı rastgele service'e sentetik değer yazdı, yedeği taradı, restore sonrası aynı değeri gördü ve sildi. Gerçek kullanıcı credentials kullanılmadı.
- 31 Rust testi, 4 React testi, strict TS/lint/production web build geçti; son kontroller ve regresyon ayrıntıları TEST-REPORT'ta.

## P7'de korunacak gerçek sınırlar

Final production Apple Silicon `.app/.dmg`, native menü/dirty kapanış, onboarding, VoiceOver, iki boyut QA, 5.000 eser/10.000 seans performansı ve tam A01–A22 değerlendirmesi P7 kapsamındadır. Eski P0 `.app` güncel final değildir. GitHub URL/CI/draft release ve notarization yok. Canlı MB/CAA ve gerçek IGDB OAuth geçmiş sayılmamalı; P5 sınırları aşağıdadır.

Mevcut eksik/bozuk active pointer otomatik boş DB yaratmaz; hata ekranı/retry verir. Sağlam pointer ile üç crash recovery senaryosu P6'da geçti. Pointer'ın haricen silinmesi için otomatik generation seçimi veya özel kurtarma seçicisi yok; dosyalar korunur. Eski generation temizliği yapılmaz (diskte kalır). Bu davranışları veri sıfırlama ile değiştirme.

## Önceki fazların tekrar notları

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


## Ortak altyapı ve çıktılar

- Tek tipli IPC istemcisi `src/infra/tauri/client.ts`; komut izinleri build.rs/lib.rs/capabilities üzerinden sınırlı.
- StorageGate normal işlerde read, export/restore'da write; ağ işlemlerinde SQLite transaction tutulmaz.
- Native hedef `src-tauri/target/native/debug/hush`; test-only binary. Unit/release hedefinden ayrı tutulur.
- WDIO 9 / embedded 1.5.1; lockfile yükseltme. Ekran görüntüleri `artifacts/` altında ve Git dışı.
- Test verisi geçici `hush-native-*` dizinindedir; Application Support'a dokunulmaz. DB sıfırlama yok.
- Eski P0 app: `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app`. Güncel final değildir; DMG henüz yok.
