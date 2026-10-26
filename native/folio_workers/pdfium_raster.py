"""One-page subprocess: exact pixel rasterization and bounded cancellation lifetime."""
import argparse
import pypdfium2 as pdfium
import pypdfium2.raw as raw
from PIL import ImageCms

def rasterize(source,index,width,height,target,ppi):
    if not 1<=width<=30000 or not 1<=height<=30000 or width*height>90_000_000:raise ValueError('Raster dimensions exceed limit')
    with pdfium.PdfDocument(source) as document:
        page=document[index]
        try:
            bitmap=pdfium.PdfBitmap.new_native(width,height,raw.FPDFBitmap_BGR,rev_byteorder=True)
            try:
                raw.FPDFBitmap_FillRect(bitmap,0,0,width,height,0xffffffff)
                raw.FPDF_RenderPageBitmap(bitmap,page,0,0,width,height,0,raw.FPDF_REVERSE_BYTE_ORDER|raw.FPDF_PRINTING)
                with bitmap.to_pil() as image:
                    image.save(target,'JPEG',quality=95,subsampling=0,dpi=(ppi,ppi),icc_profile=ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes())
            finally:bitmap.close()
        finally:page.close()

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('source');parser.add_argument('index',type=int);parser.add_argument('width',type=int);parser.add_argument('height',type=int);parser.add_argument('target');parser.add_argument('ppi',type=int);a=parser.parse_args()
    rasterize(a.source,a.index,a.width,a.height,a.target,a.ppi)
