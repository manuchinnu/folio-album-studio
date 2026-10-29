//! Budgeted opaque proof canvas. Resampling/color policy belongs to the caller.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all="camelCase")]
struct UnitRequest {
    width: u32,
    height: u32,
    placements: Vec<crate::grid::Placement>,
    sources: Vec<UnitSource>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all="camelCase")]
struct UnitSource {
    relative: String,
    sha256: String,
    oriented_width: u32,
    oriented_height: u32,
}

/// Diagnostic proof boundary, not the persisted project or production preflight.
pub fn save_unit_json(input: &str, root: &std::path::Path, destination: &std::path::Path)
    -> Result<Vec<usize>,String> {
    if input.len()>1_048_576 { return Err("Unit request exceeds 1 MiB".into()); }
    let request: UnitRequest = serde_json::from_str(input).map_err(|e|e.to_string())?;
    if request.placements.is_empty() || request.placements.len()>64 || request.sources.len()>64 {
        return Err("Proof unit requires 1..64 placements and at most 64 sources".into());
    }
    let sources: Vec<_> = request.sources.iter().map(|s| VerifiedSource {
        relative:&s.relative,sha256:&s.sha256,oriented_width:s.oriented_width,oriented_height:s.oriented_height
    }).collect();
    let (jpeg,assumed) = render_verified_jpeg(root,&sources,request.width,request.height,
        &request.placements,64*1024*1024,40_000_000,512*1024*1024)?;
    crate::publication::write_new(destination,&jpeg).map_err(|e|e.to_string())?;
    Ok(assumed)
}
pub struct VerifiedSource<'a> {
    pub relative: &'a str,
    pub sha256: &'a str,
    pub oriented_width: u32,
    pub oriented_height: u32,
}

/// Source-bound proof rendering. Assumed-profile source indices accompany the
/// JPEG so the caller can persist provenance instead of silently discarding it.
pub fn render_verified_jpeg(root: &std::path::Path, sources: &[VerifiedSource<'_>],
    width: u32, height: u32, placements: &[crate::grid::Placement],
    max_encoded_bytes: u64, max_source_pixels: u64, max_working_bytes: u64)
    -> Result<(Vec<u8>, Vec<usize>), String> {
    if placements.iter().any(|p| p.source_index >= sources.len()) {
        return Err("Placement references an unknown source".into());
    }
    let mut assumed = Vec::new();
    let jpeg = render_jpeg_unit(width,height,placements,max_working_bytes, |index| {
        let source = &sources[index];
        let decoded = crate::decode::verified_oriented(root,source.relative,source.sha256,
            max_encoded_bytes,max_source_pixels)?;
        if (decoded.pixels.width(),decoded.pixels.height()) != (source.oriented_width,source.oriented_height) {
            return Err("Source dimensions differ from preflight".into());
        }
        let (normalized, was_assumed) = crate::color::normalize_with_provenance(decoded)?;
        if was_assumed && !assumed.contains(&index) { assumed.push(index); }
        match normalized.pixels {
            image::DynamicImage::ImageRgb8(rgb) => Ok(rgb),
            _ => Err("Proof rendering requires explicit alpha/depth conversion".into()),
        }
    })?;
    Ok((jpeg,assumed))
}

/// Encode a rendered sRGB unit as a tagged proof. The working budget covers
/// raster operations; compressed encoder allocations still need RSS validation.
pub fn render_jpeg_unit<F>(width: u32, height: u32, placements: &[crate::grid::Placement],
    max_working_bytes: u64, load: F) -> Result<Vec<u8>, String>
where F: FnMut(usize) -> Result<image::RgbImage, String> {
    let page = render_unit(width,height,placements,max_working_bytes,load)?;
    let icc = lcms2::Profile::new_srgb().icc().map_err(|e| e.to_string())?;
    crate::jpeg_proof::encode(&page,&icc,95)
}

/// Render a preflighted unit, loading one normalized opaque RGB8 source at a
/// time. Loader must enforce source identity and its own decode budget.
pub fn render_unit<F>(width: u32, height: u32, placements: &[crate::grid::Placement],
    max_working_bytes: u64, mut load: F) -> Result<image::RgbImage, String>
where F: FnMut(usize) -> Result<image::RgbImage, String> {
    for placement in placements { frame_pixels(placement.rect,width,height)?; }
    let cap = usize::try_from(max_working_bytes).unwrap_or(usize::MAX);
    let mut page = canvas(width,height,cap)?;
    for placement in placements {
        let source = load(placement.source_index)?;
        paint_frame(&mut page,&source,placement.crop,placement.rect,max_working_bytes)?;
    }
    Ok(page)
}

/// Render one already-normalized opaque source into a physical frame. The
/// destination is changed only after geometry, cropping and resampling succeed.
pub fn paint_frame(page: &mut image::RgbImage, source: &image::RgbImage,
    crop: crate::crop::Crop, frame: crate::safety::RectMm, max_working_bytes: u64)
    -> Result<(), &'static str> {
    let [x,y,w,h] = frame_pixels(frame,page.width(),page.height())?;
    let available = max_working_bytes.checked_sub(page.as_raw().len() as u64)
        .ok_or("Canvas exceeds composition memory budget")?;
    let tile = crop_tile(source,crop,w,h,available)?;
    paste(page,&tile,x,y)
}

pub fn crop_tile(source: &image::RgbImage, crop: crate::crop::Crop, width: u32, height: u32,
    max_working_bytes: u64) -> Result<image::RgbImage, &'static str> {
    let [x,y,w,h] = crop.pixel_bounds(source.width(),source.height())?;
    let retained = source.as_raw().len() as u64;
    let crop_bytes = u64::from(w)*u64::from(h)*3;
    let remaining = max_working_bytes.checked_sub(retained).ok_or("Crop exceeds memory budget")?;
    if crop_bytes > remaining { return Err("Crop exceeds memory budget"); }
    let mut cropped = canvas(w,h,usize::try_from(remaining).unwrap_or(usize::MAX))
        .map_err(|_| "Cannot allocate cropped source")?;
    let source_stride = source.width() as usize * 3;
    let crop_stride = w as usize * 3;
    for (row,destination) in cropped.as_mut().chunks_exact_mut(crop_stride).enumerate() {
        let start = (y as usize + row) * source_stride + x as usize * 3;
        destination.copy_from_slice(&source.as_raw()[start..start+crop_stride]);
    }
    resize_tile(&cropped,width,height,remaining)
}

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
    let weights = u64::from(source.width().max(source.height()).max(width)).checked_mul(16);
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
    fn file_backed_render_verifies_identity_dimensions_and_provenance() {
        use sha2::{Digest,Sha256};
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
            .join(format!("render-source-{}-{}",std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("synthetic.png");
        image::RgbImage::from_pixel(8,8,image::Rgb([40,100,180])).save(&path).unwrap();
        let hash = format!("{:x}",Sha256::digest(std::fs::read(&path).unwrap()));
        let mut sources = [VerifiedSource {relative:"synthetic.png",sha256:&hash,oriented_width:8,oriented_height:8}];
        let mm = 25.4/300.0;
        let placements = [crate::grid::Placement {source_index:0,
            rect:crate::safety::RectMm{x:0.0,y:0.0,width:16.0*mm,height:16.0*mm},
            crop:crate::crop::Crop::new(0.0,0.0,1.0,1.0).unwrap()}];
        let (jpeg,assumed) = render_verified_jpeg(&root,&sources,16,16,&placements,10000,1000,100000).unwrap();
        assert_eq!(assumed,vec![0]);
        let decoded = image::load_from_memory(&jpeg).unwrap().to_rgb8();
        assert_eq!(decoded.dimensions(),(16,16));
        for (a,b) in decoded.get_pixel(8,8).0.iter().zip([40u8,100,180]) { assert!(a.abs_diff(b)<=3); }
        sources[0].oriented_width = 9;
        assert!(render_verified_jpeg(&root,&sources,16,16,&placements,10000,1000,100000)
            .unwrap_err().contains("dimensions"));
        sources[0].oriented_width = 8;
        std::fs::write(&path,b"changed synthetic source").unwrap();
        assert!(render_verified_jpeg(&root,&sources,16,16,&placements,10000,1000,100000).is_err());
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
    #[test]
    fn verified_renderer_rejects_unknown_source_before_file_access() {
        let placement = crate::grid::Placement { source_index:0,
            rect:crate::safety::RectMm {x:0.0,y:0.0,width:1.0,height:1.0},
            crop:crate::crop::Crop::new(0.0,0.0,1.0,1.0).unwrap() };
        let error = render_verified_jpeg(std::path::Path::new("missing"),&[],20,20,
            &[placement],1000,1000,100000).unwrap_err();
        assert_eq!(error,"Placement references an unknown source");
    }
    #[test]
    fn unit_loads_in_order_and_invalid_geometry_prevents_loading() {
        let mm = 25.4/300.0;
        let placement = |index,x| crate::grid::Placement { source_index:index,
            rect:crate::safety::RectMm {x:x*mm,y:0.0,width:4.0*mm,height:4.0*mm},
            crop:crate::crop::Crop::new(0.0,0.0,1.0,1.0).unwrap() };
        let mut loaded = Vec::new();
        let page = render_unit(8,4,&[placement(3,0.0),placement(7,4.0)],100000, |i| {
            loaded.push(i);
            Ok(image::RgbImage::from_pixel(4,4,image::Rgb([i as u8,20,30])))
        }).unwrap();
        assert_eq!(loaded,vec![3,7]);
        assert_eq!(page.get_pixel(0,0)[0],3);
        assert_eq!(page.get_pixel(7,3)[0],7);
        assert!(render_unit(8,4,&[placement(3,0.0),placement(7,5.0)],100000,
            |_| panic!("preflight must precede source access")).is_err());
        assert!(render_unit(8,4,&[placement(3,0.0)],100000,
            |_| Err("source changed".into())).is_err());
    }
    #[test]
    fn composed_unit_encodes_to_tagged_jpeg() {
        use image::ImageDecoder;
        let mm = 25.4/300.0;
        let placements = [crate::grid::Placement { source_index:0,
            rect:crate::safety::RectMm {x:0.0,y:0.0,width:32.0*mm,height:16.0*mm},
            crop:crate::crop::Crop::new(0.0,0.0,1.0,1.0).unwrap() }];
        let bytes = render_jpeg_unit(32,16,&placements,100000,
            |_| Ok(image::RgbImage::from_pixel(16,8,image::Rgb([30,90,180])))).unwrap();
        let mut decoder = image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(&bytes)).unwrap();
        assert_eq!(decoder.dimensions(),(32,16));
        assert!(decoder.icc_profile().unwrap().is_some());
        let decoded = image::load_from_memory(&bytes).unwrap().to_rgb8();
        for p in decoded.pixels() { for (a,b) in p.0.iter().zip([30u8,90,180]) { assert!(a.abs_diff(b)<=3); }}
    }
    #[test]
    fn physical_frame_pipeline_paints_only_target_and_is_transactional_on_error() {
        let mut page = canvas(12,8,288).unwrap();
        let source = image::RgbImage::from_pixel(10,10,image::Rgb([15,60,210]));
        let crop = crate::crop::Crop::new(0.2,0.2,0.6,0.6).unwrap();
        let mm = 25.4/300.0;
        let frame = crate::safety::RectMm { x:3.0*mm,y:2.0*mm,width:6.0*mm,height:4.0*mm };
        paint_frame(&mut page,&source,crop,frame,100000).unwrap();
        for (x,y,p) in page.enumerate_pixels() {
            assert_eq!(p.0, if (3..9).contains(&x) && (2..6).contains(&y) { [15,60,210] } else { [255;3] });
        }
        let before = page.clone();
        assert!(paint_frame(&mut page,&source,crop,frame,300).is_err());
        assert_eq!(page,before);
    }
    #[test]
    fn crop_uses_reference_half_even_edges_and_correct_source_region() {
        let source = image::RgbImage::from_fn(5,5, |x,y| image::Rgb([x as u8,y as u8,99]));
        let crop = crate::crop::Crop::new(0.1,0.3,0.6,0.6).unwrap();
        assert_eq!(crop.pixel_bounds(5,5).unwrap(),[0,2,4,2]);
        let tile = crop_tile(&source,crop,4,2,10000).unwrap();
        for (x,y,p) in tile.enumerate_pixels() { assert_eq!(p,source.get_pixel(x,y+2)); }
        assert!(crop_tile(&source,crop,4,2,75).is_err());
    }
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
