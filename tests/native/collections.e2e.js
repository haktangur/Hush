import { browser, expect } from '@wdio/globals';
import { DatabaseSync } from 'node:sqlite';
import { readFileSync, writeFileSync, mkdirSync, copyFileSync } from 'node:fs';
import { join } from 'node:path';
import { randomUUID, createHash } from 'node:crypto';
const root=process.env.HUSH_TEST_DATA_DIR;
const evidencePath=join(root,'collections-evidence.json');
const names={game:'Oyunlar',album:'Albümler',book:'Kitaplar'};
async function raw(command,request={}){return browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request)}
async function ipc(command,request={}){const r=await raw(command,request);if(!r.ok)throw new Error(`${command}: ${JSON.stringify(r.error)}`);return r.value}
async function rejected(command,request,code){const r=await raw(command,request);expect(r.ok).toBe(false);expect(r.error.code).toBe(code)}
async function top(){return (await browser.$$('[role="dialog"]')).at(-1)}
async function button(text){return (await top()).$(`button=${text}`)}
async function click(text){await (await button(text)).click()}
async function field(label){return (await top()).$(`.//label[normalize-space(text())="${label}"]/*[self::input or self::textarea or self::select]`)}
async function fill(label,value){await (await field(label)).setValue(String(value))}
async function select(label,value){const el=await field(label);await browser.execute((e,v)=>{e.value=v;e.dispatchEvent(new Event('change',{bubbles:true}))},el,value)}
async function countDialogs(n){await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===n)}
async function close(){await click('Kapat');await countDialogs(0)}
async function open(kind){await browser.$('nav').$(`button=${names[kind]}`).click();await countDialogs(1)}
async function card(title){await fill('Koleksiyonda ara',title);await (await top()).$(`strong=${title}`).click();await expect((await top()).$(`h2=${title}`)).toBeDisplayed()}
async function get(id){return ipc('library_get',{id})}
async function extras(id){return ipc('library_extras',{id})}
async function list(filter={}){return ipc('library_list',{sort:'title_asc',offset:0,limit:100,...filter})}
async function saveForm(){await click('Kaydet');await countDialogs(1)}
async function row(text){return (await top()).$(`.//div[./button[normalize-space(.)="Düzenle"] and .//text()[contains(.,"${text}")]]`)}
async function rowClick(text,action){await (await row(text)).$(`button=${action}`).click()}
async function trashOpen(){await browser.$('nav').$('button=Çöp kutusu').click();await countDialogs(1)}
async function roomImages(n){await browser.waitUntil(()=>browser.execute(n=>{const imgs=[...document.querySelectorAll('[data-testid="room-scene"] img')].filter(i=>!i.hasAttribute('aria-hidden'));return imgs.length===n&&imgs.every(i=>i.complete&&i.naturalWidth>0)},n),{timeout:10000,timeoutMsg:`Expected ${n} decoded slot images`})}
function seedAsset(){const generation=JSON.parse(readFileSync(join(root,'active.json'))).generationId;const base=join(root,'storage',generation);const id=randomUUID();const dir=join(base,'assets',id);mkdirSync(dir,{recursive:true});for(const name of ['image.webp','thumb.webp'])copyFileSync('tests/fixtures/room-album.webp',join(dir,name));const bytes=readFileSync(join(dir,'image.webp'));const db=new DatabaseSync(join(base,'library.sqlite3'));db.prepare('INSERT INTO assets VALUES (?,?,?,?,?,?,?,?,?)').run(id,`assets/${id}/image.webp`,`assets/${id}/thumb.webp`,createHash('sha256').update(bytes).digest('hex'),'image/webp',bytes.length,128,128,new Date().toISOString());db.close();return id}
function draft(kind,title){return {id:randomUUID(),kind,title,creator:'Sentetik üretici',releaseYear:2026,coverAssetId:null,rating:0,favorite:false,notes:'Yalnız P3 testi',tagNames:[],startedOn:null,completedOn:null,provenance:null,status:'planned',details:kind==='book'?{totalPages:200,currentPage:0,isbn:null,publisher:'',language:'tr'}:kind==='game'?{platform:'Mac',developer:'Hush',progressPercent:0,returnNote:''}:{firstListenedOn:null,spotifyUrl:null,appleMusicUrl:null}}}
async function childUi(item,remove=false){
 const type=item.kind==='book'?'quote':'track';const text=`P3 ${type} ${item.id.slice(0,5)}`;
 await click('Ekle');await countDialogs(2);
 await fill(type==='quote'?'Alıntı':'Başlık',text);await fill('Kişisel yorum','İlk yorum');if(type==='quote')await fill('Sayfa',12);else {await fill('Süre (saniye)',180);await (await field('Favori')).click()}
 await saveForm();await rowClick(text,'Düzenle');await fill('Kişisel yorum','Kalıcı yorum');await saveForm();
 if(remove){await rowClick(text,'Kaldır');await browser.waitUntil(async()=>(await extras(item.id))[type==='quote'?'quotes':'tracks'].length===0);return}
 const children=(await extras(item.id))[type==='quote'?'quotes':'tracks'];expect(children.length).toBe(1);expect(type==='quote'?children[0].comment:children[0].notes).toBe('Kalıcı yorum');if(type==='track')expect(children[0].favorite).toBe(true);
}
let records=[];
describe('P3 real macOS local collections acceptance',()=>{
 it('creates and edits two of every kind, sessions and typed children',async function(){
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');if(process.env.HUSH_REOPEN)return this.skip();
  expect((await list()).total).toBe(0);await browser.setWindowSize(1440,900);
  const asset=seedAsset();
  for(const kind of ['game','album','book'])for(const index of [1,2]){
   await open(kind);await click('Ekle');await browser.$('input[name="title"]').setValue(`P3 ${kind} ${index}`);await browser.$('input[name="creator"]').setValue('Sentetik üretici');await browser.$('input[name="rating"]').setValue(String(index));await browser.$('input[name="tags"]').setValue('İSTANBUL, istanbul, IŞIK');
   if(kind==='book'){await browser.$('input[name="totalPages"]').setValue('200');await browser.$('input[name="currentPage"]').setValue('37')}
   if(kind==='game'){await browser.$('input[name="progress"]').setValue('42');await browser.$('input[name="returnNote"]').setValue('Kuzey kapısına dön')}
   if(kind==='album'){await browser.$('input[name="spotify"]').setValue('https://open.spotify.com/');await browser.$('input[name="appleMusic"]').setValue('https://music.apple.com/')}
   await saveForm();await click('Düzenle');await browser.$('textarea[name="notes"]').setValue(`Düzenlenmiş ${kind} ${index}`);await browser.$('input[name="favorite"]').click();await select('Durum','completed');await saveForm();
   let item=(await list({kind,query:`P3 ${kind} ${index}`})).items[0];expect(item.notes).toBe(`Düzenlenmiş ${kind} ${index}`);expect(item.tagNames).toEqual(['İSTANBUL','IŞIK']);expect(item.rating).toBe(index);
   if(kind==='book')expect(item.details.currentPage).toBe(200);if(kind==='game'){expect(item.details.progressPercent).toBe(100);expect(item.completedOn).not.toBeNull()}
   if(kind==='album'){await expect(await button('Spotify’da aç')).toBeDisplayed();await expect(await button('Apple Music’te aç')).toBeDisplayed();if(index===1){await click('Spotify’da aç');await click('Apple Music’te aç');await rejected('external_open',{url:'file:///tmp/never-open'},'VALIDATION');expect(await browser.$$('[role="alert"]').length).toBe(0)}}
   await click('Seans ekle');await fill('dakika',25);await fill('Kişisel not','Silinecek seans');await saveForm();await rowClick('Silinecek seans','Düzenle');await fill('dakika',45);await fill('Kişisel not','Kalıcı seans');await saveForm();expect((await extras(item.id)).totalMinutes).toBe(45);
   await rowClick('Kalıcı seans','Kaldır');await browser.waitUntil(async()=>(await extras(item.id)).totalMinutes===0);
   await click('Seans ekle');await fill('dakika',35);await fill('Kişisel not','Restart seansı');await saveForm();
   if(kind!=='game'){await childUi(item,true);await childUi(await get(item.id))}
   await close();
   if(kind==='game'){
    const screenshot={id:randomUUID(),assetId:asset,caption:'Sentetik screenshot',sortOrder:0,createdAt:new Date().toISOString()};
    item=await get(item.id);await ipc('library_children_replace',{itemId:item.id,expectedRevision:item.revision,children:{kind:'game',screenshots:[screenshot]}});
    await open(kind);await card(item.title);await rowClick('Sentetik screenshot','Düzenle');await fill('Kişisel not','Düzenlenmiş screenshot');await saveForm();await (await top()).$('button[aria-label="Ekran görüntüsünü büyüt"]').click();await countDialogs(2);await browser.waitUntil(()=>browser.execute(()=>{const i=[...document.querySelectorAll('[role=dialog]')].at(-1).querySelector('img');return i.complete&&i.naturalWidth>0}));await browser.saveScreenshot('artifacts/p3-screenshot.png');await click('Kapat');await countDialogs(1);
    await rowClick('Düzenlenmiş screenshot','Kaldır');await browser.waitUntil(async()=>(await extras(item.id)).screenshots.length===0);await close();item=await get(item.id);await ipc('library_children_replace',{itemId:item.id,expectedRevision:item.revision,children:{kind:'game',screenshots:[{...screenshot,caption:'Restart screenshot'}]}});
   }
   // Fixture image only; all parent and child persistence above uses real IPC.
   item=await get(item.id);const {revision,createdAt,updatedAt,deletedAt,...value}=item;await ipc('library_save',{item:{...value,coverAssetId:asset},expectedRevision:revision});records.push(await get(item.id));
  }
  writeFileSync(evidencePath,JSON.stringify({records},null,2));
 });
 it('rejects wrong child kinds, stale revisions and trashed parent mutations',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  for(const item of records){
   const before=await extras(item.id);const correct=item.kind==='book'?{kind:'book',quotes:before.quotes}:item.kind==='album'?{kind:'album',tracks:before.tracks}:{kind:'game',screenshots:before.screenshots};
   const wrong=item.kind==='book'?{kind:'album',tracks:[]}:{kind:'book',quotes:[]};
   await rejected('library_children_replace',{itemId:item.id,expectedRevision:item.revision,children:wrong},'VALIDATION');
   await rejected('library_children_replace',{itemId:item.id,expectedRevision:item.revision-1,children:correct},'CONFLICT');
   const session=before.sessions[0];const {revision,createdAt,updatedAt,...sd}=session;
   await rejected('session_save',{draft:{...sd,notes:'Kaybolmamalı'},expectedRevision:revision+1},'CONFLICT');await rejected('session_delete',{id:session.id,expectedRevision:revision+1},'CONFLICT');
   await rejected('session_save',{draft:{...sd,itemId:records.find(r=>r.id!==item.id).id},expectedRevision:revision},'CONFLICT');
   expect(await extras(item.id)).toEqual(before);await browser.saveScreenshot('artifacts/p3-restored-poster.png');
  }
 });
 it('handles duplicate UX, collections, filters, sorting and 60-item pagination',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  await open('book');await click('Ekle');await browser.$('input[name="title"]').setValue('P3 book 1');await click('Kaydet');await expect(browser.$('p=Bu başlıkta bir eser zaten var. Ayrı bir baskı olarak kaydedebilirsin.')).toBeDisplayed();await click('Kaydı aç');await countDialogs(1);await expect((await top()).$('h2=P3 book 1')).toBeDisplayed();await close();
  await open('book');await click('Ekle');await browser.$('input[name="title"]').setValue('P3 book 1');await click('Kaydet');await click('Ayrı kayıt olarak devam et');await saveForm();expect((await list({query:'P3 book 1'})).total).toBe(2);await click('Düzenle');await browser.$('input[name="title"]').setValue('P3 ayrı baskı');await saveForm();await close();
  await browser.$('header').$('button=Koleksiyonlar').click();await click('Ekle');await fill('Başlık','P3 karışık raf');await fill('Kişisel not','İlk açıklama');
  for(const item of records.filter((_,i)=>i%2===0)){await (await field(item.title)).click()}
  await saveForm();let collection=(await ipc('collection_list'))[0];expect(collection.itemIds.length).toBe(3);
  await rowClick('P3 karışık raf','Düzenle');await fill('Başlık','P3 düzenlenmiş raf');await (await field(records[1].title)).click();await saveForm();collection=(await ipc('collection_list'))[0];expect(collection.itemIds.length).toBe(4);await close();
  await open('game');await select('Koleksiyonlar',collection.id);expect(await (await top()).$$('strong').length).toBe(2);await select('Durum','completed');await fill('Etiketler (virgülle ayır)','istanbul, ışık');await (await field('Favori')).click();await browser.waitUntil(async()=>await (await top()).$$('strong').length===2);await select('Görünüm','list');await expect((await top()).$('[data-library-view="list"]')).toBeDisplayed();await select('Sırala','rating_desc');await browser.waitUntil(async()=>(await (await top()).$('strong').getText())==='P3 game 2');await fill('Koleksiyonda ara','bulunmayan');await expect((await top()).$('h2=Eşleşen eser yok.')).toBeDisplayed();await close();
  await browser.$('header').$('button=Koleksiyonlar').click();await rowClick('P3 düzenlenmiş raf','Düzenle');await (await field(records[1].title)).click();await saveForm();collection=(await ipc('collection_list'))[0];expect(collection.itemIds.length).toBe(3);await rowClick('P3 düzenlenmiş raf','Kaldır');await browser.waitUntil(async()=>(await ipc('collection_list')).length===0);await close();for(const item of records)expect((await get(item.id)).id).toBe(item.id);
  for(let i=0;i<61;i++)await ipc('library_save',{item:draft('book',`Sayfalama ${String(i).padStart(2,'0')}`),expectedRevision:null});
  await open('book');await fill('Koleksiyonda ara','Sayfalama');await select('Sırala','title_asc');await browser.waitUntil(async()=>await (await top()).$$('strong').length===60);await click('Sonraki');await browser.waitUntil(async()=>await (await top()).$$('strong').length===1);await expect((await top()).$('strong')).toHaveText('Sayfalama 60');await click('Önceki');await browser.waitUntil(async()=>await (await top()).$$('strong').length===60);await click('Sonraki');await select('Sırala','created_desc');await browser.waitUntil(async()=>await (await top()).$$('strong').length===60);await expect(await button('Önceki')).toBeDisabled();await browser.saveScreenshot('artifacts/p3-pagination.png');await close();
 });
 it('trashes and restores all six records with live poster fallback, then purges with cascade',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  for(const original of records){
   const item=await get(original.id),before=await extras(item.id);
   await browser.$('[data-hotspot="poster_left"]').click();await click(item.title);await countDialogs(0);await roomImages(1);
   await open(item.kind);await card(item.title);await click('Çöp kutusuna taşı');await roomImages(0);await close();await browser.saveScreenshot('artifacts/p3-trash-fallback.png');
   await rejected('library_get',{id:item.id},'NOT_FOUND');await rejected('library_children_replace',{itemId:item.id,expectedRevision:item.revision+1,children:{kind:'book',quotes:[]}},'NOT_FOUND');
   await rejected('session_save',{draft:{id:randomUUID(),itemId:item.id,occurredAt:new Date().toISOString(),durationMinutes:10,notes:''},expectedRevision:null},'NOT_FOUND');
   await trashOpen();await card(item.title);await click('Geri al');await roomImages(1);await close();expect(await extras(item.id)).toEqual(before);await browser.saveScreenshot('artifacts/p3-restored-poster.png');
  }
  const purgedIds=[];
  for(const kind of ['game','album','book']){
  const doomed=await ipc('library_save',{item:{...draft(kind,`P3 kalıcı silinecek ${kind}`),coverAssetId:records[0].coverAssetId},expectedRevision:null});
  const shot={id:randomUUID(),assetId:doomed.coverAssetId,caption:'Purge screenshot',sortOrder:0,createdAt:new Date().toISOString()};
  const changed=await ipc('library_children_replace',{itemId:doomed.id,expectedRevision:doomed.revision,children:kind==='game'?{kind,screenshots:[shot]}:kind==='album'?{kind,tracks:[{id:randomUUID(),discNumber:1,trackNumber:1,title:'Purge track',durationSeconds:60,favorite:true,notes:'Purge note'}]}:{kind,quotes:[{id:randomUUID(),body:'Purge quote',pageNumber:1,comment:'Purge comment',createdAt:new Date().toISOString(),updatedAt:new Date().toISOString()}]}});
  await ipc('session_save',{draft:{id:randomUUID(),itemId:doomed.id,occurredAt:new Date().toISOString(),durationMinutes:10,notes:'Purge session'},expectedRevision:null});
  const col=await ipc('collection_save',{draft:{id:randomUUID(),name:`P3 kalan raf ${kind}`,description:''},expectedRevision:null,itemIds:[doomed.id,records[0].id]});
  await browser.$('[data-hotspot="poster_left"]').click();await click(doomed.title);await countDialogs(0);await roomImages(1);
  await rejected('trash_purge',{id:doomed.id},'NOT_FOUND');
  await open(kind);await card(doomed.title);await click('Çöp kutusuna taşı');await roomImages(0);await close();await trashOpen();await card(doomed.title);await click('Kalıcı sil');await countDialogs(2);await click('Vazgeç');await countDialogs(1);expect((await list({trashed:true,query:doomed.title})).total).toBe(1);await click('Kalıcı sil');await countDialogs(2);await click('Kalıcı sil');await countDialogs(1);await close();await roomImages(0);
  expect((await ipc('app_bootstrap')).slots.poster_left).toEqual({type:'default'});expect((await ipc('collection_list')).find(c=>c.id===col.id).itemIds).toEqual([records[0].id]);
  const generation=JSON.parse(readFileSync(join(root,'active.json'))).generationId;const db=new DatabaseSync(join(root,'storage',generation,'library.sqlite3'),{readOnly:true});for(const [table,column] of [['sessions','item_id'],['game_screenshots','game_id'],['album_tracks','album_id'],['book_quotes','book_id'],['collection_items','item_id']])expect(db.prepare(`SELECT count(*) AS n FROM ${table} WHERE ${column}=?`).get(doomed.id).n).toBe(0);db.close();
  purgedIds.push(changed.id);
  }
  // Reassign a surviving poster and persist every full DTO and child list for a new process.
  await browser.$('[data-hotspot="poster_left"]').click();await click(records[0].title);await countDialogs(0);await roomImages(1);
  const snapshots=[];for(const item of records)snapshots.push({item:await get(item.id),extras:await extras(item.id)});
  writeFileSync(evidencePath,JSON.stringify({snapshots,slots:(await ipc('app_bootstrap')).slots,collections:await ipc('collection_list'),total:(await list()).total,purgedIds},null,2));await browser.saveScreenshot('artifacts/p3-saved.png');
 });
 it('reopens all parents, children, tags, collections and poster in an independent process',async()=>{
  const evidence=JSON.parse(readFileSync(evidencePath));expect((await list()).total).toBe(evidence.total);expect((await ipc('app_bootstrap')).slots).toEqual(evidence.slots);expect(await ipc('collection_list')).toEqual(evidence.collections);for(const id of evidence.purgedIds)await rejected('library_get',{id},'NOT_FOUND');await roomImages(1);
  for(const snapshot of evidence.snapshots){expect(await get(snapshot.item.id)).toEqual(snapshot.item);expect(await extras(snapshot.item.id)).toEqual(snapshot.extras);await open(snapshot.item.kind);await card(snapshot.item.title);await expect((await top()).$(`p=${snapshot.item.notes}`)).toBeDisplayed();await browser.waitUntil(()=>browser.execute(()=>{const image=document.querySelector('[role=dialog] img');return image?.complete&&image.naturalWidth>0}));await close()}
  await browser.saveScreenshot(`artifacts/p3-${process.env.HUSH_REOPEN?'reopened':'verified'}.png`);
 });
});
