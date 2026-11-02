import { existsSync, renameSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { createHash } from 'node:crypto';
import { browser, expect } from '@wdio/globals';
async function fit(){await browser.waitUntil(()=>browser.execute(()=>{const scene=document.querySelector('[data-testid="room-scene"]');const a=scene.getBoundingClientRect(),b=scene.parentElement.getBoundingClientRect();return a.width<=b.width+1&&a.height<=b.height+1&&a.top>=b.top-1}),{timeout:5000})}
async function visibleCover(){
 await expect(browser.$('img[alt="Eser kapağı"]')).toBeDisplayed();
 await browser.waitUntil(()=>browser.execute(()=>{const image=document.querySelector('img[alt="Eser kapağı"]');return image?.complete&&image.naturalWidth>0&&image.naturalHeight>0}),{timeout:10000,timeoutMsg:'Local cover must decode in the real WebView'});
}
const digest=path=>createHash('sha256').update(readFileSync(path)).digest('hex');
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
   if(process.env.HUSH_NATIVE_COVER){
   await browser.$('button=Kapak seç').click();
   console.log(`Native chooser: select ${process.env.HUSH_TEST_COVER_SOURCE}`);
   await browser.$('button=Kapağı kaldır').waitForDisplayed({timeout:300000});
   }
   await browser.$('button=Kaydet').click();await expect(browser.$('h2=Native sentetik kitap')).toBeDisplayed();
   if(process.env.HUSH_NATIVE_COVER){
   await visibleCover();
   renameSync(process.env.HUSH_TEST_COVER_SOURCE,process.env.HUSH_TEST_COVER_SOURCE+'.moved');
   expect(existsSync(process.env.HUSH_TEST_COVER_SOURCE)).toBe(false);
   }
   await browser.saveScreenshot('artifacts/native-p1.png');
  }
  const result=await browser.tauri.execute(({core})=>core.invoke('library_list',{request:{kind:'book',sort:'updated_desc',offset:0,limit:60}}));
  expect(result.ok).toBe(true);expect(result.value.items).toHaveLength(1);expect(result.value.items[0].details.currentPage).toBe(37);expect(result.value.items[0].notes).toBe('Yeniden açılışta korunmalı.');
  if(process.env.HUSH_NATIVE_COVER){
  const item=result.value.items[0];
  expect(item.details.totalPages).toBe(200);
  expect(item.coverAssetId).toMatch(/^[0-9a-f-]{36}$/);
  const resolved=await browser.tauri.execute(({core},id)=>core.invoke('asset_resolve',{request:{assetIds:[id]}}),item.coverAssetId);
  expect(resolved.ok).toBe(true);
  const asset=resolved.value[0];
  const evidence={itemId:item.id,assetId:item.coverAssetId,imageHash:digest(asset.path),thumbnailHash:digest(asset.thumbnailPath)};
  const evidencePath=join(process.env.HUSH_TEST_DATA_DIR,'cover-evidence.json');
  expect(existsSync(process.env.HUSH_TEST_COVER_SOURCE)).toBe(false);
  expect(existsSync(process.env.HUSH_TEST_COVER_SOURCE+'.moved')).toBe(true);
  if(process.env.HUSH_REOPEN){
   expect(evidence).toEqual(JSON.parse(readFileSync(evidencePath,'utf8')));
   await browser.$('nav').$('button=Kitaplar').click();
   await browser.$('strong=Native sentetik kitap').click();
   await expect(browser.$('h2=Native sentetik kitap')).toBeDisplayed();
   await visibleCover();
   await browser.saveScreenshot('artifacts/native-p1-reopened.png');
   await browser.$('button=Kapat').click();
  }else{writeFileSync(evidencePath,JSON.stringify(evidence,null,2));}
  }

 });
 it('persists an autosaved journal draft across process restart',async()=>{
  if(!process.env.HUSH_REOPEN){
   await browser.$('button=Kapat').click();
   await browser.$('nav').$('button=Günlük').click();
   await browser.$('[role="dialog"]').$('button=Ekle').click();
   await browser.$('[role="dialog"] form input[maxlength="200"]').setValue('Native günlük taslağı');
   await browser.$('[role="dialog"] form textarea').setValue('Taslak yeniden açılışta korunmalı.');
   await expect(browser.$('p=Taslak bu Mac’e kaydedildi.')).toBeDisplayed();
  }else{
   const result=await browser.tauri.execute(({core})=>core.invoke('draft_get',{request:{}}));
   expect(result.ok).toBe(true);expect(result.value.entry.body).toBe('Taslak yeniden açılışta korunmalı.');
   await browser.$('nav').$('button=Günlük').click();
   await browser.$('button=Taslağı sürdür').click();
   await browser.$('button=Kaydet').click();
   await expect(browser.$('h2=Native günlük taslağı')).toBeDisplayed();
   const draft=await browser.tauri.execute(({core})=>core.invoke('draft_get',{request:{}}));expect(draft.value).toBe(null);
  }
 });
});
