//! Rust JPEG/PNG decoding spike with optional EXIF orientation handling.
//! ICC conversion and HEIC parity remain required before production use.
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};
use std::{io::Cursor, path::Path};

pub struct OrientedImage {
    pub pixels: DynamicImage,
    /// Retain for subsequent color conversion; samples are NOT yet sRGB-normalized.
    pub icc_profile: Option<Vec<u8>>,
}

pub fn verified_oriented(root: &Path, relative: &str, sha256: &str,
    max_encoded_bytes: u64, max_pixels: u64) -> Result<OrientedImage, String> {
    let bytes = crate::source_path::read_verified(root, relative, sha256, max_encoded_bytes)?;
    oriented(&bytes, max_pixels)
}

pub fn oriented(bytes: &[u8], max_pixels: u64) -> Result<OrientedImage, String> {
    decode(bytes, max_pixels, true)
}

pub fn verified_raw(root: &Path, relative: &str, sha256: &str,
    max_encoded_bytes: u64, max_pixels: u64) -> Result<DynamicImage, String> {
    let bytes = crate::source_path::read_verified(root, relative, sha256, max_encoded_bytes)?;
    raw(&bytes, max_pixels)
}

pub fn raw(bytes: &[u8], max_pixels: u64) -> Result<DynamicImage, String> {
    decode(bytes, max_pixels, false).map(|image| image.pixels)
}

fn decode(bytes: &[u8], max_pixels: u64, apply_orientation: bool) -> Result<OrientedImage, String> {
    if max_pixels == 0 { return Err("Pixel budget must be positive".into()); }
    let format = image::guess_format(bytes).map_err(|e| e.to_string())?;
    if !matches!(format, ImageFormat::Jpeg | ImageFormat::Png) {
        return Err("Raw spike decoder supports JPEG and PNG only".into());
    }
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    let mut limits = Limits::default();
    limits.max_alloc = Some(512 * 1024 * 1024);
    reader.limits(limits.clone());
    let mut decoder = reader.into_decoder().map_err(|e| e.to_string())?;
    let dimensions = decoder.dimensions();
    if dimensions.0 == 0 || dimensions.1 == 0
        || u64::from(dimensions.0) * u64::from(dimensions.1) > max_pixels {
        return Err("Image exceeds pixel budget".into());
    }
    limits.max_image_width = Some(dimensions.0);
    limits.max_image_height = Some(dimensions.1);
    // Mirror ImageReader::decode's output-buffer reservation before allocation.
    // This is not a promise about whole-process peak RSS.
    limits.reserve(decoder.total_bytes()).map_err(|e| e.to_string())?;
    decoder.set_limits(limits).map_err(|e| e.to_string())?;
    let orientation = if apply_orientation { Some(decoder.orientation().map_err(|e| e.to_string())?) }
        else { None };
    let icc_profile = if apply_orientation { decoder.icc_profile().map_err(|e| e.to_string())? }
        else { None };
    let mut pixels = DynamicImage::from_decoder(decoder).map_err(|e| e.to_string())?;
    if let Some(orientation) = orientation { pixels.apply_orientation(orientation); }
    Ok(OrientedImage { pixels, icc_profile })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jpeg_exif_rotation_is_applied_to_decoded_samples() {
        let source = image::RgbImage::from_fn(7, 5, |x,y|
            image::Rgb([x as u8 * 30, y as u8 * 40, 19]));
        let mut encoded = Cursor::new(Vec::new());
        DynamicImage::ImageRgb8(source).write_to(&mut encoded, ImageFormat::Jpeg).unwrap();
        let jpeg = encoded.into_inner();
        let original = raw(&jpeg, 35).unwrap().to_rgb8();
        for orientation in 1..=8u8 {
            // Independent coordinate oracle for all eight EXIF orientations.
            let mut exif = b"Exif\0\0II\x2a\0\x08\0\0\0\x01\0\x12\x01\x03\0\x01\0\0\0\x01\0\0\0\0\0\0\0".to_vec();
            exif[24] = orientation;
            let mut tagged = jpeg[..2].to_vec();
            tagged.extend_from_slice(&[0xff,0xe1]);
            tagged.extend_from_slice(&((exif.len()+2) as u16).to_be_bytes());
            tagged.extend_from_slice(&exif);
            tagged.extend_from_slice(&jpeg[2..]);
            let result = oriented(&tagged, 35).unwrap();
            let actual = result.pixels.to_rgb8();
            let (w,h) = original.dimensions();
            assert_eq!(actual.dimensions(), if orientation >= 5 {(h,w)} else {(w,h)});
            for (x,y,pixel) in original.enumerate_pixels() {
                let (dx,dy) = match orientation {
                    1 => (x,y), 2 => (w-1-x,y), 3 => (w-1-x,h-1-y),
                    4 => (x,h-1-y), 5 => (y,x), 6 => (h-1-y,x),
                    7 => (h-1-y,w-1-x), 8 => (y,w-1-x), _ => unreachable!(),
                };
                assert_eq!(actual.get_pixel(dx,dy), pixel, "orientation {orientation}");
            }
            assert!(result.icc_profile.is_none());
        }
    }
    #[test]
    fn synthetic_png_preserves_samples_and_enforces_budget() {
        let source = image::RgbaImage::from_fn(7, 5, |x,y|
            image::Rgba([x as u8 * 30, y as u8 * 40, 19, 127]));
        let mut encoded = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(source.clone()).write_to(&mut encoded, ImageFormat::Png).unwrap();
        let bytes = encoded.into_inner();
        assert_eq!(raw(&bytes, 35).unwrap().to_rgba8(), source);
        assert!(raw(&bytes, 34).is_err());
        assert!(raw(&bytes[..12], 35).is_err());
        assert!(raw(b"not an image", 35).is_err());
    }
}
