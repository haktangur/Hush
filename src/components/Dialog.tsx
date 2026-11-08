import { useEffect, useRef, useState, type ComponentProps } from 'react';
import { tr } from '../i18n/tr';
import * as Radix from '@radix-ui/react-dialog';
export { Root, Portal, Overlay, Title, Description, Close, Trigger } from '@radix-ui/react-dialog';

let lastRoomOpener: HTMLElement | null = null;

// Panels are opened by scene/menu buttons, not Radix Trigger instances.
export function Content(props: ComponentProps<typeof Radix.Content>) {
 const node = useRef<HTMLDivElement>(null);
 const [closeNotice,setCloseNotice] = useState(false);
 // Only the topmost dialog announces a blocked Cmd+Q/Cmd+W, so VoiceOver reads it once.
 useEffect(()=>{const show=()=>{const all=document.querySelectorAll('[role="dialog"]');if(all[all.length-1]===node.current)setCloseNotice(true)};window.addEventListener('hush-close-blocked',show);return()=>window.removeEventListener('hush-close-blocked',show)},[]);
 // Entry motion (docs/03: ~200 ms fade + ≤12 px slide). Web Animations instead of CSS keyframes so that a hidden or
 // occluded window (WebKit suspends its timeline) never leaves a panel frozen at opacity 0; reduced motion skips it.
 useEffect(()=>{
  const el=node.current;if(!el||typeof el.animate!=='function')return;
  const motion=document.documentElement.dataset.motion;
  const reduced=motion==='reduced'||(motion!=='full'&&window.matchMedia?.('(prefers-reduced-motion: reduce)').matches);
  if(reduced||document.visibilityState!=='visible'){el.dataset.entry='instant';return}
  el.dataset.entry='animated';
  const animation=el.animate([{opacity:0,translate:'0 8px'},{opacity:1,translate:'0 0'}],{duration:200,easing:'ease-out'});
  const finish=()=>{if(document.visibilityState!=='visible')animation.finish()};
  document.addEventListener('visibilitychange',finish);
  return()=>{document.removeEventListener('visibilitychange',finish);animation.cancel()};
 },[]);
 const [opener] = useState(() => {
  const active = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  if (active && !active.closest('[role="dialog"]')) lastRoomOpener = active;
  return active;
 });
 return <Radix.Content {...props} ref={node} onKeyDown={event => {
  props.onKeyDown?.(event);
  if (event.defaultPrevented || !event.metaKey || event.key.toLowerCase() !== 's') return;
  // Only the topmost dialog saves; a form submits through its own handler, panels may mark a save button.
  const all = document.querySelectorAll('[role="dialog"]');
  if (all[all.length - 1] !== node.current) return;
  event.preventDefault();
  // Only real edit forms (they have an action footer); never the catalog search form inside a panel.
  const form = node.current?.querySelector<HTMLFormElement>('form:has(> footer)');
  if (form) form.requestSubmit(); else node.current?.querySelector<HTMLButtonElement>('[data-save]:not(:disabled)')?.click();
 }} onCloseAutoFocus={event => {
  props.onCloseAutoFocus?.(event);
  const target = opener?.isConnected ? opener : lastRoomOpener;
  if (!event.defaultPrevented && target?.isConnected) {
   event.preventDefault();
   target.focus();
  }
 }}>{closeNotice&&<p role="alert">{tr.closePanelFirst}</p>}{props.children}</Radix.Content>;
}
