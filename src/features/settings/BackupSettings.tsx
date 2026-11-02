import {useState} from 'react';
import {backup} from '../../infra/tauri/client';
import type {RestorePreview} from '../../domain/types';
import {errorText} from '../library/ItemForm';
import {tr} from '../../i18n/tr';
import styles from '../library/Library.module.css';
export function BackupSettings({disabled,onBusy,onRestored}:{disabled:boolean;onBusy:(value:boolean)=>void;onRestored:()=>void}) {
 const [busy,setBusy]=useState(false);const [preview,setPreview]=useState<RestorePreview>();const [error,setError]=useState('');const [message,setMessage]=useState('');
 async function run(action:()=>Promise<void>){setBusy(true);onBusy(true);setError('');setMessage('');try{await action()}catch(e){setError(errorText(e))}finally{setBusy(false);onBusy(false)}}
 return <section className={styles.activity}><h3>{tr.backup}</h3><p>{tr.backupPrivacy}</p>{disabled&&<p>{tr.backupDirty}</p>}<div className={styles.actions}><button disabled={busy||disabled} onClick={()=>void run(async()=>{if(await backup.export())setMessage(tr.backupSaved)})}>{tr.exportBackup}</button><button disabled={busy||disabled} onClick={()=>void run(async()=>{setPreview((await backup.inspect())??undefined)})}>{tr.inspectBackup}</button></div>{busy&&<p role="status">{tr.backupBusy}</p>}{preview&&<div><p>{tr.backupSummary}: {new Date(preview.createdAt).toLocaleString('tr-TR')} · Hush {preview.appVersion}</p><p>{tr.itemsCount}: {preview.counts.items} · {tr.journal}: {preview.counts.journals} · {tr.assetsCount}: {preview.counts.assets}</p><p>{tr.restoreWarning}</p><div className={styles.actions}><button disabled={busy} onClick={()=>setPreview(undefined)}>{tr.cancel}</button><button disabled={busy||disabled} onClick={()=>void run(async()=>{await backup.restore(preview.inspectionToken);onRestored()})}>{tr.confirmRestore}</button></div></div>}{message&&<p role="status">{message}</p>}{error&&<p role="alert">{error}</p>}</section>
}
