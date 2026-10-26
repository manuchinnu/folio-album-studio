import {resolveCaptureTime,type CaptureTime,type Evidence,type MetadataSource} from './metadata.js';
function evidence(value:CaptureTime,raw:string,source:MetadataSource,sourceFile:string,field:string):Evidence<CaptureTime>|null{
 const item={value,raw,source,sourceFile,field,confidence:source==='filename'?.6:1};return resolveCaptureTime([item]).selected;
}
export function parseExifTime(raw:string,offset:string|null,sourceFile:string):Evidence<CaptureTime>|null{
 const m=/^(\d{4}):(\d{2}):(\d{2}) (\d{2}):(\d{2}):(\d{2})$/.exec(raw);if(!m)return null;
 const local=`${m[1]}-${m[2]}-${m[3]}T${m[4]}:${m[5]}:${m[6]}`;
 if(!offset)return evidence({local,precision:'second',timezone:'unknown'},raw,'embedded',sourceFile,'DateTimeOriginal');
 const z=/^([+-])(\d{2}):(\d{2})$/.exec(offset);if(!z||Number(z[3])>59)return null;
 const offsetMinutes=(z[1]==='-'?-1:1)*(Number(z[2])*60+Number(z[3]));
 const epochSeconds=Date.parse(local+offset)/1000;return evidence({local,precision:'second',timezone:'known',offsetMinutes,epochSeconds},raw+' '+offset,'embedded',sourceFile,'DateTimeOriginal+OffsetTimeOriginal');
}
export function parseEpoch(raw:string|number,source:MetadataSource,sourceFile:string,field:string):Evidence<CaptureTime>|null{
 if(typeof raw==='string'&&!/^-?\d+(?:\.\d+)?$/.test(raw))return null;
 const epochSeconds=Number(raw);if(!Number.isFinite(epochSeconds)||Math.abs(epochSeconds)>253402300799)return null;
 const date=new Date(epochSeconds*1000);if(!Number.isFinite(date.getTime()))return null;
 return evidence({local:date.toISOString().slice(0,19),precision:'second',timezone:'known',offsetMinutes:0,epochSeconds},String(raw),source,sourceFile,field);
}
export function parseIsoTime(raw:string,source:MetadataSource,sourceFile:string,field:string):Evidence<CaptureTime>|null{
 const m=/^(\d{4}-\d{2}-\d{2})(?:T(\d{2}:\d{2}:\d{2})(?:\.\d+)?(Z|[+-]\d{2}:\d{2})?)?$/.exec(raw);if(!m)return null;
 const local=m[1]+(m[2]?'T'+m[2]:'');if(!m[3])return evidence({local,precision:m[2]?'second':'date',timezone:'unknown'},raw,source,sourceFile,field);
 const offset=m[3]==='Z'?'+00:00':m[3];const parsed=parseExifTime(local.slice(0,10).replaceAll('-',':')+' '+m[2],offset,sourceFile);
 return parsed?{...parsed,raw,source,field}:null;
}
export function parseGpsCoordinate(raw:string,axis:'latitude'|'longitude'):number|null{
 if(/^[+-]?\d+(?:\.\d+)?$/.test(raw)){const value=Number(raw);return Math.abs(value)<=(axis==='latitude'?90:180)?value:null;}
 const m=/^(\d+(?:\.\d+)?)(?:,(\d+(?:\.\d+)?))?([NSEW])$/.exec(raw);if(!m||!(axis==='latitude'?'NS':'EW').includes(m[3]!))return null;
 if(m[2]!==undefined&&(!Number.isInteger(Number(m[1]))||Number(m[2])>=60))return null;
 const value=(Number(m[1])+Number(m[2]??0)/60)*('SW'.includes(m[3]!)?-1:1);return Math.abs(value)<=(axis==='latitude'?90:180)?value:null;
}
export function parseFilenameTime(filename:string):Evidence<CaptureTime>|null{
 const name=filename.replaceAll('\\','/').split('/').at(-1)!;
 const pixel=/^PXL_(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})\d{3}(?:[._-].*)?\.[a-z0-9]+$/i.exec(name);
 if(pixel)return evidence({local:`${pixel[1]}-${pixel[2]}-${pixel[3]}T${pixel[4]}:${pixel[5]}:${pixel[6]}`,precision:'second',timezone:'unknown'},name,'filename',filename,'recognized-pixel-filename; subseconds retained in raw');
 const timed=/^(?:IMG_|PXL_|VID_)(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})(?:[._-].*)?\.[a-z0-9]+$/i.exec(name);
 if(timed)return evidence({local:`${timed[1]}-${timed[2]}-${timed[3]}T${timed[4]}:${timed[5]}:${timed[6]}`,precision:'second',timezone:'unknown'},name,'filename',filename,'recognized-filename');
 const wa=/^IMG-(\d{4})(\d{2})(\d{2})-WA\d+\.[a-z0-9]+$/i.exec(name);
 const shot=/^Screenshot[_ -](\d{4})[-_](\d{2})[-_](\d{2})(?:[ _-](\d{2})[-_:](\d{2})[-_:](\d{2}))?\.[a-z0-9]+$/i.exec(name);
 const m=wa??shot;if(!m)return null;const time=shot?.[4]!==undefined?`T${shot[4]}:${shot[5]}:${shot[6]}`:'';
 return evidence({local:`${m[1]}-${m[2]}-${m[3]}${time}`,precision:time?'second':'date',timezone:'unknown'},name,'filename',filename,'recognized-filename');
}
