# Sonraki oturum — P7 manuel kabul ve GitHub

```text
/Users/haktan/Desktop/Hush projesinde devam et. Önce AGENTS.md, docs/PROGRESS.md, docs/HANDOFF.md, docs/TEST-REPORT.md ve docs/08 D21–D22'yi oku; git status ve son commit'i kontrol et, mevcut değişiklikleri koru.

P0–P6 ve P7'nin otomatik/paketleme kısmı tamam. Açık olanlar:
1. Kullanıcı destekli manuel kabul (TEST-REPORT → "Manuel kabul"): VoiceOver hotspot/form adları, kirli formda Cmd+Q/Cmd+W'nin durması, Escape → Geri, temiz durumda Cmd+W → Dock → Cmd+Q, Türkçe menüler, DMG'den kurulum/açılış. Kullanıcının bildirdiği sonucu aynen yaz; gelmeyen adımı geçti sayma. Hata çıkarsa düzelt, `npm run check`, ilgili native test, `npm run build:mac`, `npm run verify:release` tekrarla.
2. GitHub: remote yok. Kullanıcı yeni ve açık yetki verirse (ör. `gh auth login`'i kendisi yaptıktan sonra) kişisel hesapta private `hush` (doluysa `hush-desktop`) deposu, push, CI sonucu ve draft release. Yetki yoksa yeniden isteme; CLI indirme/Finder/System Events izinlerini dolaşma.

Native/HTTP testleri sandbox dışında koşmalı. Test verisi yalnız yeni hush-native-* geçici dizinlerinde. Kullanıcının Application Support verisine dokunma, veritabanını sıfırlama, IPC mock kullanma. Notarization, canlı MB/CAA ve gerçek IGDB OAuth yokluğunu başarı gibi raporlama.
```
