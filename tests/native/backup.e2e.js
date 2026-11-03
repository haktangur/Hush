import {browser,expect} from '@wdio/globals';
import {DatabaseSync} from 'node:sqlite';
import {readFileSync,writeFileSync,mkdirSync,copyFileSync,existsSync,unlinkSync} from 'node:fs';
import {join} from 'node:path';
import {randomUUID,createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
const root=process.env.HUSH_TEST_DATA_DIR, phase=process.env.HUSH_BACKUP_PHASE||'roundtrip';
const hash=b=>createHash('sha256').update(b).digest('hex');
const pointer=()=>JSON.parse(readFileSync(join(root,'active.json'))).generationId;
const base=()=>join(root,'storage',pointer());
const select=(save,name)=>writeFileSync(join(root,save?'backup-save-selection':'backup-open-selection'),name);
async function raw(c,r={}){return browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),c,r)}
async function ipc(c,r={}){const result=await raw(c,r);if(!result.ok)throw Error(`${c}: ${JSON.stringify(result.error)}`);return result.value}
function snapshot(){const db=new DatabaseSync(join(base(),'library.sqlite3'),{readOnly:true});const tables={};for(const {name} of db.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").all()){tables[name]=db.prepare(`SELECT * FROM "${name}"`).all().sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)))}db.close();const assets={};for(const a of tables.assets)for(const p of [a.relative_path,a.thumbnail_path])assets[p]=hash(readFileSync(join(base(),p)));return {tables,assets}}
function seedAsset(){const id=randomUUID(),dir=join(base(),'assets',id);mkdirSync(dir,{recursive:true});for(const name of ['image.webp','thumb.webp'])copyFileSync('tests/fixtures/room-album.webp',join(dir,name));const bytes=readFileSync(join(dir,'image.webp'));const db=new DatabaseSync(join(base(),'library.sqlite3'));db.prepare('INSERT INTO assets VALUES (?,?,?,?,?,?,?,?,?)').run(id,`assets/${id}/image.webp`,`assets/${id}/thumb.webp`,hash(bytes),'image/webp',bytes.length,128,128,new Date().toISOString());db.close();return id}
async function populate(){
 const ids=[],assets=Array.from({length:5},seedAsset),now=new Date().toISOString();
 for(const kind of ['game','album','book'])for(const trash of [false,true]){
  let item=await ipc('library_save',{expectedRevision:null,item:{id:randomUUID(),kind,title:`P6 ${kind} ${trash?'trash':'active'}`,creator:'Sentetik üretici',releaseYear:2026,coverAssetId:assets[ids.length%3],status:'active',rating:8,favorite:true,notes:'Türkçe kişisel not — ıİ %_',tagNames:['İstanbul','Işık'],startedOn:'2026-10-01',completedOn:null,provenance:kind==='book'?{provider:'open_library',entityType:'edition',externalId:`OL${ids.length+1}M`,editionId:null,sourceUrl:'https://openlibrary.org/',fetchedAt:now,attribution:'Sentetik kaynak'}:null,details:kind==='game'?{platform:'Mac',developer:'Hush',progressPercent:42,returnNote:'Kuzey kapısı'}:kind==='book'?{isbn:null,publisher:'Hush',language:'tr',totalPages:240,currentPage:37}:{firstListenedOn:'2026-10-02',spotifyUrl:'https://open.spotify.com/',appleMusicUrl:null}}});
  await ipc('session_save',{expectedRevision:null,draft:{id:randomUUID(),itemId:item.id,occurredAt:'2026-11-01T08:30:00Z',durationMinutes:35,notes:'Sentetik seans'}});
  const children=kind==='game'?{kind,screenshots:[{id:randomUUID(),assetId:assets[3],caption:'Sentetik ekran',sortOrder:0,createdAt:now}]}:kind==='book'?{kind,quotes:[{id:randomUUID(),body:'Sentetik alıntı',pageNumber:12,comment:'Kişisel yorum',createdAt:now,updatedAt:now}]}:{kind,tracks:[{id:randomUUID(),discNumber:1,trackNumber:1,title:'Sentetik parça',durationSeconds:123,favorite:true,notes:'Parça yorumu'}]};
  item=await ipc('library_children_replace',{itemId:item.id,expectedRevision:item.revision,children});ids.push(item.id);
  if(trash)await ipc('library_trash',{id:item.id,expectedRevision:item.revision});
 }
 await ipc('collection_save',{expectedRevision:null,draft:{id:randomUUID(),name:'P6 karma raf',description:'Üç tür'},itemIds:ids.filter((_,i)=>i%2===0)});
 for(const trash of [false,true]){const entry=await ipc('journal_save',{expectedRevision:null,entry:{id:randomUUID(),title:'P6 günlük',entryDate:'2026-11-01',mood:'calm',body:'Günlük gövdesi',itemIds:[ids[0],ids[2],ids[4]]}});if(trash)await ipc('journal_trash',{id:entry.id,expectedRevision:entry.revision})}
 await ipc('draft_put',{expectedRevision:null,entry:{id:randomUUID(),title:'P6 taslak',entryDate:'2026-11-02',mood:'focused',body:'Autosave içeriği',itemIds:[ids[4]]}});
 for(const [slotId,itemId,assetId] of [['poster_left',ids[4],null],['poster_right',null,assets[4]],['record_stand',ids[2],null]])await ipc('room_slot_set',{slotId,itemId,assetId});
 const b=await ipc('app_bootstrap');b.settings.value.room={...b.settings.value.room,preset:'quiet_dawn',motion:'reduced',neonIntensity:27,lampOn:false};await ipc('settings_save',{settings:b.settings.value,expectedRevision:b.settings.revision});
 return ids;
}
async function settings(){await browser.$('header').$('button=Ayarlar').click()}
async function images(){await browser.waitUntil(()=>browser.execute(()=>{const a=[...document.querySelectorAll('[data-testid="room-scene"] img')].filter(i=>!i.hasAttribute('aria-hidden'));return a.length===3&&a.every(i=>i.complete&&i.naturalWidth>0)}),{timeout:10000})}
async function mutate(){const b=await ipc('app_bootstrap');b.settings.value.room.neonIntensity=81;await ipc('settings_save',{settings:b.settings.value,expectedRevision:b.settings.revision});await ipc('draft_delete');writeFileSync(join(root,'changed.json'),JSON.stringify(snapshot()));}
describe(`P6 real native backup ${phase}`,()=>{
 it('runs real storage commands and verifies complete generations',async()=>{
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  if(phase==='roundtrip'){
   await populate();const expected=snapshot();expect(Object.values(expected.tables).every(rows=>rows.length>0)).toBe(true);writeFileSync(join(root,'expected.json'),JSON.stringify(expected));
   // Root/cache/log sentinels must never enter the archive.
   writeFileSync(join(root,'excluded-sentinel'),'synthetic-excluded-marker');
   select(true,'');expect(await ipc('backup_export')).toBeNull();select(false,'');expect(await ipc('backup_inspect')).toBeNull();expect(snapshot()).toEqual(expected);
   select(true,'complete.hushbackup');await settings();await browser.$('button=Yedek dışa aktar').click();await expect(browser.$('p=Yedek dosyası kaydedildi.')).toBeDisplayed();await browser.$('button=Kapat').click();
   execFileSync('python3',['tests/native/backup-fixtures.py',root]);
   const archiveCheck=JSON.parse(execFileSync('python3',['-c',`import zipfile,json,sys;z=zipfile.ZipFile(sys.argv[1]);print(json.dumps({'names':z.namelist(),'sentinel':any(b'synthetic-excluded-marker' in z.read(n) for n in z.namelist())}))`,join(root,'complete.hushbackup')],{encoding:'utf8'}));expect(archiveCheck.sentinel).toBe(false);expect(archiveCheck.names.length).toBe(12);
   for(const name of ['not-zip','truncated','traversal','absolute','backslash','unexpected','hash','missing','image','future','fk','details','settings','draft','trigger','slot','duplicate','symlink','oversize']){const old=pointer();select(false,`${name}.hushbackup`);const r=await raw('backup_inspect');expect(r.ok).toBe(false);expect({name,code:r.error.code}).toEqual({name,code:name==='future'?'UNSUPPORTED_SCHEMA':'INVALID_BACKUP'});expect(pointer()).toBe(old);expect(snapshot()).toEqual(expected)}
   select(false,'complete.hushbackup');const revoked=await ipc('backup_inspect');select(false,'');expect(await ipc('backup_inspect')).toBeNull();expect((await raw('backup_restore',{inspectionToken:revoked.inspectionToken})).error.code).toBe('EXPIRED_BACKUP');
   select(false,'not-zip.hushbackup');await settings();await browser.$('button=Yedek seç ve incele').click();await expect(browser.$('[role="alert"]')).toHaveText('Yedek doğrulanamadı. Mevcut verilerin değiştirilmedi.');await browser.saveScreenshot('artifacts/p6-rejected.png');await browser.$('button=Kapat').click();
   const savedHash=hash(readFileSync(join(root,'complete.hushbackup')));
   for(const [point,command] of [['export_write','backup_export'],['inspect_space','backup_inspect'],['restore_write','backup_restore']]){
    select(false,'complete.hushbackup');const preview=await ipc('backup_inspect');const before=pointer();writeFileSync(join(root,'backup-io-fault'),point);
    const failed=await raw(command,command==='backup_restore'?{inspectionToken:preview.inspectionToken}:{});expect(failed.error.code).toBe('DISK_FULL');unlinkSync(join(root,'backup-io-fault'));expect(pointer()).toBe(before);expect(snapshot()).toEqual(expected);expect(hash(readFileSync(join(root,'complete.hushbackup')))).toBe(savedHash);
   }
   await mutate();const changed=snapshot(),old=pointer();select(false,'complete.hushbackup');await settings();await browser.$('button=Yedek seç ve incele').click();await expect(browser.$('button=Bu yedeği geri yükle')).toBeDisplayed();expect(snapshot()).toEqual(changed);await browser.saveScreenshot('artifacts/p6-preview.png');await browser.$('button=Vazgeç').click();expect(pointer()).toBe(old);expect(snapshot()).toEqual(changed);
   await browser.$('button=Yedek seç ve incele').click();await browser.$('button=Bu yedeği geri yükle').waitForDisplayed();await browser.$('button=Bu yedeği geri yükle').click();await browser.waitUntil(async()=>(await browser.$$('[role="dialog"]')).length===0);expect(pointer()).not.toBe(old);expect(snapshot()).toEqual(expected);expect(existsSync(join(root,'storage',old,'library.sqlite3'))).toBe(true);await images();await browser.saveScreenshot('artifacts/p6-restored.png');
   const preview=await ipc('backup_inspect');await ipc('backup_restore',{inspectionToken:preview.inspectionToken});expect((await raw('backup_restore',{inspectionToken:preview.inspectionToken})).error.code).toBe('EXPIRED_BACKUP');expect(snapshot()).toEqual(expected);
   writeFileSync(join(root,'roundtrip-passed'),'true');
  } else if(phase.startsWith('crash-')){
   expect(snapshot()).toEqual(JSON.parse(readFileSync(join(root,'expected.json'))));await mutate();const point=phase.slice(6);writeFileSync(join(root,'pre-crash-pointer'),pointer());writeFileSync(join(root,'backup-fault'),point);select(false,'complete.hushbackup');const preview=await ipc('backup_inspect');
   // Awaiting this invocation is deliberately interrupted by the runner's SIGKILL.
   await ipc('backup_restore',{inspectionToken:preview.inspectionToken});throw Error('Fault point did not stop restore');
  } else {
   const after=phase==='verify-after_pointer',crash=phase.startsWith('verify-');expect(snapshot()).toEqual(JSON.parse(readFileSync(join(root,crash&&!after?'changed.json':'expected.json'))));
   if(crash){const old=readFileSync(join(root,'pre-crash-pointer'),'utf8');if(after)expect(pointer()).not.toBe(old);else expect(pointer()).toBe(old);expect(existsSync(join(root,'storage',old,'library.sqlite3'))).toBe(true);unlinkSync(join(root,'backup-fault'));select(false,'complete.hushbackup');const preview=await ipc('backup_inspect');await ipc('backup_restore',{inspectionToken:preview.inspectionToken});expect(snapshot()).toEqual(JSON.parse(readFileSync(join(root,'expected.json'))))}
   await browser.refresh();await images();await browser.saveScreenshot(`artifacts/p6-${phase}.png`);writeFileSync(join(root,`${phase}-passed`),'true');
  }
 });
});
