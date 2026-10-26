import {spawn} from 'node:child_process';
import {resolve} from 'node:path';
import {existsSync,writeFileSync,mkdirSync} from 'node:fs';
import type {Request} from '../../packages/core/schema.js';
export function python(){return process.env.FOLIO_PYTHON??resolve(process.platform==='win32'?'.venv/Scripts/python.exe':'.venv/bin/python');}
export async function nativeRequest(request:unknown,signal?:AbortSignal):Promise<any>{
 const output=resolve('artifacts');mkdirSync(output,{recursive:true});
 return new Promise((done,reject)=>{const child=spawn(python(),['native/folio_workers/worker.py','--root',process.cwd(),'--output',output],{windowsHide:true,stdio:['pipe','pipe','pipe']});let stdout='',stderr='';
 const cancel=()=>{const id=(request as Request).jobId;if(/^[a-zA-Z0-9_.-]+$/.test(id))writeFileSync(resolve(output,id+'.cancel'),'cancel');};
 signal?.addEventListener('abort',cancel,{once:true});if(signal?.aborted)cancel();
 child.stdout.on('data',b=>{stdout+=String(b)});child.stderr.on('data',b=>{stderr+=String(b)});child.on('error',reject);
 child.on('close',code=>{signal?.removeEventListener('abort',cancel);if(code!==0)return reject(new Error(`Native exit ${code}: ${stderr}`));try{done(JSON.parse(stdout.trim()));}catch{reject(new Error(`Invalid native response: ${stdout} ${stderr}`));}});
 child.stdin.end(JSON.stringify(request)+'\n');
 });
}
