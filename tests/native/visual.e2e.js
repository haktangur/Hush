import {browser,expect} from '@wdio/globals';
import {mkdirSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {randomUUID} from 'node:crypto';
// Visual QA tour: synthetic records through real IPC, screenshots of every screen at both required window sizes.
const root=process.env.HUSH_TEST_DATA_DIR;
const out='artifacts/ui';
async function ipc(command,request={}){const r=await browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request);expect(r.ok).toBe(true);return r.value}
const dialogs=()=>browser.$$('[role="dialog"]');
async function closeAll(){for(let i=0;i<4&&(await dialogs()).length;i++){await browser.keys('Escape');await browser.pause(150)}}
async function shortcut(key){await browser.keys(['Meta',key]);await browser.keys('NULL');await browser.pause(350)}
async function layoutIssues(){return browser.execute(()=>{const issues=[];for(const el of document.querySelectorAll('[role=dialog] *, header *, nav *')){const r=el.getBoundingClientRect();if(!r.width)continue;if(r.right>innerWidth+1||r.left<-1)issues.push(`${el.tagName}.${el.className}: x ${Math.round(r.left)}–${Math.round(r.right)}`)}for(const d of document.querySelectorAll('[role=dialog]')){if(d.scrollWidth>d.clientWidth+1)issues.push(`dialog horizontal scroll ${d.scrollWidth}>${d.clientWidth}`)}return issues.slice(0,8)})}
const report={};
async function shot(size,name){await browser.pause(300);await browser.saveScreenshot(join(out,`${size}-${name}.png`));const issues=await layoutIssues();if(issues.length)report[`${size}-${name}`]=issues}
function item(kind,i){const titles={book:['Kuzey Işıkları Altında','Sessiz Şehir','Yağmurun Dili'],game:['Neon Liman','Gece Vardiyası','Kayıp Sinyal'],album:['Mor Bulutlar','Gece Treni','Sahil Yolu']}[kind];return {id:randomUUID(),kind,title:titles[i],creator:['Ayşe Demir','Deniz Kaya','Mert Işık'][i],releaseYear:2018+i,coverAssetId:null,status:kind==='album'?['planned','active','completed'][i]:['planned','active','completed'][i],rating:6+i,favorite:i===1,notes:'Sentetik görsel QA kaydı. Yağmurlu bir akşam için not.',tagNames:['gece','şehir'],startedOn:'2026-10-0'+(i+1),completedOn:null,provenance:null,details:kind==='game'?{platform:'Mac',developer:'Bağımsız stüdyo',progressPercent:30*i,returnNote:'Liman kapısından devam'}:kind==='book'?{isbn:null,publisher:'Hush Yayınları',language:'tr',totalPages:320,currentPage:40*(i+1)}:{firstListenedOn:'2026-10-02',spotifyUrl:null,appleMusicUrl:null}}}
describe('Visual QA tour',()=>{
 it('captures every screen at 1440×900 and 1100×720',async function(){
  if(process.env.HUSH_REOPEN)return this.skip();
  mkdirSync(out,{recursive:true});
  await expect(browser.$('[role=status]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  const saved=[];for(const kind of ['book','game','album'])for(let i=0;i<3;i++)saved.push(await ipc('library_save',{expectedRevision:null,item:item(kind,i)}));
  for(const it of saved.slice(0,4))await ipc('session_save',{expectedRevision:null,draft:{id:randomUUID(),itemId:it.id,occurredAt:'2026-11-03T19:30:00Z',durationMinutes:45,notes:'Akşam seansı'}});
  await ipc('journal_save',{expectedRevision:null,entry:{id:randomUUID(),title:'Yağmurlu salı',entryDate:'2026-11-03',mood:'calm',body:'Pencere kenarında kitap okudum, sonra eski bir albümü dinledim.',itemIds:[saved[0].id]}});
  await ipc('collection_save',{expectedRevision:null,draft:{id:randomUUID(),name:'Yağmurlu geceler',description:'Gece için seçtiklerim'},itemIds:[saved[0].id,saved[3].id]});
  await browser.execute(()=>location.reload());await expect(browser.$('[role=status]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  for(const [w,h] of [[1440,900],[1100,720]]){
   const size=`${w}`;await browser.setWindowSize(w,h);await browser.pause(400);await closeAll();
   if(await browser.$('.onboarding').isExisting())await shot(size,'01-room-onboarding');
   if(await browser.$('button=Odaya gir').isExisting()){await browser.$('button=Odaya gir').click();await browser.pause(300)}
   await shot(size,'02-room');
   await shortcut('4');report.selectStyle=await browser.execute(()=>{const s=document.querySelector('[role=dialog] select');const c=s&&getComputedStyle(s);return c&&{bg:c.backgroundImage,appearance:c.appearance||c.webkitAppearance,pos:c.backgroundPosition,h:c.height}});await closeAll();
   for(const [key,name] of [['2','03-games'],['3','04-albums'],['4','05-books']]){await shortcut(key);await shot(size,name);await closeAll()}
   await shortcut('4');const list=await browser.$('button=Kompakt liste');if(await list.isExisting()){await list.click();await shot(size,'06-books-list');await browser.$('button=Kapaklar').click()}
   const card=await browser.$('[role=dialog] [data-library-view] button');if(await card.isExisting()){await card.click();await shot(size,'07-book-detail')}
   await closeAll();await shortcut('4');await shortcut('n');await shot(size,'08-book-form');await browser.execute(()=>{const d=[...document.querySelectorAll('[role=dialog]')].at(-1);d.scrollTop=d.scrollHeight});await shot(size,'09-book-form-bottom');await closeAll();
   await shortcut('2');await shortcut('n');await shot(size,'10-game-form');await closeAll();
   await shortcut('5');await shot(size,'11-journal');await shortcut('n');await shot(size,'12-journal-editor');await closeAll();
   await shortcut('6');await shot(size,'13-calendar');await closeAll();
   await shortcut('k');const q=await browser.$('[role=dialog] input');await q.setValue('ş');await browser.pause(400);await shot(size,'14-search');await closeAll();
   await browser.$('header').$('button=Koleksiyonlar').click();await shot(size,'15-collections');await closeAll();
   await shortcut(',');await shot(size,'16-settings');await browser.execute(()=>{const d=[...document.querySelectorAll('[role=dialog]')].at(-1).querySelector('[class*=panelBody]');if(d)d.scrollTop=d.scrollHeight});await shot(size,'17-settings-bottom');await closeAll();
   await browser.$('nav').$('button=Çöp kutusu').click();await shot(size,'18-trash');await closeAll();
   await shortcut('7');await shot(size,'19-poster');await closeAll();
  }
  writeFileSync(join(out,'layout-report.json'),JSON.stringify(report,null,2));
  writeFileSync(join(root,'visual-done'),'1');
 });
});
