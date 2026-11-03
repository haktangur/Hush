# Hush — Codex geliştirme paketi

**Sürüm:** Mimari 1.0 · **Tarih:** 1 Kasım 2026 · **Durum:** Uygulamaya hazır tasarım; henüz uygulama kodu değildir.

Hush, oyunlarını, albümlerini, kitaplarını ve günlüğünü cozy cyberpunk bir dijital odada sakladığın, Türkçe bir macOS uygulamasıdır. Bu paket ürün kararlarını, mimariyi, tasarımı, veri sözleşmelerini ve teslim ölçütlerini tanımlar.

## Nasıl kullanılır?

1. ZIP dosyasını aç. `Hush-Codex-Paketi` klasörünün içeriğini yeni Hush proje klasörüne yerleştir. `AGENTS.md`, proje kökünde kalsın.
2. Bu proje klasörünü Codex'te aç.
3. `CODEX_BASLANGIC.md` dosyasındaki başlangıç talimatını Codex'e ver veya dosyayı okumasını iste.
4. Codex, `docs/06-UYGULAMA-PLANI.md` sırasını izleyerek uygulamayı geliştirir. Bu paketin teslim edilmiş olması, uygulamanın yazıldığı veya test edildiği anlamına gelmez.

## Kesinleşen ürün kararları

- Ad: **Hush**. Arayüz: **Türkçe**. Kullanım: tek kişinin kendi Mac'i.
- Ana ekran: çizim tarzında, katmanlı **2D cozy cyberpunk oda**; gerçek zamanlı 3D yok.
- Oyun terminali, plak istasyonu, kitaplık, günlük defteri, etkinlik takvimi, iki dijital poster ve masa lambası işlevsel.
- Otomatik arama **ve** elle ekleme var. Kitap: Open Library. Albüm: MusicBrainz + Cover Art Archive. Oyun: IGDB; kullanıcı bağlantıyı sonradan kurabilir.
- Kişisel kayıtlar, puanlar, notlar ve kapakların yerel kopyaları Mac'te saklanır. Sunucu, hesap sistemi, abonelik ve bulut eşitleme yok.
- Spotify/Apple Music bağlantıları dışarıda açılır. Uygulama müzik akışı veya Spotify oynatıcısı değildir.
- Kaynak kodu için GitHub entegrasyonu yapılır. Yeni depo varsayılan olarak **private** olur. Kişisel koleksiyon ve API sırları GitHub'a yüklenmez.
- Hedef: Apple Silicon macOS uygulaması, `.app` + `.dmg`. Hazırlık sırasında bu Mac `arm64`, macOS `26.6.2` olarak görüldü; geliştirici kendi çalışma ortamını yeniden doğrular.
- Mimari: **Tauri 2 + React + TypeScript + Rust + SQLite**.

## Okuma sırası

| Dosya | Amaç |
|---|---|
| [CODEX_BASLANGIC.md](CODEX_BASLANGIC.md) | Codex'e verilecek doğrudan uygulama talimatı |
| [AGENTS.md](AGENTS.md) | Proje boyunca geçerli kısa geliştirme kuralları |
| [01-URUN-VE-KAPSAM.md](docs/01-URUN-VE-KAPSAM.md) | Kullanıcı akışları, ekranlar ve sürüm sınırları |
| [02-MIMARI.md](docs/02-MIMARI.md) | Teknoloji seçimi, modüller, depolama ve güvenlik |
| [03-ODA-VE-TASARIM.md](docs/03-ODA-VE-TASARIM.md) | Odanın yerleşimi, nesneler, animasyonlar ve görsel ölçütler |
| [04-VERI-VE-KOMUTLAR.md](docs/04-VERI-VE-KOMUTLAR.md) | Veri modeli, işlemler ve yedekleme |
| [05-KATALOG-ENTEGRASYONLARI.md](docs/05-KATALOG-ENTEGRASYONLARI.md) | API'ler, anahtar yönetimi, önbellek ve hata davranışları |
| [06-UYGULAMA-PLANI.md](docs/06-UYGULAMA-PLANI.md) | Bağımlılık sırasına göre görevler ve kabul kapıları |
| [07-GITHUB-TEST-VE-TESLIM.md](docs/07-GITHUB-TEST-VE-TESLIM.md) | GitHub, testler, macOS paketleme ve teslim |
| [08-KARARLAR-VE-KAYNAKLAR.md](docs/08-KARARLAR-VE-KAYNAKLAR.md) | Karar kayıtları, varsayımlar ve resmi kaynaklar |
| [contracts/](contracts/) | Başlangıç SQL şeması, TypeScript veri tipleri, oda yerleşimi ve tasarım değişkenleri |
| [PAKET-DOGRULAMA.md](PAKET-DOGRULAMA.md) | Bu mimari paketinde kontrol edilenler ve uygulama aşamasında kalanlar |

## Paket kapsamı hakkında

SQL ve veri/oda sözleşmeleri başlangıç uygulamasına taşınabilir. Bunlar hazır çalışan bir Tauri projesi değildir. Paket sürümleri, Rust araç zinciri ve GitHub Actions sürümleri geliştirme başlangıcında uyumluluk kontrolünden sonra sabitlenir. Çalıştırılmamış uygulama testleri geçmiş gibi raporlanmaz.

Görsel üretim hizmeti, ücretli API veya Apple Developer üyeliği ilk yerel sürümün ön koşulu değildir. Uygulama içindeki görseller özgün SVG çizimiyle üretilebilir. Dağıtım için Apple imzası/noter onayı daha sonra ayrı olarak eklenebilir.

## Güncel geliştirme durumu — 3 Kasım 2026

P0–P6 kabul kapıları tamamlandı; P7 kalite ve final macOS paketleme henüz açık. Güncel sonuçlar [test raporunda](docs/TEST-REPORT.md), devir bilgileri [HANDOFF](docs/HANDOFF.md) ve [P7 başlangıç promptunda](docs/NEXT_CHAT_PROMPT.md).

P6'yı ayrı sentetik veriyle tekrar çalıştırmak için `npm run test:native:p6`. Bu komut test derlemesi kullanır; tam yedek eşdeğerliği, bozuk arşivler ve üç gerçek SIGKILL/yeniden açılış noktasını doğrular. Dosya seçimi e2e fixture seçimiyle sağlanır; IPC/depolama gerçektir. Disk-dolu senaryosu kontrollü hata enjeksiyonudur. Test arşivleri/geçici veriler Git dışındadır.

Ayarlar → Yedekleme ve geri yükleme bölümünden `.hushbackup` dışa aktarılabilir. Geri yükleme önce özeti gösterir ve mevcut verinin değiştirilmesi için açık onay ister. Yedek kişisel içerik taşır, şifreli değildir; Keychain bağlantı sırlarını içermez. Eski generation kurtarma için diskte tutulur. Final `.app/.dmg` ve GitHub bağlantısı henüz teslim edilmedi.
