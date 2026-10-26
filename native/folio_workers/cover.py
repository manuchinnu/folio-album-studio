"""Defensive cover geometry at the native boundary, independent of TS checks."""
def cover_geometry(project):
    lab=project['lab'];profile=lab['coverProfile'];content=project['cover']
    if profile is None:
        if content is not None:raise ValueError('Cover content without cover profile')
        return None
    if content is None:raise ValueError('Photo cover profile requires cover content')
    physical=len(project['pages'])*(2 if lab['exportUnit']=='spread' else 1)
    if physical%2:raise ValueError('Cover requires even physical page count')
    if profile['spineModel']=='lab_table':
        rows=[r for r in profile['entries'] if r['sheets']==physical//2]
        if len(rows)!=1:raise ValueError('Missing or duplicate lab cover table entry')
        row=rows[0];wrap=row['wrap'];width=row['frontWidthMm']+row['backWidthMm']+row['spineMm'];height=row['heightMm'];bleed=row['bleed']
    else:
        wrap=profile['wrap'];width=lab['trim']['w']*(1 if lab['exportUnit']=='spread' else 2)
        width+=0 if lab['binding']=='spiral' else physical/2*lab['spine']['thicknessPerLeafMm']+lab['spine']['fixedMm']
        height=lab['trim']['h'];bleed=profile['bleed']
    width+=wrap['left']+wrap['right'];height+=wrap['top']+wrap['bottom']
    if abs(content['widthMm']-width)>.001 or abs(content['heightMm']-height)>.001:raise ValueError('Cover does not match profile geometry')
    return {'bleed':bleed}
