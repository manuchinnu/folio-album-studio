import type {Lab} from './schema.js';
import {spineMm} from './geometry.js';
export function coverGeometry(lab:Lab,units:number){
 const profile=lab.coverProfile;if(profile===null)return null;
 const physicalPages=units*(lab.exportUnit==='spread'?2:1);
 if(physicalPages%2)throw new Error('Cover requires an even physical page count');
 if(profile.spineModel==='formula'){
  const front=lab.trim.w/(lab.exportUnit==='spread'?2:1),spine=spineMm(lab,physicalPages),w=profile.wrap;
  return {widthMm:2*front+spine+w.left+w.right,heightMm:lab.trim.h+w.top+w.bottom,spineMm:spine,bleed:profile.bleed};
 }
 const rows=profile.entries.filter(e=>e.sheets===physicalPages/2);
 if(rows.length!==1)throw new Error('Missing or duplicate lab cover table entry for sheet count');
 const row=rows[0]!,w=row.wrap;
 return {widthMm:row.frontWidthMm+row.backWidthMm+row.spineMm+w.left+w.right,heightMm:row.heightMm+w.top+w.bottom,spineMm:row.spineMm,bleed:row.bleed};
}
