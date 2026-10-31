# 01 — Ürün ve kapsam

## Ürün tanımı

Hush kişisel bir kültür odasıdır. Açılışta kullanıcıyı yağmurlu geceye bakan, mor ve turkuaz neonların amber masa ışığıyla dengelendiği bir oda karşılar. Odadaki gerçek eşyalar oyun, müzik, kitap ve günlük arşivine giriş olur. Kişisel veriler yereldir; kataloglar sadece keşif ve bilgi doldurmak içindir.

Tek kullanıcı, tek Mac ve tek aktif veri kümesi vardır. Hesap, sosyal akış, arkadaş listesi, sunucu veya abonelik ekranı yapılmaz. Tamamen çevrimdışı kullanım desteklenir; sadece katalog araması ve dış bağlantılar internet ister.

## İlk açılış

1. En fazla iki kısa adım: “Hush'a hoş geldin” ve arama/gizlilik açıklaması. Atlanabilir.
2. “Odaya gir” ile boş fakat görsel olarak tamamlanmış oda açılır. Sahte kitap/oyun kayıtları eklenmez.
3. Kısa ipucu: “Eşyalara tıklayarak koleksiyonlarını açabilirsin.” Nesne etiketlerini aç/kapat anahtarı görünür.
4. “İlk oyununu ekle”, “Bir albüm bul”, “Kitap ekle” ilgili eşyada küçük boş durum ipuçlarıdır.
5. IGDB kurmak zorunlu değildir. İnternet araması ilk kullanıldığında yalnız arama teriminin sağlayıcıya gönderildiği açıklanır; günlüklerin gönderildiği izlenimi yaratılmaz.

## Navigasyon

Sahne her zaman uygulamanın ana sayfasıdır. Üstte küçük Hush işareti, genel arama, “Ekle” ve ayarlar bulunur. Altta kompakt erişilebilir kısayol şeridi: Oda, Oyunlar, Albümler, Kitaplar, Günlük, Takvim. İsimler ve ikonlar birlikte kullanılır.

Oda nesnesi bir koleksiyon paneli açar. Panel içinde öğe detayı, düzenleme ve geri dönüş kendi geçmişini tutar. Aynı anda yalnızca bir ana panel ve onun üzerinde en fazla bir form/onay diyaloğu bulunur. Panel kapanınca tetikleyen nesneye odak döner. Açık form kirliyse kaydet/at/vazgeç sorulur; panel dışına tıklayarak veri kaybedilmez.

## Oyunlar

- Ortak alanlar: başlık, kapak, üretici/yazar/sanatçı metni, çıkış yılı, 0–10 tam sayı kişisel puan, favori, etiketler, kişisel not, koleksiyonlar.
- Oyun alanları: geliştirici, oynandığı platform metni, tür etiketleri, isteğe bağlı 0–100 ilerleme, başlangıç ve bitiş tarihi, geri dönerken hatırlamak için kısa not.
- Durumlar: Başlayacağım, Oynuyorum, Bitirdim, Ara verdim, Bıraktım.
- Manuel oynama seansı: tarih/saat, dakika, kısa not. Toplam süre seansların toplamıdır; ikinci bir değiştirilebilir toplam süre alanı tutulmaz. Eski oynama süresi “Önceki süre” notlu bir seansla girilebilir.
- Oyun başına yerel ekran görüntüleri ekle, büyük görüntüle, açıklama yaz, kaldır. Otomatik ekran yakalama izni gerekmez.
- Liste: kapak kartları ve kompakt liste görünümü; durum, favori, etiket, koleksiyon filtreleri; başlık/eklenme/puan/son güncellenme sıralaması.
- “Bitirdim” yapmak ilerlemeyi 100'e getirir; tarih boşsa bugün atanır, kullanıcı düzenleyebilir. Yeniden açmak tarihsel bitirme bilgisini kendiliğinden silmez.

## Albümler

- Sanatçı, albüm adı, yıl, kapak, isteğe bağlı tür/ruh hâli etiketleri, puan, favori, not.
- Durumlar: Dinleyeceğim, Dinliyorum, Dinledim.
- İsteğe bağlı ilk dinleme tarihi ve manuel dinleme seansları. Kayıtlar gerçek Spotify/Apple Music dinleme geçmişi gibi sunulmaz.
- Parça listesi: disk/sıra, ad, isteğe bağlı süre, favori ve kısa kişisel yorum. Parçalar elle eklenebilir; katalog kaydı seçili baskıya sahipse sağlayıcıdan doldurulur.
- Spotify ve Apple Music için isteğe bağlı iki HTTPS bağlantısı; sistemin varsayılan tarayıcısında açılır. Arayüzde “Spotify'da aç” yazılır, sahte oynat/duraklat düğmesi kullanılmaz.
- Odanın plak istasyonunda sergilenecek bir albüm seçilebilir. Dönen plak dekoratif animasyondur; müzik çalındığı anlamına gelmez.

## Kitaplar

- Yazar, kitap adı, kapak, yıl, isteğe bağlı ISBN, yayınevi, dil, toplam sayfa ve bulunduğu sayfa.
- Durumlar: Okuyacağım, Okuyorum, Okudum, Ara verdim, Bıraktım.
- Sayfa ilerlemesi `mevcut / toplam`; toplam bilinmiyorsa yüzde gösterilmez. Toplamdan fazla mevcut sayfa kabul edilmez. Toplam sonradan küçültülürse form hata gösterir, sessiz düzeltme yapmaz.
- Alıntılar: düz metin, isteğe bağlı sayfa ve kişisel yorum. Kullanıcının eklediği alıntılar dış servise gönderilmez.
- Manuel okuma seansı tarih/süre/not tutar; sayfa numarası kitabın ayrıntı formunda güncellenir. V1'de seanslardan otomatik sayfa toplamı türetilmez.
- “Okudum” toplam sayfa biliniyorsa mevcut sayfayı toplam sayfaya eşitler; toplam bilinmiyorsa uydurmaz.

## Ortak koleksiyon davranışları

- Bir eser, sağlayıcı aramasıyla veya elle eklenir. Arama sonucu önce düzenlenebilir önizleme formuna gelir; seçmek tek başına kaydetmez.
- Kişisel puan sağlayıcı puanından ayrıdır; V1 sağlayıcı puanı göstermez.
- Aynı sağlayıcı kimliği zaten varsa “Kütüphanende var” ve “Kaydı aç” çıkar. Elle eklenen aynı başlıkta yalnız uyarı verilir; farklı baskılar engellenmez.
- İstenilen adlarla karışık koleksiyonlar: örneğin “Yağmurlu geceler”. Oluştur/düzenle/sil, öğe ekle/çıkar. Koleksiyon silmek içindeki eserleri silmez.
- Etiketler Türkçe büyük/küçük harf normalizasyonu ile tekilleştirilir; arayüz özgün gösterim adını korur.
- Öğe silme önce çöp kutusuna taşıma şeklindedir. İlgili seanslar/alındılar/notlar silinmez, normal listelerden gizlenir. Çöp kutusundan geri al veya kalıcı sil seçeneği vardır.
- Liste/detay verileri kaydetme sonrası aynı anda güncellenir. Hata durumunda form açık kalır; başarı bildirimi yalnız işlem tamamlanınca gösterilir.

## Günlük ve takvim

- Günlük defteri tarihli kayıt listesi açar. Kayıt: başlık, gün, ruh hâli (`calm`, `happy`, `focused`, `tired`, `melancholic`, `excited` veya boş), düz metin gövde, isteğe bağlı ilişkili eserler.
- Aynı gün birden fazla kayıt olabilir. Gövde için limit 50.000 karakter; v1 zengin metin/HTML editörü gerektirmez.
- Açık kaydedilmemiş günlük taslağı 750 ms gecikmeyle yerel taslak tablosuna yazılır; kaydedildiğinde silinir. Uygulama açılışında “Taslağı sürdür / Sil” sunulur. Bu taslak, günlük kaydının yerine geçmez.
- Takvim ay görünümünde günlükler ve manuel seansları gösterir. Gün seçimi sağdaki olay listesini açar. Kişisel puan değişikliği gibi her veri değişikliği takvim etkinliği yapılmaz.
- Ay özeti: eklenen eser sayısı, günlüklü gün, toplam manuel oyun/okuma/dinleme dakikası. Kullanıcının kaydetmediği aktivite varsayılmaz.

## Oda özelleştirmesi

- Üç hazır ışık düzeni: Cozy Night (varsayılan), Violet Rain, Quiet Dawn. Aynı odanın palet/ışık değişimleridir; üç ayrı sahne değildir.
- Neon parlaklığı 0–100, amber lamba açık/kapalı, yağmur açık/kapalı, animasyon `sistem / azalt / tam`.
- İki duvar posterine koleksiyondan eser kapağı veya kullanıcının seçtiği yerel görsel atanır. Kaldırılınca özgün varsayılan poster görünür.
- Plak standına bir albüm atanabilir. Mobilyalar v1'de sabittir. Serbest sürükleme, eşya satın alma veya oda genişletme v1 dışıdır.
- Ortam sesi v1'de yoktur. Otomatik müzik veya yağmur sesi başlamaz.

## Genel arama ve ayarlar

Genel arama yalnız yerel kayıtları arar: eser başlığı, üretici, kişisel not, etiket, günlük başlığı/gövdesi. İnternette arama ayrı “Katalogda ara” bölümüdür; her yazılan özel not internete gönderilmez. Sonuçlar türüne göre gruplandırılır.

Ayarlar: görünüm/hareket, katalog bağlantıları, yedek dışa aktar/geri yükle, çöp kutusu, uygulama hakkında/kaynaklar. “Tüm veriyi sıfırla” v1'de eklenmez; gereksiz veri kaybı yolu oluşturulmaz.

## V1 dışındaki işler

Steam/Spotify hesap senkronizasyonu, müzik yayını, yapay zekâ önerileri, sosyal paylaşım, bulut veritabanı, mobil uygulama, çoklu oda, gerçek zamanlı 3D, achievements API, canlı süre sayacı, uygulama içi otomatik güncelleme, App Store yayını ve genel ticari dağıtım sonraki sürümlerdir. Önceki beyin fırtınasındaki ayrı uygulama fikirleri Hush'un zorunlu kapsamına eklenmez.
