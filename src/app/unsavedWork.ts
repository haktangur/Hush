import { useEffect } from 'react';

// Forms register here while they hold unsaved text or an in-flight operation.
// Native Cmd+Q / Cmd+W consult this instead of "is any panel open", so a read-only
// panel never blocks quitting while a dirty form always does.
const active = new Set<symbol>();

export function hasUnsavedWork() {
 return active.size > 0;
}

export function useUnsavedWork(when: boolean) {
 useEffect(() => {
  if (!when) return;
  const token = Symbol('unsaved');
  active.add(token);
  return () => { active.delete(token); };
 }, [when]);
}
