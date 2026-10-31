import { useQuery } from '@tanstack/react-query';
import { desktop } from '../infra/tauri/client';
import { tr } from '../i18n/tr';
export function App(){
 const state=useQuery({queryKey:['bootstrap'],queryFn:desktop.bootstrap});
 return <main><header><strong>{tr.name}</strong><span>{tr.phaseDetail}</span></header><section><p className="eyebrow">{tr.phase}</p><h1>{tr.welcome}</h1><p>{tr.subtitle}</p><div role="status">{state.isPending?tr.loading:state.isError?tr.error:tr.ready}</div>{state.isError&&<button onClick={()=>void state.refetch()}>{tr.retry}</button>}</section></main>;
}
