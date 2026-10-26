"""Sample matched pixels in ICC-managed Lab; report CIEDE2000 without exclusions."""
import argparse,io,json,math
from pathlib import Path
from PIL import Image,ImageCms

def de2000(a,b):
    L1,a1,b1=a;L2,a2,b2=b
    C1=math.hypot(a1,b1);C2=math.hypot(a2,b2);C=(C1+C2)/2
    G=.5*(1-math.sqrt(C**7/(C**7+25**7)))
    ap1=(1+G)*a1;ap2=(1+G)*a2
    cp1=math.hypot(ap1,b1);cp2=math.hypot(ap2,b2)
    hp1=math.degrees(math.atan2(b1,ap1))%360;hp2=math.degrees(math.atan2(b2,ap2))%360
    dl=L2-L1;dc=cp2-cp1;dh=hp2-hp1
    if cp1*cp2==0:dh=0
    elif dh>180:dh-=360
    elif dh< -180:dh+=360
    dH=2*math.sqrt(cp1*cp2)*math.sin(math.radians(dh/2))
    lm=(L1+L2)/2;cm=(cp1+cp2)/2
    if cp1*cp2==0:hm=hp1+hp2
    elif abs(hp1-hp2)<=180:hm=(hp1+hp2)/2
    elif hp1+hp2<360:hm=(hp1+hp2+360)/2
    else:hm=(hp1+hp2-360)/2
    T=1-.17*math.cos(math.radians(hm-30))+.24*math.cos(math.radians(2*hm))+.32*math.cos(math.radians(3*hm+6))-.20*math.cos(math.radians(4*hm-63))
    sl=1+.015*(lm-50)**2/math.sqrt(20+(lm-50)**2);sc=1+.045*cm;sh=1+.015*cm*T
    rt=-2*math.sqrt(cm**7/(cm**7+25**7))*math.sin(math.radians(60*math.exp(-((hm-275)/25)**2)))
    return math.sqrt((dl/sl)**2+(dc/sc)**2+(dH/sh)**2+rt*(dc/sc)*(dH/sh))

assert abs(de2000((50,2.6772,-79.7751),(50,0,-82.7485))-2.0425)<.0001

def lab_image(path):
    with Image.open(path) as im:
        profile=im.info.get('icc_profile')
        if not profile:raise ValueError('Missing ICC')
        return ImageCms.profileToProfile(im,ImageCms.ImageCmsProfile(io.BytesIO(profile)),ImageCms.createProfile('LAB'),outputMode='LAB')

def compare(first,second):
    with lab_image(first) as a,lab_image(second) as b:
        if a.size!=b.size:raise ValueError('Mismatched raster dimensions')
        values=[]
        # Deterministic grid of actual pixels; no resizing or edge exclusion.
        stride=max(1,min(a.size)//200)
        for y in range(0,a.height,stride):
            for x in range(0,a.width,stride):
                p=a.getpixel((x,y));q=b.getpixel((x,y))
                values.append(de2000((p[0]*100/255,p[1]-128,p[2]-128),(q[0]*100/255,q[1]-128,q[2]-128)))
        values.sort();mean=sum(values)/len(values)
        return {'samples':len(values),'stridePixels':stride,'meanDeltaE2000':mean,'p95DeltaE2000':values[int(.95*(len(values)-1))],'maxDeltaE2000':values[-1],'passed':mean<=2,'exclusions':[]}

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('reference',type=Path);p.add_argument('candidate',type=Path);p.add_argument('report',type=Path);a=p.parse_args()
    r=compare(a.reference,a.candidate);a.report.write_text(json.dumps(r,indent=2));print(json.dumps(r));raise SystemExit(0 if r['passed'] else 1)
