import {browser,expect} from '@wdio/globals';
import {DatabaseSync} from 'node:sqlite';
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {randomUUID} from 'node:crypto';
import {execFileSync} from 'node:child_process';
const root=process.env.HUSH_TEST_DATA_DIR;
async function ipc(command,request={}){const r=await browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request);expect(r.ok).toBe(true);return r.value}
function seed(){
 const generation=JSON.parse(readFileSync(join(root,'active.json'))).generationId;
 const db=new DatabaseSync(join(root,'storage',generation,'library.sqlite3'));
 expect(db.prepare('SELECT count(*) n FROM library_items').get().n).toBe(0);
 db.exec('PRAGMA foreign_keys=ON; BEGIN');
 const item=db.prepare('INSERT INTO library_items(id,kind,title,creator,status,notes,search_text,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?)');
 const session=db.prepare('INSERT INTO sessions VALUES(?,?,?,?,?,?,?,?)');
 const now=new Date().toISOString();
 for(let i=0;i<5000;i++){
  const id=randomUUID(),kind=['book','game','album'][i%3],title=`Sentetik ${String(i).padStart(5,'0')} — Yağmurlu Şehir Işıkları`;
  item.run(id,kind,title,'Sentetik üretici','planned','Yalnız performans fixture verisi',`${title.toLocaleLowerCase('tr')} sentetik üretici`,now,now);
  db.prepare(`INSERT INTO ${kind}_details(item_id) VALUES(?)`).run(id);
  for(let j=0;j<2;j++)session.run(randomUUID(),id,now,30,'Sentetik seans',1,now,now);
 }
 db.exec('COMMIT');
 expect(db.prepare('PRAGMA integrity_check').get().integrity_check).toBe('ok');
 expect(db.prepare('PRAGMA foreign_key_check').all()).toEqual([]);db.close();
}
describe('P7 release-profile native performance',()=>{
 it('measures 5000 items and 10000 sessions through real IPC and UI',async()=>{
  await expect(browser.$('[role=status]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  if(!process.env.HUSH_REOPEN){seed();return}
  expect((await ipc('library_list',{sort:'title_asc',offset:0,limit:60})).total).toBe(5000);
  if(await browser.$('button=Odaya gir').isExisting())await browser.$('button=Odaya gir').click();
  const measured=await browser.tauri.execute(async({core})=>{
   const search=[],panel=[];
   for(let i=0;i<100;i++){const start=performance.now();const r=await core.invoke('search_local',{request:{query:i%2?'sentetik':`sentetik ${String(i*43).padStart(5,'0')}`,limit:30}});if(!r.ok)throw Error(r.error.code);search.push(performance.now()-start)}
   return {search,webviewElapsedMs:performance.now()};
  });
  const panels=[],paint=[];
  for(let i=0;i<20;i++){
   const t=performance.now();
   paint.push(await browser.executeAsync(done=>{
    const start=performance.now();const observer=new MutationObserver(()=>{
     if(document.querySelector('[role=dialog] [data-library-view] strong')){observer.disconnect();requestAnimationFrame(()=>requestAnimationFrame(()=>done(performance.now()-start)))}
    });observer.observe(document.body,{subtree:true,childList:true});document.querySelector('[data-hotspot=bookshelf]').click();
   }));
   panels.push(performance.now()-t);await browser.keys('Escape');
  }
  const frames=await browser.executeAsync(done=>{const times=[];let last=performance.now();function frame(now){times.push(now-last);last=now;if(times.length<120)requestAnimationFrame(frame);else done(times)}requestAnimationFrame(frame)});
  await browser.saveScreenshot('artifacts/p7-performance-5000.png');
  await browser.minimizeWindow();
  await browser.waitUntil(()=>browser.execute(()=>document.querySelector('[data-testid=room-scene]').getAnimations({subtree:true}).every(a=>a.playState==='paused')));
  const processes=execFileSync('ps',['-axo','pid,pcpu,rss,comm'],{encoding:'utf8'}).split('\n').filter(l=>l.includes('/target/native/release/hush'));
  const p95=a=>[...a].sort((a,b)=>a-b)[Math.ceil(a.length*.95)-1];
  const evidence={profile:'release with e2e instrumentation; not the production bundle',items:5000,sessions:10000,searchP95Ms:p95(measured.search),searchSamples:measured.search,panelFirstPaintP95Ms:p95(paint),panelFirstPaintSamples:paint,panelAutomationP95Ms:p95(panels),panelSamples:panels,frameP95Ms:p95(frames),frames,processes,minimizedAnimationsPaused:true,webviewElapsedMs:measured.webviewElapsedMs};
  writeFileSync(join(root,'performance.json'),JSON.stringify(evidence,null,2));
  expect(evidence.searchP95Ms).toBeLessThanOrEqual(200);
 });
});
