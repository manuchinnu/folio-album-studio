import {test} from 'node:test';import assert from 'node:assert/strict';
import {makeProject} from '../apps/cli/demo.js';
import {coverGeometry} from '../packages/core/cover.js';
import {migrateProject} from '../packages/core/migrate.js';
import {preflight} from '../packages/core/preflight.js';
test('legacy migration copies input and retains formula cover dimensions',()=>{
 const old:any=makeProject();old.schemaVersion=1;delete old.lab.exportUnit;delete old.lab.centreSafeMm;delete old.lab.coverProfile;
 const migrated=migrateProject(old);assert.equal(old.schemaVersion,1);assert.equal(migrated.schemaVersion,2);
 assert.equal(coverGeometry(migrated.lab,4)!.widthMm,421.5);assert.deepEqual(migrateProject(migrated),migrated);
});
test('lab table uses sheet count and independent panels, wrap and bleed',()=>{
 const p=makeProject(),zero={top:0,bottom:0,left:0,right:0};p.lab.exportUnit='spread';p.lab.trim.w=762;
 p.lab.coverProfile={spineModel:'lab_table',entries:[{sheets:4,frontWidthMm:380,backWidthMm:379,heightMm:303,spineMm:9,wrap:{...zero,left:10,right:11},bleed:{...zero,top:2}}]};
 const g=coverGeometry(p.lab,4)!;assert.equal(g.widthMm,789);assert.equal(g.heightMm,303);assert.equal(g.bleed.top,2);
 assert.throws(()=>coverGeometry(p.lab,40),/table entry/);
 assert.ok(preflight({...p,pages:Array.from({length:40},()=>p.pages[0]!)},true).some(i=>i.code==='COVER_TABLE'&&i.severity==='error'));
});
test('no-photo-cover product emits info, while inconsistent cover is blocked',()=>{
 const p=makeProject();p.lab.coverProfile=null;p.cover=null;
 assert.ok(preflight(p,true).some(i=>i.code==='NO_PHOTO_COVER'&&i.severity==='info'));
 assert.equal(preflight(p,true).filter(i=>i.severity==='error').length,0);
 p.cover=makeProject().cover;assert.ok(preflight(p,true).some(i=>i.code==='COVER_PROFILE'));
});
test('spread text and protected faces warn at centre without rejecting panoramas',()=>{
 const p=makeProject();p.lab.exportUnit='spread';p.lab.centreSafeMm=12;p.lab.gutter=0;p.lab.coverProfile=null;p.cover=null;
 for(const page of p.pages)page.side='spread';
 const page=p.pages[0]!;page.elements=[{type:'text',id:'fold-text',text:'Fold',fontId:'Vera',fontSizeMm:3,color:'#000000',rect:{x:100,y:20,w:10,h:10}},{type:'image',id:'fold-photo',assetId:p.assets[0]!.id,rect:{x:0,y:0,w:210,h:297},crop:{x:0,y:0,w:1,h:1}}];
 p.assets[0]!.faces=[{x:.48,y:.2,w:.04,h:.1}];
 const issues=preflight(p,true);assert.ok(issues.some(i=>i.code==='CENTRE_TEXT'&&i.severity==='warning'));assert.ok(issues.some(i=>i.code==='CENTRE_FACE'&&i.severity==='warning'));assert.ok(!issues.some(i=>i.code==='FACE_GUTTER'));
});

test('layflat fold-crossing face and text have no fold findings even with legacy gutter values',()=>{
 const p=makeProject();p.lab.binding='layflat';p.lab.exportUnit='spread';p.lab.centreSafeMm=12;p.lab.gutter=30;
 for(const page of p.pages)page.side='spread';
 const page=p.pages[0]!;page.elements=[{type:'text',id:'fold-text',text:'Fold',fontId:'Vera',fontSizeMm:3,color:'#000000',rect:{x:100,y:20,w:10,h:10}},{type:'image',id:'fold-photo',assetId:p.assets[0]!.id,rect:{x:20,y:20,w:170,h:250},crop:{x:0,y:0,w:1,h:1}}];
 p.assets[0]!.faces=[{x:.48,y:.2,w:.04,h:.1}];
 assert.deepEqual(preflight(p,true).filter(i=>i.pageId===page.id),[]);
 assert.ok(!preflight(p,true).some(i=>['CENTRE_TEXT','CENTRE_FACE','FACE_GUTTER'].includes(i.code)));
});
