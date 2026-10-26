/** Deterministic baseline policy. These technical signals are not learned aesthetics. */
export interface Candidate{id:string;sha256:string;phash:string;timeSeconds:number|null;timeBasis:'utc'|'local'|'unknown';meanLuma:number;detail:number;width:number;height:number}
export interface Group{primary:Candidate;alternates:Candidate[]}
export function hamming(a:string,b:string):number{if(!/^[a-f0-9]{16}$/.test(a)||!/^[a-f0-9]{16}$/.test(b))throw new Error('Invalid perceptual hash');let bits=BigInt('0x'+a)^BigInt('0x'+b),count=0;while(bits){bits&=bits-1n;count++}return count;}
export function technicalScore(c:Candidate):number{
 if(!Number.isFinite(c.detail)||!Number.isFinite(c.meanLuma)||c.detail<0||c.meanLuma<0||c.meanLuma>255)throw new Error('Invalid quality signals');
 const exposure=1-Math.min(1,Math.abs(c.meanLuma-127.5)/127.5);const detail=Math.min(1,c.detail/20);return .6*detail+.4*exposure;
}
export function duplicateGroups(input:readonly Candidate[]):Group[]{
 const ranked=[...input].sort((a,b)=>technicalScore(b)-technicalScore(a)||a.id.localeCompare(b.id));const groups:Group[]=[];
 for(const c of ranked){const match=groups.find(g=>g.primary.sha256===c.sha256||(c.timeSeconds!==null&&g.primary.timeSeconds!==null&&c.timeBasis!=='unknown'&&c.timeBasis===g.primary.timeBasis&&Math.abs(c.timeSeconds-g.primary.timeSeconds)<=60&&hamming(c.phash,g.primary.phash)<=6));if(match)match.alternates.push(c);else groups.push({primary:c,alternates:[]});}
 return groups;
}
export function timeEvents(groups:readonly Group[],gapSeconds=4*3600):Group[][]{
 const sorted=[...groups].sort((a,b)=>a.primary.timeBasis.localeCompare(b.primary.timeBasis)||(a.primary.timeSeconds??Infinity)-(b.primary.timeSeconds??Infinity)||a.primary.id.localeCompare(b.primary.id));const events:Group[][]=[];
 for(const group of sorted){const last=events.at(-1)?.at(-1)?.primary,current=group.primary;
  if(!last||current.timeSeconds===null||last.timeSeconds===null||current.timeBasis!==last.timeBasis||current.timeSeconds-last.timeSeconds>gapSeconds)events.push([group]);else events.at(-1)!.push(group);
 }return events;
}
export function selectBaseline(input:readonly Candidate[],budget:number):{selected:Candidate[];groups:Group[];events:Group[][];infeasible:string|null}{
 if(!Number.isInteger(budget)||budget<1)throw new Error('Photo budget must be a positive integer');
 const groups=duplicateGroups(input),events=timeEvents(groups);
 if(events.length>budget)return {selected:[],groups,events,infeasible:`${events.length} event groups need at least ${events.length} photo slots; budget is ${budget}.`};
 const selected=events.map(e=>[...e].sort((a,b)=>technicalScore(b.primary)-technicalScore(a.primary)||a.primary.id.localeCompare(b.primary.id))[0]!.primary);
 const ids=new Set(selected.map(c=>c.id));for(const g of groups)if(selected.length<budget&&!ids.has(g.primary.id)){selected.push(g.primary);ids.add(g.primary.id)}
 return {selected,groups,events,infeasible:null};
}
