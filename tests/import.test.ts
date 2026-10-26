import {test} from 'node:test';import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';import {resolve} from 'node:path';import {randomUUID} from 'node:crypto';
import {enumeratePhotos,inspectPhotos} from '../apps/cli/import-port.js';
import {parseExifTime,parseEpoch,parseFilenameTime} from '../packages/core/metadata-parsers.js';
import {collectMetadata} from '../apps/cli/metadata-port.js';
test('linked recursive import decodes required formats and EXIF orientations, preserving originals',async()=>{
 const first=await enumeratePhotos('fixtures/import'),again=await enumeratePhotos('fixtures/import');assert.deepEqual(first.paths,again.paths);assert.ok(first.paths.includes('nested/image.png'));assert.ok(first.skipped.some(x=>x.relativePath==='camera.cr2'));
 const before=await readFile('fixtures/import/orientation-6.jpg');const result=await inspectPhotos(first,'artifacts/import-'+randomUUID());
 assert.equal(result.errors.length,1);assert.equal(result.errors[0]?.relativePath,'corrupt.jpg');assert.equal(result.images.length,12);
 for(let n=1;n<=8;n++){const item=result.images.find(x=>x.relativePath===`orientation-${n}.jpg`)!;assert.equal(item.orientation,n);assert.deepEqual([item.width,item.height],n<5?[120,80]:[80,120]);}
 assert.equal(result.images.find(x=>x.relativePath==='misnamed.jpeg')?.format,'PNG');assert.ok(result.images.some(x=>x.format==='HEIF'));assert.ok(result.images.some(x=>x.format==='WEBP'));assert.deepEqual(await readFile('fixtures/import/orientation-6.jpg'),before);
 await writeFile('artifacts/import-latest.json',JSON.stringify(result,null,2));
 const metadata=await collectMetadata(first.root,result.images);const known=metadata.records.find(r=>r.relativePath==='nested/image.png')!;assert.equal(known.time.selected?.source,'takeout');assert.equal(known.location.selected?.source,'takeout');assert.equal(known.location.rejected[0]?.source,'takeoutExif');
 assert.ok(metadata.records.find(r=>r.relativePath==='image.webp')?.warnings.some(w=>w.startsWith('Ambiguous')));
 const xmp=metadata.records.find(r=>r.relativePath==='orientation-1.jpg')!;assert.equal(xmp.time.selected?.source,'xmpSidecar');assert.equal(xmp.time.selected?.value.offsetMinutes,330);assert.equal(xmp.location.selected?.value.latitude,12.5);
 assert.ok(result.images.find(i=>i.relativePath==='orientation-2.jpg')?.warnings.some(w=>w.includes('DTDForbidden')));assert.ok(metadata.issues.some(i=>i.file.includes('too-deep.json')&&i.message.includes('nesting')));
});
test('native import independently rejects escape and absolute paths',async()=>{const result=await inspectPhotos({root:resolve('fixtures/import'),paths:['../../package.json',resolve('fixtures/import/image.webp')],skipped:[]},'artifacts/import-rejected');assert.equal(result.images.length,0);assert.equal(result.errors.length,2);});
test('time parsers preserve precision, validate dates and reconcile offsets',()=>{
 assert.equal(parseExifTime('2024:02:29 12:30:00','+05:30','a.jpg')?.value.epochSeconds,Date.parse('2024-02-29T07:00:00Z')/1000);
 assert.equal(parseExifTime('2025:02:29 12:30:00',null,'a.jpg'),null);assert.equal(parseExifTime('2024:01:01 12:30:00','+05:99','a.jpg'),null);
 assert.equal(parseFilenameTime('IMG-20240401-WA0001.jpg')?.value.precision,'date');assert.equal(parseFilenameTime('PXL_20240401_123000123.jpg')?.value.local,'2024-04-01T12:30:00');
 assert.equal(parseFilenameTime('PXL_20240401_123000.123.jpg')?.value.timezone,'unknown');assert.equal(parseFilenameTime('IMG_20240401_123000.jpg')?.value.local,'2024-04-01T12:30:00');
 assert.equal(parseEpoch('bad','takeout','a.json','photoTakenTime'),null);
});
