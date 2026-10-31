# 04 — Veri modeli, komutlar ve geri yükleme

## Şema sözleşmesi

`contracts/0001_initial.sql` başlangıç migration'ıdır; uygulamada `src-tauri/migrations/0001_initial.sql` olarak kullanılır. SQL kısıtları tek başına iş kurallarının tamamı değildir; Rust doğrulaması zorunludur.

| Tablo grubu | Sorumluluk |
|---|---|
| library_items + üç details tablosu | Eserlerin ortak alanı ve türe özel alanlar |
| catalog_refs | Sağlayıcı kimliği/kaynak; kişisel notlardan ayrı |
| assets | Yerel görsel dosyalarının manifesti |
| tags, item_tags | Etiketler |
| collections, collection_items | Kullanıcının özel raf/listeleri |
| sessions | Manuel oyun, dinleme ve okuma süreleri; tür eserden türetilir |
| album_tracks | Albüm parça listesi ve kişisel parça notları |
| book_quotes | Kitap alıntıları |
| game_screenshots | Oyun ekran görüntüleri |
| journal_entries, journal_items | Günlük ve ilişkilendirilmiş eserler |
| drafts | Tamamlanmamış günlük taslakları |
| room_slots, app_settings | Poster/plak atamaları ve görünüm tercihleri |

Her eserin tam olarak bir doğru details satırı olmalı. Bunun “en az bir” kısmını SQL FK garanti etmez: library save transaction'ı common ve details satırını birlikte yazar; repository okuma/restore doğrulaması da bunu kontrol eder. Eser `kind`'ı oluşturulduktan sonra değiştirilemez.

UI DTO'ları camelCase; SQL alanları snake_case. `contracts/domain.ts` public veri sözleşmesidir. Rust enum ve struct'larında serde karşılıkları belirlenir. TypeScript ve Rust fixture round-trip testi aynı JSON'un iki tarafta da kabul edildiğini doğrular.

## Doğrulama ve düzenleme semantiği

- Tüm UUID, tarih, saat, URL ve enum'ları Rust doğrular. UI kontrolleri kullanıcı deneyimi içindir.
- Başlık 1–300, creator 500, kişisel not 20.000, seans notu 2.000, etiket 1–60, en fazla 30 etiket/eser. Günlük başlığı 1–200, gövde 50.000. Metni trim et; iç boşlukları ve satır sonlarını bozma.
- Puan `null` veya 0–10 tam sayı; sıfır “puan verilmedi” değildir. Tarihler gerçek takvim günü olmalı. Başlangıç ve bitiş varsa bitiş başlangıçtan önce olamaz.
- Rust Turkish-fold yardımcı fonksiyonu önce `I -> ı`, `İ -> i`, sonra Unicode lowercase/NFC uygular. Bu normalizasyon exact/search için kararlıdır. `cagri` ile `çağrı` eşleşmesi v1 şartı değildir. Aynı fixture'lar UI ve Rust'ta denenir.
- `search_text` başlık + creator + notes + etiketlerin normalize birleşimidir; ilgili alan değişince aynı transaction'da güncellenir. Günlük ayrı search_text tutar. SQL `LIKE` aramasında `%`, `_` ve escape karakteri literal olarak escape edilir.
- 5.000 kayıt hedefi için normalize LIKE + sayfalama yeterli başlangıç kararıdır. Ölçüm hedefi aşılırsa FTS eklemeli migration ile eklenebilir; baştan zorunlu değildir.
- Kaydetme full replacement semantiği kullanır: gönderilen item DTO, ortak ve tür alanlarının tamamını içerir. `null` alanı temizler; gizli PATCH/undefined yorumları yok.
- `expectedRevision=null` yeni kayıt; update için mevcut revision gerekir. Kaydetme `UPDATE ... WHERE id=? AND revision=?`, revision+1. Alt kayıt batch değişiklikleri ebeveyn revision'ını artırır.
- Seanslar kendi revision'ıyla güncellenir. Track/quote/screenshot listeleri tek transaction'da ebeveyn revision'ına bağlı replace işlemleriyle değişir; tekrar ID yaratıp kişisel not kaybetme.
- Eser trash'e taşındığında slot ilişkisinin ID'si kalabilir ama görünüm varsayılan kapak gösterir. Restore ile görünüm geri gelir. Kalıcı silmede FK temizler. Record stand yalnız `album` ve posterler aktif eser veya image kabul eder.
- Kalıcı eser silme seans/parça/alıntı/screenshot ve koleksiyon bağlantılarını cascade kaldırır; günlük metni korunur, sadece linki kalkar. Kullanıcıya sonuç onayında belirtilir.

## Komut protokolü

Komutlar `invoke<ApiResult<T>>(command, { request })` biçiminde çağrılır; argümansız çağrıda `{request:{}}`. Envelope başarılıysa `{ok:true,value}`, hatalıysa `{ok:false,error:{code,message,fieldErrors?,retryable}}`. Uygulama istemcisi tek yerde normalize eder. Tauri transport hatası ayrıca `INTERNAL`/`UNAVAILABLE` olarak map edilir; raw Rust hata metni kullanıcıya gösterilmez.

| Komut | Girdi → çıktı | Önemli davranış |
|---|---|---|
| app_bootstrap | boş → settings, slots, counts, storageEpoch | recovery/migration bitmeden başarı dönmez |
| library_list | kind?, query?, status?, tags?, collectionId?, favorite?, sort, offset, limit → Page<ItemSummary> | limit 1–100, varsayılan 60; trash ayrı |
| library_get | id → LibraryItem | silinmiş öğe normal çağrıda NOT_FOUND |
| library_save | ItemDraft, expectedRevision, initialTracks? → LibraryItem | common/details/tags/provenance atomik; initialTracks yalnız yeni albüm import'unda aynı transaction'a girer |
| library_trash / library_restore | id, expectedRevision → void | revision kontrolü |
| trash_list | entity?, offset, limit → Page<TrashEntry> | eser ve günlük desteklenir |
| trash_purge | entity, id → void | UI açık onaydan sonra; ilişkiler yukarıdaki gibi |
| session_list | itemId, offset, limit → Page<Session> | toplam dakika ayrıca döner |
| session_save / session_delete | draft + revision / id + revision → Session / void | süre 1–1440 dakika |
| album_tracks_replace | albumId, expectedRevision, tracks → LibraryItem | ID korunur; disk/sıra unique |
| book_quotes_replace | bookId, expectedRevision, quotes → LibraryItem | alınan metin dış ağa gitmez |
| game_screenshots_replace | gameId, expectedRevision, screenshots → LibraryItem | asset varlığı doğrulanır |
| collection_list / collection_save / collection_delete | filtre / draft / id+revision → list / collection / void | eserler silinmez |
| collection_members_replace | collectionId, expectedRevision, itemIds → collection | tek transaction, sıra listedeki sıra |
| journal_list / journal_get | tarih/aralık/query/sayfa / id → Page / entry | yerel gün semantiği |
| journal_save | JournalDraft, expectedRevision → entry | ilişkiler ve taslak temizleme atomik |
| journal_trash / journal_restore | id, expectedRevision → void | eserlerle aynı trash modeli |
| draft_get / draft_put / draft_delete | key / key+payload / key → draft?/void | yalnız günlük taslağı şeması |
| search_local | query, limit → grouped results | sadece yerel eser ve günlük |
| calendar_month | year, month, timeZone → days + summary | IANA timezone validate; günlük günü kaymaz |
| asset_choose_import | purpose:cover/screenshot/poster → Asset|null | native picker; iptal hata değil |
| asset_resolve | assetIds → safe paths + dimensions | max 100; yalnız uygulama AssetStore |
| room_slot_set | slotId, itemId|null, assetId|null → RoomSlots | bir slot iki kaynak alamaz |
| settings_save | Settings, expectedRevision → SettingsSnapshot | sır yok, şema doğrulanır |
| external_open | url → void | HTTPS + kullanıcı tıklaması |
| catalog_search | kind, query, page → CatalogPage | sağlayıcı sabit eşleme; API belgesi |
| catalog_prepare | candidateKey, editionId? → CatalogDraft | kişisel alanlar boş, geçici cover lease |
| catalog_refresh_preview | itemId → MetadataDiff | otomatik üzerine yazmaz |
| provider_status | boş → ProviderState[] | token veya client secret içermez |
| provider_igdb_configure | clientId, clientSecret → ProviderState | transient giriş; Keychain |
| provider_disconnect | provider → void | cache/credential, kişisel kayıtlar korunur |
| backup_export | boş → ExportResult|null | native save dialog; iptal null |
| backup_inspect | boş → RestorePreview|null | native file dialog; staged doğrulama |
| backup_restore | inspectionToken → RestoreResult | token tek kullanımlık, aktif generation swap |

`candidateKey` sağlayıcı + entity + dış ID'yi taşıyan doğrulanmış tiptir; keyfi endpoint/URL kabul edilmez. Import işlemleri yüksek maliyetli URL indirmeye genel kapı açmaz. Girdi/çıktı tiplerinin kalan Page/filter/summary biçimlerini bu tabloya uygun dar DTO'lar olarak yaz; `any` veya JSON string geçişiyle bypass etme.

Katalog tekrar kontrolü çöp kutusunu da kapsar. `existingState='trashed'` sonucu normal detail'e götürmek yerine çöp kutusunda geri alma eylemi gösterir. Slot/asset/child-list değişiklikleri silinmiş bir ebeveyne uygulanmaz. `initialTracks` update'te veya oyun/kitap kaydında verilirse VALIDATION hatasıdır; sonradan parça düzenleme ayrı batch komutunu kullanır.

### Hata kodları ve UX

`VALIDATION` alan yanında; `NOT_FOUND` kaydın artık olmadığını açıklar; `CONFLICT` mevcut formu korur ve yenileme seçeneği verir; `DB_BUSY` yeniden dene; `DISK_FULL`/`IO` kaydetmenin tamamlanmadığını söyler; `PROVIDER_NOT_CONFIGURED` bağlantı/elle ekleme; `OFFLINE` kayıtlı veriye devam; `RATE_LIMITED` bekleme bilgisi; `PROVIDER_UNAVAILABLE` tekrar/elle ekleme; `INVALID_BACKUP` mevcut verinin korunduğunu söyler; `UNSUPPORTED_SCHEMA` daha yeni uygulama gerektiğini belirtir; `INTERNAL` korelasyon ID ile kısa açıklama.

## State ve invalidation

Query key örnekleri: `['library',kind,filters]`, `['item',id]`, `['sessions',itemId]`, `['journal',filters]`, `['calendar',year,month,timeZone]`, `['room']`, `['settings']`. Eser/seans/journal mutation'ı kendi detail/list'i ile calendar/room özetlerini invalidate eder. Çok küçük favori değişiminde optimistic UI yapılabilir ama geri alma şarttır; form kaydetmede pessimistic akış tercih edilir.

Restore başarıda `storageEpoch` değişir; tüm query cache, açık detaylar, asset URL cache, taslak editör ve seçili item ID'leri temizlenir; bootstrap yeniden çalışır ve oda açılır. Eski generation'a ait path ekran üzerinde kalmaz.

## Yedek dosyası

Uzantı `.hushbackup`; içerik ZIP. Manifest `formatVersion`, `schemaVersion`, `appVersion`, `createdAt`, `files[{path,sha256,bytes}]` tutar. Arşivde `manifest.json`, bağımsız SQLite snapshot `library.sqlite3` ve DB'nin referans verdiği `assets/...` dosyaları bulunur. Journal drafts ve trash kayıtları da korunur. Arama önbelleği, loglar, mutlak yollar, Keychain/API bilgileri dahil edilmez. Şifreleme v1'de yok; dışa aktarma ekranı bunun kişisel içerik taşıyan dosya olduğunu kısa biçimde belirtir.

### Export sırası

1. Native kaydetme diyaloğu ile hedef al; kullanıcı iptal ederse yan etki yok.
2. Global storage operation gate al: item/journal/settings/asset mutasyonlarını geçici durdur; UI “Yedek hazırlanıyor” gösterir.
3. SQLite online backup API ile staging'de tutarlı DB snapshot üret. Açık WAL dosyasını doğrudan kopyalama. [rusqlite backup](https://docs.rs/rusqlite/latest/rusqlite/backup/index.html)
4. Snapshot'tan referanslı asset listesini al; dosyaları snapshot alanına kopyala ve hash üret. Gate bu tutarlı kopya hazır olunca bırakılabilir. Referanslı dosya eksikse export başarısızdır, tamamlandı denmez.
5. Snapshot'ı zip'e yaz; hedef dizinde `.partial` dosyası kullan. Sync + rename tamamlanınca başarı dön. Disk dolarsa partial temizlenir, eski yedek korunur.

### Restore sırası

1. Native picker ile seç; yalnız staging'e aç. ZIP entry adlarında absolute path, `..`, backslash kaçışı, symlink, duplicate path ve izin verilmeyen dosya türünü reddet. ZIP magic, en fazla 20.000 entry, en fazla 2 GiB toplam açılmış içerik, DB için 256 MiB sınırı uygula; disk boşluğunu kontrol et.
2. Manifest boyut/hash kontrolü; DB `integrity_check`, `foreign_key_check`, schema sürümü ve app düzeyinde item-kind/asset path/slot/JSON doğrulaması. Restore'daki asset'leri de güvenli decoder ile doğrula. Gerekirse desteklenen eski şemayı yalnız staging'de migrate et. İleri şemayı reddet.
3. Özet göster: tarih, eser/günlük/görsel sayıları, mevcut verinin değiştirileceği. Henüz aktif veride değişiklik yok. Preview token staging dizinine bağlı, opak ve süreli olsun.
4. Kullanıcı “Bu yedeği geri yükle” dediğinde storage gate al; açık dirty formlar çözülsün. Yeni mutations kapatılır. Stage hazırken mevcut DB'yi checkpoint/close et.
5. Doğrulanmış staged veri aynı filesystem üzerinde `storage/<newUUID>` dizinine atomik rename edilir. `active.json.tmp` yeni generation ID ile yaz/sync et; `active.json` üzerine atomik rename + mümkünse parent dir sync. **Commit noktası active pointer'ın değişmesidir.**
6. Yeni bağlantıyı aç, bootstrap doğrula. Hata varsa önceki generation pointer'ını atomik geri koy; önceki dosyalar hâlâ durur. UI'a başarı gösterme.
7. Başarılıysa cache'leri temizle, yeni storageEpoch döndür. Bir önceki generation kurtarma için tutulur; yalnız daha eski, aktif olmayan generation'lar sonraki başarılı açılışta temizlenebilir.

Çökme senaryosu: pointer değişmeden çökme → eski veri açılır. Pointer değiştikten sonra çökme → yeni doğrulanmış veri açılır; eski veri recovery olarak korunur. Başlangıçta pointer eksik/bozuksa asla otomatik boş DB yaratıp üzerine geçme; bilinen geçerli generation'ı kurtarma ekranında sun. Hiç DB bulunmayan gerçek ilk kurulum ayrı durumdur.

Testler rename/pointer yazma öncesi/sonrası fault injection içermeli. SQLite transaction ile filesystem rename'in tek atomik işlem olduğunu varsayma; bu protokol iki alanı bilinçli olarak ayırır.
