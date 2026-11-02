import { useEffect, useRef, useState, type CSSProperties } from 'react';
import layout from '../../../contracts/room-layout.json';
import room from '../../assets/room/room.svg';
import { useQuery } from '@tanstack/react-query';
import { assets, library } from '../../infra/tauri/client';
import type { Settings, RoomSlots, SlotSource } from '../../domain/types';
import styles from './Room.module.css';
import { tr } from '../../i18n/tr';
function SlotImage({source,position}:{source:SlotSource;position:CSSProperties}){const item=useQuery({queryKey:['item',source.type==='item'?source.itemId:null],queryFn:()=>library.get(source.type==='item'?source.itemId:''),enabled:source.type==='item'});const id=source.type==='image'?source.assetId:item.data?.coverAssetId;const asset=useQuery({queryKey:['asset',id],queryFn:()=>assets.resolve(id!),enabled:!!id});return asset.data?<img src={asset.data.image} style={position} className={styles.slotImage} alt=""/>:null}
export function Room({settings,slots,paused,onAction}:{settings:Settings;slots:RoomSlots;paused:boolean;onAction:(action:string)=>void}){
 const host=useRef<HTMLDivElement>(null);const [scale,setScale]=useState(.65);const [hidden,setHidden]=useState(document.hidden);const [focused,setFocused]=useState(true);
 useEffect(()=>{const el=host.current;if(!el)return;const observer=new ResizeObserver(([entry])=>setScale(Math.min(entry.contentRect.width/1600,entry.contentRect.height/1000)));observer.observe(el);const visibility=()=>setHidden(document.hidden);const focus=()=>setFocused(true);const blur=()=>setFocused(false);window.addEventListener('focus',focus);window.addEventListener('blur',blur);document.addEventListener('visibilitychange',visibility);return()=>{observer.disconnect();window.removeEventListener('focus',focus);window.removeEventListener('blur',blur);document.removeEventListener('visibilitychange',visibility)}},[]);
 return <div ref={host} className={styles.viewport} role="region" aria-label={tr.roomLabel} data-motion={settings.room.motion} data-paused={paused||hidden||!focused} data-preset={settings.room.preset}><div data-testid="room-scene" className={styles.scene} style={{transform:`translate(-50%,-50%) scale(${scale})`,'--label-size':`${Math.max(20,12/scale)}px`,'--intensity':settings.room.neonIntensity/100} as CSSProperties}>
 <img className={styles.art} src={room} alt="" aria-hidden="true"/>{settings.room.rainOn&&<div className={styles.rain} aria-hidden="true"/>}<div className={styles.recordPlane} aria-hidden="true"><div className={styles.record}/></div><div className={styles.neon} aria-hidden="true"/>{settings.room.lampOn&&<div className={styles.lamp} aria-hidden="true"/>}
 <SlotImage source={slots.poster_left} position={{left:1185,top:135,width:140,height:200}}/><SlotImage source={slots.poster_right} position={{left:1365,top:195,width:140,height:200}}/><SlotImage source={slots.record_stand} position={{left:1370,top:506,width:88,height:88,transform:'rotate(-4deg)'}}/>
 {layout.hotspots.map(h=><button key={h.id} data-hotspot={h.id} aria-keyshortcuts={h.id==='lamp'?'Meta+l':undefined} aria-pressed={h.id==='lamp'?settings.room.lampOn:undefined} className={styles.hotspot} aria-label={tr.hotspots[h.id as keyof typeof tr.hotspots]} onClick={()=>onAction(h.action)} style={{left:h.x,top:h.y,width:Math.max(h.width,44/scale),height:Math.max(h.height,44/scale)}}><span className={settings.room.labelsVisible?styles.visible:''}>{tr.hotspots[h.id as keyof typeof tr.hotspots]}</span></button>)}
 </div><p className={styles.hint}>{tr.roomHint}</p></div>
}
