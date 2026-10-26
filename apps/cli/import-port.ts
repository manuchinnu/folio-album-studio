import {readdir,realpath,stat,mkdir} from 'node:fs/promises';
import {resolve,relative,isAbsolute,join} from 'node:path';
import {spawn} from 'node:child_process';
import {python} from './native-port.js';
import {parseImportResponse,type ImportResponse} from '../../packages/core/import-schema.js';
export interface Enumeration{root:string;paths:string[];skipped:{relativePath:string;reason:string}[]}
export async function enumeratePhotos(input:string,signal?:AbortSignal):Promise<Enumeration>{
 const root=await realpath(input);if(!(await stat(root)).isDirectory())throw new Error('Select a directory');
 const result:Enumeration={root,paths:[],skipped:[]};let entries=0;
 async function walk(directory:string){for(const entry of (await readdir(directory,{withFileTypes:true})).sort((a,b)=>a.name.localeCompare(b.name,'en'))){
  signal?.throwIfAborted();if(++entries>100000)throw new Error('Folder exceeds 100,000 entries');
  const path=join(directory,entry.name),rel=relative(root,path).replaceAll('\\','/');
  if(entry.isSymbolicLink()){result.skipped.push({relativePath:rel,reason:'Symbolic link is not followed'});continue;}
  const real=await realpath(path),inside=relative(root,real);if(inside==='..'||inside.startsWith('..\\')||inside.startsWith('../')||isAbsolute(inside)){result.skipped.push({relativePath:rel,reason:'Path escapes granted root'});continue;}
  if(entry.isDirectory())await walk(real);
  else if(entry.isFile()&&/\.(jpe?g|png|webp|heic|heif)$/i.test(entry.name)){if(result.paths.length===3000)throw new Error('Folder exceeds 3,000 supported photos');result.paths.push(rel);}
  else if(entry.isFile()&&/\.(cr2|nef|arw|dng)$/i.test(entry.name))result.skipped.push({relativePath:rel,reason:'RAW embedded-preview adapter is not available yet'});
 }}await walk(root);return result;
}
export async function inspectPhotos(input:Enumeration,proxyDirectory:string,signal?:AbortSignal):Promise<ImportResponse>{
 const output=resolve(proxyDirectory);await mkdir(output,{recursive:true});signal?.throwIfAborted();
 return new Promise((done,reject)=>{
  const child=spawn(python(),['native/folio_workers/import_worker.py','--root',input.root,'--output',output],{windowsHide:true,stdio:['pipe','pipe','pipe']});let stdout='',stderr='';
  const abort=()=>child.kill();signal?.addEventListener('abort',abort,{once:true});
  child.stdout.on('data',b=>{stdout+=String(b);if(stdout.length>16_000_000)child.kill()});child.stderr.on('data',b=>{stderr=(stderr+String(b)).slice(-4000)});child.on('error',reject);
  child.on('close',code=>{signal?.removeEventListener('abort',abort);if(signal?.aborted)return reject(new Error('Import cancelled; originals unchanged'));if(code!==0)return reject(new Error(`Native import exit ${code}: ${stderr}`));try{done(parseImportResponse(JSON.parse(stdout)))}catch(error){reject(error)}});
  if(signal?.aborted)abort();child.stdin.end(JSON.stringify({protocolVersion:1,paths:input.paths,proxySize:512})+'\n');
 });
}
