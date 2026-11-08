# Hush 0.1.0 — Kurulum ve kullanım

Apple Silicon (M serisi), macOS 14 veya sonrası gerekir. Intel için doğrulanmamıştır.

1. `Hush-0.1.0-aarch64.dmg` dosyasını çift tıklayarak aç.
2. Hush uygulamasını Applications/Uygulamalar bağlantısına sürükle.
3. Uygulamalar’dan Hush’u aç. Yerel paket ad-hoc imzalıdır; Apple notarization yoktur. macOS engellerse yalnız güvendiğin bu dosya için Sistem Ayarları → Gizlilik ve Güvenlik bölümündeki açma onayını kullan. Sistem güvenlik korumalarını kapatma.
4. “Odaya gir” veya “Atla” ile başla. Uygulama boş başlar, demo kayıt eklemez. İnternet olmadan elle kayıt kullanılabilir.

Odadaki eşya veya alt menü aynı koleksiyon ekranını açar. Kitaplar/Oyunlar/Albümler → Ekle ile elle kayıt veya katalog araması seçilir. Kaydet tamamlanmadan başarı gösterilmez. Kapaklar içeri aktarılınca uygulamanın yerel kopyası saklanır; orijinal dosyanı taşıyabilirsin. Günlük taslakları kısa gecikmeyle yerel kaydedilir, sonraki açılışta “Taslağı sürdür” ile açılır.

Cmd+1 oda; Cmd+2–6 oyun/albüm/kitap/günlük/takvim; Cmd+7/8 poster; Cmd+9 plak standı; Cmd+L lamba; Cmd+K yerel arama; Cmd+, ayarlar. Cmd+N odada yeni kitap, koleksiyon veya günlük panelinde o bağlama uygun yeni kayıt açar. Açık eser/günlük formunda Cmd+S kaydeder. Escape en üst paneli kapatır; kirli formda seçim ister. Tab/Shift+Tab ile gezinilir. Ayarlar → Hareket → Azalt animasyonları durdurur; Sistem seçeneği macOS hareket tercihini izler.

Cmd+W veya kırmızı kapatma düğmesi pencereyi gizler; Dock simgesinden yeniden açılır. Cmd+Q uygulamadan çıkar; yalnız görüntüleme amaçlı açık panel bunu engellemez. Kaydedilmemiş bir form (yazılmış metin, değişen kapak, kaydedilmemiş katalog önizlemesi) veya süren bir kaydetme/geri yükleme varsa Cmd+W/Q durur ve formda uyarı görünür: Kaydet veya Vazgeç/Değişiklikleri at, sonra yeniden Cmd+W/Q kullan. Not: Dock menüsündeki “Çık” ve Mac'i kapatma bu korumayı atlar; açık formu önce kaydet.

Veriler `~/Library/Application Support/app.hush.desktop/` içinde tutulur; uygulama paketinin içinde değildir. `active.json` aktif generation'ı seçer. Bu dosyayı veya SQLite dosyalarını elle değiştirme/silme. Eski geri yükleme generation'ları kurtarma için diskte kalır. Arama cache'i silinebilir sistem cache dizinindedir. Sırlar macOS Keychain’dedir.

Ayarlar → Yedekleme ve geri yükleme → Yedek dışa aktar ile `.hushbackup` oluştur. Dosya günlük/not/görsellerini içerir ve **şifreli değildir**; güvenli bir yerde sakla. API sırları dahil değildir. Geri yüklemeden önce mevcut verinin yedeğini al; “Yedek seç ve incele” özetinden sonra açık onay ver. Bozuk veya yeni şemalı yedek mevcut verinin yerine geçmez. Pointer haricen bozulduysa uygulama hata gösterir; boş veritabanı oluşturmaz. Özel kurtarma seçicisi yoktur; mevcut klasörü koru.

Open Library kitapları, MusicBrainz/CAA albüm bilgisi ve kapaklarını sağlar. Katalog araması için internet gerekir; kaydedilmiş kayıt/kapaklar çevrimdışı açılır. MusicBrainz kullanmadan önce Ayarlar’a sana ait geçerli HTTPS iletişim profilini girip Kaydet. IGDB isteğe bağlıdır: Twitch uygulama kimliği ve secret yalnız uygulama ayarında girilir, Keychain’e yazılır; sohbete veya `.env` dosyasına koyma. Bağlantı olmadan elle oyun eklenebilir. Spotify/Apple Music düğmeleri dış tarayıcıda açılır; Hush müzik çalmaz.

Sürümün gerçek testleri ve sınırları `TEST-REPORT.md`, lisans/kaynak bilgileri `THIRD-PARTY-NOTICES.md` içindedir. Kaynak kod: https://github.com/haktangur/Hush (public). Draft release henüz oluşturulmadı.
