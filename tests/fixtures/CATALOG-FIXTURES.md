# P5 sentetik katalog kabulü

`tests/native/catalog-fixtures.mjs` yalnız yeni geçici test dizinine hash adlı cache dosyaları yazar. OL900000001W/M ve 1111/2222/3333 biçimindeki MBID'ler gerçek servis kayıtları olarak sunulmaz; canlı servise gönderilmez. JSON tamamen sentetiktir. Albüm kapağı mevcut özgün `room-album.webp` fixture'ıdır (kaynak: ROOM-FIXTURES.md).

`npm run test:native -- --catalog` gerçek macOS Tauri/WebKit, Rust adaptörleri ve SQLite ile çalışır. IPC mock veya HTTP mock adaptörü yoktur; arama/ayrıntı cache girdileri sentetiktir. Pozitif kapak cache’i gerçek decoder/import'tan geçer. Negatif kapak cache’i ve bozuk görsel fixture'ı ayrı denenir.

`--catalog-live` ayrıca insan tarafından başlatılan bir Open Library canlı arama/baskı/kayıt/refresh akışını açar; sonuçlar `live-open-library*.json` dosyalarına ayrı yazılır. Varsayılan test canlı ağ gerektirmez. Canlı servis metadata ve kapakları Git'e eklenmez.

Offline ikinci süreçte e2e-only `offline` işareti Rust HTTP gönderimini reddeder; WebView zaten uzak görsel indirmez. Bu sistemin Wi-Fi bağlantısını kapatma iddiası değildir. Test root, cache ve Keychain namespace üretimden ayrıdır. Sır içermeyen `http-attempts.txt` sadece gönderim sayısını tutar. Release'te bu dallar derlenmez.

Rust HTTP hata testleri yalnız loopback TCP fixture sunucusuna gerçek reqwest çağrıları yapar. Keychain testi her çalıştırmada rastgele sentetik değer/benzersiz servis oluşturur, okur, siler; gerçek IGDB hesabı veya OAuth bağlantısı testi değildir.
