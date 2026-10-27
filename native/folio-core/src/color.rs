//! ICC conversion preserving supported sample depth and straight alpha.
use lcms2::{Intent, PixelFormat, Profile, Transform};

/// Small diagnostic samples for independent parity tooling, never full albums.
pub fn sample_values(input: &crate::decode::OrientedImage) -> serde_json::Value {
    if u64::from(input.pixels.width()) * u64::from(input.pixels.height()) > 4096 {
        return serde_json::Value::Null;
    }
    match &input.pixels {
        image::DynamicImage::ImageRgb8(p) => serde_json::json!(p.as_raw()),
        image::DynamicImage::ImageRgba8(p) => serde_json::json!(p.as_raw()),
        image::DynamicImage::ImageRgb16(p) => serde_json::json!(p.as_raw()),
        image::DynamicImage::ImageRgba16(p) => serde_json::json!(p.as_raw()),
        _ => serde_json::Value::Null,
    }
}

/// Dispatch without discarding alpha or reducing bit depth implicitly.
pub fn normalize_tagged(input: crate::decode::OrientedImage)
    -> Result<crate::decode::OrientedImage, String> {
    if matches!(&input.pixels, image::DynamicImage::ImageLuma16(_) | image::DynamicImage::ImageLumaA16(_)) {
        return normalize_gray16(input);
    }
    if matches!(&input.pixels, image::DynamicImage::ImageRgb16(_) | image::DynamicImage::ImageRgba16(_)) {
        return normalize_rgb16(input);
    }
    let (pixels, profile) = match &input.pixels {
        image::DynamicImage::ImageRgb8(_) => {
            let (pixels, profile) = normalize_rgb8(input)?;
            (image::DynamicImage::ImageRgb8(pixels), profile)
        },
        image::DynamicImage::ImageRgba8(_) => {
            let (pixels, profile) = normalize_rgba8(input)?;
            (image::DynamicImage::ImageRgba8(pixels), profile)
        },
        image::DynamicImage::ImageLuma8(_) => {
            let (pixels, profile) = normalize_gray8(input)?;
            (image::DynamicImage::ImageRgb8(pixels), profile)
        },
        image::DynamicImage::ImageLumaA8(_) => {
            let (pixels, profile) = normalize_gray_alpha8(input)?;
            (image::DynamicImage::ImageRgba8(pixels), profile)
        },
        _ => return Err("Sample format requires an additional color conversion path".into()),
    };
    Ok(crate::decode::OrientedImage { pixels, icc_profile: Some(profile) })
}

/// Preserve gray sample precision and optional straight alpha through ICC.
pub fn normalize_gray16(input: crate::decode::OrientedImage)
    -> Result<crate::decode::OrientedImage, String> {
    let icc = input.icc_profile.as_ref().ok_or("Untagged image requires an explicit source profile")?;
    let source = Profile::new_icc(icc).map_err(|e| e.to_string())?;
    let output_icc = Profile::new_srgb().icc().map_err(|e| e.to_string())?;
    let target = Profile::new_icc(&output_icc).map_err(|e| e.to_string())?;
    let transform: Transform<u16, [u16; 3]> = Transform::new_flags(
        &source, PixelFormat::GRAY_16, &target, PixelFormat::RGB_16,
        Intent::RelativeColorimetric, lcms2::Flags::NO_OPTIMIZE).map_err(|e| e.to_string())?;
    let (samples, channels): (&[u16], usize) = match &input.pixels {
        image::DynamicImage::ImageLuma16(p) => (p.as_raw(), 1),
        image::DynamicImage::ImageLumaA16(p) => (p.as_raw(), 2),
        _ => return Err("Expected Gray16 or GrayAlpha16".into()),
    };
    let output_channels = channels + 2;
    let count = (samples.len() / channels).checked_mul(output_channels).ok_or("RGB size overflow")?;
    let mut output = Vec::new();
    output.try_reserve_exact(count).map_err(|_| "Cannot allocate gray16 conversion")?;
    output.resize(count, 0u16);
    let mut gray = [0u16; 1024];
    let mut rgb = [[0u16; 3]; 1024];
    for (src, dst) in samples.chunks(1024 * channels).zip(output.chunks_mut(1024 * output_channels)) {
        let count = src.len() / channels;
        for (g,p) in gray[..count].iter_mut().zip(src.chunks_exact(channels)) { *g = p[0]; }
        transform.transform_pixels(&gray[..count], &mut rgb[..count]);
        for ((out,color),raw) in dst.chunks_exact_mut(output_channels).zip(&rgb[..count]).zip(src.chunks_exact(channels)) {
            out[..3].copy_from_slice(color);
            if channels == 2 { out[3] = raw[1]; }
        }
    }
    let (w,h) = (input.pixels.width(), input.pixels.height());
    let pixels = if channels == 1 {
        image::DynamicImage::ImageRgb16(image::ImageBuffer::from_raw(w,h,output).ok_or("Invalid RGB length")?)
    } else {
        image::DynamicImage::ImageRgba16(image::ImageBuffer::from_raw(w,h,output).ok_or("Invalid RGBA length")?)
    };
    Ok(crate::decode::OrientedImage { pixels, icc_profile: Some(output_icc) })
}

/// Native-endian 16-bit RGB transform; alpha is never sent through the ICC LUT.
pub fn normalize_rgb16(mut input: crate::decode::OrientedImage)
    -> Result<crate::decode::OrientedImage, String> {
    let icc = input.icc_profile.as_ref().ok_or("Untagged image requires an explicit source profile")?;
    let source = Profile::new_icc(icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let output_icc = target.icc().map_err(|e| e.to_string())?;
    // Transform to the exact serialized profile shipped with the pixels.
    let target = Profile::new_icc(&output_icc).map_err(|e| e.to_string())?;
    let transform: Transform<[u16; 3], [u16; 3]> = Transform::new(
        &source, PixelFormat::RGB_16, &target, PixelFormat::RGB_16,
        Intent::RelativeColorimetric).map_err(|e| e.to_string())?;
    let (samples, channels): (&mut [u16], usize) = match &mut input.pixels {
        image::DynamicImage::ImageRgb16(pixels) => (pixels.as_mut(), 3),
        image::DynamicImage::ImageRgba16(pixels) => (pixels.as_mut(), 4),
        _ => return Err("RGB16 normalization requires RGB16 or RGBA16 samples".into()),
    };
    let mut batch = [[0u16; 3]; 1024];
    for chunk in samples.chunks_mut(1024 * channels) {
        let count = chunk.len() / channels;
        for (pixel, raw) in batch[..count].iter_mut().zip(chunk.chunks_exact(channels)) {
            pixel.copy_from_slice(&raw[..3]);
        }
        transform.transform_in_place(&mut batch[..count]);
        for (raw, pixel) in chunk.chunks_exact_mut(channels).zip(&batch[..count]) {
            raw[..3].copy_from_slice(pixel);
        }
    }
    input.icc_profile = Some(output_icc);
    Ok(input)
}

/// Transform grayscale through its ICC profile while retaining straight alpha.
pub fn normalize_gray_alpha8(input: crate::decode::OrientedImage)
    -> Result<(image::RgbaImage, Vec<u8>), String> {
    let icc = input.icc_profile.ok_or("Untagged image requires an explicit source profile")?;
    let pixels = match input.pixels {
        image::DynamicImage::ImageLumaA8(pixels) => pixels,
        _ => return Err("Gray-alpha8 normalization requires grayscale-alpha8 samples".into()),
    };
    let source = Profile::new_icc(&icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let output_icc = target.icc().map_err(|e| e.to_string())?;
    let transform: Transform<u8, [u8; 3]> = Transform::new(
        &source, PixelFormat::GRAY_8, &target, PixelFormat::RGB_8,
        Intent::RelativeColorimetric).map_err(|e| e.to_string())?;
    let count = pixels.as_raw().len().checked_mul(2).ok_or("RGBA size overflow")?;
    let mut output = Vec::new();
    output.try_reserve_exact(count).map_err(|_| "Cannot allocate gray-alpha conversion")?;
    output.resize(count, 0);
    let mut gray = [0u8; 1024];
    let mut rgb = [[0u8; 3]; 1024];
    for (src, dst) in pixels.as_raw().chunks(2048).zip(output.chunks_mut(4096)) {
        let count = src.len() / 2;
        for (sample, raw) in gray[..count].iter_mut().zip(src.chunks_exact(2)) {
            *sample = raw[0];
        }
        transform.transform_pixels(&gray[..count], &mut rgb[..count]);
        for ((out, color), raw) in dst.chunks_exact_mut(4).zip(&rgb[..count]).zip(src.chunks_exact(2)) {
            out[..3].copy_from_slice(color);
            out[3] = raw[1];
        }
    }
    let output = image::RgbaImage::from_raw(pixels.width(), pixels.height(), output)
        .ok_or("Invalid gray-alpha output length")?;
    Ok((output, output_icc))
}

/// Tagged grayscale needs a gray-input transform, not RGB expansion before ICC.
pub fn normalize_gray8(input: crate::decode::OrientedImage)
    -> Result<(image::RgbImage, Vec<u8>), String> {
    let icc = input.icc_profile.ok_or("Untagged image requires an explicit source profile")?;
    let pixels = match input.pixels {
        image::DynamicImage::ImageLuma8(pixels) => pixels,
        _ => return Err("Gray8 normalization requires grayscale8 samples".into()),
    };
    let source = Profile::new_icc(&icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let output_icc = target.icc().map_err(|e| e.to_string())?;
    let transform: Transform<u8, [u8; 3]> = Transform::new(
        &source, PixelFormat::GRAY_8, &target, PixelFormat::RGB_8,
        Intent::RelativeColorimetric).map_err(|e| e.to_string())?;
    let count = pixels.as_raw().len().checked_mul(3).ok_or("RGB size overflow")?;
    let mut output = Vec::new();
    output.try_reserve_exact(count).map_err(|_| "Cannot allocate grayscale conversion")?;
    output.resize(count, 0);
    let mut batch = [[0u8; 3]; 1024];
    for (src,dst) in pixels.as_raw().chunks(1024).zip(output.chunks_mut(3072)) {
        transform.transform_pixels(src, &mut batch[..src.len()]);
        for (raw,pixel) in dst.chunks_exact_mut(3).zip(&batch[..src.len()]) {
            raw.copy_from_slice(pixel);
        }
    }
    let output = image::RgbImage::from_raw(pixels.width(), pixels.height(), output)
        .ok_or("Invalid grayscale output length")?;
    Ok((output, output_icc))
}

/// Convert straight-alpha RGBA8 RGB channels while preserving alpha exactly.
/// Compositing against the album background is a separate renderer operation.
pub fn normalize_rgba8(input: crate::decode::OrientedImage)
    -> Result<(image::RgbaImage, Vec<u8>), String> {
    let icc = input.icc_profile.ok_or("Untagged image requires an explicit source profile")?;
    let mut pixels = match input.pixels {
        image::DynamicImage::ImageRgba8(pixels) => pixels,
        _ => return Err("RGBA8 normalization requires straight-alpha RGBA8 samples".into()),
    };
    let source = Profile::new_icc(&icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let output_icc = target.icc().map_err(|e| e.to_string())?;
    let transform: Transform<[u8; 3], [u8; 3]> = Transform::new(
        &source, PixelFormat::RGB_8, &target, PixelFormat::RGB_8,
        Intent::RelativeColorimetric).map_err(|e| e.to_string())?;
    let mut batch = [[0u8; 3]; 1024];
    for bytes in pixels.as_mut().chunks_mut(4096) {
        let count = bytes.len() / 4;
        for (pixel, raw) in batch[..count].iter_mut().zip(bytes.chunks_exact(4)) {
            pixel.copy_from_slice(&raw[..3]);
        }
        transform.transform_in_place(&mut batch[..count]);
        for (raw, pixel) in bytes.chunks_exact_mut(4).zip(&batch[..count]) {
            raw[..3].copy_from_slice(pixel);
        }
    }
    Ok((pixels, output_icc))
}

/// Normalize an oriented RGB8 image without making another full-size pixel
/// buffer. Other sample formats and untagged policy remain explicit work items.
pub fn normalize_rgb8(input: crate::decode::OrientedImage)
    -> Result<(image::RgbImage, Vec<u8>), String> {
    let icc = input.icc_profile.ok_or("Untagged image requires an explicit source profile")?;
    let mut pixels = match input.pixels {
        image::DynamicImage::ImageRgb8(pixels) => pixels,
        _ => return Err("RGB8 normalization cannot silently reduce other sample formats".into()),
    };
    let source = Profile::new_icc(&icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let output_icc = target.icc().map_err(|e| e.to_string())?;
    let transform: Transform<[u8; 3], [u8; 3]> = Transform::new(
        &source, PixelFormat::RGB_8, &target, PixelFormat::RGB_8,
        Intent::RelativeColorimetric).map_err(|e| e.to_string())?;
    let mut batch = [[0u8; 3]; 1024];
    for bytes in pixels.as_mut().chunks_mut(3072) {
        let count = bytes.len() / 3;
        for (pixel, raw) in batch[..count].iter_mut().zip(bytes.chunks_exact(3)) {
            pixel.copy_from_slice(raw);
        }
        transform.transform_in_place(&mut batch[..count]);
        for (raw, pixel) in bytes.chunks_exact_mut(3).zip(&batch[..count]) {
            raw.copy_from_slice(pixel);
        }
    }
    Ok((pixels, output_icc))
}

/// Convert RGB samples in place using relative colorimetric intent. Invalid or
/// incompatible profiles fail; never silently discard a supplied ICC profile.
pub fn rgb8_to_srgb(pixels: &mut [[u8; 3]], source_icc: &[u8]) -> Result<(), String> {
    let source = Profile::new_icc(source_icc).map_err(|e| format!("Invalid source ICC: {e}"))?;
    let target = Profile::new_srgb();
    let transform: Transform<[u8; 3], [u8; 3]> = Transform::new(
        &source, PixelFormat::RGB_8, &target, PixelFormat::RGB_8,
        Intent::RelativeColorimetric).map_err(|e| format!("Cannot convert source ICC: {e}"))?;
    transform.transform_in_place(pixels);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gray16_transfer_and_alpha_preserve_precision() {
        let profile = Profile::new_gray(&lcms2::CIExyY { x:0.3127, y:0.3290, Y:1.0 },
            &lcms2::ToneCurve::new(2.2)).unwrap().icc().unwrap();
        let pixels = image::ImageBuffer::from_fn(2051,1, |x,_|
            image::LumaA([(x * 65535 / 2050) as u16, (x * 31) as u16]));
        let result = normalize_tagged(crate::decode::OrientedImage {
            pixels: image::DynamicImage::ImageLumaA16(pixels.clone()), icc_profile: Some(profile.clone()) }).unwrap();
        let gray = image::ImageBuffer::from_fn(2051,1, |x,_| image::Luma([pixels.get_pixel(x,0)[0]]));
        let plain = normalize_tagged(crate::decode::OrientedImage {
            pixels: image::DynamicImage::ImageLuma16(gray.clone()), icc_profile: Some(profile) }).unwrap();
        let plain_rgb = plain.pixels.as_rgb16().unwrap();
        let rgb = result.pixels.as_rgba16().unwrap();
        for (x,_,p) in rgb.enumerate_pixels() {
            assert_eq!(&p.0[..3], &plain_rgb.get_pixel(x,0).0);
            assert_eq!(p[3], pixels.get_pixel(x,0)[1]);
            let linear = (f64::from(pixels.get_pixel(x,0)[0]) / 65535.0).powf(2.2);
            let expected = if linear <= 0.0031308 { linear * 12.92 }
                else { 1.055 * linear.powf(1.0/2.4) - 0.055 };
            for c in 0..3 { assert!((f64::from(p[c]) / 65535.0 - expected).abs() < 0.001); }
        }
        assert!(rgb.pixels().any(|p| p[0] % 257 != 0));
        for profile in [None, Some(b"invalid".to_vec()), Some(Profile::new_srgb().icc().unwrap())] {
            assert!(normalize_tagged(crate::decode::OrientedImage {
                pixels: image::DynamicImage::ImageLuma16(gray.clone()), icc_profile: profile }).is_err());
        }
    }
    #[test]
    fn rgb16_dispatch_retains_precision_and_alpha_across_batches() {
        let profile = Profile::new_srgb().icc().unwrap();
        let rgba = image::ImageBuffer::from_fn(257, 5, |x,y|
            image::Rgba([(x * 251) as u16, (y * 12345) as u16, 12345u16, (x * 239 + y) as u16]));
        let rgb = image::ImageBuffer::from_fn(257, 5, |x,y| {
            let p = rgba.get_pixel(x,y);
            image::Rgb([p[0],p[1],p[2]])
        });
        for original in [image::DynamicImage::ImageRgb16(rgb), image::DynamicImage::ImageRgba16(rgba)] {
            let result = normalize_tagged(crate::decode::OrientedImage {
                pixels: original.clone(), icc_profile: Some(profile.clone()) }).unwrap();
            assert_eq!(result.pixels.color(), original.color());
            let channels = original.color().channel_count() as usize;
            let before: &[u16] = if channels == 3 { original.as_rgb16().unwrap().as_raw() }
                else { original.as_rgba16().unwrap().as_raw() };
            let after: &[u16] = if channels == 3 { result.pixels.as_rgb16().unwrap().as_raw() }
                else { result.pixels.as_rgba16().unwrap().as_raw() };
            let max_error = before.iter().zip(after).enumerate().map(|(i,(a,b))| {
                if channels == 4 && i % 4 == 3 { assert_eq!(a,b); }
                a.abs_diff(*b)
            }).max().unwrap();
            // Serialized ICC curves/matrices are quantized; this is a numerical
            // identity control, not a bitwise bypass of color management.
            assert!(max_error <= 8, "16-bit identity max error: {max_error}");
            assert!(Profile::new_icc(&result.icc_profile.unwrap()).is_ok());
        }
    }
    #[test]
    fn gray_alpha_dispatch_preserves_alpha_and_matches_gray_color() {
        let profile = Profile::new_gray(&lcms2::CIExyY { x:0.3127, y:0.3290, Y:1.0 },
            &lcms2::ToneCurve::new(2.2)).unwrap().icc().unwrap();
        let pixels = image::GrayAlphaImage::from_fn(257, 5, |x,y|
            image::LumaA([(x + y * 19) as u8, x as u8]));
        let gray = image::GrayImage::from_fn(257, 5, |x,y| image::Luma([pixels.get_pixel(x,y)[0]]));
        let (expected, _) = normalize_gray8(crate::decode::OrientedImage {
            pixels: gray.into(), icc_profile: Some(profile.clone()) }).unwrap();
        let result = normalize_tagged(crate::decode::OrientedImage {
            pixels: pixels.clone().into(), icc_profile: Some(profile) }).unwrap();
        let rgba = result.pixels.as_rgba8().unwrap();
        for (x,y,pixel) in rgba.enumerate_pixels() {
            assert_eq!(&pixel.0[..3], &expected.get_pixel(x,y).0);
            assert_eq!(pixel[3], pixels.get_pixel(x,y)[1]);
        }
        assert!(Profile::new_icc(&result.icc_profile.unwrap()).is_ok());
    }
    #[test]
    fn tagged_gray_ramp_is_monotonic_and_rgb_profile_is_rejected() {
        let profile = Profile::new_gray(&lcms2::CIExyY { x:0.3127, y:0.3290, Y:1.0 },
            &lcms2::ToneCurve::new(2.2)).unwrap().icc().unwrap();
        let pixels = image::GrayImage::from_fn(256, 1, |x,_| image::Luma([x as u8]));
        let input = crate::decode::OrientedImage { pixels:pixels.clone().into(), icc_profile:Some(profile) };
        let (rgb,_) = normalize_gray8(input).unwrap();
        assert_eq!(rgb.get_pixel(0,0).0, [0,0,0]);
        assert_eq!(rgb.get_pixel(255,0).0, [255,255,255]);
        for x in 1..256 { for channel in 0..3 {
            assert!(rgb.get_pixel(x,0)[channel] >= rgb.get_pixel(x-1,0)[channel]);
            // Independent transfer-function oracle: gamma 2.2 gray to sRGB.
            // Allow two codes for ICC matrix/adaptation and integer quantization.
            let linear = (x as f64 / 255.0).powf(2.2);
            let srgb = if linear <= 0.0031308 { 12.92 * linear }
                else { 1.055 * linear.powf(1.0 / 2.4) - 0.055 };
            let expected = (srgb * 255.0).round() as i16;
            assert!((i16::from(rgb.get_pixel(x,0)[channel]) - expected).abs() <= 2);
        }}
        let input = crate::decode::OrientedImage { pixels:pixels.into(),
            icc_profile:Some(Profile::new_srgb().icc().unwrap()) };
        assert!(normalize_gray8(input).is_err());
    }
    #[test]
    fn rgba_conversion_preserves_all_alpha_values_across_batches() {
        let pixels = image::RgbaImage::from_fn(257, 5, |x,y|
            image::Rgba([x as u8, y as u8 * 40, 99, x as u8]));
        let expected = pixels.clone();
        let input = crate::decode::OrientedImage { pixels: pixels.into(),
            icc_profile: Some(Profile::new_srgb().icc().unwrap()) };
        let (result, _) = normalize_rgba8(input).unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn image_normalization_keeps_partial_batch_and_rejects_untagged() {
        let pixels = image::RgbImage::from_fn(35, 31, |x,y| image::Rgb([x as u8,y as u8,200]));
        let expected = pixels.clone();
        let input = crate::decode::OrientedImage { pixels: pixels.into(),
            icc_profile: Some(Profile::new_srgb().icc().unwrap()) };
        let (result, icc) = normalize_rgb8(input).unwrap();
        assert_eq!(result, expected);
        assert!(Profile::new_icc(&icc).is_ok());
        let input = crate::decode::OrientedImage { pixels: expected.into(), icc_profile: None };
        assert!(normalize_rgb8(input).is_err());
    }
    #[test]
    fn srgb_identity_and_invalid_profile() {
        let icc = Profile::new_srgb().icc().unwrap();
        let mut samples = [[0,0,0], [255,255,255], [255,0,0], [17,93,201]];
        let original = samples;
        rgb8_to_srgb(&mut samples, &icc).unwrap();
        assert_eq!(samples, original);
        assert!(rgb8_to_srgb(&mut samples, b"invalid").is_err());
        assert_eq!(samples, original);
    }
}
