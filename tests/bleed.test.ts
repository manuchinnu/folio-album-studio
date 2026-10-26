import {test} from 'node:test';
import assert from 'node:assert/strict';
import {makeProject} from '../apps/cli/demo.js';
import {preflight} from '../packages/core/preflight.js';
import {insufficientBleedEdges} from '../packages/core/bleed.js';
test('trim and partial bleed warn; full asymmetric bleed and inset elements do not',()=>{
 const p=makeProject(),page=p.pages[0]!,b={left:2,right:4,top:5,bottom:3};
 assert.deepEqual(insufficientBleedEdges({x:0,y:0,w:210,h:297},page,b),['left','right','top','bottom']);
 assert.deepEqual(insufficientBleedEdges({x:-1,y:-1,w:212,h:299},page,b),['left','right','top','bottom']);
 assert.deepEqual(insufficientBleedEdges({x:-2,y:-5,w:216,h:305},page,b),[]);
 assert.deepEqual(insufficientBleedEdges({x:.51,y:.51,w:208.98,h:295.98},page,b),[]);
 assert.deepEqual(insufficientBleedEdges({x:0,y:-20,w:10,h:5},page,b),[]);
});
test('preflight flags fills and corrected generated spine reaches bleed',()=>{
 const p=makeProject();assert.equal(preflight(p,true).some(x=>x.code==='EDGE_BLEED'),false);
 p.cover!.elements.push({type:'fill',id:'trim-fill',color:'#000000',rect:{x:20,y:0,w:10,h:20}});
 assert.ok(preflight(p,true).some(x=>x.code==='EDGE_BLEED'&&x.objectId==='trim-fill'&&x.severity==='warning'));
});
