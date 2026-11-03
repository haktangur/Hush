import {browser,expect} from '@wdio/globals';
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
const root=process.env.HUSH_TEST_DATA_DIR;
async function raw(c,r={}){return browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),c,r)}
async function ipc(c,r={}){const out=await raw(c,r);if(!out.ok)throw Error(`${c}: ${out.error.code}`);return out.value}
async function top(){return (await browser.$$('[role="dialog"]')).at(-1)}
async function click(text){await (await top()).$(`button=${text}`).click()}
async function field(label){return (await top()).$(`.//label[normalize-space(text())="${label}"]/*[self::input or self::textarea or self::select]`)}
async function fill(label,value){await (await field(label)).setValue(value)}
async function select(label,value){await browser.execute((e,v)=>{e.value=v;e.dispatchEvent(new Event('change',{bubbles:true}))},await field(label),value)}
async function open(name){await browser.$('nav').$(`button=${name}`).click()}
const entries=[];
describe('P5 synthetic provider cache, real macOS IPC and SQLite',()=>{
 it('searches, selects editions, previews and saves book and album',async function(){
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');if(process.env.HUSH_REOPEN)return this.skip();
  const b=await ipc('app_bootstrap');await ipc('settings_save',{settings:{...b.settings.value,catalog:{...b.settings.value.catalog,enabled:true}},expectedRevision:b.settings.revision});
  for(const [kind,name,query,edition,title] of [['book','Kitaplar','P5 sentetik kitap','OL900000001M','P5 kitap baskısı'],['album','Albümler','P5 sentetik albüm','22222222-2222-4222-8222-222222222222','P5 sentetik albüm']]){
   await open(name);await fill('Eser veya üretici adı',query);await click('Ara');await expect((await top()).$('p=Önbellekten')).toBeDisplayed();await click('Önizle ve düzenle');await select('Baskı / sürüm',edition);await (await top()).$$('button=Önizle ve düzenle').then(bs=>bs.at(-1).click());
   await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===2);await expect(await field('Başlık')).toHaveValue(title);
   expect((await ipc('library_list',{kind,sort:'updated_desc',offset:0,limit:60})).total).toBe(0);
   await fill('Kişisel not','P5 korunacak not');await fill('Kişisel puan (0–10)','8');if(kind==='book')await fill('Bulunduğun sayfa','37');await click('Kaydet');await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===1);
   const saved=(await ipc('library_list',{kind,sort:'updated_desc',offset:0,limit:60})).items[0];expect(saved.notes).toBe('P5 korunacak not');expect(saved.rating).toBe(8);expect(saved.provenance.editionId).toBe(edition);
   const extras=await ipc('library_extras',{id:saved.id});if(kind==='album'){expect(saved.coverAssetId).not.toBeNull();expect(extras.tracks[0].title).toBe('Sentetik parça');expect(extras.tracks[0].durationSeconds).toBe(123)}else expect(saved.details.currentPage).toBe(37);
   const duplicate=await raw('library_save',{item:{...saved,id:crypto.randomUUID()},expectedRevision:null});expect(duplicate.ok).toBe(false);expect(duplicate.error.code).toBe('CONFLICT');
   entries.push({saved,extras});await browser.saveScreenshot(`artifacts/p5-${kind}-saved.png`);await click('Kapat');
  }
  writeFileSync(join(root,'catalog-evidence.json'),JSON.stringify(entries,null,2));
 });
 it('opens the existing edition instead of offering a duplicate save',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  await open('Kitaplar');await fill('Eser veya üretici adı','P5 sentetik kitap');await click('Ara');await click('Önizle ve düzenle');await select('Baskı / sürüm','OL900000001M');await (await top()).$$('button=Önizle ve düzenle').then(bs=>bs.at(-1).click());await click('Kütüphanende var · Aç');await expect((await top()).$('h2=P5 kitap baskısı')).toBeDisplayed();expect(await browser.$$('[role="dialog"]').length).toBe(1);await click('Kapat');
 });
 it('keeps missing or malformed covers recoverable without saving a record',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  const candidateKey={provider:'musicbrainz',entityType:'release_group',externalId:'11111111-1111-4111-8111-111111111111'};
  for(const editionId of [null,'33333333-3333-4333-8333-333333333333']){const draft=await ipc('catalog_prepare',{candidateKey,editionId});expect(draft.item.coverAssetId).toBeNull();expect(draft.warnings).toContain('COVER_UNAVAILABLE')}
  expect((await ipc('library_list',{kind:'album',sort:'updated_desc',offset:0,limit:60})).total).toBe(1);
 });
 it('allows manual games with no IGDB configuration and rejects empty credentials',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  expect((await ipc('provider_status')).state).toBe('not_configured');
  const rejected=await raw('provider_igdb_configure',{clientId:'',clientSecret:''});expect(rejected.error.code).toBe('VALIDATION');
  await open('Oyunlar');await click('Ekle');await fill('Başlık','P5 elle oyun');await fill('Kişisel not','IGDB gerektirmez');await click('Kaydet');await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===1);await expect((await top()).$('h2=P5 elle oyun')).toBeDisplayed();await click('Kapat');
 });
 it('records a separate live Open Library search result without claiming fixture success as live',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  if(!process.env.HUSH_CATALOG_LIVE){writeFileSync(join(root,'offline'),'');writeFileSync(join(root,'http-before-reopen.txt'),readFileSync(join(root,'http-attempts.txt')));return this.skip();}
  const result=await raw('catalog_search',{kind:'book',query:'The Left Hand of Darkness',page:1});
  writeFileSync(join(root,'live-open-library.json'),JSON.stringify({ok:result.ok,code:result.error?.code,fromCache:result.value?.fromCache,count:result.value?.items.length},null,2));
  if(result.ok && result.value.items.length){
   await open('Kitaplar');await fill('Eser veya üretici adı','The Left Hand of Darkness');await click('Ara');await (await top()).$$('button=Önizle ve düzenle').then(bs=>bs[0].click());
   await browser.waitUntil(async()=>await (await field('Baskı / sürüm')).isExisting());
   await browser.waitUntil(async()=>await (await (await top()).$$('button=Önizle ve düzenle')).at(-1).isEnabled(),{timeout:120000});
   const editions=await (await field('Baskı / sürüm')).$$('option');if(editions.length>1)await select('Baskı / sürüm',await editions[1].getAttribute('value'));
   await (await top()).$$('button=Önizle ve düzenle').then(bs=>bs.at(-1).click());
   await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===2,{timeout:120000});
   await fill('Kişisel not','P5 canlı katalog kişisel test notu');await fill('Kişisel puan (0–10)','7');await click('Kaydet');await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===1);
   let saved=(await ipc('library_list',{kind:'book',sort:'updated_desc',offset:0,limit:60})).items.find(i=>i.notes==='P5 canlı katalog kişisel test notu');
   const originalTitle=saved.title;const changed=await ipc('library_save',{item:{...saved,title:'P5 yerel başlık'},expectedRevision:saved.revision});
   await click('Kapat');await open('Kitaplar');await (await top()).$('button*=P5 yerel başlık').click();await click('Katalog bilgisini yenile');
   await browser.waitUntil(async()=>await (await top()).$('button=Seçili değişiklikleri uygula').isExisting(),{timeout:120000});
   const titleCheckbox=(await top()).$('.//label[contains(., "Başlık:")]/input');await titleCheckbox.click();await click('Seçili değişiklikleri uygula');await expect((await top()).$('h2='+originalTitle)).toBeDisplayed();
   saved=await ipc('library_get',{id:saved.id});expect(saved.notes).toBe(changed.notes);expect(saved.rating).toBe(changed.rating);expect(saved.details.currentPage).toBe(changed.details.currentPage);expect(saved.status).toBe(changed.status);
   const extras=await ipc('library_extras',{id:saved.id});entries.push({saved,extras});writeFileSync(join(root,'catalog-evidence.json'),JSON.stringify(entries,null,2));
   writeFileSync(join(root,'live-open-library-flow.json'),JSON.stringify({search:true,edition:!!saved.provenance.editionId,preview:true,save:true,refresh:true,cover:!!saved.coverAssetId},null,2));await browser.saveScreenshot('artifacts/p5-live-refresh.png');await click('Kapat');
  }
  // This is observational live evidence, not an assertion that the remote service is available.
  writeFileSync(join(root,'offline'),'');
  writeFileSync(join(root,'http-before-reopen.txt'),readFileSync(join(root,'http-attempts.txt')));
 });
 it('reopens exact saved records and tracks with outbound HTTP disabled',async function(){
  if(!process.env.HUSH_REOPEN)return this.skip();const prior=JSON.parse(readFileSync(join(root,'catalog-evidence.json')));
  for(const {saved,extras} of prior){expect(await ipc('library_get',{id:saved.id})).toEqual(saved);expect(await ipc('library_extras',{id:saved.id})).toEqual(extras);await open(saved.kind==='book'?'Kitaplar':'Albümler');await (await top()).$(`button*=${saved.title}`).click();await expect((await top()).$('h2='+saved.title)).toBeDisplayed();if(saved.coverAssetId)await browser.waitUntil(async()=>await browser.execute(()=>Array.from(document.querySelectorAll('img[alt="Eser kapağı"]')).some(i=>i.complete&&i.naturalWidth>0)));await browser.saveScreenshot(`artifacts/p5-${saved.kind}-offline.png`);await click('Kapat')}
  expect(readFileSync(join(root,'http-attempts.txt'),'utf8')).toBe(readFileSync(join(root,'http-before-reopen.txt'),'utf8'));
 });
});
