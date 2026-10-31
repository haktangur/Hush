# Hush — proje kuralları

## Hedef ve kaynaklar
- Bu proje, Türkçe cozy cyberpunk dijital oda temalı kişisel macOS uygulamasıdır.
- Ürün, mimari ve kabul ölçütleri `docs/01`–`docs/08` dosyalarındadır. Kodlamadan önce bunları oku. `contracts/` başlangıç sözleşmelerini içerir.
- Kullanıcının son talimatı bu dosyanın önündedir. Uyumsuzluğu karar kaydında açıkla.
- İlk teslim Apple Silicon `.app` ve `.dmg` içerir; yalnız tarayıcı demosu yeterli değildir.

## Uygulama
- Tauri 2, React, TypeScript strict, Vite, Rust ve SQLite. Tek repo; backend sunucusu yok.
- UI -> tipli istemci -> Tauri komutları -> Rust servisleri -> SQLite/dosya sistemi. Bileşenlerde SQL, genel dosya erişimi veya doğrudan katalog isteği yok.
- Kişisel veri Application Support'ta; sırlar Keychain'de. Üretim verisi, anahtar, günlük içeriği ve kapaklar Git'e girmez.
- SQLite migration'ları eklemeli olsun. Uygulanmış migration'ı değiştirme. Veri kurtarmak yerine veritabanını sıfırlayan hata çözümü yasaktır.
- Formlar kaydedilmiş gibi davranmadan gerçek işlem sonucunu beklesin. Hatalarda kullanıcı metni korunsun.
- Türkçe UI metinlerini `src/i18n/tr.ts` içinde tut; kod ve dosya adları İngilizce olabilir.
- Oda öğeleri, normal menü ve klavye aynı ekranları açar. Hareket azaltma ve odak yönetimi zorunlu.
- Kullanıcı verisi boş başlayabilir; sahnenin mobilyaları buna rağmen tamamlanmış görünür. Demo kayıtları sessizce eklenmez.
- API bilgileri yokken elle ekleme çalışır. Gizli anahtar frontend'e, `.env` dosyasına veya yedeğe konmaz.

## Çalışma ve GitHub
- `docs/06-UYGULAMA-PLANI.md` sırasını izle, `docs/PROGRESS.md` güncel olsun.
- Mevcut kullanıcı değişikliklerini koru. Başka projeye ait Git remote'unu değiştirme.
- Kullanıcının başlangıç talimatı kapsamında private GitHub deposunu bağla, commit/push ve CI yapılandırmasını tamamla.
- Zorlayıcı push, geçmiş silme, sır yazdırma veya ücretli hizmet açma yok. Kimlik bilgisi eksikliği yerel kodlamayı durdurmasın.
- Paket sürümlerini doğrula ve lockfile'ları commit et. Bağımlılıkları görev sırasında gereksiz yere yükseltme.

## Tamamlanma
- `npm run check`, Rust testleri, UI/IPC kontrolleri ve gerçek macOS smoke testleri geçsin.
- Düzeltilebilir hatayı belgeleyip bırakma; çöz. Ortam engeli varsa kapsamını açıkça belirt.
- Release paketinde WDIO/test komutları, test veri dizini override'ı ve mock adaptörü bulunmasın.
- Son mesajda uygulama çıktıları, GitHub bağlantısı, geçen testler ve kalan gerçek sınırlar yer alsın.
