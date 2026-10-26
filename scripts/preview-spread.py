"""Review-only overlay; never modifies the print JPEG."""
import argparse
from pathlib import Path
from PIL import Image,ImageDraw
parser=argparse.ArgumentParser();parser.add_argument('directory',type=Path);args=parser.parse_args()
with Image.open(args.directory/'01.jpg') as original:
    original.thumbnail((1800,900));preview=original.convert('RGB')
draw=ImageDraw.Draw(preview);scale=preview.width/768
centre=preview.width/2
draw.line((centre,0,centre,preview.height),fill='#ff4040',width=2)
draw.rectangle((8,8,660,38),fill='#ffffff')
draw.text((14,16),'REVIEW OVERLAY ONLY: red = fold (layflat continuous canvas)',fill='#000000')
target=args.directory/'review-spread-overlay.jpg';preview.save(target,quality=85)
assert target.stat().st_size<=1_000_000
print(target.resolve())
