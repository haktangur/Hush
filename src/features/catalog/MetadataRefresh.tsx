import {useQuery} from '@tanstack/react-query';
import {useState} from 'react';
import {catalog,assets} from '../../infra/tauri/client';
import type {LibraryItem,MetadataDiff} from '../../domain/types';
import {tr} from '../../i18n/tr';
import {errorText} from '../library/ItemForm';
function MetadataCover({id,label}:{id:unknown;label:string}) {
 const cover=useQuery({queryKey:['asset',id],queryFn:()=>assets.resolve(String(id)),enabled:typeof id==='string'});
 return <span>{label}{cover.data&&<img src={cover.data.thumbnail} alt={label} style={{width:70,height:90,objectFit:'contain'}}/>}</span>;
}
export function MetadataRefresh({item,onSaved}:{item:LibraryItem;onSaved:(item:LibraryItem)=>void}) {
 const [diff,setDiff]=useState<MetadataDiff>();const [fields,setFields]=useState<string[]>([]);const [busy,setBusy]=useState(false);const [error,setError]=useState('');
 const labels:Record<string,string>={title:tr.title,creator:tr.creator,releaseYear:tr.year,coverAssetId:tr.cover,isbn:tr.isbn,publisher:tr.publisher,language:tr.language,totalPages:tr.totalPages,platform:tr.platform,developer:tr.developer};
 async function preview(){setBusy(true);setError('');try{const d=await catalog.refreshPreview(item.id);setDiff(d);setFields([])}catch(e){setError(errorText(e))}finally{setBusy(false)}}
 async function apply(){if(!diff)return;setBusy(true);setError('');try{onSaved(await catalog.refreshApply(diff.token,fields));setDiff(undefined)}catch(e){setError(errorText(e))}finally{setBusy(false)}}
 return <section><button disabled={busy} onClick={()=>void preview()}>{tr.refreshMetadata}</button>{diff&&<div><p>{tr.refreshHint}</p>{diff.fields.length===0&&<p>{tr.noMetadataChanges}</p>}{diff.fields.map(f=><label key={f.field} style={{display:'block'}}><input type="checkbox" disabled={busy} checked={fields.includes(f.field)} onChange={e=>setFields(e.target.checked?[...fields,f.field]:fields.filter(x=>x!==f.field))}/>{labels[f.field]}: {f.field==='coverAssetId'?<><MetadataCover id={f.before} label={tr.metadataBefore}/><MetadataCover id={f.after} label={tr.metadataAfter}/></>:`${String(f.before??'—')} → ${String(f.after??'—')}`}</label>)}{diff.warnings.length>0&&<p>{tr.coverUnavailable}</p>}<button disabled={busy||fields.length===0} onClick={()=>void apply()}>{tr.applyMetadata}</button><button disabled={busy} onClick={()=>setDiff(undefined)}>{tr.cancel}</button></div>}{error&&<p role="alert">{error}</p>}</section>;
}
