# P2 sentetik görseller

- `room-cover.webp`: projenin özgün Hush ikonunun P1 Rust importuyla yeniden kodlanmış kopyası (1600×1600). Kişisel kapak değildir.
- `room-album.webp`: bu proje için üretilmiş 128×128 RGB ay/şehir test çizimi. Piksel kuralı: merkez (64,50), yarıçap 30 içi amber (221,164,106); y>84 ve x%24<16 için mavi (50,71,91); diğer pikseller (25,29,49). Kilitli Rust image crate'iyle WebP kaydedildi.

İkisi de proje içi özgün test varlığıdır. Production kaynaklarına dahil edilmez.
P2 runner yalnız yeni `HUSH_TEST_DATA_DIR` altında bu asset dosyalarını ve manifest satırlarını fixture olarak kurar. Eserler, ayarlar ve slot atamaları gerçek IPC/UI üzerinden kaydedilir. Bu fixture kurulumu native picker testi değildir; picker kabulü P1'de ayrı doğrulanmıştır.
