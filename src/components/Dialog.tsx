import { useState, type ComponentProps } from 'react';
import * as Radix from '@radix-ui/react-dialog';
export { Root, Portal, Overlay, Title, Description, Close, Trigger } from '@radix-ui/react-dialog';

let lastRoomOpener: HTMLElement | null = null;

// Panels are opened by scene/menu buttons, not Radix Trigger instances.
export function Content(props: ComponentProps<typeof Radix.Content>) {
 const [opener] = useState(() => {
  const active = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  if (active && !active.closest('[role="dialog"]')) lastRoomOpener = active;
  return active;
 });
 return <Radix.Content {...props} onCloseAutoFocus={event => {
  props.onCloseAutoFocus?.(event);
  const target = opener?.isConnected ? opener : lastRoomOpener;
  if (!event.defaultPrevented && target?.isConnected) {
   event.preventDefault();
   target.focus();
  }
 }} />;
}
