# 05 — Katalog entegrasyonları

## Karar

Otomatik arama + elle ekleme birlikte teslim edilir. Kataloglar kişisel koleksiyonun ana veri kaynağı değildir: kullanıcı bir sonucu önizler, düzenler ve kaydeder; seçilen metadata ile kapak yerel kopyaya dönüşür. Uygulamanın her açılışında katalogya başvurulmaz. API yokken veya servis kapalıyken kayıtlı eserler, günlük, puan ve kapaklar çalışmaya devam eder.

| Tür | Sağlayıcı | Kimlik bilgisi | V1 durumu |
|---|---|---|---|
| Kitap | Open Library | Okuma/arama için kişisel anahtar yok | Hazır arama + elle giriş |
| Albüm | MusicBrainz | Genel katalog okumasında API anahtarı yok; tanımlayıcı User-Agent gerekir | Hazır arama + elle giriş |
| Albüm kapağı | Cover Art Archive | API anahtarı yok | Bulunursa indir, yoksa placeholder |
| Oyun | IGDB / Twitch | Kullanıcının kendi client ID ve secret'ı | Adaptör hazır; bağlantı sonradan yapılır |

Bu sağlayıcı seçimi uygulamanın kişisel kullanım kapsamı içindir. Katalogların tüm eserleri, Türkçe adları veya her kapağı içerdiği vaat edilmez. Ticari/genel dağıtım ayrı kapsam ve sağlayıcı koşulu değerlendirmesi gerektirir. [Open Library](https://openlibrary.org/developers/api), [MusicBrainz API](https://musicbrainz.org/doc/MusicBrainz_API), [IGDB](https://api-docs.igdb.com/)

## Ortak adaptör sözleşmesi

```text
CatalogProvider
  status() -> ProviderState
  search(query, page) -> CatalogPage
  prepare(candidateKey, editionId?) -> CatalogDraft
  metadata_diff(localItem) -> MetadataDiff
```

Somut sağlayıcı Rust modülüdür. DTO ortak alanlara normalize edilir; ham provider JSON React'a yayılmaz. `CatalogCandidate`: provider, entityType, externalId, title, creator, year?, providerPageUrl, coverAvailable, editionHint?, existingItemId?. Cover URL frontend'e açık indirme yetkisi vermez.

`CatalogDraft`: ItemDraft için yalnız metadata alanları, isteğe bağlı parça listesi, asset lease, provenance ve warnings. Arama listesindeki kayda tıklamak kalıcı eser oluşturmaz. “Kütüphaneme ekle” ile atomik local save yapılır. İptal edilmiş draft'ın geçici kapağı TTL/lease temizliğiyle gider.

## Arama deneyimi

1. “Ekle” → Oyun / Albüm / Kitap → `Katalogda ara` veya `Elle ekle` sekmesi.
2. Kullanıcı en az iki karakter girer. Otomatik her tuşta request atmak yerine **Ara / Enter** ile arama yapılır. Tek aramada 20 sonuç, “Daha fazla” ile sonraki sayfa.
3. Sonuçlar başlık, üretici, yıl ve küçük kapak içerir. Kapak fetch yalnız görünür ilk 8 sonuç için, öncelik kullanıcı seçtiği kayıtta. Kapak gelmemesi listeyi engellemez.
4. Sonuç seçimi ayrıntı önizlemesini açar; kitap baskısı veya albüm sürümü gerekirse seçilir. Kişisel durum/puan/not varsayılanı kullanıcıya ait boş alanlardır.
5. Kullanıcı kaydeder; sonrası internetten bağımsız. Katalog notu/özetini kişisel not alanına doldurma.
6. API ayarı eksikse açıklama + “Bağlantıyı ayarla” + “Elle ekle” görünür. Başarısız bağlantı boş koleksiyon gibi sunulmaz.

## Ağ, kuyruk ve cache

Tüm istekler Rust `reqwest` üzerinden gider. Client tarafına API secret verilmez. Her provider için tek rate-limit kuyruğu; arama, ayrıntı ve önizleme fetch'i aynı bütçeyi paylaşır. Hush tasarım kararı olarak Open Library ve MusicBrainz için istekler arasında en az 1100 ms, IGDB için en az 500 ms kullan. Bunlar üst limitin altında kalmaya yönelik uygulama sınırlarıdır; sunucu `Retry-After` isterse daha uzun bekle. MusicBrainz'in belgelenmiş temel sınırı 1 istek/sn'dir. [MusicBrainz rate limiting](https://musicbrainz.org/doc/MusicBrainz_API/Rate_Limiting)

Connect timeout 5 sn, toplam metadata timeout 15 sn, kapak 20 sn. Metadata gövdesi en fazla 5 MiB; kapak akışı 20 MiB ve decode sınırı ile durdurulur. Otomatik retry yalnız timeout/429/503 için en fazla iki; jitter + Retry-After. 400/403 sürekli denenmez. 401 IGDB token yenileme bir kez yapılır. Kullanıcı yeni sorguya geçtiğinde eski sonuçlar UI'ı ezemez; request ID ve cancellation token kullan.

Cache: sağlayıcı+sorgu+sayfa anahtarı, arama 24 saat, detay 7 gün, olmayan kapak 1 saat. Toplam disk önbelleği 100 MiB LRU. Kişisel DB ve kalıcı eser kapağı cache temizliğinden etkilenmez. `search_query` cache dosya adı hash olsun; sorgu log'a yazılmasın. Offline cached sonuç kullanılırsa “Önbellekten” etiketle; kaydedilmiş eserler her durumda local DB'den gelir.

User-Agent: `Hush/<appVersion> (<actual-contact-url-or-email>)`. Geliştirici doğrulanmış kişisel GitHub profilini iletişim URL'si olarak kullanabilir; olmayan bir e-posta veya URL uydurmaz. Ayarlarda düzenlenebilir. MusicBrainz için geçerli iletişim URL'si yoksa ilk canlı bağlantı öncesi bu tek alanın tamamlanması istenir; diğer özellikler çalışır. Open Library'nin tanımlayıcı iletişim bilgisi ve insan tarafından başlatılan istek/caching yönergelerine uy. Katalogdan toplu arşiv kazıma veya arka planda sürekli tüm kütüphaneyi yenileme yok. [Open Library kullanım yönergeleri](https://openlibrary.org/developers/api)

HTTP hedefleri provider modülünde sabittir. Yalnız HTTPS. URL parser ile host doğrula; doğrudan IP, localhost, özel ağ, kullanıcı bilgili URL ve custom port kabul edilmez. Redirect'ler otomatik sınırsız takip edilmez; her hop yeniden allowlist kontrolü, en fazla 5. Cover Art Archive için `coverartarchive.org`, `archive.org` ve doğrulanmış `.archive.org` alt alanları gerekebilir; domain suffix sınırı nokta ile kontrol edilir. Yeni CDN gerektiğinde fixture ve gerekçeyle eklenir, genel `*` kullanılmaz.

## Kitap: Open Library

Arama: `GET https://openlibrary.org/search.json` parametreleri `q`, `limit=20`, `page`, `lang=tr`, dar `fields` listesi (`key,title,author_name,first_publish_year,cover_i,edition_key`). URL query builder kullan. Search API eser ve baskı düzeyindeki alanları birlikte verebilir; eksik alanlar normaldir. [Open Library Search API](https://openlibrary.org/dev/docs/api/search)

Arama sonucu work kimliğiyle gelir. Önizlemede kullanıcının baskı seçmesi opsiyoneldir. Work için `/works/{id}.json`, baskılar için `/works/{id}/editions.json?limit=20&offset=...`, seçili edition için `/books/{id}.json`. ID formatını doğrula; response'taki key'den keyfi host türetme. Sayfa sayısı ve ISBN yalnız seçili edition'dan güvenilir alan mevcutsa doldurulur; work seviyesinde sayfa sayısı uydurulmaz.

Cover, geçerli `cover_i` varsa `https://covers.openlibrary.org/b/id/{coverId}-L.jpg?default=false`; bulunamazsa kapaksız kayıt yapılır. Küçük preview `-S` veya `-M`. Work adı/first_publish_year, seçili edition bilgisiyle karıştırılmasın: seçili baskı yılı varsa onu göster; yoksa ilk yayın yılı olarak etiketle.

Provenance: edition seçilmişse entityType `edition` ve externalId edition ID; aksi halde `work` ve work ID. İki farklı edition ayrı eser olabilir; aynı work+edition tekrar eklenmez. Work kaydının daha sonra edition'a zenginleştirilmesi uniqueness kontrolü ile aynı item ID üzerinde yapılır; sessiz duplicate oluşturmaz.

## Albüm: MusicBrainz + Cover Art Archive

Kanonik albüm kimliği `release-group` MBID. Search: `GET https://musicbrainz.org/ws/2/release-group/` query `releasegroup:<escaped user text> AND primarytype:album`, `fmt=json`, `limit=20`, `offset`. Lucene özel karakterlerini escape et; kullanıcı metni query sözdizimi haline gelmesin. Kullanıcı araması orijinal metinle sonuç vermiyorsa elle ekleme sun. [MusicBrainz arama alanları](https://musicbrainz.org/doc/MusicBrainz_API/Search)

Release-group ayrıntısından title, artist-credit, first-release-date alınır. Artist credit joinphrase'leri korunur; tek sanatçı varsayma. İlk yayın tarihi `YYYY`/`YYYY-MM` gibi kısmi olabilir: yıl alanını doldur, gün uydurma.

Parçalar belirli `release` içindedir; release-group'tan doğrudan track listesi varsayma. Önizlemede “Parçaları da ekle” seçilirse release listesi sun; kullanıcı bir baskı seçer. Seçili `release/{mbid}?inc=recordings+artist-credits&fmt=json` üzerinden `media` disklerini sırayla normalize et. Track title ve recording title ayrımını koru; duration milliseconds → seconds dönüşümünü doğrula.

Kapak için önce seçilmiş release varsa `/release/{mbid}/front-500`, yoksa `/release-group/{mbid}/front-500` Cover Art Archive endpoint'i. 307 yönlendirmesi olağan, 404 “kapak yok” durumudur. UI'da kapak yokluğu albüm kaydını engellemez. [Cover Art Archive API](https://musicbrainz.org/doc/Cover_Art_Archive/API)

Provenance entityType `release_group`, externalId MBID, editionId seçilen release MBID. V1'de aynı release-group'un bir kişisel kaydı vardır; farklı baskı koleksiyonculuğu ayrı ürün özelliği değildir. Spotify/Apple Music linki MusicBrainz aramasından kesin bulunmuş gibi üretilmez; kullanıcı ekler. Katalogdan track refresh kişisel favori/notları ezmez; metadata diff açıkça onaylanır.

## Oyun: IGDB

IGDB için Twitch hesabı, 2FA, geliştirici uygulaması ve client ID/secret gerekir. Kullanıcı bunu daha sonra kuracak; bu eksiklik kodlamayı veya yerel teslimi durdurmaz. IGDB kişisel/ticari olmayan kullanım için API sunar; güncel koşullar bağlantı kurulurken doğrulanır. [IGDB hesap ve auth](https://api-docs.igdb.com/)

Hush dağıtılan ortak bir client secret içermez. Kullanıcı kendi geliştirici secret'ını **kendi Mac'indeki** ayara girer; Rust bunu Keychain'de `service=app.hush.desktop`, ayrı `igdb-client-id`/`igdb-client-secret` hesaplarında saklar. Arayüz input'u kaydetme sonrası temizler, tekrar secret okuma API'si yoktur. Bu kişisel kullanım modeli, herkese dağıtılan uygulamada confidential secret'ı güvenli tutma çözümü diye sunulmaz. Genel dağıtımda server-side broker/sağlayıcıca desteklenen auth mimarisi yeniden ele alınır. [Twitch client credentials](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/)

Rust, `POST https://id.twitch.tv/oauth2/token` form-urlencoded body ile client_credentials token ister. Secret URL/query/log içinde yer almaz. Token yalnız RAM'de, expires_at ile tutulur; sona yaklaşınca yenilenir. Eşzamanlı iki token refresh isteğini bir single-flight kilidi birleştirir. Geçerli credentials yoksa hata `PROVIDER_NOT_CONFIGURED` veya anlaşılır bağlantı hatasıdır.

Arama endpoint'i `POST https://api.igdb.com/v4/games`. Headers: Client-ID ve Bearer token. Sabit fields listesi: `id,name,first_release_date,cover.image_id,platforms.name,involved_companies.company.name,involved_companies.developer,url`. `search` içindeki kullanıcı metnini APICalypse string literal için escape et; limit/offset integer ve clamp edilir. İstemciye genel query language alanı verme.

Image ID alfasayısal/underscore doğrulanır; kapak `https://images.igdb.com/igdb/image/upload/t_cover_big/{imageId}.jpg` biçiminden alınır. Provider summary kişisel notes'a yazılmaz; otomatik screenshot indirme yok. Seçilen platformlar detay formunda düzenlenebilir; geliştirici işaretli company öncelikli, bulunamazsa boş. Unix release timestamp UTC yıl bilgisine çevrilir.

IGDB bağlantısını kaldırmak Keychain bilgilerini ve RAM token'ını siler. Kayıtlı oyun/kapak/puanları silmez. Keychain erişimi reddedilirse düz metin fallback yok; kullanıcı elle eklemeye devam eder.

## Metadata yenileme

Otomatik arka plan overwrite yok. Detay menüsünde “Katalog bilgisini yenile” kullanıcı tarafından başlatılır. Eski/yeni title, creator, year, cover ve türe özgü metadata için fark önizlemesi sun. Kişisel puan, not, durum, ilerleme, seans, favori ve kullanıcı etiketleri değişmez. Kullanıcının seçtiği alanlar tek local transaction'da uygulanır; katalog isteği bu transaction dışında yapılır.

## Entegrasyon testleri

Provider fixture'ları: başarı, alanı eksik sonuç, Türkçe/Unicode sorgu, escaped quote, 401, 403, 404 cover, 429+Retry-After, 503, timeout, büyük gövde, uygunsuz redirect. Testlerde canlı API zorunlu değildir; gerçek ağ smoke testi ayrı ve açıkça raporlanır. IGDB anahtarı olmadan fixture testleri + kapalı bağlantı UX testi yapılır; gerçek IGDB bağlantısı denenmediyse teslim raporunda yazılır.
