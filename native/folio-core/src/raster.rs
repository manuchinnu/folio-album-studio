//! Budgeted opaque proof canvas. Resampling/color policy belongs to the caller.
/// Candidate Lanczos3 tile resampler for the spike. Reference parity must be
/// measured before this becomes the production resampling policy.
pub fn resize_tile(source: &image::RgbImage, width: u32, height: u32, max_working_bytes: u64)
    -> Result<image::RgbImage, &'static str> {
    if width == 0 || height == 0 || source.width() == 0 || source.height() == 0 {
        return Err("Empty resize dimensions");
    }
    // Two explicit passes preserve the reference's horizontal-first RGB8
    // intermediate clipping. Conservatively include both float scratch sizes.
    let intermediate = u64::from(source.width()).checked_mul(u64::from(source.height()))
        .and_then(|n| n.checked_mul(16))
        .and_then(|n| n.checked_add(u64::from(width).checked_mul(u64::from(height))?.checked_mul(16)?))
        .and_then(|n| n.checked_add(u64::from(width).checked_mul(u64::from(source.height()))?.checked_mul(3)?));
    let output = u64::from(width).checked_mul(u64::from(height)).and_then(|n| n.checked_mul(3));
    let weights = u64::from(source.width().max(source.height())).checked_mul(16);
    let required = intermediate.and_then(|n| n.checked_add(output?))
        .and_then(|n| n.checked_add(source.as_raw().len() as u64))
        .and_then(|n| n.checked_add(weights?));
    if required.filter(|n| *n <= max_working_bytes && *n <= usize::MAX as u64).is_none() {
        return Err("Resize exceeds working memory budget");
    }
    let horizontal = image::imageops::resize(source,width,source.height(),image::imageops::FilterType::Lanczos3);
    Ok(image::imageops::resize(&horizontal,width,height,image::imageops::FilterType::Lanczos3))
}

/// Quantize absolute physical edges, so adjacent frames share the same pixel
/// boundary instead of accumulating independently rounded widths.
pub fn frame_pixels(frame: crate::safety::RectMm, width: u32, height: u32)
    -> Result<[u32;4], &'static str> {
    if [frame.x,frame.y,frame.width,frame.height].iter().any(|v| !v.is_finite())
        || frame.x < 0.0 || frame.y < 0.0 || frame.width <= 0.0 || frame.height <= 0.0 {
        return Err("Invalid raster frame");
    }
    let scale = 300.0 / 25.4;
    let edges = [frame.x*scale,frame.y*scale,(frame.x+frame.width)*scale,(frame.y+frame.height)*scale];
    if edges.iter().any(|v| !v.is_finite()) || edges[2] > width as f64 + 0.000001
        || edges[3] > height as f64 + 0.000001 { return Err("Raster frame exceeds canvas"); }
    let [left,top,right,bottom] = edges.map(|v| v.round() as u32);
    if right <= left || bottom <= top { return Err("Raster frame is smaller than one pixel"); }
    Ok([left,top,right-left,bottom-top])
}

pub fn canvas(width: u32, height: u32, max_bytes: usize) -> Result<image::RgbImage, String> {
    if width == 0 || height == 0 { return Err("Empty canvas".into()); }
    let bytes = (width as usize).checked_mul(height as usize)
        .and_then(|n| n.checked_mul(3)).ok_or("Canvas size overflow")?;
    if bytes > max_bytes { return Err("Canvas exceeds memory budget".into()); }
    let mut pixels = Vec::new();
    pixels.try_reserve_exact(bytes).map_err(|_| "Cannot allocate canvas")?;
    pixels.resize(bytes, 255);
    image::RgbImage::from_raw(width,height,pixels).ok_or("Invalid canvas dimensions".into())
}

/// Paste an already-rendered opaque tile without scaling or implicit clipping.
/// Validate all bounds before touching the destination.
pub fn paste(canvas: &mut image::RgbImage, tile: &image::RgbImage, x: u32, y: u32) -> Result<(), &'static str> {
    if tile.width() == 0 || tile.height() == 0
        || x.checked_add(tile.width()).filter(|v| *v <= canvas.width()).is_none()
        || y.checked_add(tile.height()).filter(|v| *v <= canvas.height()).is_none() {
        return Err("Tile exceeds canvas");
    }
    let stride = canvas.width() as usize * 3;
    let tile_stride = tile.width() as usize * 3;
    for (row, source) in tile.as_raw().chunks_exact(tile_stride).enumerate() {
        let start = (y as usize + row) * stride + x as usize * 3;
        canvas.as_mut()[start..start+tile_stride].copy_from_slice(source);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resampling_preserves_constant_color_and_checks_budget_before_allocation() {
        let source = image::RgbImage::from_pixel(31,17,image::Rgb([20,100,230]));
        assert!(resize_tile(&source,13,7,1).is_err());
        assert!(resize_tile(&source,u32::MAX,u32::MAX,u64::MAX).is_err());
        for (w,h) in [(13,7),(50,33)] {
            let tile = resize_tile(&source,w,h,1_000_000).unwrap();
            assert_eq!(tile.dimensions(),(w,h));
            assert!(tile.pixels().all(|p| p.0 == [20,100,230]));
        }
    }
    #[test]
    fn fractional_fold_shares_edge_and_keeps_odd_spread_width() {
        let mm = 25.4/300.0;
        let frame = |x,w| crate::safety::RectMm {x,y:0.0,width:w,height:3520.0*mm};
        let left = frame_pixels(frame(0.0,4393.5*mm),8787,3520).unwrap();
        let right = frame_pixels(frame(4393.5*mm,4393.5*mm),8787,3520).unwrap();
        assert_eq!(left[0]+left[2],right[0]);
        assert_eq!(left[2]+right[2],8787);
        assert!(frame_pixels(frame(0.0,8788.0*mm),8787,3520).is_err());
        assert!(frame_pixels(frame(f64::NAN,1.0),8787,3520).is_err());
    }
    #[test]
    fn budget_and_placement_preserve_background_and_reject_partial_writes() {
        assert!(canvas(10,10,299).is_err());
        assert!(canvas(u32::MAX,u32::MAX,usize::MAX).is_err());
        let mut page = canvas(5,4,60).unwrap();
        let tile = image::RgbImage::from_fn(2,2, |x,y| image::Rgb([x as u8,y as u8,42]));
        paste(&mut page,&tile,3,2).unwrap();
        for y in 0..4 { for x in 0..5 {
            let expected = if x>=3 && y>=2 { *tile.get_pixel(x-3,y-2) } else { image::Rgb([255;3]) };
            assert_eq!(*page.get_pixel(x,y),expected);
        }}
        let original = page.clone();
        assert!(paste(&mut page,&tile,4,2).is_err());
        assert!(paste(&mut page,&tile,u32::MAX,0).is_err());
        assert_eq!(page,original);
    }
}
