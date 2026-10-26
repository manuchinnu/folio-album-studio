"""Negative controls for independently supplied mixed geometry expectations."""
import importlib.util
from pathlib import Path
import unittest
import tempfile
import pikepdf
from PIL import Image, ImageCms

spec = importlib.util.spec_from_file_location('mixed', Path(__file__).with_name('audit-mixed.py'))
mixed = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mixed)


class MixedAuditTests(unittest.TestCase):
    def test_real_files_inventory_icc_and_pixel_mutations(self):
        # Tests audit I/O only; blank PDF pages are not renderer evidence.
        scratch = Path(__file__).resolve().parents[1] / 'artifacts'
        scratch.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='audit-test-', dir=scratch) as temp:
            directory = Path(temp)
            icc = ImageCms.ImageCmsProfile(ImageCms.createProfile('sRGB')).tobytes()
            pdf = pikepdf.Pdf.new()
            profile = pdf.make_stream(icc)
            profile.N = 3
            pdf.Root.OutputIntents = pikepdf.Array([pikepdf.Dictionary(
                Type=pikepdf.Name.OutputIntent, S=pikepdf.Name.GTS_PDFA1,
                OutputConditionIdentifier='sRGB', DestOutputProfile=profile)])
            units = []
            for number, width in enumerate((40, 79, 40), 1):
                box = [0, 0, width * 72 / 300, 30 * 72 / 300]
                page = pdf.add_blank_page(page_size=(box[2], box[3]))
                page.TrimBox = pikepdf.Array(box)
                page.BleedBox = pikepdf.Array(box)
                name = f'{number:02d}.jpg'
                Image.new('RGB', (width, 30), 'white').save(directory / name, icc_profile=icc)
                units.append(dict(jpeg=name, pixels=[width, 30], mediaBox=box, trimBox=box, bleedBox=box))
            pdf.save(directory / 'interior.pdf')
            pdf.close()
            config = {'units': units}
            self.assertTrue(mixed.run(directory, config)['passed'])
            Image.new('RGB', (80, 30)).save(directory / '02.jpg', icc_profile=icc)
            self.assertIn('Wrong JPEG size 02.jpg', mixed.run(directory, config)['errors'])
            Image.new('RGB', (79, 30)).save(directory / '02.jpg')
            self.assertIn('JPEG lacks ICC 02.jpg', mixed.run(directory, config)['errors'])
            (directory / '02.jpg').unlink()
            self.assertIn('Incorrect JPEG inventory', mixed.run(directory, config)['errors'])

    def test_one_pixel_error_fails_for_single_and_spread(self):
        for width in (4394, 8787):
            box = [0, 0, width * 72 / 300, 3520 * 72 / 300]
            expected = dict(jpeg='01.jpg', pixels=[width, 3520], mediaBox=box, trimBox=box, bleedBox=box)
            page = {key: box[:] for key in ('mediaBox', 'trimBox', 'bleedBox')}
            mixed.validate_config({'units': [expected]})
            self.assertEqual(mixed.geometry_errors(page, (width, 3520), expected), [])
            self.assertTrue(mixed.geometry_errors(page, (width + 1, 3520), expected))
            page['trimBox'][2] += 72 / 300
            self.assertTrue(mixed.geometry_errors(page, (width, 3520), expected))

    def test_invalid_expectations_rejected(self):
        with self.assertRaises(ValueError):
            mixed.validate_config({'units': []})


if __name__ == '__main__':
    unittest.main()
