import { browser, expect } from '@wdio/globals';
import { DatabaseSync } from 'node:sqlite';
import { readFileSync, writeFileSync, mkdirSync, copyFileSync } from 'node:fs';
import { join } from 'node:path';
import { randomUUID, createHash } from 'node:crypto';
const root=process.env.HUSH_TEST_DATA_DIR;
const evidencePath=join(root,'room-evidence.json');
const hotspot=id=>browser.$(`[data-hotspot="${id}"]`);
const dialog=()=>browser.$('[role="dialog"]');
async function ipc(command,request={}){const r=await browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request);expect(r.ok).toBe(true);return r.value}
async function closed(){await browser.waitUntil(async()=>await browser.$$('[role="dialog"]').length===0)}
async function escape(){await browser.keys('Escape');await closed()}
async function title(text){await expect(dialog().$('h2')).toHaveText(text)}
async function focusIs(selector){await browser.waitUntil(()=>browser.execute(s=>document.activeElement===document.querySelector(s),selector),{timeout:3000,timeoutMsg:`Focus must return to ${selector}`})}
async function shortcut(key){await browser.keys(['Meta',key]);await browser.keys('NULL')}
async function fit(){await browser.waitUntil(()=>browser.execute(()=>{const s=document.querySelector('[data-testid="room-scene"]').getBoundingClientRect();const p=document.querySelector('[data-testid="room-scene"]').parentElement.getBoundingClientRect();return s.width<=p.width+1&&s.height<=p.height+1&&s.top>=p.top-1&&[...document.querySelectorAll('[data-hotspot]')].every(e=>{const r=e.getBoundingClientRect();return r.width>=44&&r.height>=44&&r.left>=0&&r.right<=innerWidth&&r.top>=0&&r.bottom<=innerHeight})}),{timeout:5000})}
async function animations(expected){await browser.waitUntil(()=>browser.execute(expected=>{const el=document.querySelector('[data-testid="room-scene"]');const a=el.getAnimations({subtree:true});return expected==='none'?a.length===0:a.length>0&&a.every(a=>a.playState===expected)},expected))}
async function settings(preset,motion){await browser.$('header').$('button=Ayarlar').click();await browser.execute((preset,motion)=>{const selects=document.querySelectorAll('[role=dialog] select');for(const [i,value] of [preset,motion].entries()){selects[i].value=value;selects[i].dispatchEvent(new Event('change',{bubbles:true}))}},preset,motion);await dialog().$('button=Kaydet').click();await closed();await browser.waitUntil(()=>browser.execute(p=>document.querySelector('main').dataset.preset===p,preset))}
const destinations=[['bookshelf','Kitaplar','4'],['game_terminal','Oyunlar','2'],['record_station','Albümler','3'],['diary','Günlük','5'],['calendar','Takvim','6'],['poster_left','Sol poster','7'],['poster_right','Sağ poster','8']];
// Only synthetic image fixtures are inserted here. All item, slot and settings writes use real IPC/UI.
function seedAsset(fixture='room-cover.webp',size=1600){const generation=JSON.parse(readFileSync(join(root,'active.json'))).generationId;const base=join(root,'storage',generation);const id=randomUUID();const dir=join(base,'assets',id);mkdirSync(dir,{recursive:true});for(const name of ['image.webp','thumb.webp'])copyFileSync(join('tests/fixtures',fixture),join(dir,name));const bytes=readFileSync(join(dir,'image.webp'));const db=new DatabaseSync(join(base,'library.sqlite3'));db.prepare('INSERT INTO assets VALUES (?,?,?,?,?,?,?,?,?)').run(id,`assets/${id}/image.webp`,`assets/${id}/thumb.webp`,createHash('sha256').update(bytes).digest('hex'),'image/webp',bytes.length,size,size,new Date().toISOString());db.close();return id}
describe('P2 real macOS room acceptance',()=>{
 it('checks all hotspots, menu/keyboard paths, focus and minimum size',async function(){
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  if(process.env.HUSH_REOPEN){return this.skip()}
  expect((await ipc('library_list',{sort:'title_asc',offset:0,limit:60})).total).toBe(0);
  for(const width of [1440,1100]){
   await browser.setWindowSize(width,width===1440?900:720);await fit();await browser.saveScreenshot(`artifacts/p2-empty-${width}.png`);
   expect(await browser.$$('[data-hotspot]').length).toBe(8);
   for(const [id,label,key] of destinations){
    await hotspot(id).click();await title(label);await browser.keys(['Shift','Tab']);await browser.keys('NULL');expect(await browser.execute(()=>!!document.activeElement.closest('[role="dialog"]'))).toBe(true);await escape();await focusIs(`[data-hotspot="${id}"]`);
    await shortcut(key);await title(label);await escape();await focusIs(`[data-hotspot="${id}"]`);
    if(!id.startsWith('poster')){await browser.$('nav').$(`button=${label}`).click();await title(label);await escape()}
    else{await browser.$('header').$('button=Ayarlar').click();await dialog().$(`button=${label}`).click();await title(label);await escape()}
   }
   const before=(await ipc('app_bootstrap')).settings.value.room.lampOn;
   await hotspot('lamp').click();await browser.waitUntil(async()=>(await ipc('app_bootstrap')).settings.value.room.lampOn!==before);await focusIs('[data-hotspot="lamp"]');await shortcut('l');await browser.waitUntil(async()=>(await ipc('app_bootstrap')).settings.value.room.lampOn===before);
  }
 });
 it('checks nested Escape, validation, motion and three light presets',async function(){
  if(process.env.HUSH_REOPEN){return this.skip()}
  await hotspot('bookshelf').click();await dialog().$('button=Ekle').click();await browser.$('input[name="title"]').setValue('Uzun Türkçe başlık — Yağmurlu gecelerde şehrin sessiz köşelerinden kalan hikâyeler');await browser.$('input[name="totalPages"]').setValue('20');await browser.$('input[name="currentPage"]').setValue('37');await browser.$('textarea[name="notes"]').setValue('Hata sonrası korunmalı.');await browser.$('button=Kaydet').click();await expect(browser.$('[role="alert"]')).toBeDisplayed();await browser.saveScreenshot('artifacts/p2-form-error-1100.png');await browser.keys('Escape');await expect(browser.$('p=Kaydedilmemiş değişikliklerin var.')).toBeDisplayed();await browser.$('button=Geri').click();await expect(browser.$('textarea[name="notes"]')).toHaveValue('Hata sonrası korunmalı.');await browser.keys('Escape');await browser.$('button=Değişiklikleri at').click();await title('Kitaplar');await escape();
  for(const preset of ['violet_rain','quiet_dawn','cozy_night']){await settings(preset,'full');await animations('running');await browser.saveScreenshot(`artifacts/p2-${preset}.png`)}
  await hotspot('bookshelf').click();await animations('paused');await escape();await animations('running');
  await browser.minimizeWindow();await animations('paused');await browser.setWindowSize(1100,720);await browser.maximizeWindow();await browser.setWindowSize(1100,720);
  await settings('quiet_dawn','reduced');await animations('none');
 });
 it('persists populated room, both posters, record stand and light settings across process restart',async()=>{
  if(!process.env.HUSH_REOPEN){
   const cover=seedAsset();const albumCover=seedAsset('room-album.webp',128);
   const base={creator:'Hush sentetik test',releaseYear:null,rating:null,favorite:false,notes:'Yalnız P2 kabul verisi',tagNames:[],startedOn:null,completedOn:null,provenance:null,status:'planned'};
   const book=await ipc('library_save',{expectedRevision:null,item:{...base,id:randomUUID(),kind:'book',title:'P2 — Yağmurun ardından uzun Türkçe kitap başlığı',coverAssetId:cover,details:{totalPages:200,currentPage:37,isbn:null,publisher:'',language:'tr'}}});
   const album=await ipc('library_save',{expectedRevision:null,item:{...base,id:randomUUID(),kind:'album',title:'P2 — Sessiz şehir albümü',coverAssetId:albumCover,details:{firstListenedOn:null,spotifyUrl:null,appleMusicUrl:null}}});
   for(const [slot,item] of [['poster_left',book],['poster_right',album]]){await hotspot(slot).click();await dialog().$(`button=${item.title}`).click();await closed()}
   await browser.$('header').$('button=Ayarlar').click();await dialog().$('button=Plak standı').click();await dialog().$(`button=${album.title}`).click();await closed();
   await hotspot('poster_left').click();await dialog().$('button=Atanan eseri aç').click();await expect(browser.$(`h2=${book.title}`)).toBeDisplayed();await escape();
   await browser.$('header').$('button=Ayarlar').click();const checks=await dialog().$$('input[type="checkbox"]');await checks[0].click();await checks[1].click();await checks[2].click();await dialog().$('input[type="range"]').setValue('27');await dialog().$('button=Kaydet').click();await closed();
   const state=await ipc('app_bootstrap');writeFileSync(evidencePath,JSON.stringify({settings:state.settings,slots:state.slots,bookId:book.id,albumId:album.id},null,2));
  }
  const evidence=JSON.parse(readFileSync(evidencePath));const state=await ipc('app_bootstrap');expect(state.settings).toEqual(evidence.settings);expect(state.slots).toEqual(evidence.slots);await animations('none');
  await browser.waitUntil(()=>browser.execute(()=>[...document.querySelectorAll('[data-testid="room-scene"] img')].every(i=>i.complete&&i.naturalWidth>0)));
  for(const width of [1440,1100]){await browser.setWindowSize(width,width===1440?900:720);await fit();await browser.saveScreenshot(`artifacts/p2-filled-${process.env.HUSH_REOPEN?'reopened':'saved'}-${width}.png`)}
  await hotspot('bookshelf').click();await browser.$('strong=P2 — Yağmurun ardından uzun Türkçe kitap başlığı').click();await expect(browser.$('h2=P2 — Yağmurun ardından uzun Türkçe kitap başlığı')).toBeDisplayed();await browser.saveScreenshot('artifacts/p2-book-panel.png');await escape();
 });
});
