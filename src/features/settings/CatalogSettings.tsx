import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { catalog } from '../../infra/tauri/client';
import { errorText } from '../library/ItemForm';
import { tr } from '../../i18n/tr';
import styles from '../library/Library.module.css';

export function CatalogSettings({onBusy}:{onBusy:(busy:boolean)=>void}) {
 const status=useQuery({queryKey:['provider-status'],queryFn:catalog.status,retry:false});
 const [clientId,setClientId]=useState('');
 const [secret,setSecret]=useState('');
 const [busy,setBusy]=useState(false);
 const [error,setError]=useState('');
 async function connect() {
  setBusy(true);onBusy(true);setError('');
  try {await catalog.configure(clientId,secret);setClientId('');setSecret('');await status.refetch()}
  catch(e){setError(errorText(e))}
  finally{setSecret('');setBusy(false);onBusy(false)}
 }
 async function disconnect() {
  setBusy(true);onBusy(true);setError('');
  try{await catalog.disconnect();setClientId('');setSecret('');await status.refetch()}
  catch(e){setError(errorText(e))}
  finally{setSecret('');setBusy(false);onBusy(false)}
 }
 return <section className={styles.activity} aria-label={tr.igdb}>
  <h3>{tr.igdb}</h3><p>{tr.keychainHint}</p>
  <p role="status">{status.isPending?tr.load:status.data?.state==='ready'?tr.credentialsStored:tr.notConnected}</p>
  <form onSubmit={e=>{e.preventDefault();void connect()}}>
   <div className={styles.formGrid}>
    <label className={styles.field}>{tr.clientId}<input autoComplete="off" maxLength={300} required value={clientId} disabled={busy} onChange={e=>setClientId(e.target.value)}/></label>
    <label className={styles.field}>{tr.clientSecret}<input type="password" autoComplete="new-password" maxLength={1000} required value={secret} disabled={busy} onChange={e=>setSecret(e.target.value)}/></label>
   </div>
   <div className={styles.actions}><button disabled={busy}>{busy?tr.load:tr.connect}</button><button type="button" disabled={busy||status.data?.state!=='ready'} onClick={()=>void disconnect()}>{tr.disconnect}</button></div>
  </form>
  {(error||status.error)&&<p role="alert">{error||errorText(status.error)}</p>}
 </section>;
}
