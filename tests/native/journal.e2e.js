import { browser, expect } from '@wdio/globals';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { randomUUID } from 'node:crypto';
const root=process.env.HUSH_TEST_DATA_DIR;
const evidencePath=join(root,'journal-evidence.json');
async function raw(command,request={}){return browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request)}
async function ipc(command,request={}){const r=await raw(command,request);if(!r.ok)throw new Error(`${command}: ${JSON.stringify(r.error)}`);return r.value}
async function top(){return (await browser.$$('[role="dialog"]')).at(-1)}
async function click(text){await (await top()).$(`button=${text}`).click()}
async function field(label){return (await top()).$(`.//label[normalize-space(text())="${label}"]/*[self::input or self::textarea or self::select]`)}
async function fill(label,value){await (await field(label)).setValue(value)}
async function select(label,value){await browser.execute((e,v)=>{e.value=v;e.dispatchEvent(new Event('change',{bubbles:true}))},await field(label),value)}
async function dialogs(n){await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===n)}
async function open(name){await browser.$('nav').$(`button=${name}`).click();await dialogs(1)}
async function close(){await click('Kapat');await dialogs(0)}
async function save(){await click('Kaydet');await dialogs(1)}
async function journals(filter={}){return ipc('journal_list',{offset:0,limit:100,...filter})}
async function article(title){return (await top()).$(`.//article[.//h2[normalize-space(.)="${title}"]]`)}
async function entryAction(title,action){await (await article(title)).$(`button=${action}`).click()}
async function cmdK(){await browser.keys(['Meta','k']);await dialogs(1);await expect((await top()).$('h2=Genel arama')).toBeDisplayed()}
function item(kind,title){return {id:randomUUID(),kind,title,creator:`P4 üretici ${kind}`,releaseYear:null,coverAssetId:null,rating:null,favorite:false,notes:kind==='book'?'NOTİÇERİĞİ':'',tagNames:kind==='album'?['IŞIKETİKET']:[],startedOn:null,completedOn:null,provenance:null,status:'planned',details:kind==='book'?{totalPages:null,currentPage:0,isbn:null,publisher:'',language:''}:kind==='game'?{platform:'',developer:'',progressPercent:null,returnNote:''}:{firstListenedOn:null,spotifyUrl:null,appleMusicUrl:null}}}
let records=[],entries=[],calendars={};
describe('P4 real macOS journal, calendar and local search acceptance',()=>{
 it('creates multiple same-day entries, edits moods/links and restores trash',async function(){
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');if(process.env.HUSH_REOPEN)return this.skip();
  await browser.setWindowSize(1440,900);expect((await journals()).total).toBe(0);
  for(const [kind,title] of [['game','IŞIK'],['album','İNCİ'],['book','Oran %_']])records.push(await ipc('library_save',{item:item(kind,title),expectedRevision:null}));
  await open('Günlük');await expect((await top()).$('p=Henüz günlük kaydı yok.')).toBeDisplayed();
  for(let i=0;i<2;i++){
   await click('Ekle');await fill('Başlık',`P4 günlük ${i}`);await fill('Gün','2026-11-01');await fill('Bugünden kalanlar',`Sentetik gövde ${i} %_ GÖVDEİZİ`);await select('Ruh hâli',i?'happy':'calm');await (await field(records[i].title)).click();await save();
   await entryAction(`P4 günlük ${i}`,'Düzenle');await fill('Bugünden kalanlar',`Düzenlenmiş gövde ${i} %_ GÖVDEİZİ`);await (await field(records[2].title)).click();await save();
  }
  entries=(await journals()).items;expect(entries).toHaveLength(2);
  for(const e of entries){const i=Number(e.title.at(-1));expect(e.entryDate).toBe('2026-11-01');expect(e.itemIds.sort()).toEqual([records[i].id,records[2].id].sort());expect(e.mood).toBe(i?'happy':'calm')}
  for(const mood of ['focused','tired','melancholic','excited', '']){await entryAction('P4 günlük 0','Düzenle');await select('Ruh hâli',mood);await save();expect((await journals({query:'P4 günlük 0'})).items[0].mood).toBe(mood||null)}
  // Real concurrent revision change must reject the UI save and preserve its text.
  await entryAction('P4 günlük 0','Düzenle');await fill('Bugünden kalanlar','Çakışmada korunacak sentetik metin');
  const current=(await journals({query:'P4 günlük 0'})).items[0];const {revision,createdAt,updatedAt,deletedAt,...entry}=current;
  await ipc('journal_save',{entry:{...entry,mood:'focused'},expectedRevision:revision});await click('Kaydet');await expect((await top()).$('[role="alert"]')).toBeDisplayed();await expect(await field('Bugünden kalanlar')).toHaveValue('Çakışmada korunacak sentetik metin');await click('Vazgeç');await click('Değişiklikleri at');await dialogs(1);
  for(let i=0;i<2;i++){await entryAction(`P4 günlük ${i}`,'Çöp kutusuna taşı');await browser.waitUntil(async()=>(await journals({query:`P4 günlük ${i}`})).total===0)}
  await expect((await top()).$('p=Henüz günlük kaydı yok.')).toBeDisplayed();expect((await ipc('search_local',{query:'GÖVDEİZİ',limit:30})).journals).toHaveLength(0);await close();
  await browser.$('header').$('button=Ayarlar').click();await click('Silinen günlükler');await dialogs(1);
  for(let i=0;i<2;i++){await entryAction(`P4 günlük ${i}`,'Geri al');await browser.waitUntil(async()=>(await journals({trashed:true,query:`P4 günlük ${i}`})).total===0)}await close();
  entries=(await journals()).items;for(const e of entries){expect(e.itemIds).toHaveLength(2);expect(e.entryDate).toBe('2026-11-01')}
  await open('Günlük');await fill('Koleksiyonda ara','olmayan günlük');await expect((await top()).$('p=Eşleşen günlük yok.')).toBeDisplayed();await click('Filtreyi temizle');await browser.waitUntil(async()=>await (await top()).$$('article').length===2);await browser.saveScreenshot('artifacts/p4-journals.png');await close();
 });
 it('groups sessions at month, midnight and DST boundaries with exact totals',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  for(const r of records)for(const [occurredAt,durationMinutes] of [['2026-10-31T22:00:00.000Z',10],['2026-11-01T07:30:00.000Z',20],['2026-12-01T01:00:00.000Z',30]])await ipc('session_save',{draft:{id:randomUUID(),itemId:r.id,occurredAt,durationMinutes,notes:'P4 ay sınırı'},expectedRevision:null});
  for(const [occurredAt,durationMinutes] of [['2026-11-01T08:30:00.000Z',4],['2026-11-01T09:30:00.000Z',6]])await ipc('session_save',{draft:{id:randomUUID(),itemId:records[0].id,occurredAt,durationMinutes,notes:'P4 DST tekrar saati'},expectedRevision:null});
  for(const zone of ['Europe/Istanbul','America/Los_Angeles','UTC'])for(const month of [10,11,12]){
   const c=await ipc('calendar_month',{year:2026,month,timeZone:zone});calendars[`${zone}/${month}`]=c;
   const base=zone==='Europe/Istanbul'?{10:0,11:30,12:30}:zone==='America/Los_Angeles'?{10:10,11:50,12:0}:{10:10,11:20,12:30};
   expect([c.gameMinutes,c.bookMinutes,c.albumMinutes]).toEqual([base[month]+(month===11?10:0),base[month],base[month]]);expect(c.journalDays).toBe(month===11?1:0);
   for(const e of c.events.filter(e=>e.kind==='journal'))expect(e.day).toBe('2026-11-01');
   if(month===11){expect(c.events.filter(e=>e.day==='2026-11-01'&&e.kind==='game')).toHaveLength(zone==='Europe/Istanbul'?4:3);}
   expect(c.addedItems).toBe(records.filter(r=>{const parts=new Intl.DateTimeFormat('en-US',{timeZone:zone,year:'numeric',month:'numeric'}).formatToParts(new Date(r.createdAt));return Number(parts.find(p=>p.type==='year').value)===2026&&Number(parts.find(p=>p.type==='month').value)===month}).length);
  }
  expect((await raw('calendar_month',{year:2026,month:11,timeZone:'Invalid/Zone'})).error.code).toBe('VALIDATION');
  await open('Takvim');const delta=await browser.execute(()=>{const d=new Date();return (2026-d.getFullYear())*12+10-d.getMonth()});for(let n=0;n<Math.abs(delta);n++)await click(delta>0?'Sonraki':'Önceki');for(const zone of ['Europe/Istanbul','America/Los_Angeles','UTC']){await select('Saat dilimi',zone);const c=calendars[`${zone}/11`];await browser.waitUntil(async()=>(await (await top()).getText()).includes(`Oyunlar: ${c.gameMinutes} · Kitaplar: ${c.bookMinutes} · Albümler: ${c.albumMinutes}`));await (await top()).$('button[aria-label="2026-11-01"]').click();await expect((await top()).$('h3=2026-11-01')).toBeDisplayed();expect((await (await top()).getText()).includes('P4 günlük 0')).toBe(true);await click('Önceki');await click('Sonraki')}
  await select('Saat dilimi','America/Los_Angeles');await (await top()).$('button[aria-label="2026-11-30"]').click();await browser.waitUntil(async()=>(await (await top()).getText()).includes('30 dakika'));await browser.saveScreenshot('artifacts/p4-calendar.png');await (await top()).$('button[aria-label="2026-11-15"]').click();await expect((await top()).$('p=Bu gün için kayıt yok.')).toBeDisplayed();await close();
  // Mutation changes month totals immediately on reopening; trashed parents hide sessions.
  await ipc('library_trash',{id:records[0].id,expectedRevision:records[0].revision});expect((await ipc('calendar_month',{year:2026,month:11,timeZone:'UTC'})).gameMinutes).toBe(0);await ipc('library_restore',{id:records[0].id,expectedRevision:records[0].revision+1});
  const temp=await ipc('session_save',{draft:{id:randomUUID(),itemId:records[0].id,occurredAt:'2026-11-01T00:00:00Z',durationMinutes:7,notes:'Silinecek'},expectedRevision:null});expect((await ipc('calendar_month',{year:2026,month:11,timeZone:'UTC'})).gameMinutes).toBe(37);await ipc('session_delete',{id:temp.id,expectedRevision:temp.revision});expect((await ipc('calendar_month',{year:2026,month:11,timeZone:'UTC'})).gameMinutes).toBe(30);
 });
 it('uses Cmd+K, Turkish case pairs and literal search without outgoing HTTP',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  const before=readFileSync(join(root,'http-attempts.txt'),'utf8');expect(before).toBe('');
  await browser.execute(()=>performance.clearResourceTimings());
  // Pair semantics: dotless I/ı is distinct from dotted İ/i.
  for(const [q,expected] of [['IŞIK','IŞIK'],['ışık','IŞIK'],['İNCİ','İNCİ'],['inci','İNCİ'],['I','IŞIK'],['ı','IŞIK'],['İ','İNCİ'],['i','İNCİ']]){const result=await ipc('search_local',{query:q,limit:30});expect(result.items.some(i=>i.title===expected)).toBe(true)}
  for(const q of ['IŞIK','ışık','İNCİ','inci','%_','notiçeriği','ışıketiket','gövdeizi','P4 üretici book','bulunmayan']){
   await cmdK();await fill('Koleksiyonda ara',q);const result=await ipc('search_local',{query:q,limit:30});
   if(q==='bulunmayan')await expect((await top()).$('p=Eşleşen yerel kayıt yok.')).toBeDisplayed();else {await browser.waitUntil(async()=>(await (await top()).$$('button').length)===result.items.length+result.journals.length+1);expect(result.items.length+result.journals.length).toBeGreaterThan(0)}
   if(q==='%_'){expect(result.items.map(i=>i.id)).toEqual([records[2].id]);expect(result.journals).toHaveLength(2);await browser.saveScreenshot('artifacts/p4-search.png')}
   await close();
  }
  // Exact single-letter fixtures avoid incidental letters in creator/notes/tags.
  const isolated=[];for(const title of ['I','İ','ı','i'])isolated.push(await ipc('library_save',{item:{...item('book',title),creator:'',notes:''},expectedRevision:null}));
  for(const q of ['I','ı','İ','i']){const ids=(await ipc('search_local',{query:q,limit:30})).items.filter(i=>isolated.some(x=>x.id===i.id)).map(i=>i.title).sort();expect(ids).toEqual((q==='I'||q==='ı'?['I','ı']:['İ','i']).sort())}
  for(const r of isolated)await ipc('library_trash',{id:r.id,expectedRevision:r.revision});
  await cmdK();await fill('Koleksiyonda ara','notiçeriği');await click('Oran %_ · P4 üretici book');await expect((await top()).$('h2=Oran %_')).toBeDisplayed();await close();
  const duplicate=await ipc('journal_save',{entry:{id:randomUUID(),title:'P4 günlük 1',entryDate:'2026-10-31',body:'Aynı başlıklı ayrı günlük',mood:null,itemIds:[]},expectedRevision:null});
  await cmdK();await fill('Koleksiyonda ara','P4 günlük 1');await click('2026-11-01 · P4 günlük 1');await expect((await top()).$('h2=P4 günlük 1')).toBeDisplayed();expect(await (await top()).$$('article').length).toBe(1);expect(await (await top()).getText()).not.toContain('Aynı başlıklı ayrı günlük');await close();await ipc('journal_trash',{id:duplicate.id,expectedRevision:duplicate.revision});
  const network=await browser.execute(()=>performance.getEntriesByType('resource').filter(e=>/^https?:/.test(e.name)).map(e=>e.name));expect(network).toEqual([]);expect(readFileSync(join(root,'http-attempts.txt'),'utf8')).toBe(before);
  writeFileSync(join(root,'search-network-evidence.json'),JSON.stringify({rustHttpAttempts:0,webViewHttpResources:network,queries:24},null,2));
 });
 it('leaves a complete autosaved draft for a separate process',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();await open('Günlük');await click('Ekle');await fill('Başlık','P4 yeniden açılan taslak');await fill('Gün','2026-11-01');await select('Ruh hâli','melancholic');await fill('Bugünden kalanlar','Son autosave metni: I İ ı i %_');await (await field(records[1].title)).click();await expect((await top()).$('p=Taslak bu Mac’e kaydedildi.')).toBeDisplayed();const draft=await ipc('draft_get');expect(draft.entry.body).toBe('Son autosave metni: I İ ı i %_');expect(draft.entry.itemIds).toEqual([records[1].id]);expect((await journals()).total).toBe(2);writeFileSync(evidencePath,JSON.stringify({entries:(await journals()).items,records,calendars,draft},null,2));await browser.saveScreenshot('artifacts/p4-draft.png');
 });
 it('recovers and saves the exact draft after process restart',async function(){
  if(!process.env.HUSH_REOPEN)return this.skip();const prior=JSON.parse(readFileSync(evidencePath));expect(await ipc('draft_get')).toEqual(prior.draft);expect((await journals()).items).toEqual(prior.entries);
  for(const [key,expected] of Object.entries(prior.calendars)){const at=key.lastIndexOf('/');expect(await ipc('calendar_month',{year:2026,month:Number(key.slice(at+1)),timeZone:key.slice(0,at)})).toEqual(expected)}
  await open('Günlük');await click('Taslağı sürdür');await expect(await field('Başlık')).toHaveValue(prior.draft.entry.title);await expect(await field('Gün')).toHaveValue('2026-11-01');await expect(await field('Ruh hâli')).toHaveValue('melancholic');await expect(await field('Bugünden kalanlar')).toHaveValue(prior.draft.entry.body);await expect(await field(prior.records[1].title)).toBeChecked();await save();expect(await ipc('draft_get')).toBeNull();const saved=(await journals({query:'P4 yeniden açılan taslak'})).items[0];for(const [key,value] of Object.entries(prior.draft.entry))expect(saved[key]).toEqual(value);expect((await journals()).total).toBe(3);await browser.saveScreenshot('artifacts/p4-reopened.png');await close();
 });
});
