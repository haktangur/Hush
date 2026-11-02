# Araç zinciri

2026-11-01 yerel doğrulaması: macOS 26.6.2 (25G83), Apple Silicon arm64; Xcode Command Line Tools `/Library/Developer/CommandLineTools`.

- Node 24.18.0 / npm 11.16.0
- Rust / Cargo 1.97.1, `aarch64-apple-darwin`
- Tauri CLI/API 2.12.4; Rust Tauri 2.12.5, tauri-build 2.7.4
- React 19.3.1, Vite 8.3.5, plugin-react 6.1.2
- TypeScript 6.0.3, ESLint 9.39.5, typescript-eslint 8.72.1
- Vitest 5.0.5

Tüm doğrudan npm sürümleri exact; geçişli sürümler package-lock.json ve src-tauri/Cargo.lock içinde sabittir. TypeScript 7.0.2 mevcut olsa da typescript-eslint `<6.1.0` gerektirdiği için 6.0.3 seçildi. eslint-plugin-jsx-a11y ESLint 10 desteklemediği için ESLint 9.39.5 seçildi. Peer dependency kontrolleri atlanmadı.

CI başlangıç etiketi macos-15; bulut runner/CI doğrulaması henüz yapılmadı. Actions checkout v5, setup-node v5, upload-artifact v4 SHA'ları resmi Git depolarının tag ref'lerinden doğrulandı.

Resmi başvuru: https://v2.tauri.app/start/prerequisites/ ve https://v2.tauri.app/security/capabilities/ (2026-11-01).

2026-11-02 ekleri: ZIP 9.2.1 `cargo search`/`cargo info` ile crates.io üzerinde doğrulandı; yalnız deflate-flate2-zlib-rs açık. Disk alanı için mevcut kilitli rustix 1.1.6 kullanılıyor. reqwest 0.13.5 (rustls/json/form/query), keyring 3.6.3 (apple-native), chrono-tz 0.10.4; sürümler Cargo.lock içinde. Native WDIO 9 + Tauri servis/plugin 1.5.1, ayrı target/native hedefi kullanıyor.
