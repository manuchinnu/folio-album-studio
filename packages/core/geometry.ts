import type {Asset,Crop,Lab,Rect} from './schema.js';
export type Mm=number & {readonly __unit:'mm'}; export type Pixels=number & {readonly __unit:'px'};
export function mm(value:number):Mm {if(!Number.isFinite(value))throw new Error('Nonfinite mm');return value as Mm;}
export const mmToPt=(value:number)=>value*72/25.4;
export const mmToPx=(value:number,ppi:number):Pixels=>Math.round(value*ppi/25.4) as Pixels;
export const presets={a4Portrait:{w:210,h:297,orientation:'portrait'},a4Landscape:{w:297,h:210,orientation:'landscape'},square8:{w:203.2,h:203.2,orientation:'square'},square12:{w:304.8,h:304.8,orientation:'square'}} as const;
export function pageBoxes(width:number,height:number,lab:Lab){const b=lab.bleed;return {media:[0,0,mmToPt(width+b.left+b.right),mmToPt(height+b.top+b.bottom)],trim:[mmToPt(b.left),mmToPt(b.bottom),mmToPt(b.left+width),mmToPt(b.bottom+height)],bleed:[0,0,mmToPt(width+b.left+b.right),mmToPt(height+b.top+b.bottom)]};}
export function spineMm(lab:Lab,pages:number){if(lab.binding==='spiral')return 0;if(pages%2)throw new Error('Two-sided leaf count requires an even page count');return pages/2*lab.spine.thicknessPerLeafMm+lab.spine.fixedMm;}
export function effectiveDpi(asset:Asset,crop:Crop,rect:Rect){return Math.min(asset.width*crop.w/(rect.w/25.4),asset.height*crop.h/(rect.h/25.4));}
export function contains(outer:Rect,inner:Rect){const e=1e-7;return inner.x>=outer.x-e&&inner.y>=outer.y-e&&inner.x+inner.w<=outer.x+outer.w+e&&inner.y+inner.h<=outer.y+outer.h+e;}
export function overlaps(a:Rect,b:Rect){return a.x<b.x+b.w&&a.x+a.w>b.x&&a.y<b.y+b.h&&a.y+a.h>b.y;}
export function safeRect(width:number,height:number,side:string,lab:Lab):Rect{const left=Math.max(lab.safe,lab.binding!=='layflat'&&side==='right'?lab.gutter:0),right=Math.max(lab.safe,lab.binding!=='layflat'&&side==='left'?lab.gutter:0);return {x:left,y:lab.safe,w:width-left-right,h:height-2*lab.safe};}
export function projectFace(face:Crop,crop:Crop,rect:Rect):Rect{return {x:rect.x+(face.x-crop.x)/crop.w*rect.w,y:rect.y+(face.y-crop.y)/crop.h*rect.h,w:face.w/crop.w*rect.w,h:face.h/crop.h*rect.h};}
