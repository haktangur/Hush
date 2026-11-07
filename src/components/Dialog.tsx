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
 const [opener] = useState(() => {
  const active = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  if (active && !active.closest('[role="dialog"]')) lastRoomOpener = active;
  return active;
 });
 return <Radix.Content {...props} ref={node} onCloseAutoFocus={event => {
  props.onCloseAutoFocus?.(event);
  const target = opener?.isConnected ? opener : lastRoomOpener;
  if (!event.defaultPrevented && target?.isConnected) {
   event.preventDefault();
   target.focus();
  }
 }}>{closeNotice&&<p role="alert">{tr.closePanelFirst}</p>}{props.children}</Radix.Content>;
}
