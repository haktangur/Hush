# Codex'e başlangıç talimatı

Aşağıdaki talimatı Hush proje klasöründe çalışan Codex'e ver:

---

Bu projede geliştirici olarak çalış. Hush adlı, Türkçe arayüzlü, kendi Mac'imde kullanacağım cozy cyberpunk dijital oda uygulamasını bu klasördeki mimariye göre tamamla. Mimariyi yeniden tasarlamak veya sadece plan sunmak yerine çalışır uygulamayı geliştir.

Önce `AGENTS.md`, `README.md` ve `docs/01`–`docs/08` dosyalarını oku. `contracts/` içindeki SQL, TypeScript, oda yerleşimi ve tasarım değişkenlerini başlangıç sözleşmeleri olarak kullan. Belirgin bir çelişki veya çalışma ortamında gerçek bir uyumsuzluk varsa en küçük düzeltmeyi yap, gerekçesini karar kaydına ekle. Ürün kapsamını sessizce küçültme.

Hush'un ilk sürümünde oyun terminali, plak istasyonu, kitaplık, günlük defteri, takvim, iki değiştirilebilir poster ve ışık ayarları olan tamamlanmış bir 2D oda istiyorum. Oda öğeleri gerçek, kalıcı koleksiyon ekranlarını açsın. Oyun, albüm ve kitap ekleme/düzenleme/silme, puanlar, notlar, kitap ilerlemesi ve alıntılar, oyun seansları ve ekran görüntüleri, albüm parçaları, etiketler, koleksiyonlar, arama ve yedekleme/geri yükleme çalışsın. Mekânın görsel kalitesi de işlevsellik kadar önemli: sahne kaba dikdörtgen kutulardan veya bir arka planın üstüne rastgele konan düğmelerden oluşmasın.

Tauri 2 + React + TypeScript + Rust + SQLite kullan. Ana ekran bir web sitesi değil, paketlenmiş macOS uygulaması olsun. Kullanıcı verisi Application Support altında saklansın; localStorage ana veri kaynağı olmasın. API erişimi Rust katmanından yapılsın. Kitaplar Open Library'den, albümler MusicBrainz ve Cover Art Archive'dan aranabilsin. IGDB oyun adaptörü ve bağlantı ayarı hazır olsun; Twitch/IGDB bilgilerimi sonra gireceğim. Bu bilgiler yokken oyunları elle ekleyebileyim. Anahtarları benden sohbete yazmamı isteme; macOS Keychain'de saklayan ayar ekranını hazırla.

`docs/06-UYGULAMA-PLANI.md` sırasına göre çalış. İlk olarak iskelet ve çalışan veri kaydı, ardından tamamlanmış oda ve kalan özellikler gelsin. Gerçek `.app` derlemesini sona bırakma; ilk aşamada basit bir paket üretip ortamı doğrula. İlerlemeyi `docs/PROGRESS.md` içinde güncel tut. Her aşamada geçen kontrolleri, kalan işleri ve varsa engeli yaz. Yalnızca bir prototip, tarayıcı önizlemesi veya sahte verilerle çalışan demo teslim edip durma.

**GitHub entegrasyonunu yap:** mevcut Git deposunu ve remote'u incele. Bu projeye ait bir remote varsa onu kullan. Yoksa GitHub oturumu açıksa benim kişisel hesabımda `hush` adıyla private depo oluştur; isim doluysa `hush-desktop` kullan. Mevcut ilgisiz bir depoyu sahiplenme veya üzerine yazma. GitHub bağlantısını kur, küçük anlamlı commit'ler oluştur, kaynakları push et, test ve macOS build için Actions iş akışlarını ekle. Kimlik doğrulaması veya depo sahibi seçimi gerçekten eksikse sadece o bilgiyi sor; yerel geliştirmeyi sürdür. Kişisel verilerimi, API sırlarını, sertifikaları, test ekran görüntülerindeki özel verileri veya yedekleri depoya koyma. GitHub uygulamanın son kullanıcı arayüzüne eklenen bir özellik değildir; projenin kaynak kodu ve sürüm altyapısıdır.

Son teslimde Apple Silicon için çalışan `Hush.app`, kurulabilir `.dmg`, kaynak kod, GitHub depo bağlantısı, CI sonucu ve Türkçe kurulum yönergesi ver. Yerel build ad-hoc imzalı olabilir; Apple notarization yapılmadıysa bunu açıkça belirt. GitHub Release'i draft olarak hazırla. Apple hesabı satın alma, genel yayın veya ücretli hizmet açma. Üretim paketinde test sürücüsü, sahte veri adaptörü ve gizli anahtar bulunmasın.

`docs/07-GITHUB-TEST-VE-TESLIM.md` kabul senaryolarını uygula. Gerçek uygulamada kapatıp açınca verinin kalmasını, kapakların internet olmadan görünmesini ve yedekten geri dönüşü doğrula. Görsel kontrolleri en az 1440×900 ve 1100×720 pencerelerde yap. Çalıştırmadığın testi geçti diye yazma; derleme başarısını tek başına uygulamanın tamamlanması sayma.

Makul uygulama ayrıntılarını kendin çöz. Yalnızca ürün kararını gerçekten değiştiren, kullanıcı hesabı gerektiren veya mevcut veriyi etkileyen bir belirsizlikte sor. Bitene kadar kodlama, doğrulama ve düzeltme işini sürdür.

---
