import {useState} from 'react';
import {useQuery} from '@tanstack/react-query';
import * as Dialog from '../../components/Dialog';
import type {LibraryItem} from '../../domain/types';
import {searchLocal} from '../../infra/tauri/client';
import {tr} from '../../i18n/tr';
import {errorText} from '../library/ItemForm';
import styles from '../library/Library.module.css';
export function Search({onClose,onItem,onJournal}:{onClose:()=>void;onItem:(item:LibraryItem)=>void;onJournal:(query:string)=>void}){const [query,setQuery]=useState('');const result=useQuery({queryKey:['search',query],queryFn:()=>searchLocal(query),enabled:query.trim().length>0});return <Dialog.Root open onOpenChange={o=>{if(!o)onClose()}}><Dialog.Portal><Dialog.Overlay className={styles.overlay}/><Dialog.Content className={styles.formDialog}><Dialog.Title>{tr.localSearch}</Dialog.Title><Dialog.Description>{tr.localSearchHint}</Dialog.Description><label className={styles.field}>{tr.search}<input value={query} onChange={e=>setQuery(e.target.value)}/></label>{result.isError&&<p role="alert">{errorText(result.error)}</p>}{(['game','album','book']as const).map(kind=><div key={kind}><h3>{kind==='game'?tr.games:kind==='album'?tr.albums:tr.books}</h3>{result.data?.items.filter(i=>i.kind===kind).map(i=><button key={i.id} onClick={()=>onItem(i)}>{i.title} · {i.creator}</button>)}</div>)}<h3>{tr.journal}</h3>{result.data?.journals.map(e=><button key={e.id} onClick={()=>onJournal(e.title)}>{e.entryDate} · {e.title}</button>)}<footer className={styles.actions}><button onClick={onClose}>{tr.close}</button></footer></Dialog.Content></Dialog.Portal></Dialog.Root>}
