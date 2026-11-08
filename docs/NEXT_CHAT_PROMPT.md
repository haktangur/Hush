# Sonraki oturum — P7 manuel kabul ve GitHub

```text
<proje kökü> projesinde devam et. Önce AGENTS.md, docs/PROGRESS.md, docs/HANDOFF.md, docs/TEST-REPORT.md ve docs/08 D21–D22'yi oku; git status ve son commit'i kontrol et, mevcut değişiklikleri koru.

P0–P6 ve P7'nin otomatik/paketleme kısmı tamam. Açık olanlar:
1. Kullanıcı destekli manuel kabul (TEST-REPORT → "Manuel kabul"): VoiceOver hotspot/form adları, kirli formda Cmd+Q/Cmd+W'nin durması, Escape → Geri, temiz durumda Cmd+W → Dock → Cmd+Q, Türkçe menüler, DMG'den kurulum/açılış. Kullanıcının bildirdiği sonucu aynen yaz; gelmeyen adımı geçti sayma. Hata çıkarsa düzelt, `npm run check`, ilgili native test, `npm run build:mac`, `npm run verify:release` tekrarla.
2. GitHub: `origin` = https://github.com/haktangur/Hush (public; kullanıcı kararı, D24). Commit e-postası repo yerel ayarında GitHub noreply; gmail adresi hiçbir commit'e girmemeli. CI sonucunu kontrol et; draft release yalnız kullanıcı isterse `release.yml` workflow_dispatch ile. CLI indirme/Finder/System Events izinlerini dolaşma.

Native/HTTP testleri sandbox dışında koşmalı. Test verisi yalnız yeni hush-native-* geçici dizinlerinde. Kullanıcının Application Support verisine dokunma, veritabanını sıfırlama, IPC mock kullanma. Notarization, canlı MB/CAA ve gerçek IGDB OAuth yokluğunu başarı gibi raporlama.
```
