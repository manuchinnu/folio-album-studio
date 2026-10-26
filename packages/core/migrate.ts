import {Value} from '@sinclair/typebox/value';
import {ProjectSchema,type Project} from './schema.js';
/** Copy before migration so historical input and linked original paths remain untouched. */
export function migrateProject(input:unknown):Project{
 const p=structuredClone(input) as any;
 if(p?.schemaVersion===1){
  if(!p.lab||p.lab.exportUnit!==undefined||p.lab.coverProfile!==undefined)throw new Error('Invalid legacy profile');
  p.schemaVersion=2;p.lab.exportUnit='page';p.lab.centreSafeMm=0;
  p.lab.coverProfile=p.cover===null?null:{spineModel:'formula',wrap:{top:0,right:0,bottom:0,left:0},bleed:structuredClone(p.lab.bleed)};
 }
 if(!Value.Check(ProjectSchema,p))throw new Error('Unsupported or invalid project schema');
 return p;
}
