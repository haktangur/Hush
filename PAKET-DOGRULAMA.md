# Mimari paketinin doğrulanması

**Tarih:** 1 Kasım 2026.

Bu rapor mimari dosyalarına ve başlangıç sözleşmelerine aittir. Hush uygulamasının build, runtime, native UI, API veya yedek/restore kabul testlerinin geçtiği anlamına gelmez; uygulama henüz geliştirilmemiştir.

## Yapılan kontroller

- SQLite başlangıç migration'ı boş bellek DB'sinde çalıştırıldı: 19 tablo, schema version 1, integrity_check başarılı, foreign_key_check temiz.
- Üç eser türünün başlangıç kayıtları doğrulandı.
- Yanlış türün details tablosuna bağlanması, albümün geçersiz durumu, geçersiz sayfa/puan/seans sınırları reddedildi.
- Sağlayıcı kimliği tekrarı ve posterin aynı anda iki farklı kaynak alması reddedildi.
- Eser kalıcı silindiğinde günlük gövdesinin korunması, link ve slot referanslarının temizlenmesi doğrulandı.
- Albüm parçaları ve kitap alıntılarının parent silmede cascade davranışı doğrulandı.
- Koleksiyon silmenin eserleri koruduğu doğrulandı.
- Oda yerleşiminde sekiz benzersiz hotspot ve dekor koordinatlarının 1600×1000 sahne sınırlarında kaldığı doğrulandı.
- Markdown içindeki yerel bağlantılar, kapalı kod blokları ve eski çalışma adının kalmaması kontrol edildi.

**Sonuç:** Bu kapsamda 15 otomatik kontrol geçti.

TypeScript sözleşmesi Node 24 type-stripping yoluyla parse edildi; default ayarların Türkçe ve Cozy Night olduğu doğrulandı. Bu işlem `tsc` statik typecheck veya gerçek Rust/TypeScript IPC uyumluluk testi değildir; bunlar uygulama geliştirmesinde yapılır.

Varsayılan paletin metin/arka plan kontrastları hesaplandı: ana metin 12,80:1–16,48:1, ikincil metin 7,49:1–9,64:1. Bu sayılar düz renk token'ları içindir; gerçek sahne, saydamlık ve tüm tema durumları görsel QA gerektirir.

## Henüz yapılmayanlar

- Uygulama kodlaması, Tauri build veya `.app`/`.dmg` üretimi.
- GitHub deposu oluşturma/push, CI çalıştırma, Release hazırlama.
- Gerçek katalog araması veya kullanıcının IGDB bilgileriyle bağlantı.
- Nihai SVG oda çizimi, gerçek UI/VoiceOver/performance testi.
- Uygulamanın gerçek backup/restore, fault injection ve restart kabul testleri.

Bunlar `docs/06-UYGULAMA-PLANI.md` ve `docs/07-GITHUB-TEST-VE-TESLIM.md` ile geliştiriciye verilmiş zorunlu işlerdir.
