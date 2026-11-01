import { browser, expect } from '@wdio/globals';
async function fit(){await browser.waitUntil(()=>browser.execute(()=>{const scene=document.querySelector('[data-testid="room-scene"]');const a=scene.getBoundingClientRect(),b=scene.parentElement.getBoundingClientRect();return a.width<=b.width+1&&a.height<=b.height+1&&a.top>=b.top-1}),{timeout:5000})}
describe('Hush native persistence',()=>{
 it('saves or reopens a book through real IPC',async()=>{
  await expect(browser.$('[role="status"]')).toHaveText('Yerel masaüstü bağlantısı hazır.');
  if(!process.env.HUSH_REOPEN){
   await browser.setWindowSize(1440,900);await fit();await browser.saveScreenshot('artifacts/room-1440.png');
   await browser.setWindowSize(1100,720);await fit();await browser.saveScreenshot('artifacts/room-1100.png');
   await browser.setWindowSize(1440,900);
   await browser.$('nav').$('button=Kitaplar').click();await browser.$('[role="dialog"]').$('button=Ekle').click();
   await browser.$('input[name="title"]').setValue('Native sentetik kitap');
   await browser.$('input[name="totalPages"]').setValue('200');await browser.$('input[name="currentPage"]').setValue('37');
   await browser.$('textarea[name="notes"]').setValue('Yeniden açılışta korunmalı.');
   await browser.$('button=Kaydet').click();await expect(browser.$('h2=Native sentetik kitap')).toBeDisplayed();
   await browser.saveScreenshot('artifacts/native-p1.png');
  }
  const result=await browser.tauri.execute(({core})=>core.invoke('library_list',{request:{kind:'book',sort:'updated_desc',offset:0,limit:60}}));
  expect(result.ok).toBe(true);expect(result.value.items).toHaveLength(1);expect(result.value.items[0].details.currentPage).toBe(37);expect(result.value.items[0].notes).toBe('Yeniden açılışta korunmalı.');
 });
});
