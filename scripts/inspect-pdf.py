"""Independent output audit: reads PDF operators/resources, not renderer metadata."""
import argparse,json,math
from pathlib import Path
import pikepdf
from PIL import Image

def multiply(a,b):
    return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])

def audit(path):
    result={'file':path.name,'pages':[],'errors':[]}
    with pikepdf.open(path) as pdf:
        intents=pdf.Root.get('/OutputIntents',[])
        result['outputIntents']=[{'identifier':str(i.get('/OutputConditionIdentifier','')),'iccBytes':len(i.DestOutputProfile.read_bytes())} for i in intents]
        if not intents:result['errors'].append('No output intent')
        for number,page in enumerate(pdf.pages,1):
            entry={'number':number,'mediaBox':[float(x) for x in page.MediaBox],'trimBox':[float(x) for x in page.TrimBox],'bleedBox':[float(x) for x in page.BleedBox],'fonts':[],'images':[]}
            for key,font in page.Resources.get('/Font',{}).items():
                descriptors=[]
                if '/FontDescriptor' in font:descriptors.append(font.FontDescriptor)
                for descendant in font.get('/DescendantFonts',[]):
                    if '/FontDescriptor' in descendant:descriptors.append(descendant.FontDescriptor)
                embedded=any(any(k in descriptor for k in ['/FontFile','/FontFile2','/FontFile3']) for descriptor in descriptors)
                entry['fonts'].append({'resource':str(key),'name':str(font.get('/BaseFont','')),'embedded':embedded})
                if not embedded:result['errors'].append(f'Page {number}: font not embedded {key}')
            state=(1,0,0,1,0,0);stack=[]
            for instruction in pikepdf.parse_content_stream(page):
                operands,operator=instruction;op=str(operator)
                if op=='q':stack.append(state)
                elif op=='Q':state=stack.pop()
                elif op=='cm':state=multiply(state,tuple(float(v) for v in operands))
                elif op=='Do':
                    name=operands[0];image=page.Resources.get('/XObject',{}).get(name)
                    if image is None or image.get('/Subtype')!=pikepdf.Name.Image:continue
                    width_pt=math.hypot(state[0],state[1]);height_pt=math.hypot(state[2],state[3]);ppi=min(int(image.Width)*72/width_pt,int(image.Height)*72/height_pt)
                    cs=image.get('/ColorSpace');icc=isinstance(cs,pikepdf.Array) and str(cs[0])=='/ICCBased' and len(cs[1].read_bytes())>0
                    entry['images'].append({'resource':str(name),'width':int(image.Width),'height':int(image.Height),'placedWidthPt':width_pt,'placedHeightPt':height_pt,'effectivePpi':ppi,'iccBased':icc})
                    if not icc:result['errors'].append(f'Page {number}: image missing ICC')
                    if ppi<199.9:result['errors'].append(f'Page {number}: red-DPI image')
            result['pages'].append(entry)
    return result

def main():
    parser=argparse.ArgumentParser();parser.add_argument('directory',type=Path);parser.add_argument('--expected-pages',type=int,required=True);parser.add_argument('--square-layflat',action='store_true');parser.add_argument('--spread-12x15',action='store_true');args=parser.parse_args()
    audits=[audit(args.directory/name) for name in (['interior.pdf'] if args.spread_12x15 else ['interior.pdf','cover.pdf'])]
    errors=[e for a in audits for e in a['errors']]
    if len(audits[0]['pages'])!=args.expected_pages:errors.append('Wrong interior count')
    # Synthetic-profile expectations are specified independently from TS geometry.
    trim_w,trim_h=(762,304.8) if args.spread_12x15 else ((305,305) if args.square_layflat else (210,297))
    unit=72/25.4;expected_media=[0,0,(trim_w+6)*unit,(trim_h+6)*unit];expected_trim=[3*unit,3*unit,(trim_w+3)*unit,(trim_h+3)*unit]
    for page in audits[0]['pages']:
        for actual,expected in [(page['mediaBox'],expected_media),(page['trimBox'],expected_trim),(page['bleedBox'],expected_media)]:
            if any(abs(x-y)>.01 for x,y in zip(actual,expected)):errors.append('Interior PDF box mismatch')
    cover_width=610 if args.square_layflat else 420+args.expected_pages/2*.25+1
    if not args.spread_12x15 and abs(audits[1]['pages'][0]['trimBox'][2]-audits[1]['pages'][0]['trimBox'][0]-cover_width*unit)>.01:errors.append('Cover spine width incorrect')
    jpegs=[]
    for file in sorted(f for f in args.directory.glob('*.jpg') if not f.name.startswith('review-')):
        with Image.open(file) as image:
            expected=(round((cover_width+6)/25.4*300),round((trim_h+6)/25.4*300)) if file.name=='cover.jpg' else (round((trim_w+6)/25.4*300),round((trim_h+6)/25.4*300))
            if image.size!=expected:errors.append('Wrong JPEG size '+file.name)
            if not image.info.get('icc_profile'):errors.append('JPEG lacks ICC '+file.name)
            jpegs.append({'file':file.name,'pixels':image.size,'iccBytes':len(image.info.get('icc_profile',b''))})
    if len(jpegs)!=args.expected_pages+(0 if args.spread_12x15 else 1):errors.append('Wrong JPEG count')
    if args.spread_12x15 and [j['file'] for j in jpegs]!=[f'{i:02d}.jpg' for i in range(1,args.expected_pages+1)]:errors.append('Incorrect spread filenames')
    (args.directory/'jpeg-listing.json').write_text(json.dumps(jpegs,indent=2),encoding='utf-8')
    report={'pdfs':audits,'jpegs':jpegs,'errors':errors,'passed':not errors}
    (args.directory/'independent-audit.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
    print(json.dumps({'passed':not errors,'errors':errors,'interiorPages':len(audits[0]['pages']),'jpegCount':len(jpegs),'minimumEmbeddedPpi':min(i['effectivePpi'] for a in audits for p in a['pages'] for i in p['images'])},indent=2))
    if errors:raise SystemExit(1)

if __name__=='__main__':main()
