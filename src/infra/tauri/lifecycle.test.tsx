import { beforeAll, beforeEach, expect, test, vi } from 'vitest';
import { act, render } from '@testing-library/react';
const mocks = vi.hoisted(() => ({
 invoke: vi.fn().mockResolvedValue(undefined),
 listen: vi.fn().mockResolvedValue(() => {}),
 menu: vi.fn().mockResolvedValue({ setAsAppMenu: vi.fn().mockResolvedValue(undefined) }),
}));
vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => true, invoke: mocks.invoke }));
vi.mock('@tauri-apps/api/app', () => ({ getVersion: () => Promise.resolve('0.1.0') }));
vi.mock('@tauri-apps/api/event', () => ({ listen: mocks.listen }));
vi.mock('@tauri-apps/api/menu', () => ({ Menu: { new: mocks.menu } }));
import { installLifecycle } from './lifecycle';
import { useUnsavedWork } from '../../app/unsavedWork';
type Entry = { id?: string; item?: unknown; action?: (id: string) => void; items?: Entry[] };
let entries: Entry[] = [];
let emitClose: (quit: boolean) => Promise<void>;
beforeAll(async () => {
 await installLifecycle();
 const menus = mocks.menu.mock.calls[0][0] as { items: Entry[] };
 entries = menus.items.flatMap(m => m.items ?? []);
 const handler = mocks.listen.mock.calls[0][1] as (e: { payload: boolean }) => Promise<void>;
 emitClose = quit => handler({ payload: quit });
});
beforeEach(() => mocks.invoke.mockClear());
const action = (id: string) => entries.find(e => e.id === id)!.action!(id);
function Form({ dirty }: { dirty: boolean }) { useUnsavedWork(dirty); return <input defaultValue="unsaved text" />; }

test('menu uses custom quit/close items instead of predefined terminate actions', () => {
 expect(entries.some(e => e.item === 'Quit' || e.item === 'CloseWindow')).toBe(false);
 expect(entries.map(e => e.id)).toEqual(expect.arrayContaining(['hush-quit', 'hush-close']));
});

test('Cmd+Q, Cmd+W and the red close button keep a dirty form open', async () => {
 const view = render(<Form dirty />);
 const blocked = vi.fn(); window.addEventListener('hush-close-blocked', blocked);
 action('hush-quit'); action('hush-close');
 await act(() => emitClose(true)); await act(() => emitClose(false));
 expect(blocked).toHaveBeenCalledTimes(4);
 expect(mocks.invoke).not.toHaveBeenCalled();
 expect(view.container.querySelector('input')!.value).toBe('unsaved text');
 window.removeEventListener('hush-close-blocked', blocked);
 view.unmount();
});

test('a clean open panel does not block quitting; finishing a form releases the guard', async () => {
 const view = render(<><div role="dialog"><p>read-only panel</p></div><Form dirty /></>);
 view.rerender(<><div role="dialog"><p>read-only panel</p></div><Form dirty={false} /></>);
 action('hush-close');
 expect(mocks.invoke).toHaveBeenLastCalledWith('app_finish_close', { quit: false });
 action('hush-quit');
 expect(mocks.invoke).toHaveBeenLastCalledWith('app_finish_close', { quit: true });
 view.unmount();
});
