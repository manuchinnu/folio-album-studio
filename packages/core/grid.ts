import type {Asset,Crop,Lab,Page,Rect} from './schema.js';
import {safeRect,effectiveDpi,contains} from './geometry.js';
export function centerCrop(asset:Pick<Asset,'width'|'height'>,rect:Rect):Crop{
 const ratio=rect.w/rect.h,source=asset.width/asset.height;
 return source>ratio?{x:(1-ratio/source)/2,y:0,w:ratio/source,h:1}:{x:0,y:(1-source/ratio)/2,w:1,h:source/ratio};
}
/** Baseline: one or two images per page, chronological input order, no silent repetition. */
export function gridPages(assets:readonly Asset[],count:number,lab:Lab):{pages:Page[];infeasible:string|null}{
 const fail=(reason:string)=>({pages:[],infeasible:reason});
 if(!Number.isInteger(count)||count<lab.pages.min||count>lab.pages.max||count%lab.pages.multiple||(lab.exportUnit==='page'&&count%2))return fail('Page count violates the selected product rules.');
 if(assets.length<count||assets.length>count*2)return fail(`This baseline needs between ${count} and ${count*2} distinct photos.`);
 if(new Set(assets.map(a=>a.sha256)).size!==assets.length)return fail('Duplicate originals cannot fill separate grid slots.');
 const pages:Page[]=[];let cursor=0;
 for(let i=0;i<count;i++){
  const side=lab.exportUnit==='spread'?'spread':i%2?'left':'right',safe=safeRect(lab.trim.w,lab.trim.h,side,lab);const remainingPages=count-i;const perPage=Math.ceil((assets.length-cursor)/remainingPages);
  const horizontal=lab.trim.w>lab.trim.h,gap=6;const width=horizontal?(safe.w-gap*(perPage-1))/perPage:safe.w,height=horizontal?safe.h:(safe.h-gap*(perPage-1))/perPage;
  if(width<=0||height<=0)return fail('Safe zones leave no usable area.');
  const page:Page={id:`grid-page-${i+1}`,side,widthMm:lab.trim.w,heightMm:lab.trim.h,background:'#f5f1e8',elements:[]};
  for(let j=0;j<perPage;j++){const asset=assets[cursor++]!,rect={x:safe.x+(horizontal?j*(width+gap):0),y:safe.y+(horizontal?0:j*(height+gap)),w:width,h:height};const crop=centerCrop(asset,rect);
   if(effectiveDpi(asset,crop,rect)<lab.dpi.redBelow)return fail(`Photo ${asset.id} has insufficient native resolution for page ${i+1}.`);
   if(asset.faces.some(face=>!contains(crop,face)))return fail(`Centered crop would cut a protected face in photo ${asset.id}; choose an alternate or change layout.`);
   page.elements.push({type:'image',id:`grid-image-${i+1}-${j+1}`,assetId:asset.id,rect,crop});
  }pages.push(page);
 }return {pages,infeasible:null};
}
