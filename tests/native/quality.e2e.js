import {browser,expect} from '@wdio/globals';
import {readFileSync,writeFileSync} from 'node:fs';
import {join} from 'node:path';
const root=process.env.HUSH_TEST_DATA_DIR;
async function ipc(command,request={}){const r=await browser.tauri.execute(({core},c,r)=>core.invoke(c,{request:r}),command,request);expect(r.ok).toBe(true);return r.value}
describe('P7 native quality and scale',()=>{
 it('preserves forms across actual native window close and keyboard save',async()=>{
  await expect(browser.$('[role=status]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  if(!process.env.HUSH_REOPEN){
   await browser.$('button=Odaya gir').click();
   await browser.waitUntil(()=>browser.execute(()=>!document.querySelector('.onboarding')));
   await browser.keys(['Meta','n']);await browser.keys('NULL');
   await browser.$('input[name=title]').setValue('Native kapanışta korunacak metin');
   // Real Tauri close event, not a synthetic Escape or mocked IPC.
   await browser.tauri.execute(({core})=>core.invoke('plugin:window|close',{label:'main'}));
   await expect(browser.$('p=Kapatmadan önce açık paneldeki işlemi tamamla. Değişikliklerini kaydet veya vazgeç; sonra yeniden kapat.')).toBeDisplayed();
   await expect(browser.$('input[name=title]')).toHaveValue('Native kapanışta korunacak metin');
   await browser.keys(['Meta','s']);await browser.keys('NULL');
   await browser.waitUntil(async()=>await browser.$$('input[name=title]').length===0);
   // A clean read-only panel must not block: the same native close now hides the window (Dock reopen restores it).
   const visible=()=>browser.tauri.execute(({core})=>core.invoke('plugin:window|is_visible',{label:'main'}));
   await browser.tauri.execute(({core})=>core.invoke('plugin:window|close',{label:'main'}));
   await browser.waitUntil(async()=>(await visible())===false,{timeout:5000});
   await browser.tauri.execute(({core})=>core.invoke('plugin:window|show',{label:'main'}));
   await browser.waitUntil(async()=>(await visible())===true,{timeout:5000});
   await expect(browser.$('[role=dialog]')).toBeDisplayed();
   await browser.keys('Escape');
   const entries=await ipc('library_list',{sort:'title_asc',offset:0,limit:60});expect(entries.total).toBe(1);
   // Keep this first process for lifecycle evidence; scale fixture gets its own clean root via a separate run.
   writeFileSync(join(root,'quality-lifecycle.json'),JSON.stringify({nativeClosePreserved:true,cmdSSaved:true,cleanCloseHidesWindow:true,id:entries.items[0].id}));
  }else{
   expect((await ipc('library_list',{sort:'title_asc',offset:0,limit:60})).total).toBe(1);
   expect((await ipc('app_bootstrap')).settings.value.onboardingCompleted).toBe(true);
  }
 });
});
