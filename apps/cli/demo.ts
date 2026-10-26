import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {randomUUID} from 'node:crypto';
import type {Asset,Lab,Project,Request,Page,Rect} from '../../packages/core/schema.js';
import {parseRequest} from '../../packages/core/schema.js';
import {spineMm} from '../../packages/core/geometry.js';
import {preflight} from '../../packages/core/preflight.js';
import {nativeRequest} from './native-port.js';
export function makeProject(count=4):Project{
 const lab=JSON.parse(readFileSync('fixtures/synthetic-lab.json','utf8')) as Lab;
 const manifest=JSON.parse(readFileSync('fixtures/media/manifest.json','utf8')) as Pick<Project,'assets'|'fonts'>;
 const crop=(a:Asset,r:Rect)=>{const ratio=r.w/r.h,source=a.width/a.height;return source>ratio?{x:(1-ratio/source)/2,y:0,w:ratio/source,h:1}:{x:0,y:(1-source/ratio)/2,w:1,h:source/ratio};};
 const pages:Page[]=Array.from({length:count},(_,i)=>{const a=manifest.assets[i%manifest.assets.length]!;const r={x:14,y:35,w:182,h:214};return {id:`page-${i+1}`,side:i%2?'left':'right',widthMm:210,heightMm:297,background:'#f5f1e8',elements:[{type:'text',id:`heading-${i}`,text:i%2?'A different perspective':'The quiet between',rect:{x:14,y:14,w:182,h:14},fontId:'Vera',fontSizeMm:7,color:'#2c473f'},{type:'image',id:`image-${i}`,assetId:a.id,rect:r,crop:crop(a,r)},{type:'text',id:`caption-${i}`,text:'SYNTHETIC LANDSCAPES / PRINT PIPELINE STUDY',rect:{x:14,y:257,w:182,h:7},fontId:'Vera',fontSizeMm:2.6,color:'#6e776e'},{type:'text',id:`proof-${i}`,text:`ENGINEERING PROOF - NOT LAB READY    /    ${String(i+1).padStart(2,'0')}`,rect:{x:14,y:276,w:182,h:7},fontId:'Vera',fontSizeMm:2.6,color:'#805942'}]};});
 const spine=spineMm(lab,count),asset=manifest.assets[1]!,r={x:210+spine+14,y:54,w:182,h:205};
 const cover:Page={id:'cover',side:'cover',widthMm:420+spine,heightMm:297,background:'#e9e4d7',elements:[{type:'fill',id:'cover-spine',rect:{x:210,y:-lab.bleed.top,w:spine,h:297+lab.bleed.top+lab.bleed.bottom},color:'#344e45'},{type:'text',id:'back-title',text:'FOLIO / THE PRINT STUDY',rect:{x:20,y:132,w:170,h:15},fontId:'Vera',fontSizeMm:5,color:'#344e45'},{type:'text',id:'back-note',text:'Synthetic fixtures. No personal photographs.',rect:{x:20,y:153,w:170,h:8},fontId:'Vera',fontSizeMm:3,color:'#6e776e'},{type:'text',id:'cover-title',text:'The quiet between',rect:{x:210+spine+14,y:17,w:182,h:17},fontId:'Vera',fontSizeMm:8,color:'#344e45'},{type:'image',id:'cover-image',assetId:asset.id,rect:r,crop:crop(asset,r)},{type:'text',id:'cover-note',text:'ENGINEERING PROOF - NOT LAB READY',rect:{x:210+spine+14,y:276,w:182,h:7},fontId:'Vera',fontSizeMm:2.6,color:'#805942'}]};
 return {schemaVersion:2,id:'folio-print-study',title:'The quiet between',lab,...manifest,pages,cover};
}
export async function runDemo(count:number,project:Project=makeProject(count),label='demo'){const issues=preflight(project,true);const request:Request={protocolVersion:1,jobId:`${label}-${count}-${randomUUID().slice(0,8)}`,operation:'export',project,proof:true,cancelled:false};parseRequest(request);
 mkdirSync('artifacts',{recursive:true});writeFileSync('artifacts/latest-request.json',JSON.stringify(request,null,2));
 if(issues.some(i=>i.severity==='error'))throw new Error(JSON.stringify(issues));
 const start=performance.now();let nodePeak=process.memoryUsage().rss;const monitor=setInterval(()=>{nodePeak=Math.max(nodePeak,process.memoryUsage().rss)},20);
 let result;try{result=await nativeRequest(request)}finally{clearInterval(monitor);nodePeak=Math.max(nodePeak,process.memoryUsage().rss)}
 if(result.status!=='completed')throw new Error(result.message);
 result.metrics.elapsedEndToEndSeconds=(performance.now()-start)/1000;result.metrics.peakNodeSampledBytes=nodePeak;result.metrics.combinedSampledPeakBoundBytes=nodePeak+result.metrics.peakNativeTreeBytes;result.metrics.memoryMethod='20ms RSS samples; sum of independent Node and native-tree sampled peaks; transients between samples may be missed';
 writeFileSync(resolve(result.directory,'preflight.json'),JSON.stringify({profile:project.lab,proofOnly:true,issues},null,2));
 writeFileSync(resolve(result.directory,'preflight.txt'),`FOLIO ENGINEERING PROOF — NOT LAB READY\n\nProfile: ${project.lab.name}\nInterior pages: ${count}\nCover: ${project.cover ? "photo cover" : "none"}\n\n${issues.map(i=>`${i.severity.toUpperCase()} ${i.code}: ${i.message}`).join('\n')}\n\nExternal lab verification remains open.\n`);
 writeFileSync(resolve(result.directory,'metrics.json'),JSON.stringify(result.metrics,null,2));writeFileSync('artifacts/latest-result.json',JSON.stringify(result,null,2));console.log(JSON.stringify(result,null,2));return result;}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href)await runDemo(Number(process.argv[2]??4));
