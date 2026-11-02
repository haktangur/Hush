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
