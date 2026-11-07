import { isTauri, invoke } from '@tauri-apps/api/core';
import { getVersion } from '@tauri-apps/api/app';
import { listen } from '@tauri-apps/api/event';
import { Menu } from '@tauri-apps/api/menu';
import { hasUnsavedWork } from '../../app/unsavedWork';
import { tr } from '../../i18n/tr';

let nativeMenu: Promise<Menu> | undefined;
// quit=false hides the window (Cmd+W / red button); quit=true exits (Cmd+Q).
// Unsaved form text or an in-flight form operation blocks both; read-only panels do not.
export async function requestClose(quit: boolean) {
 if (hasUnsavedWork()) {
  window.dispatchEvent(new Event('hush-close-blocked'));
  return;
 }
 await invoke('app_finish_close', { quit });
}
export async function installLifecycle() {
 if (!isTauri()) return () => {};
 const unlisten = await listen<boolean>('hush-close-request', ({ payload }) => requestClose(payload));
 const m = tr.nativeMenu;
 const version = await getVersion().catch(() => '');
 nativeMenu ??= Menu.new({ items: [
  { text: tr.name, items: [
   { text: m.about, item: { About: { name: tr.name, version, credits: m.credits } } },
   { text: m.hide, item: 'Hide' },
   // Custom items: the predefined Quit/CloseWindow call NSApp terminate:/performClose: directly.
   { id: 'hush-quit', text: m.quit, accelerator: 'CmdOrCtrl+Q', action: () => void requestClose(true) },
  ] },
  { text: m.edit, items: [{ text: m.undo, item: 'Undo' }, { text: m.redo, item: 'Redo' }, { text: m.cut, item: 'Cut' }, { text: m.copy, item: 'Copy' }, { text: m.paste, item: 'Paste' }, { text: m.selectAll, item: 'SelectAll' }] },
  { text: m.window, items: [{ text: m.minimize, item: 'Minimize' }, { id: 'hush-close', text: m.close, accelerator: 'CmdOrCtrl+W', action: () => void requestClose(false) }] },
 ] });
 await (await nativeMenu).setAsAppMenu();
 return unlisten;
}
