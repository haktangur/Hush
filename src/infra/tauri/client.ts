import { invoke } from '@tauri-apps/api/core';
import type { ApiResult } from '../../domain/types';
export interface Bootstrap {version:string; ready:boolean}
async function call<T>(command:string, request:object):Promise<T>{
 const result=await invoke<ApiResult<T>>(command,{request});
 if(!result.ok) throw new Error(result.error.code);
 return result.value;
}
export const desktop = {bootstrap:()=>call<Bootstrap>('app_bootstrap',{})};
