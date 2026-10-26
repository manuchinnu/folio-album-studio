"""Independent mixed-size audit. Expectations must be a reviewed input file.

Never read expected geometry from renderer logs or generated output manifests.
Retains the reference PDF ICC, font embedding and effective-PPI checks.
"""
import argparse
import importlib.util
import json
import math
from pathlib import Path
from PIL import Image

spec = importlib.util.spec_from_file_location('reference_audit', Path(__file__).with_name('inspect-pdf.py'))
reference = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reference)


def validate_config(config):
    if set(config) != {'units'} or not config['units']:
        raise ValueError('Expected a nonempty units list')
    names = set()
    for unit in config['units']:
        if set(unit) != {'jpeg', 'pixels', 'mediaBox', 'trimBox', 'bleedBox'}:
            raise ValueError('Unexpected unit fields')
        name = unit['jpeg']
        if not isinstance(name, str) or Path(name).name != name or not name.endswith('.jpg') or name in names:
            raise ValueError('JPEG names must be unique basenames')
        names.add(name)
        if len(unit['pixels']) != 2 or any(type(v) is not int or v <= 0 for v in unit['pixels']):
            raise ValueError('Pixels must be positive integers')
        for key in ('mediaBox', 'trimBox', 'bleedBox'):
            box = unit[key]
            if len(box) != 4 or any(type(v) not in (int, float) or not math.isfinite(v) for v in box):
                raise ValueError('Invalid PDF box')
            if box[2] <= box[0] or box[3] <= box[1]:
                raise ValueError('Empty PDF box')


def geometry_errors(page, pixels, expected):
    errors = []
    if list(pixels) != expected['pixels']:
        errors.append('Wrong JPEG size ' + expected['jpeg'])
    for key in ('mediaBox', 'trimBox', 'bleedBox'):
        actual = page[key]
        if len(actual) != 4 or any(not math.isfinite(x) or abs(x-y) > .01 for x, y in zip(actual, expected[key])):
            errors.append('PDF box mismatch ' + key + ' ' + expected['jpeg'])
    return errors


def run(directory, config):
    validate_config(config)
    pdf = reference.audit(directory / 'interior.pdf')
    errors = list(pdf['errors'])
    units = config['units']
    if len(pdf['pages']) != len(units):
        errors.append('Wrong interior count')
    actual_names = {p.name for p in directory.glob('*.jpg') if not p.name.startswith('review-')}
    if actual_names != {u['jpeg'] for u in units}:
        errors.append('Incorrect JPEG inventory')
    for page, unit in zip(pdf['pages'], units):
        path = directory / unit['jpeg']
        if not path.exists():
            continue  # inventory failure already recorded
        with Image.open(path) as image:
            errors.extend(geometry_errors(page, image.size, unit))
            if not image.info.get('icc_profile'):
                errors.append('JPEG lacks ICC ' + unit['jpeg'])
    return {'pdf': pdf, 'errors': errors, 'passed': not errors}


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('directory', type=Path)
    parser.add_argument('--expectations', type=Path, required=True)
    args = parser.parse_args()
    report = run(args.directory, json.loads(args.expectations.read_text(encoding='utf-8')))
    (args.directory / 'mixed-independent-audit.json').write_text(json.dumps(report, indent=2), encoding='utf-8')
    print(json.dumps({'passed': report['passed'], 'errors': report['errors']}))
    raise SystemExit(0 if report['passed'] else 1)
