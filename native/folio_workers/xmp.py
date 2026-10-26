"""Bounded XMP extraction with DTD, entity and external resource access disabled."""
from defusedxml.ElementTree import fromstring

FIELDS={'DateTimeOriginal','CreateDate','GPSLatitude','GPSLongitude','GPSAltitude'}
NAMESPACES={'http://ns.adobe.com/exif/1.0/','http://ns.adobe.com/xap/1.0/'}
def extract_xmp(raw):
    if isinstance(raw,str):raw=raw.encode('utf-8')
    if len(raw)>1_048_576:raise ValueError('XMP exceeds 1 MiB')
    root=fromstring(raw,forbid_dtd=True,forbid_entities=True,forbid_external=True)
    fields=[]
    def add(tag,value):
        if not tag.startswith('{'):return
        namespace,name=tag[1:].split('}',1)
        if namespace in NAMESPACES and name in FIELDS and value is not None:
            if len(value)>4096:raise ValueError('XMP field exceeds length limit')
            fields.append({'name':name,'raw':value})
            if len(fields)>100:raise ValueError('XMP has too many metadata fields')
    def walk(node,depth=0):
        if depth>20:raise ValueError('XMP nesting exceeds limit')
        add(node.tag,node.text)
        for key,value in node.attrib.items():add(key,value)
        for child in node:walk(child,depth+1)
    walk(root)
    return fields
