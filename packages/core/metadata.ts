/** Field-wise provenance policy. Adapters parse sources; core chooses without inventing data. */
export type MetadataSource='user'|'embedded'|'embeddedXmp'|'takeoutExif'|'takeout'|'xmpSidecar'|'filename'|'filesystem';
export interface Evidence<T>{value:T;raw:unknown;source:MetadataSource;sourceFile:string;field:string;confidence:number;corroborated?:boolean}
export interface CaptureTime{local:string;precision:'date'|'second';timezone:'known'|'unknown';offsetMinutes?:number;epochSeconds?:number}
export interface Location{latitude:number;longitude:number;altitude?:number}
export interface Resolution<T>{selected:Evidence<T>|null;alternatives:Evidence<T>[];rejected:Evidence<T>[];ambiguous:boolean}
const priority:Record<MetadataSource,number>={user:0,embedded:1,embeddedXmp:1.5,takeoutExif:2,takeout:3,xmpSidecar:4,filename:5,filesystem:6};
function resolveField<T>(input:readonly Evidence<T>[],valid:(e:Evidence<T>)=>boolean,equal:(a:T,b:T)=>boolean):Resolution<T>{
 const accepted=input.filter(e=>Number.isFinite(e.confidence)&&e.confidence>=0&&e.confidence<=1&&valid(e));
 const rejected=input.filter(e=>!accepted.includes(e));
 const ordered=[...accepted].sort((a,b)=>priority[a.source]-priority[b.source]);
 const first=ordered[0];if(!first)return {selected:null,alternatives:[],rejected,ambiguous:false};
 const peers=ordered.filter(e=>priority[e.source]===priority[first.source]);
 // Contradictory equal-precedence evidence must be surfaced, never arbitrary first-match selection.
 if(peers.some(e=>!equal(e.value,first.value)))return {selected:null,alternatives:ordered,rejected,ambiguous:true};
 return {selected:first,alternatives:ordered.slice(1),rejected,ambiguous:false};
}
export function resolveLocation(input:readonly Evidence<Location>[]):Resolution<Location>{return resolveField(input,e=>{
 const v=e.value;if(e.source==='filename'||e.source==='filesystem')return false;
 if(!Number.isFinite(v.latitude)||Math.abs(v.latitude)>90||!Number.isFinite(v.longitude)||Math.abs(v.longitude)>180)return false;
 if(v.altitude!==undefined&&!Number.isFinite(v.altitude))return false;
 if((e.source==='takeout'||e.source==='takeoutExif')&&v.latitude===0&&v.longitude===0&&(v.altitude??0)===0&&!e.corroborated)return false;
 return true;
},(a,b)=>a.latitude===b.latitude&&a.longitude===b.longitude&&a.altitude===b.altitude);}
export function resolveCaptureTime(input:readonly Evidence<CaptureTime>[]):Resolution<CaptureTime>{return resolveField(input,e=>{
 const v=e.value;const match=/^(\d{4})-(\d{2})-(\d{2})(?:T(\d{2}):(\d{2}):(\d{2}))?$/.exec(v.local);if(!match)return false;
 if(v.precision==='date'&&match[4]!==undefined||v.precision==='second'&&match[4]===undefined)return false;
 const year=Number(match[1]),month=Number(match[2]),day=Number(match[3]);const leap=year%4===0&&(year%100!==0||year%400===0);
 const days=[31,leap?29:28,31,30,31,30,31,31,30,31,30,31];if(year<1||month<1||month>12||day<1||day>days[month-1]!)return false;
 if(match[4]!==undefined&&(Number(match[4])>23||Number(match[5])>59||Number(match[6])>59))return false;
 if(v.timezone==='unknown'&&(v.offsetMinutes!==undefined||v.epochSeconds!==undefined))return false;
 if(v.timezone==='known'&&(v.offsetMinutes===undefined||!Number.isInteger(v.offsetMinutes)||Math.abs(v.offsetMinutes)>840))return false;
 if(v.epochSeconds!==undefined){if(!Number.isFinite(v.epochSeconds)||v.precision!=='second'||v.offsetMinutes===undefined)return false;const expected=Date.parse(v.local+'Z')/1000-v.offsetMinutes*60;if(Math.abs(expected-v.epochSeconds)>=1)return false;}
 return true;
},(a,b)=>a.local===b.local&&a.precision===b.precision&&a.timezone===b.timezone&&a.offsetMinutes===b.offsetMinutes&&a.epochSeconds===b.epochSeconds);}
