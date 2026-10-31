# 07 — GitHub, test ve macOS teslimi

## GitHub kurulumu

GitHub entegrasyonu geliştirme sürecinin bir parçasıdır; Hush son kullanıcı ekranına GitHub login veya GitHub API menüsü ekleme.

1. `git status`, repo kökü ve remote'ları incele. Üst klasörün başka projesine yanlışlıkla commit yapma; Hush bağımsız repo olacaksa kendi kökünde oluştur.
2. Mevcut Hush remote'u varsa koru. Yoksa GitHub connector veya `gh` mevcut kimliğini kontrol et. Kimlik yoksa kullanıcıyı resmi giriş akışına yönlendir; token'ı sohbete yazmasını isteme.
3. Bu başlangıç talimatı, kullanıcının kişisel hesabında yeni **private** kaynak kod deposu kurmayı kapsar. Önerilen ad `hush`; doluysa `hush-desktop`. İkisi de varsa başka projeyi yeniden kullanmadan hedefi sor.
4. İlk commit öncesi `.gitignore`, gerekli kaynak/doküman listesi ve sır kontrolü. `git add .` öncesinde içerik gözden geçir.
5. İlk repo kuruluşunda `main` bootstrap commit'i. Sonrasında `feat/<konu>`, `fix/<konu>` dalları; PR açıklaması değişiklik + doğrulama içerir. Kullanıcı çalışırken kendi onaysız değişikliklerini taşıma/silme. Auto-merge/force-push ekleme.
6. Milestone commit'leri küçük ve anlamlı olsun. Çalışma sonunda branch'ler push edilmiş olsun; hazırlanmış PR varsa bağlantısını ver. GitHub yetkisi engelliyse yerel repo/commit/CI dosyalarını tamamla ve kalan tek giriş adımını belirt.

Yeni depo için CLI örneği, ancak auth ve hedef doğrulandıktan sonra:

```sh
gh repo create hush --private --source=. --remote=origin --push
```

`gh repo create` mevcut yerel kaynağı ve remote ayarını destekler. [GitHub CLI dokümanı](https://cli.github.com/manual/gh_repo_create)

## Depoya girmeyecek dosyalar

`node_modules`, `dist`, `src-tauri/target`, `*.app`, `*.dmg`, `*.hushbackup`, kişisel DB/WAL/SHM dosyaları, çalışma logları, `.env*` sırları, `.p12`, `.p8`, `.key`, kullanıcı görselleri. İzin verilen `.env.example` ancak gerçek secret içermez; v1 runtime Keychain kullandığı için secret env örneği gerekmez.

`package-lock.json`, `Cargo.lock`, migration'lar, özgün oda görselleri ve lisans kaydı Git'e girer. Sentetik fixture ve görsel QA çıktıları kişisel veri içermediği doğrulanırsa tutulabilir. Kişisel yedekleri repo private diye yükleme.

## Komut sözleşmesi

Uygulama iskeleti aşağıdaki package script'lerini oluşturur. Burada listelenmeleri, henüz bu pakette çalıştıkları anlamına gelmez.

| Komut | Yapacağı iş |
|---|---|
| `npm ci` | Lockfile üzerinden bağımlılık kurulumu |
| `npm run dev` | Tauri development; Vite beforeDevCommand ile başlar |
| `npm run dev:web` | Yalnız UI geliştirme; açık development/test adaptörü |
| `npm run typecheck` | TypeScript strict kontrol |
| `npm run lint` | ESLint ve erişilebilirlik temel kuralları |
| `npm run test` | Vitest + Testing Library |
| `npm run build:web` | UI production build |
| `npm run check` | typecheck + lint + UI test + build:web |
| `npm run test:native` | Test feature ile Tauri build + WDIO embedded |
| `npm run build:mac` | Release Apple Silicon `.app` + `.dmg`, test feature yok |

Rust kontrol komutları:

```sh
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --locked
```

`beforeBuildCommand` yalnız `npm run build:web` çağırır. `build:mac` Tauri CLI'ı çağırır; kendisini tekrar çağıran build döngüsü oluşturma.

## Test katmanları

**Rust:** Validasyon, normalizasyon, status geçişleri, migration upgrade, FK/cascade, revision conflict, aynı UUID retry, provider mapping, url/redirect allowlist, cache/rate-limit, Keychain mock, asset decoder sınırı, backup integrity ve restore crash recovery. Testler temp dizinde çalışır, gerçek kullanıcının Application Support verisine erişmez.

**React:** Form alanları, validation, dirty close, focus return, error/empty/loading, filter/list invalidation ve hareket azaltma. UI testinin gerçek DB persistence kanıtı olduğu söylenmez.

**Native macOS:** WebdriverIO `@wdio/tauri-service`, `driverProvider:'embedded'`; test build'inde `tauri-plugin-wdio-webdriver`, gerekirse `tauri-plugin-wdio` + JS counterpart. Güncel Tauri belgeleri embedded yaklaşımın macOS'u desteklediğini açıklar. [Tauri WebDriver](https://v2.tauri.app/develop/tests/webdriver/), [WDIO plugin setup](https://webdriver.io/docs/desktop-testing/tauri/plugin-setup)

Test feature adı `e2e`; optional plugin dependencies ve explicit Cargo cfg ile gated. Test yetkileri ayrı capabilities/config dosyasında; production config bunları dahil etmez. `HUSH_TEST_DATA_DIR` override yalnız bu feature'da derlenir. Test driver sadece loopback üzerinde dinler. Release build'inde port/test commands/JS plugin import'u bulunmadığını kontrol et. UI E2E, asıl persistence akışında IPC mock kullanmaz.

Native test runner ortamı UI otomasyonuna izin vermiyorsa Rust ve UI testleri geçse dahi native E2E geçmiş sayılmaz. Gerçek Mac'te aynı adımlar elle yapılır ve rapora “manuel” yazılır; platform sınırlaması uydurulmaz.

## Zorunlu kabul matrisi

| ID | Senaryo | Beklenen |
|---|---|---|
| A01 | Temiz kurulum, internet kapalı | Boş ama bitmiş oda açılır, elle kayıt çalışır |
| A02 | Her hotspot ve menü/klavye eşdeğeri | Aynı ekran, doğru odak, Escape davranışı |
| A03 | Her tür ekle/düzenle, uygulamayı kapat/aç | Veriler ve kapaklar korunur |
| A04 | Orijinal import dosyasını taşı | Uygulamadaki kopya görünür |
| A05 | Kitap toplam/mevcut sayfa sınırları | Negatif/aşım reddedilir, not kaybolmaz |
| A06 | Seans ekle/sil, ay değiştir | Toplam süre ve takvim doğru |
| A07 | Parça yorumu, alıntı, screenshot | Doğru eserle ilişki, restart sonrası aynı |
| A08 | Journal taslağı, zorla kapat, yeniden aç | Son autosave taslağı kurtarılır |
| A09 | Trash/restore/purge ve poster | Gizleme/geri alma ve FK temizliği tutarlı |
| A10 | Yerel arama: Türkçe ve `%_` literal | Beklenen eşleşme, ağ isteği yok |
| A11 | Open Library + MusicBrainz arama→save | Önizleme/düzenleme, offline kalıcılık |
| A12 | IGDB hiç bağlanmamış | Elle oyun ekleme çalışır; ayar yönlendirmesi anlaşılır |
| A13 | Timeout/429/bozuk kapak/anahtar reddi | Veri kaybı yok, hata türü doğru, sonsuz retry yok |
| A14 | Katalog refresh | Kişisel not/puan/progress korunur |
| A15 | Full backup→değişiklik→restore | Veri ve kapak eşdeğer, Keychain dışarıda |
| A16 | Bozuk ZIP, traversal, yeni schema, disk dolması | Restore reddedilir, eski veri çalışır |
| A17 | Restore pointer commit öncesi/sonrası çökme | Eski ya da yeni tam veri, karışık state yok |
| A18 | İki form aynı revision | İkinci eski update conflict, sessiz veri ezme yok |
| A19 | Minimum/normal pencere, VoiceOver, reduced motion | Taşma/odak kaybı yok; loop durur |
| A20 | Minimize, 5.000 eser, 10.000 seans | Ölçüm raporu, kabul hedefleri değerlendirilir |
| A21 | DMG'den gerçek `.app` açılışı | Tauri dev server gerektirmeden çalışır |
| A22 | Release içeriği | Test plugin/mock/secret/personal data yok |

## GitHub Actions

`ci.yml`: pull_request + main/feature push. Varsayılan token permissions `contents: read`. Aynı branch eski CI run'ı cancel eden concurrency. Doğrulanmış güncel macOS runner etiketi ve hedef mimariyi TOOLCHAIN.md'ye kaydet. `npm ci`, Rust locked kurulumu, lint/typecheck/unit/build; Rust fmt/clippy/test; native test build ve smoke; production `.app` build. Cache anahtarları lockfile + toolchain içerir. Actions SHA'larını resmi depodan doğrulayarak pinle; bu belgede rastgele SHA üretme.

Hata durumunda yalnız sentetik verili test raporu/screenshot artifact yükle; 7 gün retention. Başarılı debug test binary'sini release diye sunma. Katalog testleri fixture ile, canlı key gerektirmez. CI'da IGDB secret zorunlu tutma.

`release.yml`: `workflow_dispatch` veya `v*` tag; versiyonların package.json/Cargo.toml/tauri.conf.json ile eşleştiğini denetle. Aynı commit'in kontrollerini çalıştır/yeniden kullan. Production feature set, Apple Silicon target `aarch64-apple-darwin`. `.app` bundle ve `.dmg` üret; SHA256 checksum ve build bilgilerini ekle. `.app` zip'lenirken macOS executable bitleri korunmalı. GitHub Release **draft** olarak oluşturulur. Sadece release job `contents: write` alır. Taşınan sırlar yok, repo visibility değiştirme yok. [Tauri GitHub dağıtımı](https://v2.tauri.app/distribute/pipelines/github/)

Private repo üzerinde Actions kota/runner erişimi hesaba bağlı olabilir. Kullanıcı adına ücretli runner planı etkinleştirme; gerekirse yerel build ve kaynak teslimini tamamla, bulut çalıştırma engelini açıkla.

## macOS paketleme

Uygulama kendi Mac'inde kullanılacağı için v1'de Apple Developer üyeliği ön koşul yapılmaz. Tauri `bundle.macOS.signingIdentity: '-'` ile ad-hoc imza kullanılır. Bu Apple tarafından notarize edilmiş genel dağıtım değildir; indirilmiş paketlerde macOS güvenlik onayı gerekebilir. Sistem korumalarını genel olarak kapatan komut önerme. [Tauri macOS imza seçenekleri](https://v2.tauri.app/distribute/sign/macos/)

`minimumSystemVersion: '14.0'`, Apple Silicon release target; özgün Hush `.icns` icon, About bilgisi, native app/edit/window menüleri. `.app` taşınınca veri bundle içinde kaybolmamalı; Application Support'tan okunur. [Tauri macOS bundle](https://v2.tauri.app/distribute/macos-application-bundle/)

Dağıtımda ileride Developer ID/notarization istenirse sertifikalar GitHub Secrets/Keychain üzerinden yönetilir. İlk sürümde hayali sertifika veya hazır Apple imzası varmış gibi raporlama yapılmaz.

## Son teslim

- `Hush.app` ve `Hush-<version>-aarch64.dmg` (ve taşımak için app ZIP'i).
- GitHub repository URL, son commit SHA, varsa PR/draft Release URL ve CI run sonucu.
- Türkçe README: çalıştırma, kurulum, veri klasörü, yedek, katalog kaynakları, IGDB bağlantısı.
- TEST-REPORT: otomatik/manuel test ayrımı, Mac/OS/architecture, gerçek sonuçlar, ölçümler ve ekran görüntüleri.
- Katalog API erişimi veya Apple notarization gibi harici eksikler varsa spesifik durum; “tamamlandı” ifadesi bunları gizlemez.

Uygulama açılmıyor, kayıt restart'ta kayboluyor, restore güvenli değil veya ana oda kaba placeholder aşamasındaysa ürün tamamlanmış değildir.
