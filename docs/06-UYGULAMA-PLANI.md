# 06 — Kodlama planı ve kabul kapıları

Bu plan kodlama sırasıdır; farklı ürün seçenekleri listesi değildir. Her aşamada uygulama açılabilir durumda kalsın. Tahmini gün/saat vaat edilmez. Aşamalar tamamlandıkça `docs/PROGRESS.md` içine tarih, commit, doğrulama ve kalan engel kaydedilir. Gerçek engel yoksa kullanıcıdan her aşama sonunda yeniden izin isteme.

## P0 — Proje ve masaüstü iskeleti

**İş:** Çalışma klasörünü ve varsa mevcut kodu oku. Node, npm, Rust, Xcode Command Line Tools ve Mac mimarisini doğrula. Paketler için uyumlu kararlı sürümleri sabitle. Tauri React TypeScript iskeletini var olan mimari dosyalarını silmeden oluştur. App identifier, native pencere, Türkçe UI merkezi, test altyapısı ve ilk Rust command'i ekle. Temel `.gitignore` oluştur; GitHub remote/auth kurulumunu başlat.

**İlk dosyalar:** package.json, tsconfig, Vite config, src/app, src/i18n/tr.ts, src-tauri/lib.rs, tauri.conf.json, capabilities, TOOLCHAIN.md, PROGRESS.md, CI.

**Kabul:** `npm run dev` gerçek Tauri penceresi açar; basit `app_bootstrap` komutu UI'a ulaşır. İlk ad-hoc `.app` Apple Silicon'da açılır. Proje Git deposudur; remote hazır veya auth engeli açıkça kaydedilmiştir. Bu aşamada oda tamamlanmış sayılmaz.

## P1 — Kalıcı veri ve bir uçtan uca eser akışı

**Bağımlılık:** P0.

**İş:** Data path/active generation, SQLite migration runner, schema, ayarlar seed'i, error envelope, LibraryService ve tipli client. İlk olarak manuel kitap ekle/düzenle/listele/trash/restore. Asset picker, decode, thumb, save ve görüntüleme. Basit panel ve form yapısı.

**Kabul:** Bir kitap ve yerel kapak eklenir; uygulama kapatılıp açıldığında ikisi de kalır. Orijinal kapak dosyası başka yere taşınsa dahi uygulamadaki kapak görünür. Yanlış sayfa sayısı reddedilir. Kaydetme hatasında metin kaybolmaz. Migration ve kind mismatch testleri geçer.

## P2 — Odanın tamamlanması ve gezinme

**Bağımlılık:** P1 (gerçek veriye bağlanmak için).

**İş:** room-layout sözleşmesini kullanarak tüm SVG katmanları, mobilyalar, şehir, yağmur, neon, lamba, terminal, pikap, kitaplık, defter, takvim, posterler. Hotspot, tooltip, nav şeridi, tek panel state machine, klavye ve focus. Üç ışık düzeni, hareket ayarı, poster/plak ataması.

**Kabul:** Sekiz hotspot iki pencere boyutunda çalışır. Kitaplık gerçek kaydı açar; diğer panel girişleri yerindedir. Kullanıcı verisi yokken oda tamamlanmış görünür. Oda tasarımındaki perspektif/ışık/okunabilirlik kontrol edilir. Kaba wireframe bu aşamanın bitişi değildir. Ayarlar ve posterler restart sonrası korunur.

## P3 — Tüm yerel koleksiyon özellikleri

**Bağımlılık:** P1 + P2.

**İş:** Ortak library bileşenlerinden oyun/albüm/kitap detay ve form ekranları. Oyun progress/return note/session/screenshots; albüm tracks/favorites/notes/external links; kitap progress/quotes. Etiketler, özel koleksiyonlar, filtre/sort/pagination ve item duplicate UX. Trash ve kalıcı silme.

**Kabul:** Her türden en az iki sentetik kayıt gerçek DB üzerinde oluştur/düzenle/sil/geri al akışından geçer. Alt kayıtlar doğru türe bağlanır. Koleksiyon silmek eserleri silmez. Eser silme/restore poster fallback'ini doğru günceller. Tüm görünür düğmeler gerçek işlem yapar.

## P4 — Günlük, takvim ve yerel arama

**Bağımlılık:** P3.

**İş:** Günlük CRUD, ilişkili eserler, mood, local draft autosave/recovery. Ay takvimi, gün paneli ve doğru süre/toplam hesapları. Cmd+K yerel arama, Türkçe normalizasyon, filtre boş durumları. Ayar/çöp kutusu ekranlarını bitir.

**Kabul:** Aynı güne birden fazla günlük eklenir. Yazılmakta olan taslak yeniden açılışta kurtarılır. Günlük günü saat dilimi değişince kaymaz; seanslar seçili timezone'a göre gruplanır. `I`, `İ`, `ı`, `i` fixture'ları beklendiği şekilde çalışır. Yerel arama internet isteği üretmez.

## P5 — Otomatik katalog araması

**Bağımlılık:** P3; P4'ü bloke etmez ama sıra korunabilir.

**İş:** Rust CatalogProvider katmanı, HTTP limit/cache/timeout, Open Library arama+baskı+kapak, MusicBrainz release-group+release+track ve CAA kapak. IGDB adapter, Keychain configuration, token single-flight. Search/preview/save akışı ve metadata refresh diff.

**Kabul:** Kitap ve albüm için canlı arama mümkünse doğrulanır; servis kesintisi varsa fixture testleriyle ayrım raporlanır. Kaydedilen sonuçlar offline açılır. IGDB bilgisiz açılışta elle oyun ekleme sorunsuzdur; credentials UI/testleri hazırdır. 429/timeout/kapaksız sonuç uygulamayı bozmaz. Kişisel puan/not metadata refresh'te korunur. API sırları frontend bundle, log, DB ve Git'te bulunmaz.

## P6 — Yedekleme ve kurtarma

**Bağımlılık:** P4 + P5 veri biçimleri sabit.

**İş:** Manifest'li backup, online DB snapshot, asset tutarlılığı, staged restore, generation pointer commit, eski generation kurtarma. İptal/bozuk arşiv/uyumsuz sürüm/disk dolması akışları. UI restore önizleme ve açık veri değiştirme onayı.

**Kabul:** Tüm türler, seanslar, quotes/tracks, günlük/draft, posterler, ayarlar ve kapaklar export→restore sonrası eşdeğer. Keychain bilgileri yedeğe girmez. Bozuk backup mevcut DB'yi etkilemez. Commit öncesi ve sonrası kesinti testleri veri kaybı olmadan açılır. Yalnız “ZIP oluştu” testi yeterli değildir.

## P7 — Kalite, GitHub ve teslim

**Bağımlılık:** P0–P6.

**İş:** Gerçek macOS E2E, accessibility, iki pencere boyutunda görsel QA, offline/network failure testleri, 5.000 eserle performans ölçümü. App icon, native menü, about/attribution. CI ve draft release workflow. Release ad-hoc sign, `.app` ve `.dmg`, Türkçe kurulum yönergesi, TEST-REPORT.

**Kabul:** 07 numaralı belgedeki zorunlu matris tamam. Test plugin'leri production derlemesinde yok. `.dmg` üzerinden uygulama açma kontrol edildi. GitHub kaynakları push edilmiş, CI sonucu biliniyor. Son mesajda artifact path/link, repository URL, test sonucu ve gerçek sınırlar var.

## İlerlemenin dosya formatı

```markdown
# Hush ilerleme
## Şu an
- Aktif aşama:
- Son çalışan commit:
- Sonraki somut iş:
## Aşamalar
- [ ] P0 — masaüstü iskeleti
- [ ] P1 — kalıcı veri
- [ ] P2 — oda
- [ ] P3 — koleksiyonlar
- [ ] P4 — günlük/takvim/arama
- [ ] P5 — kataloglar
- [ ] P6 — yedek/kurtarma
- [ ] P7 — test/GitHub/teslim
## Doğrulananlar
- Tarih, komut/akış, sonuç, kullanılan sentetik veri.
## Engeller
- Yalnız gerçek engel; etkilediği iş ve devam edilebilen iş.
## Karar değişiklikleri
- Değişiklik, gerekçe ve doküman/şema uyarlaması.
```

Bir aşama eksikken sonraki aşamanın UI'ını hazırlamak mümkündür; ancak önceki kabul kapısı geçmeden tamamlandı işareti konmaz. Kullanım limiti veya bağlam kesintisinde PROGRESS.md üzerinden devam edilir; proje yeniden sıfırlanmaz.
