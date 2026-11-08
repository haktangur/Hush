# 08 — Kararlar, varsayımlar ve resmi kaynaklar

## Kabul edilmiş kararlar

| ID | Karar | Kaynak / gerekçe |
|---|---|---|
| D01 | İsim Hush, dil Türkçe | Kullanıcının açık tercihi |
| D02 | Cozy cyberpunk dijital oda | Kullanıcının seçtiği yön; mor/turkuaz neon + sıcak amber |
| D03 | Kişisel Mac uygulaması | Kullanıcı kendi Mac'inde kullanmak istiyor |
| D04 | Otomatik arama + elle ekleme | Kullanıcının son yanıtı; oyun API'si sonradan bağlanabilir |
| D05 | GitHub entegrasyonu | Kullanıcının açık talebi; geliştirici repo/CI/release altyapısı |
| D06 | 2D katmanlı sahne | İlk sürümde kaliteli görsel, erişilebilir hit alanı, düşük uygulama karmaşıklığı |
| D07 | Tauri/React/Rust/SQLite | Platform kabuğu, esnek oda çizimi, yerel veri ve tek uygulama mimarisi |
| D08 | Yerel veri + Keychain | Hesap zorunluluğu yok; API sırları kayıt/yedekten ayrılır |
| D09 | Sunucu ve bulut eşitleme yok | Kişisel tek cihaz kapsamı; bakım/abonelik gereksinimi oluşturmaz |
| D10 | Apple Silicon ilk hedef | Hazırlık ortamı arm64 macOS 26.6.2; uygulama minimum politikası macOS 14 |
| D11 | Özgün SVG oda | Ücretli görsel üretim servisine bağımlı olmadan tamamlanabilir |
| D12 | Private kaynak deposu varsayılanı | Kullanıcı genel yayın istemedi; kod geliştirirken paylaşım kapsamı dar |
| D13 | Ad-hoc yerel build, draft release | Kullanıcının Mac'inde çalışan çıktı; Apple üyeliğini ön koşul yapmama |
| D14 | Sabit mobilya + değişebilir poster/ışık | İlk sürümün detayını korurken oda editörü kapsamını sınırlama |

D06–D14 mimari karar/varsayımlardır; kullanıcının kelimesi kelimesine söylediği tercihler diye sunulmaz. Geliştirici bunları varsayılan kabul eder, gereksiz yeniden tasarımla zaman kaybetmez. Zorunlu değişiklik varsa karar ID'siyle gerekçe kaydeder.

## Kapsamı genişletmeden çözülen belirsizlikler

- Uygulama müzik dinleme hizmeti değil, albüm arşivi ve günlüğüdür; dış müzik bağlantıları açılır.
- Bir oyun terminali ve bir albüm alanı vardır; koleksiyon büyüdükçe odaya yüzlerce fiziksel obje çizilmez.
- Her oyun, kitap ve albüm için aynı ortak UI/data altyapısı; farklılaştırılmış tür alanları bulunur.
- İlk sürümde kullanıcı sırlarıyla çalışan kişisel IGDB adaptörü vardır. Ortak gömülü secret ile herkese dağıtım yapılmaz.
- GitHub hesabı ve Apple sertifikası bu tasarım paketini hazırlamak için gerekli değildir. Kodlama aşamasında auth gerekiyorsa sadece ilgili işlem için sorulur.
- Kullanıcının iki “sıfırlama hakkı” ifadesi herhangi bir belirli Codex kotası olarak yorumlanmamıştır. Yeniden başlatmak yerine Git commit'leri, migration'lar ve PROGRESS.md ile devam edilebilir çalışma tasarlanmıştır.

## Teknik kaynaklar

Kaynaklar 1 Kasım 2026 tarihinde kontrol edildi. Belgenin ürün ve mimari kararları bu proje için hazırlanmıştır; bağlantılı resmi sayfalar yalnız ilgili platform/API davranışlarını destekler. Paketler ve sağlayıcı koşulları kodlama sırasında değişmişse geliştirici ilgili resmi kaynağı yeniden kontrol eder.

| Konu | Resmi kaynak | Bu projedeki karşılığı |
|---|---|---|
| Codex proje talimatları | [AGENTS.md](https://learn.chatgpt.com/docs/agent-configuration/agents-md) | Kök talimat kısa; ayrıntılı belgeler açıkça okunur |
| Tauri geliştirme ön koşulları | [Prerequisites](https://v2.tauri.app/start/prerequisites/) | Rust, macOS build araçları, frontend araç zinciri |
| React | [Quick Start](https://react.dev/learn) | React bileşen modeli |
| Vite | [Guide](https://vite.dev/guide/) | Frontend geliştirme ve build |
| Tauri izinleri | [Capabilities](https://v2.tauri.app/security/capabilities/) | Pencere ve komut erişim sınırları |
| Yerel asset gösterimi | [Asset protocol scope](https://v2.tauri.app/security/asset-protocol/) | Görseller için dar dosya kapsamı |
| Native uygulama testleri | [Tauri WebDriver](https://v2.tauri.app/develop/tests/webdriver/) | macOS embedded driver yaklaşımı |
| Native test kurulumu | [WDIO plugin setup](https://webdriver.io/docs/desktop-testing/tauri/plugin-setup) | Test-only feature/plugin ayrımı |
| SQLite snapshot | [rusqlite backup](https://docs.rs/rusqlite/latest/rusqlite/backup/index.html) | WAL açıkken güvenilir yedek |
| Kitap arama | [Open Library Search API](https://openlibrary.org/dev/docs/api/search) | Work/edition metadata ayrımı |
| Kitap API kullanımı | [Open Library APIs](https://openlibrary.org/developers/api) | İstek sınırı, cache ve kimlik bilgisi |
| Albüm kataloğu | [MusicBrainz API](https://musicbrainz.org/doc/MusicBrainz_API) | Genel katalog, anahtarsız okuma |
| Albüm arama | [MusicBrainz Search](https://musicbrainz.org/doc/MusicBrainz_API/Search) | release-group sorguları |
| Albüm istek sınırı | [Rate limiting](https://musicbrainz.org/doc/MusicBrainz_API/Rate_Limiting) | User-Agent ve bir istek/sn politikası |
| Albüm kapakları | [Cover Art Archive API](https://musicbrainz.org/doc/Cover_Art_Archive/API) | Kapak ve redirect/404 davranışı |
| Oyun kataloğu | [IGDB API](https://api-docs.igdb.com/) | Twitch credentials, fields, cover images |
| Twitch token | [OAuth tokens](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/) | Client credentials ve server-side secret ilkesi |
| GitHub repo | [gh repo create](https://cli.github.com/manual/gh_repo_create) | Private repo ve local source remote |
| CI dağıtımı | [Tauri GitHub pipeline](https://v2.tauri.app/distribute/pipelines/github/) | Build/release altyapısı |
| macOS bundle | [Application bundle](https://v2.tauri.app/distribute/macos-application-bundle/) | `.app`, minimum macOS sürümü |
| macOS imzası | [Code signing](https://v2.tauri.app/distribute/sign/macos/) | Ad-hoc ile notarization ayrımı |

MusicBrainz ana API sayfasının doğrudan getirilmesi bazı denemelerde 429 döndürdü; ilgili bilgiler resmi arama çıktısı ve açılabilen resmi arama/kapak sayfalarıyla kontrol edildi. Bu durum API'lerin erişilebilirliğinin garanti edilemeyeceğini de gösterir; üründeki hata ve offline akışları bu nedenle tanımlıdır.

## Bilinen teslim sınırları

Bu paket uygulama kodunu, gerçek macOS binary'sini, GitHub deposunu, canlı IGDB oturumunu veya nihai oda illüstrasyonunu içermez. Bunların hepsi geliştirici için tanımlı teslim işleridir. Başlangıç SQL'i ve makinece okunabilir sözleşmeler kontrol edilmiştir; gerçek uygulama için istenen kabul testleri henüz çalıştırılmış değildir. Sonuç ayrımı [PAKET-DOGRULAMA.md](../PAKET-DOGRULAMA.md) içinde yer alır.

## D15 — Faz sınırı ve yerel test izni (2026-11-02)

Kullanıcının son talimatı bu sohbeti yalnız P1 kabulü, belgeler ve yerel commit ile sınırlar; P2 yeni sohbette başlayacak. Bu, başlangıçtaki tüm fazlar bitene kadar devam et ve GitHub bağlantısını tamamla talimatının bu oturumdaki önüne geçer. Önceden reddedilen GitHub CLI indirmesi yeniden denenmez. Finder/System Events otomasyon izni de reddedildiği için picker otomasyonu kaldırıldı; gerçek native seçim kullanıcı tarafından, taşıma/restart/kalıcılık doğrulaması WDIO ile yapıldı. Üretim yetkileri genişletilmedi; test geçici görsel kapsamı yalnız e2e derlemesindedir.

## D16 — P2 ile sınırlı yerel kabul (2026-11-02)

Son kullanıcı talimatı bu oturumu yalnız P2 oda/gezinme kabulü, belgeler, yerel commit ve P3 başlangıç promptuyla sınırlar. D15'in faz bitince durma ve reddedilen izinleri yeniden istememe kuralı devam eder. GitHub CLI/Finder/System Events denenmedi. P3–P6 mevcut kodu korundu fakat yeni kabul verilmedi.

Poster ve plak için ayarlar menüsü girişleri ile Cmd+7/8/9, lamba için Cmd+L eklendi; mevcut Cmd+1–6/K/, yolları korundu. Uygulama içi menü P2'de doğrulandı; macOS native menü işi planlandığı gibi P7'de kalır. Yeni paket veya migration yok.

## D17 — P3 ile sınırlı kabul ve test verisi (2026-11-02)

Son kullanıcı talimatı bu oturumu yalnız P3 yerel koleksiyon kabulü, belgeler, yerel commit ve P4 başlangıç promptuyla sınırlar. Başlangıçtaki tüm fazları bitirme/GitHub push talimatı bu oturumda uygulanmaz; P4'e geçilmez. Reddedilen GitHub CLI indirmesi ve Finder/System Events otomasyonu yeniden istenmez.

P3 testleri mevcut e2e-only Tauri/WebKit sürücüsünü kullanır. Görsel fixture yalnız yeni geçici assets/DB'ye kurulur; screenshot ilişki ekleme ve kapak atama gerçek IPC üzerinden yapılır. Screenshot görüntüleme/düzenleme/silme gerçek UI'dır. Bu, native picker'ın yeniden test edildiği anlamına gelmez; P1 kullanıcı destekli import kanıtı korunur. Üretim test yetkisi/override'ı eklenmez. HTTPS dış bağlantı için yeni dar `external_open` komutu URL'yi Rust'ta doğrular ve shell yorumlaması olmadan macOS `/usr/bin/open` çalıştırır; bağımlılık eklenmez.

## D18 — P4 ile sınırlı kabul (2026-11-03)

Son kullanıcı talimatı yalnız P4 günlük/takvim/yerel arama kabulü, belgeler, yerel commit ve P5 başlangıç promptunu yetkilendirir. Başlangıçtaki tüm fazları bitirme/GitHub push talimatının önündedir; P5'e geçilmez. GitHub CLI ve Finder/System Events izinleri yeniden istenmez.

Takvim seçimi sistem saat dilimiyle başlar; panelde IANA saat dilimi seçilebilir ve sorgu anahtarına katılır. Bu seçim günlüklerin salt tarihini değiştirmez ve işletim sistemi saat dilimini değiştirmez. Yerel arama günlük sonucunda başlık yerine UUID ile daraltılır; journal_list'e opsiyonel, doğrulanan ID filtresi eklendi. Migration veya yeni IPC yetkisi gerekmedi.

Yerel aramanın ağ üretmediğini gözlemek için mevcut tek Rust HTTP send noktasına yalnız `cfg(feature="e2e")` kapsamında içeriksiz sayaç dosyası eklendi. Dosya mevcut izole HUSH_TEST_DATA_DIR altındadır; yalnız `send` işareti taşır. İstek/yanıt değiştirme ve IPC mock yok. Üretimde bu dal derlenmez; tam release binary denetimi P7'nin ayrı kabulüdür.

## D19 — P5 kabul sınırı, cache fixture ve iletişim adresi (2026-11-03)

Son kullanıcı talimatı yalnız P5, belgeler, yerel commit ve P6 promptunu kapsar; P6 uygulanmaz. GitHub CLI indirmesi ve Finder/System Events izni yeniden istenmedi. Mevcut kullanıcı değişiklikleri korunur; başlangıç ağacı 17a2e62 üzerinde temizdi.

Kullanıcıya MusicBrainz User-Agent iletişim alanının amacı açıklandı; doğrulanmış adres verilmedi. Adres uydurulmadı ve canlı MusicBrainz çağrısı yapılmadı. Cache'deki albüm verisi iletişim adresi olmadan kullanılabilir; yeni MusicBrainz HTTP çağrısı adres yoksa reddedilir. Canlı Open Library ve sentetik MusicBrainz/CAA kabul sonuçları ayrı raporlanır. Open Library kullanım/rate-limit belgesi yeniden okundu; MusicBrainz belge erişimi 429 verdi. IGDB resmi belge okuması gerçek IGDB OAuth testi sayılmaz.

Native fixture'lar provider cache'ine konur; gerçek IPC veya SQLite işlemleri taklit edilmez. Sadece e2e derlemesinde HTTP çıkışını reddeden offline işareti ve izole Keychain servis adı vardır. Release config'ine test komutu/yetkisi eklenmez. HTTP hata fixture'ları Rust testlerinde loopback kullanır. Varsayılan P5 testi canlı servise bağımlı değildir; canlı OL için açık `--catalog-live` seçeneği gerekir.

Metadata yenileme taze ayrıntı ister, on dakikalık opak önizleme token'ıyla yalnız seçilen metadata alanlarını uygular. Token revision ve storage epoch ile bağlıdır. Kişisel alanlar, seanslar, alıntılar, parçalar ve screenshot ilişkileri yeniden yazılmaz. Albüm parça listesini otomatik yenilemez; seçilen baskının parçaları ilk kayıtta atomiktir. Tamamlanmış kitabın sayfa ilerlemesi refresh'te yeniden türetilmez. Yeni toplam mevcut sayfanın altındaysa işlem reddedilir.

## D20 — P6 kabul sınırı ve gerçek kesinti yöntemi (2026-11-03)

Son kullanıcı talimatı yalnız P6, belgeler, yerel commit ve P7 promptunu kapsar. Başlangıçtaki bütün fazları bitirme/GitHub push talimatının bu oturumda önündedir; P7 çalışması yapılmaz. GitHub CLI indirmesi/Finder/System Events izinleri tekrar istenmez.

Native P6 testi gerçek IPC ve depolama kullanır. Native dosya seçiciyi otomatik yönetmek yerine yalnız e2e feature'da geçici test dizininden tek bileşenli fixture adı seçilir; production path API'si/izni genişletilmez. Bu yeni picker kabulü değildir. Asset fixture satırları ayrı test DB'sine kurulur; diğer veri yazmaları gerçek uygulama komutlarıdır.

Kesinti testinde Rust test dalı belirtilen noktada durable PID/nokta işareti yazar ve park eder; runner yalnız bu PID'ye SIGKILL gönderir. WDIO bağlantı hatası ancak işaret, SIGKILL ve ardından bağımsız süreçte tam eşdeğerlik doğrulanınca beklenen kesinti sayılır. Fiziksel elektrik kesintisi veya disk donanımı garantisi iddia edilmez. Disk dolması, dosya yazma/restore sınırlarında StorageFull hata enjeksiyonudur; gerçek disk doldurulmaz.

ZIP 9.2.1 aynı adlı girdileri index'te tekilleştirdiğinden `has_duplicate_names` ve header tutarlılığı açıkça kontrol edilir. Sürüm yükseltilmedi. Restore öncesi dosya ve dizinler sync edilir; pointer tek commit noktasıdır. Eski generation'lar silinmez. Keychain yalnız ayrı rastgele test service'inde denenir; sır yedek/Git/log içine yazılmaz.

## D21 — P7 yerel teslim ve yetki sınırı (2026-11-04)

Son kullanıcı talimatı yalnız P7 kalite/paketleme/teslimi, belge güncellemesi ve yerel commit'i yetkilendirir. Başlangıçtaki GitHub oluşturma/push talimatı bu oturumda uygulanmaz: remote/URL/CI/draft release yok, yeniden uzak erişim veya CLI indirmesi yapılmadı. Önceden doğrulanmış Action SHA'ları korundu; uzak SHA/runner erişimi bu oturumda yeniden doğrulanmadı. Workflow dosyaları yalnız yerel hazırlıktır.

DMG, Tauri'nin Finder görünüm düzenleyen otomasyonu yerine bağımsız `hdiutil` ile oluşturulur. Finder/System Events/AppleScript izni istenmez veya kullanılmaz. Bu, disk görüntüsü üretimidir; reddedilmiş UI otomasyonuna alternatif erişim değildir. Standart Applications bağlantısı ve Türkçe yönerge içerir.

Native Cmd+W/kırmızı kapatma pencereyi gizler; Dock tekrar gösterir. Cmd+Q çıkış isteği UI tarafından çözümlenir. Açık panel/form varsa işlem kesilmez; kullanıcı mevcut Kaydet/Vazgeç akışını tamamlayıp paneli kapatarak kapanışı yeniden ister. Rust storage gate devam eden işlemi bekler. Bu muhafazakâr koruma yalnız dirty alanlar yerine bütün açık panellere uygulanır. Yeni kapatma komutu dar ana pencere capability'sindedir; test komutu değildir.

İlk açılış açıklaması atlanabilir ve gerçek settings işleminden sonra tamamlandı sayılır. Cmd+N odada kitap, açık koleksiyon/günlükte bağlama uygun kayıt açar; mevcut formu terk ettirmez. Türkçe native menü/About metinleri `tr.ts` kaynaklıdır.

Performans fixture'ı yalnız yeni sentetik generation'a 5.000 eser/10.000 seans ekler; hiçbir mevcut veritabanı sıfırlanmaz. Sorgular gerçek IPC/Rust/SQLite, UI gerçek macOS WebKit'tir. Release optimizasyonlu e2e ölçümü production paket ölçümünden ayrılır. Fiziksel VoiceOver/klavye kanıtı kullanıcı destekli olup otomatik WDIO sonuçlarıyla karıştırılmaz.

## D22 — Kapanış koruması yalnız kaydedilmemiş işe uygulanır (2026-11-07, Claude devri)

D21'deki "açık her panel Cmd+Q/Cmd+W'yi engeller" kuralı daraltıldı. Salt okunur bir panel (ör. Kitaplar listesi) açıkken Cmd+Q hiçbir şey yapmıyordu; bu, 02-MIMARI §4'teki "Cmd+Q çıkar; kirli form ve devam eden restore varsa uygun durum yönetimi" ölçütüyle uyumsuzdu. Yeni kural: formlar kaydedilmemiş metin, değişmiş kapak, kaydedilmemiş katalog önizlemesi veya süren kaydet/içe aktar/geri yükleme işlemi taşıdıkları sürece `src/app/unsavedWork.ts` kaydına girer (ItemForm, günlük editörü, seans/alıntı/parça/screenshot editörü, koleksiyon editörü, Ayarlar). Kayıt boşsa Cmd+W/kırmızı düğme pencereyi gizler, Cmd+Q çıkar; doluysa ikisi de durur ve yalnız en üstteki diyalog uyarıyı bir kez okur. Rust `app_finish_close` storage gate'i bekler. Çıkış onayı tipli `QuitApproved` state'iyle tutulur.

Bilinen sınır (tao 0.37.2 kaynak incelemesi): tao macOS'ta `applicationShouldTerminate:` uygulamaz. Dock menüsündeki "Çık", oturum kapatma ve sistem kapatma `terminate:` ile doğrudan çıkar; bu yollar kaydedilmemiş form metnini korumaz (günlük taslağı 750 ms autosave ile korunur). Menü/klavye Cmd+Q ve Cmd+W özel menü öğeleriyle korunur.

Devir sırasında bulunan ve düzeltilen gerçek sonuç: önceki devir notu `npm run check`'in geçtiğini bildiriyordu, ancak lifecycle Vitest dosyası modül önbelleği nedeniyle başarısızdı. Test, menünün bir kez kurulup hem menü eylemlerini hem native kapanış olayını sınayacağı biçimde yeniden yazıldı.

## D23 — Ortak arayüz ölçüsü ve CSP uyumlu kontroller (2026-11-07)

Kullanıcı geri bildirimiyle bütün paneller tek kontrol yüksekliği (`--control-h: 40px`), tek boşluk ölçeği ve tek odak halkası kullanır; native select görünümü kaldırıldı. Üretim CSP'si `data:` görsellerine izin vermediği için select oku SVG data URI yerine CSS gradyanıyla çizilir; CSP genişletilmedi. Radix'in odakladığı diyalog kabının kendi çerçevesi gizlenir, kontrollerin `:focus-visible` halkası korunur. Görsel kalite `--visual` native turu ile iki pencere boyutunda kontrol edilir.

## D24 — Public GitHub deposu (2026-11-08)

Kullanıcı mevcut **public** `https://github.com/haktangur/Hush` deposunu verdi ve push istedi. Bu, CODEX_BASLANGIC/07'deki "varsayılan private" kuralının önündedir. Push öncesi tüm git geçmişi sır desenleri (özel anahtar, GitHub/AWS/Google/Slack/OpenAI token'ları, client_secret, Bearer, parola), riskli uzantılar (.env, .p12, .pem, .key, SQLite, .hushbackup) ve kişisel veri açısından tarandı; bulgu yok. Belgelerdeki yerel kullanıcı yolu temizlendi. `.gitignore` macOS metadata, imza materyali ve editör dosyalarıyla genişletildi. Ekran görüntüleri yalnız sentetik test verisidir. Lisans dosyası kullanıcı kararı olarak bekletildi.
