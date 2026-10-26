import {writeFileSync,readFileSync,mkdirSync} from 'node:fs';
import {RequestSchema} from '../packages/core/schema.js';
import {ImportRequestSchema,ImportResponseSchema} from '../packages/core/import-schema.js';
for(const [name,schema] of Object.entries({request:RequestSchema,'import-request':ImportRequestSchema,'import-response':ImportResponseSchema})){
 const path=`packages/core/generated/${name}.schema.json`;
 const data=JSON.stringify({$schema:'http://json-schema.org/draft-07/schema#',...schema},null,2)+'\n';
 if(process.argv.includes('--check')){if(readFileSync(path,'utf8')!==data)throw new Error(`Generated schema ${name} is stale. Run npm run schema.`);console.log(`${name}: generated schema matches TypeScript contract.`);}else{mkdirSync('packages/core/generated',{recursive:true});writeFileSync(path,data);console.log(path);}
}
