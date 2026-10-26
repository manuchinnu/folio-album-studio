import {readFileSync} from 'node:fs';
import {makeProject,runDemo} from './demo.js';
import type {Asset,Page} from '../../packages/core/schema.js';
const p=makeProject(40),asset=JSON.parse(readFileSync('fixtures/spread/manifest.json','utf8')) as Asset;
p.id='engineering-12x15-spreads';p.assets=[asset];p.cover=null;
p.lab={...p.lab,id:'engineering-12x15-landscape',name:'ENGINEERING ONLY: 12x15 landscape assumption',exportUnit:'spread',trim:{w:762,h:304.8},centreSafeMm:0,gutter:0,binding:'layflat',coverProfile:null,pages:{min:1,max:40,multiple:1}};
p.pages=Array.from({length:40},(_,i):Page=>({id:`spread-${i+1}`,side:'spread',widthMm:762,heightMm:304.8,background:'#ffffff',elements:[
 {type:'image',id:`panorama-${i}`,assetId:asset.id,rect:{x:-3,y:-3,w:768,h:310.8},crop:{x:0,y:0,w:1,h:1}},
 {type:'text',id:`mark-${i}`,text:`ENGINEERING PROOF - NOT LAB READY / SPREAD ${i+1}`,rect:{x:20,y:280,w:340,h:12},fontId:'Vera',fontSizeMm:4,color:'#ffffff'}]}));
await runDemo(40,p,'spread');
