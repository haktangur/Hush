import { invoke, convertFileSrc } from '@tauri-apps/api/core';
import type { ApiResult, AppError, LibraryItem, SaveItemRequest, Page, ItemKind, Status } from '../../domain/types';
export interface Bootstrap {version:string; ready:boolean; storageEpoch:string;settings:import('../../domain/types').SettingsSnapshot;slots:import('../../domain/types').RoomSlots}
export class DesktopError extends Error {constructor(public readonly detail:AppError){super(detail.code)}}
async function call<T>(command:string, request:object):Promise<T>{
 let result:ApiResult<T>;
 try{result=await invoke<ApiResult<T>>(command,{request});}catch{throw new DesktopError({code:'UNAVAILABLE',message:'UNAVAILABLE',retryable:true})}
 if(!result.ok) throw new DesktopError(result.error);
 return result.value;
}
export interface ListFilter {kind?:ItemKind;query?:string;status?:Status;favorite?:boolean;sort:string;offset:number;limit:number;trashed?:boolean}
export interface LibraryClient {list:(filter:ListFilter)=>Promise<Page<LibraryItem>>;get:(id:string)=>Promise<LibraryItem>;save:(request:SaveItemRequest)=>Promise<LibraryItem>;trash:(item:LibraryItem)=>Promise<void>;restore:(item:LibraryItem)=>Promise<void>}
export const library:LibraryClient={list:filter=>call('library_list',filter),get:id=>call('library_get',{id}),save:request=>call('library_save',request),trash:item=>call('library_trash',{id:item.id,expectedRevision:item.revision}),restore:item=>call('library_restore',{id:item.id,expectedRevision:item.revision})};
export const desktop = {bootstrap:()=>call<Bootstrap>('app_bootstrap',{}),saveSettings:(settings:import('../../domain/types').Settings,expectedRevision:number)=>call<import('../../domain/types').SettingsSnapshot>('settings_save',{settings,expectedRevision}),setSlot:(slotId:string,itemId:string|null,assetId:string|null)=>call<import('../../domain/types').RoomSlots>('room_slot_set',{slotId,itemId,assetId})};

export const assets={choose:(purpose:'cover'|'poster'|'screenshot'='cover')=>call<import('../../domain/types').Asset|null>('asset_choose_import',{purpose}),resolve:async(id:string)=>{const result=await call<{id:string;path:string;thumbnailPath:string}[]>('asset_resolve',{assetIds:[id]});return {image:convertFileSrc(result[0].path),thumbnail:convertFileSrc(result[0].thumbnailPath)}}};
