# Claude için devir notu — Hush P7

> Durum (2026-11-07): Bu devir işlendi. Güncel durum için PROGRESS/HANDOFF/TEST-REPORT ve NEXT_CHAT_PROMPT geçerlidir. Aşağıdaki "npm run check geçti (6 test)" iddiası devir anında doğru değildi (bkz. docs/08 D22).

## Görev

`/Users/haktan/Desktop/Hush` içinde yalnız P7 kalite, paketleme ve teslimi tamamla. P0–P6 kabul kapıları tamamlandı; önceki fazları yeniden tasarlama. Bu dosya, `docs/NEXT_CHAT_PROMPT.md` ile birlikte doğrudan çalışma bağlamıdır.

## Başlangıç durumu

- Dal: `feat/local-library`; son commit: `c83d6b0 feat: verify P6 native backup restore and crash recovery`.
- P7 değişiklikleri **commit edilmemiştir**. `git status` çıktısını koru; reset, checkout veya kullanıcı değişikliklerini silme yapma.
- Okunacaklar: `AGENTS.md`, `CODEX_BASLANGIC.md`, `README.md`, `docs/01`–`08`, `docs/PROGRESS.md`, `docs/HANDOFF.md`, `docs/TEST-REPORT.md`.
- GitHub remote/URL yok. GitHub CLI indirme ve Finder/System Events otomasyon izni önce reddedildi; bunları yeniden isteme veya başka araçla dolaşma. Uzak GitHub işlemi yapma.
- MusicBrainz/CAA için geçerli iletişim adresi, IGDB için gerçek kimlik bilgisi yok. Bunları canlı kabul geçmiş gibi raporlama.

## P7'de eklenen, ama henüz commit edilmemiş işler

- Native lifecycle: `src-tauri/src/lib.rs`, `src-tauri/src/commands/mod.rs`, `src/infra/tauri/lifecycle.ts`, capability/build manifesti.
  - Kırmızı kapatma/Cmd+W pencereyi gizlemeli; Dock'tan yeniden açılmalı.
  - Cmd+Q açık diyalog varsa kapatmamalı; form metni korunmalı ve kullanıcı mevcut Kaydet/Vazgeç akışını bitirmeli.
  - İlk uygulama özel Cmd+Q eylemi olmadan predefined Quit kullandı ve kullanıcı manuel testinde doğrudan çıktı. Bu, `hush-quit` / `hush-close` özel native menü eylemleriyle düzeltildi. `src/infra/tauri/lifecycle.test.ts` açık diyalog korunmasını test eder. **Kullanıcı düzeltme sonrası manuel doğrulama vermedi.**
- Atlanabilir onboarding, Türkçe native menu/About, Cmd+N bağlama uygun yeni kayıt kısayolu eklendi.
- Native kalite testi: `tests/native/quality.e2e.js`. Performance: `tests/native/performance.e2e.js`. E2E-only test ayarları productiona sızmamalı.
- `scripts/build-mac.mjs` app + DMG + ZIP + SHA256 üretir; `scripts/verify-release.mjs` arm64/ad-hoc/test marker/sır taraması yapar.
- `docs/KURULUM-VE-KULLANIM.md`, `docs/RELEASE-NOTES.md`, `THIRD-PARTY-NOTICES.md`; `.github/workflows/release.yml` ve güncellenmiş CI hazır.

## Gerçek sonuçlar

- `npm run check`: P7 lifecycle/Cmd+N değişikliklerinden sonra geçti; TypeScript strict, ESLint, **6** Vitest testi, production Vite build.
- Sandbox dışında `cargo test --locked --manifest-path src-tauri/Cargo.toml`: P7 başlangıcında 31 Rust testi geçti. P7 Rust lifecycle değişikliğinden sonra bu komutu, clippy ve fmt'i yeniden çalıştır.
- `npm run test:native -- --quality`: gerçek native close eventinde kitap form metni korundu; Cmd+S ile kaydedildi; ikinci süreçte kayıt bulundu.
- `npm run test:native -- --performance`: `/var/folders/zy/nlg3tsjx7b9dnf72st1bbyf00000gn/T/hush-native-KM01zy/performance.json`; 5.000 eser / 10.000 seans, local search p95 3 ms, panel first paint p95 33 ms, frame p95 18 ms, minimize sonrası animasyonlar paused. Release-optimizasyonlu **e2e** binary ölçümüdür; production cold-start sonucu değildir.
- `npm run test:native -- --regressions`: P0 temel smoke 2+2; P3 collections 5+1; P4 journal 4+1; P5 catalog 4+1; P6 export/restore, 19 bozuk arşiv, üç SIGKILL/verify geçti. Kasıtlı kill süreçlerindeki WDIO `fetch failed`/`ECONNREFUSED` beklenir; verify süreçleri başarı ölçütüdür.
- `npm run build:mac`: app/DMG/ZIP/checksum üretildi. `codesign --verify --deep --strict`, `hdiutil verify`, salt-okunur DMG içindeki app imzası ve binary SHA eşitliği geçti. Son P7 kod değişikliğinden sonra paketi yeniden üret.
- `npm run verify:release`: arm64/ad-hoc; production dependency graphında WDIO yok; bilinen test işaretleri, fixture data ve secret pattern yok.

## Manuel kabulün açık kısmı

Kullanıcı ayrı sentetik/offline test penceresinde VoiceOver ve fiziksel klavye kontrolüne başladı; ilk iki adıma kadar ilerlediğini söyledi, ancak VoiceOver'ın düğme ve form alanı adlarını okuyup okumadığını ayrıntılandırmadı. Kaydedilmemiş formda Cmd+Q uygulamayı kapattı; hata düzeltildi. Aynı test root'u korunarak yeni pencere açıldı fakat düzeltme sonrası cevap henüz yok.

Manuel doğrulama: VoiceOver ile oda hotspot'ları ve Kitaplar → Ekle alan isimleri; Cmd+S; kaydedilmemiş değişiklikte Cmd+W/Cmd+Q ve Escape → Geri; tüm paneller kapalıyken Cmd+W → Dock'tan reopen → Cmd+Q; Hush/Düzen/Pencere Türkçe native menüleri. Geçmeyen veya kullanıcı cevaplamayan adımı geçmiştir diye yazma.

## Teslim/commit öncesi zorunlu kontrol

```sh
npm run check
cargo test --locked --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
npm run test:native -- --quality
npm run test:native -- --performance
npm run test:native -- --regressions
npm run build:mac
npm run verify:release
git diff --check
```

Testleri yalnız yeni `hush-native-*` geçici dizinlerinde tut. Kullanıcının Application Support verisini değiştirme, veritabanını sıfırlama, test fixture'larını Git'e ekleme. Paket yolları `src-tauri/target/aarch64-apple-darwin/release/bundle/` altındadır ve Git dışıdır.

P7'yi tamamladıktan sonra `docs/PROGRESS.md`, `docs/HANDOFF.md`, `docs/TEST-REPORT.md`, `docs/NEXT_CHAT_PROMPT.md` ve gerekiyorsa karar kaydını gerçek sonuçlarla güncelle. Sonra ilgili kaynak/dokümanları dikkatle stage et, sır/kişisel veri/paket/test artifact olmadığını gözden geçir ve yerel commit oluştur. GitHub remote, CI run, draft release ve Apple notarization yokluğunu teslimde açıkça belirt.
