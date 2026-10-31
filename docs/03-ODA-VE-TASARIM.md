# 03 — Oda ve tasarım sözleşmesi

## Sanat yönetimi

Hush'un ana hissi: şehir dışarıda hareketli, oda içeride sakin. Yakın perspektifli, önden hafif açılı bir çizim; ceviz mobilya, yumuşak kumaş, birkaç bitki, dışarıda neon şehir. Koyu lacivert zemin üzerinde mor ve turkuaz vurgular; masada amber ışık. Temiz, yetişkin bir görünüm. Piksel sanatı, saldırgan gamer RGB, büyük holografik yazılar ve sürekli glitch yok.

Görsel oran hedefi: yaklaşık %70 koyu nötr, %20 sıcak mobilya/amber, %10 neon vurgu. İnce gren ve gölge olabilir; metinlerin üstüne doku konmaz. Neon bir ışık kaynağıdır, her kartın etrafındaki kalın çerçeve değildir.

## Sahne tekniği

SVG + HTML/CSS katmanları kullan. Statik oda çizimi özgün SVG katmanlarından oluşur; poster/ekran içeriği ve erişilebilir hotspot'lar HTML katmanıdır. Önce renkli bloklarla yerleşim doğrulanabilir, ama bitmiş sürümde özgün çizim, perspektif, malzeme, gölge ve ışık gerekir. Tek dev bitmap ve hizası değişen hotspot'lar kabul edilmez.

Sanat koordinatı **1600×1000**. Referans pencere 1440×900. Üst gezinme ve alt kısayol şeridinden kalan alanda sahne `contain` ile orantılı ölçeklenir; boş alan koyu ortamla doldurulur. Sahne kırpılarak tıklanabilir eşya kaybedilmez.

`scale = min(availableWidth/1600, availableHeight/1000)`. Görseller, posterler ve hit alanları aynı sahne kapsayıcısının içinde ölçeklenir. Tek `ResizeObserver` kullan. CSS ile ayrı ayrı viewport koordinatı hesaplama. `contracts/room-layout.json` ortak kaynaktır. Tasarım sırasında nesne yerleri değişirse çizim, config ve görsel test birlikte güncellenir.

## Yerleşim

| Nesne | x / y / genişlik / yükseklik | Görsel ve eylem |
|---|---:|---|
| Pencere | 520 / 90 / 620 / 390 | Katmanlı şehir, cam yansıması, hafif yağmur; dekoratif |
| Kitaplık | 110 / 280 / 250 / 500 | Sol duvarda ceviz raflar, kitap sırtları; Kitaplar paneli |
| Oyun terminali | 610 / 500 / 320 / 230 | Masa üstü ekran ve konsol; Oyunlar paneli |
| Plak istasyonu | 1170 / 540 / 300 / 230 | Sağda pikap, plak rafı ve seçili albüm; Albümler paneli |
| Günlük defteri | 410 / 730 / 180 / 100 | Açık defter ve kalem; Günlük paneli |
| Masa lambası | 410 / 460 / 130 / 220 | Amber ışık; tıklama lamba aç/kapat, yan ayar düğmesi görünüm paneli |
| Takvim | 390 / 250 / 120 / 150 | Duvarda tarih kartı; Takvim paneli |
| Sol poster | 1180 / 130 / 150 / 210 | Dikey kapak; boşsa atama, doluysa küçük aç/değiştir menüsü |
| Sağ poster | 1360 / 190 / 150 / 210 | Aynı davranış |
| Hush neon | 130 / 110 / 260 / 90 | Kısa Hush yazısı; dekoratif |

Hotspot'lar görünmez tahmin alanları olmasın: hover/focus'ta nesnenin çevresinde yumuşak glow ve kısa Türkçe etiket göster. Boş da olsa aynı nesne çalışır. İç içe click handler veya bir tıklamada iki eylem yok. Poster değiştir menüsü klavyeyle de erişilebilir.

```text
 ┌───────────────────────────────────────────────────────────┐
 │ Hush                           Ara   + Ekle    Ayarlar    │
 ├───────────────────────────────────────────────────────────┤
 │  HUSH neon   takvim    YAĞMURLU ŞEHİR        poster poster │
 │                                                           │
 │  kitaplık         lamba  oyun ekranı                       │
 │  kitaplık           ────── masa ──────       plak istasyonu│
 │                           defter                          │
 │                halı / sandalye / sıcak gölgeler            │
 ├───────────────────────────────────────────────────────────┤
 │ Oda   Oyunlar   Albümler   Kitaplar   Günlük   Takvim       │
 └───────────────────────────────────────────────────────────┘
```

Bu çizim renkli nihai tasarım değil, yerleşim sözleşmesidir. Üretimde nesneler mekanın içinde gerçek bir kompozisyon oluşturmalı.

## Katmanlar ve varlık listesi

| Sıra | Katman | Dosya önerisi / davranış |
|---:|---|---|
| 0 | Arka duvar ve zemin | `room-shell.svg`, ortak ışık perspektifi |
| 10 | Uzak şehir | `city-far.svg`, statik |
| 20 | Yakın şehir/tabelalar | `city-near.svg`, düşük yoğunlukta ışık |
| 30 | Pencere çerçevesi/cam | `window.svg`, yağmur kendi maskesinde |
| 40 | Kitaplık, masa, plak dolabı | ayrı SVG; temas gölgeleri birlikte |
| 50 | Etkileşimli eşyalar | monitor, console, turntable, diary, calendar, lamp |
| 55 | Dinamik kapak ve poster | HTML img; cliplenecek çerçeve ve hafif perspektif |
| 60 | Ön plan | sandalye, halı, küçük bitki; hotspot kapatmaz |
| 70 | Işık/yansıma | sabit gradient; `pointer-events:none` |
| 80 | Hotspot düğmeleri/etiketler | semantik HTML button; aynı koordinat sistemi |
| 100+ | Uygulama UI | menüler, panel, modal, toast; sahne scale'inden bağımsız |

Varsayılan posterler Hush'a özgü geometrik şehir/ay motifleri olsun. Ticari oyun/albüm kapaklarını uygulama kaynaklarına demo olarak gömme. Kullanıcının seçtiği veya sağlayıcıdan aldığı kapak yalnız yerel koleksiyon verisidir.

Varlık listesi `src/assets/room/ASSET-MANIFEST.md` içinde kaynak/yazar/lisans/boyut olarak tutulur. Görsel üretim aracı mevcutsa aynı sanat yönergesiyle destek alınabilir; zorunlu değildir ve uygulama çalışırken üretim API'si çağrılmaz. SVG ile özgün çizim esas uygulanabilir yoldur.

## Işık düzenleri

| Düzen | Ana vurgu | İkinci vurgu | Amber | Arka plan |
|---|---|---|---|---|
| Cozy Night | `#A78BFA` | `#52D8D1` | `#F2B66D` | `#0B1020` |
| Violet Rain | `#C084FC` | `#F08DC7` | `#DCA46A` | `#100D20` |
| Quiet Dawn | `#8BB8CB` | `#B5A5D9` | `#E8BC8B` | `#182131` |

`contracts/theme.css` ilk renk ve boşluk sözleşmesidir. Quiet Dawn bile parlak beyaz tema değildir. Lamba kapalı olduğunda amber yayılan ışık kapanır; mobilya ve yazılar görünür kalır. Neon yoğunluğu yalnız ışık/glow katmanına etki eder, okunabilir metin kontrastını düşürmez.

## Hareket

- Yağmur: yalnız pencere içinde iki tiled CSS katmanı, 8–14 saniye doğrusal döngü. En fazla iki hareketli yüzey; 200 ayrı damla yok.
- Plak: açık panel yokken, tam hareket modunda 12 saniyede dönüş. Parça oynatıyor göstergesi yapılmaz.
- Neon: en az 6 saniyelik, en fazla yaklaşık %8 opacity değişimi; flash/strobe yok.
- Hover/focus: 120–180 ms; fiziksel nesneyi en fazla 2 px yükseltme veya glow.
- Panel: 180–220 ms opacity + en fazla 12 px translate. Modal üstünde ikinci sahne animasyonu yok.
- `prefers-reduced-motion` veya “azalt”: yağmur/plak/neon loop durur, panel yalnız kısa fade veya anında açılır. Klavye odağı animasyonsuz görünür.
- Pencere gizli veya minimize ise tüm dekoratif loop'lar pause. Panel açıkken arka sahne animasyonları pause; çizim görünür kalabilir.

## Panel sistemi

Ana panel, içerik alanının sağında 760–980 px; dar pencerede `width: min(960px, calc(100vw - 48px))` ile merkezde. Yükseklik kullanılabilir alandan 48 px düşük. Sol tarafta oda kısmen görünür; panel arka planı %94–98 opak koyu renk. Aşırı blur yerine tek hafif scrim kullan.

Başlık ve arama/filtre satırı sabit; içerik scroll edilir. Detay açılınca panel içi geri düğmesi görünür. Oyun/kitap kapakları 2:3, albümler 1:1. Koleksiyon grid'inde oran farkları satır hizasını bozmaz; kapak alanı kontrollü, metin 2 satır kesilir ve tam başlık erişilebilir adında kalır.

Ortak ekran durumları: yükleniyor (skeleton), boş (ilgili ekleme çağrısı), sonuç yok (filtreyi temizle), hata (yeniden dene), başarı, çevrimdışı katalog, API bağlı değil. Bunların her biri Türkçe ve gerçek durumu anlatır.

## Tipografi ve erişilebilirlik

- Sistem fontu `-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif`; ağdan font indirme yok. Başlık 24–28 px, gövde 14–16 px, alt sınır 12 px yalnız ikincil etiketlerde.
- Normal metin hedef kontrast 4.5:1; odak/ikon gibi UI öğeleri 3:1. Renk tek bilgi kanalı değildir.
- En az 44×44 CSS px tıklama alanı; oda scale'i bunu düşürüyorsa hit alanını minimum CSS piksel hesabıyla genişlet ve nesne komşuluğunu test et.
- Tüm hotspot'lar gerçek `button`, anlaşılır `aria-label`; dekor SVG `aria-hidden`. Oda ekranının erişilebilir adı “Hush dijital odası”.
- Modal focus trap ve odak geri dönüşü; Escape en üst katmanı kapatır. Metin alanında yazarken harf kısayolları tetiklenmez.
- Cmd+K genel arama; Cmd+N bağlama göre yeni eser/günlük seçimi; Cmd+, ayarlar; Cmd+S açık formu kaydeder; Cmd+1 oda, Cmd+2 oyunlar, Cmd+3 albümler, Cmd+4 kitaplar, Cmd+5 günlük, Cmd+6 takvim.
- Menü yolu her özellik için oda tıklamasına alternatif sunar. VoiceOver ile temel ekle/düzenle/kapat akışı denenir.

## Görsel kabul kriterleri

1. 1440×900 ve 1100×720'de sekiz etkileşim hedefi (iki poster ayrı) görünür ve çalışır; sahne katmanları hizalıdır.
2. Mobilyalar tutarlı perspektif, temas gölgesi ve ışık yönüne sahiptir; nesneler boşlukta asılı görünmez.
3. Neonlar kişisel not okumayı zorlaştırmaz. En uzun örnek Türkçe başlık taşmaz.
4. Dolu/boş kitaplık, kapaksız eser, iki farklı poster ve lamp-off ekranları incelenir.
5. Erişilebilir menü klavyeyle odadaki tüm bölümlere ulaştırır.
6. Tek bir screenshot'ın güzel olması yeterli değildir: panel açık, form hatalı ve minimum pencere durumları da düzgün görünmelidir.
