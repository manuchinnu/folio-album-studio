//! Tagged JPEG proof encoding; layout rendering and production approval are separate.
use image::codecs::jpeg::JpegEncoder;

/// Encode already-normalized opaque RGB8. Never implicitly flatten alpha or
/// reduce source bit depth. Caller owns explicit rendering/downsampling policy.
pub fn encode(pixels: &image::RgbImage, icc: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    if !(95..=100).contains(&quality) { return Err("Proof JPEG quality must be 95..100".into()); }
    if pixels.width() == 0 || pixels.height() == 0 { return Err("Empty proof image".into()); }
    // APP2 length includes its own two bytes, 12-byte signature and two counters.
    const CHUNK: usize = 65519;
    let parts = icc.len().div_ceil(CHUNK);
    if parts == 0 || parts > 255 { return Err("ICC exceeds JPEG APP2 capacity".into()); }
    let profile = lcms2::Profile::new_icc(icc).map_err(|e| format!("Invalid output ICC: {e}"))?;
    if profile.color_space() != lcms2::ColorSpaceSignature::RgbData {
        return Err("RGB proof requires an RGB output profile".into());
    }
    let mut jpeg = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut jpeg, quality);
    encoder.set_pixel_density(image::codecs::jpeg::PixelDensity::dpi(300));
    encoder.encode(
        pixels.as_raw(), pixels.width(), pixels.height(), image::ExtendedColorType::Rgb8)
        .map_err(|e| e.to_string())?;
    // Insert metadata in the existing compressed buffer instead of retaining
    // a second complete JPEG allocation for large album units.
    let metadata_size = icc.len() + parts * 18;
    let old_len = jpeg.len();
    let new_len = old_len.checked_add(metadata_size).ok_or("JPEG size overflow")?;
    jpeg.try_reserve_exact(metadata_size).map_err(|_| "Cannot allocate JPEG metadata")?;
    jpeg.resize(new_len, 0);
    jpeg.copy_within(2..old_len, 2 + metadata_size);
    let mut offset = 2;
    for (index, data) in icc.chunks(CHUNK).enumerate() {
        let segment = &mut jpeg[offset..offset + data.len() + 18];
        segment[..2].copy_from_slice(&[0xff, 0xe2]);
        segment[2..4].copy_from_slice(&((data.len() + 16) as u16).to_be_bytes());
        segment[4..16].copy_from_slice(b"ICC_PROFILE\0");
        segment[16..18].copy_from_slice(&[(index + 1) as u8, parts as u8]);
        segment[18..].copy_from_slice(data);
        offset += segment.len();
    }
    Ok(jpeg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageDecoder;
    #[test]
    fn multipart_icc_survives_in_buffer_insertion() {
        // Synthetic profile with trailing reserved space exercises APP2 split.
        let mut icc = lcms2::Profile::new_srgb().icc().unwrap();
        icc.resize(70_000, 0);
        icc[..4].copy_from_slice(&70_000u32.to_be_bytes());
        let pixels = image::RgbImage::from_pixel(8,8,image::Rgb([80,90,100]));
        let encoded = encode(&pixels,&icc,95).unwrap();
        let mut decoder = image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(&encoded)).unwrap();
        assert_eq!(decoder.icc_profile().unwrap().unwrap(),icc);
        assert_eq!(image::load_from_memory(&encoded).unwrap().width(),8);
        let mut offset = 2;
        for (sequence, size) in [(1,65519usize),(2,4481usize)] {
            assert_eq!(&encoded[offset..offset+2], &[0xff,0xe2]);
            assert_eq!(&encoded[offset+16..offset+18], &[sequence,2]);
            assert_eq!(u16::from_be_bytes([encoded[offset+2],encoded[offset+3]]) as usize,size+16);
            offset += size+18;
        }
        assert_eq!(&encoded[offset..offset+2], &[0xff,0xe0]);
    }
    #[test]
    fn jpeg_roundtrip_keeps_dimensions_and_exact_icc() {
        let pixels = image::RgbImage::from_pixel(31,17,image::Rgb([40,120,200]));
        let icc = lcms2::Profile::new_srgb().icc().unwrap();
        let bytes = encode(&pixels,&icc,95).unwrap();
        // Inspect the encoded marker fields independently of image's decoder.
        let mut pos = 2;
        let (mut density, mut sampling) = (false, false);
        while pos + 4 <= bytes.len() {
            assert_eq!(bytes[pos], 0xff);
            let marker = bytes[pos+1];
            if marker == 0xda { break; }
            let length = u16::from_be_bytes([bytes[pos+2],bytes[pos+3]]) as usize;
            let data = &bytes[pos+4..pos+2+length];
            if marker == 0xe0 && data.starts_with(b"JFIF\0") {
                assert_eq!(data[7],1);
                assert_eq!(&data[8..12], &[1,44,1,44]);
                density = true;
            }
            if marker == 0xc0 {
                assert_eq!(data[5],3);
                for component in data[6..].chunks_exact(3) { assert_eq!(component[1],0x11); }
                sampling = true;
            }
            pos += length + 2;
        }
        assert!(density && sampling, "300 DPI and 4:4:4 are required");
        let mut decoder = image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(&bytes)).unwrap();
        assert_eq!(decoder.dimensions(),(31,17));
        assert_eq!(decoder.icc_profile().unwrap().unwrap(),icc);
        let decoded = image::load_from_memory(&bytes).unwrap().to_rgb8();
        for p in decoded.pixels() { for c in 0..3 {
            assert!(p[c].abs_diff(pixels.get_pixel(0,0)[c]) <= 3);
        }}
        assert!(encode(&pixels,&icc,94).is_err());
        assert!(encode(&pixels,b"bad",95).is_err());
        let gray = lcms2::Profile::new_gray(&lcms2::CIExyY { x:0.3127,y:0.3290,Y:1.0 },
            &lcms2::ToneCurve::new(2.2)).unwrap().icc().unwrap();
        assert!(encode(&pixels,&gray,95).is_err());
    }
}
