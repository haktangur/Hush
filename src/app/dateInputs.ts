// WebKit paints today's date into an empty <input type="date">, which reads as a saved value.
// Mark empty date fields so CSS can hide that placeholder until the field is focused.
function sync(el: HTMLInputElement) {
 el.toggleAttribute('data-empty', el.value === '');
}

function scan() {
 document.querySelectorAll<HTMLInputElement>('input[type=date]').forEach(sync);
}

export function installDateEmptyState() {
 const onEdit = (event: Event) => {
  const target = event.target;
  if (target instanceof HTMLInputElement && target.type === 'date') sync(target);
 };
 document.addEventListener('input', onEdit, true);
 document.addEventListener('change', onEdit, true);
 // Forms set default values right after mounting; rescan on the next frame as well.
 new MutationObserver(() => { scan(); requestAnimationFrame(scan); }).observe(document.body, { childList: true, subtree: true });
 scan();
}
