import {readFileSync} from 'node:fs';import {makeProject,runDemo} from './demo.js';import type {Asset,Page} from '../../packages/core/schema.js';
const project=makeProject(40),asset=JSON.parse(readFileSync('fixtures/large/manifest.json','utf8')) as Asset;
project.id='large-layflat-proof';project.assets=[asset];project.lab={...project.lab,id:'synthetic-305-layflat',name:'Synthetic 305 mm square layflat proof',trim:{w:305,h:305},gutter:0,binding:'layflat',spine:{thicknessPerLeafMm:0,fixedMm:0}};
project.pages=Array.from({length:40},(_,i):Page=>({id:`large-page-${i+1}`,side:i%2?'left':'right',widthMm:305,heightMm:305,background:'#ffffff',elements:[
 {type:'image',id:`large-image-${i}`,assetId:asset.id,rect:{x:-3,y:-3,w:311,h:311},crop:{x:i%2?0:305/616,y:0,w:311/616,h:1}},
 {type:'text',id:`large-label-${i}`,text:`ENGINEERING PROOF - NOT LAB READY / ${i+1}`,rect:{x:14,y:279,w:276,h:12},fontId:'Vera',fontSizeMm:4,color:'#ffffff'}]}));
project.cover={id:'large-cover',side:'cover',widthMm:610,heightMm:305,background:'#ffffff',elements:[{type:'image',id:'large-cover-image',assetId:asset.id,rect:{x:-3,y:-3,w:616,h:311},crop:{x:0,y:0,w:1,h:1}},{type:'text',id:'large-cover-label',text:'ENGINEERING PROOF - NOT LAB READY / FULL BLEED SPREAD',rect:{x:20,y:279,w:570,h:12},fontId:'Vera',fontSizeMm:4,color:'#ffffff'}]};
await runDemo(40,project,'large');
