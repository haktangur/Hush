import { useState, useEffect } from 'react';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { desktop } from '../infra/tauri/client';
import type { ItemKind, SlotId } from '../domain/types';
import { tr } from '../i18n/tr';
import { Library } from '../features/library/Library';
import { Room } from '../features/room/Room';
import { SlotPanel } from '../features/room/SlotPanel';
import { SettingsPanel } from '../features/settings/SettingsPanel';
import { errorText } from '../features/library/ItemForm';
type Panel=ItemKind|'trash'|'settings'|SlotId|null;
export function App(){
 const state=useQuery({queryKey:['bootstrap'],queryFn:desktop.bootstrap});const [panel,setPanel]=useState<Panel>(null);const [error,setError]=useState('');const cache=useQueryClient();
 useEffect(()=>{const key=(e:KeyboardEvent)=>{if(!e.metaKey||document.querySelector('[role="dialog"]'))return;const map:Record<string,Panel>={'1':null,'2':'game','3':'album','4':'book',',':'settings'};if(e.key in map){e.preventDefault();setPanel(map[e.key])}};window.addEventListener('keydown',key);return()=>window.removeEventListener('keydown',key)},[]);
 const refresh=()=>void cache.invalidateQueries({queryKey:['bootstrap']});
 async function action(action:string){const map:Record<string,Panel>={open_books:'book',open_games:'game',open_albums:'album',open_poster_left:'poster_left',open_poster_right:'poster_right'};if(action in map)setPanel(map[action]);else if(action==='toggle_lamp'&&state.data){try{const s=state.data.settings;await desktop.saveSettings({...s.value,room:{...s.value.room,lampOn:!s.value.room.lampOn}},s.revision);refresh()}catch(e){setError(errorText(e))}}else setPanel('settings')}
 return <main className="app" data-preset={state.data?.settings.value.room.preset}><header className="appHeader"><strong>{tr.name}<small>{tr.phaseDetail}</small></strong><div><button disabled={!state.data} onClick={()=>setPanel('book')}>{tr.add}</button><button disabled={!state.data} onClick={()=>setPanel('settings')}>{tr.settings}</button></div></header>{state.data?<><span className="srOnly" role="status">{tr.ready}</span><Room slots={state.data.slots} settings={state.data.settings.value} paused={panel!==null} onAction={a=>void action(a)}/></>:<section className="intro"><h1>{tr.welcome}</h1><p role="status">{state.isPending?tr.loading:tr.error}</p>{state.isError&&<button onClick={()=>void state.refetch()}>{tr.retry}</button>}</section>}{error&&<p role="alert">{error}</p>}<nav aria-label={tr.name}>{([[null,tr.room],['game',tr.games],['album',tr.albums],['book',tr.books],['trash',tr.trash]]as const).map(([kind,label])=><button key={label} disabled={!state.data} aria-current={panel===kind?'page':undefined} onClick={()=>setPanel(kind)}>{label}</button>)}</nav>{panel&&state.data&&((panel==='poster_left'||panel==='poster_right'||panel==='record_stand')?<SlotPanel slot={panel} onSaved={refresh} onClose={()=>setPanel(null)}/>:panel==='settings'?<SettingsPanel snapshot={state.data.settings} onSaved={refresh} onClose={()=>setPanel(null)}/>:<Library kind={panel==='trash'?undefined:panel} trashed={panel==='trash'} onClose={()=>setPanel(null)}/>)}</main>;
}
