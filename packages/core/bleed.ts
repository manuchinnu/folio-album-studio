import type {Lab,Page,Rect} from './schema.js';

/** Detect near-trim or partially extended decorations that can leave cutting slivers. */
export function insufficientBleedEdges(r:Rect,page:Page,bleed:Lab['bleed']):string[]{
 const edges:string[]=[];
 if(r.y+r.h>0&&r.y<page.heightMm){
  if(r.x<=.5&&r.x+r.w>0&&r.x> -bleed.left+1e-7)edges.push('left');
  if(r.x+r.w>=page.widthMm-.5&&r.x<page.widthMm&&r.x+r.w<page.widthMm+bleed.right-1e-7)edges.push('right');
 }
 if(r.x+r.w>0&&r.x<page.widthMm){
  if(r.y<=.5&&r.y+r.h>0&&r.y> -bleed.top+1e-7)edges.push('top');
  if(r.y+r.h>=page.heightMm-.5&&r.y<page.heightMm&&r.y+r.h<page.heightMm+bleed.bottom-1e-7)edges.push('bottom');
 }
 return edges;
}
