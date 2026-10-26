import {open,readdir,realpath,lstat} from 'node:fs/promises';import {basename,dirname,join,relative,isAbsolute} from 'node:path';
import type {InspectedImage} from '../../packages/core/import-schema.js';
import {parseEpoch,parseExifTime,parseFilenameTime,parseIsoTime,parseGpsCoordinate} from '../../packages/core/metadata-parsers.js';
import {resolveCaptureTime,resolveLocation,type Evidence,type CaptureTime,type Location} from '../../packages/core/metadata.js';
interface Sidecar{file:string;data:Record<string,unknown>}
const isObject=(v:unknown):v is Record<string,unknown>=>typeof v==='object'&&v!==null&&!Array.isArray(v);
function boundedDepth(value:unknown,depth=0):void{if(depth>20)throw new Error('Sidecar nesting exceeds limit');if(value&&typeof value==='object')for(const v of Object.values(value))boundedDepth(v,depth+1);}
async function boundedJson(root:string,path:string):Promise<Record<string,unknown>>{
 if((await lstat(path)).isSymbolicLink())throw new Error('Sidecar symbolic link refused');const canonical=await realpath(path),rel=relative(root,canonical);if(isAbsolute(rel)||rel==='..'||rel.startsWith('..\\')||rel.startsWith('../'))throw new Error('Sidecar escapes granted root');
 const handle=await open(canonical,'r');let text:string;try{const buffer=Buffer.alloc(1_048_577);let used=0;while(used<buffer.length){const result=await handle.read(buffer,used,buffer.length-used,null);if(!result.bytesRead)break;used+=result.bytesRead;}if(used>1_048_576)throw new Error('Sidecar exceeds 1 MiB');text=buffer.subarray(0,used).toString('utf8')}finally{await handle.close()}
 const value:unknown=JSON.parse(text);boundedDepth(value);if(!isObject(value))throw new Error('Sidecar must be a JSON object');return value;
}
export async function collectMetadata(root:string,images:InspectedImage[],signal?:AbortSignal){
 const directories=new Map<string,Sidecar[]>();const issues:{file:string;message:string}[]=[];
 let retainedBytes=0;
 for(const image of images){const directory=dirname(image.relativePath);if(directories.has(directory))continue;const candidates:Sidecar[]=[];const entries=await readdir(join(root,directory));
  if(entries.length>10000)throw new Error('Directory sidecar scan exceeds 10,000 entries');
  for(const name of entries.filter(n=>/\.json$/i.test(n)).sort()){signal?.throwIfAborted();if(retainedBytes>=16_000_000){issues.push({file:directory,message:'Sidecar aggregate 16 MB budget reached; remaining files unresolved'});break;}try{const file=join(directory,name).replaceAll('\\','/');const data=await boundedJson(root,join(root,file));retainedBytes+=Buffer.byteLength(JSON.stringify(data));candidates.push({file,data})}catch(error){issues.push({file:join(directory,name),message:String(error)})}}
  directories.set(directory,candidates);
 }
 const records=images.map(image=>{
  const times:Evidence<CaptureTime>[]=[];const locations:Evidence<Location>[]=[];const warnings=[...image.warnings];
  const addTime=(value:Evidence<CaptureTime>|null)=>{if(value)times.push(value)};
  if(image.captureTime){const parsed=parseExifTime(image.captureTime,image.captureOffset,image.relativePath);addTime(parsed);if(!parsed)warnings.push('Invalid embedded capture timestamp retained in inspection record')}
  if(image.gps)locations.push({value:{latitude:image.gps.latitude,longitude:image.gps.longitude,...(image.gps.altitude===null?{}:{altitude:image.gps.altitude})},raw:image.gps,source:'embedded',sourceFile:image.relativePath,field:'GPS',confidence:1});
  for(const packet of image.xmp){const source=packet.source==='embedded'?'embeddedXmp':'xmpSidecar';const dates=packet.fields.filter(f=>f.name==='DateTimeOriginal');for(const field of dates.length?dates:packet.fields.filter(f=>f.name==='CreateDate'))addTime(parseIsoTime(field.raw,source,packet.sourceFile,field.name));
   const latitudes=packet.fields.filter(f=>f.name==='GPSLatitude'),longitudes=packet.fields.filter(f=>f.name==='GPSLongitude');
   if(latitudes.length===1&&longitudes.length===1){const latitude=parseGpsCoordinate(latitudes[0]!.raw,'latitude'),longitude=parseGpsCoordinate(longitudes[0]!.raw,'longitude');if(latitude!==null&&longitude!==null)locations.push({value:{latitude,longitude},raw:packet.fields,source,sourceFile:packet.sourceFile,field:'GPSLatitude+GPSLongitude',confidence:1});}
   else if(latitudes.length||longitudes.length)warnings.push('Incomplete or ambiguous XMP GPS retained in inspection record');
  }
  const candidates=directories.get(dirname(image.relativePath))??[];
  const exact=candidates.filter(s=>s.file===image.relativePath+'.json'||s.data.title===basename(image.relativePath));
  // No fuzzy match without corroborating metadata; all ambiguity is visible.
  if(exact.length>1){warnings.push('Ambiguous Takeout sidecars: '+exact.map(s=>s.file).join(', '));}
  else if(exact.length===1){const sidecar=exact[0]!,taken=sidecar.data.photoTakenTime;if(isObject(taken)&&(typeof taken.timestamp==='string'||typeof taken.timestamp==='number'))addTime(parseEpoch(taken.timestamp,'takeout',sidecar.file,'photoTakenTime.timestamp'));
   for(const [field,source] of [['geoDataExif','takeoutExif'],['geoData','takeout']] as const){const gps=sidecar.data[field];if(isObject(gps)&&typeof gps.latitude==='number'&&typeof gps.longitude==='number')locations.push({value:{latitude:gps.latitude,longitude:gps.longitude,...(typeof gps.altitude==='number'?{altitude:gps.altitude}:{})},raw:gps,source,sourceFile:sidecar.file,field,confidence:1});}
  }
  addTime(parseFilenameTime(image.relativePath));addTime(parseEpoch(image.mtimeSeconds,'filesystem',image.relativePath,'mtime'));
  return {relativePath:image.relativePath,time:resolveCaptureTime(times),location:resolveLocation(locations),warnings};
 });
 return {records,issues};
}
