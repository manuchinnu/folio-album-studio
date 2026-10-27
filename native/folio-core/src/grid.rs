//! Deterministic one/two-photo baseline for exact mixed Canvera units.
use crate::{crop::Crop, dimensions, interior_units, safety::RectMm, CanveraSize};
use std::collections::HashSet;

pub struct Source<'a> {
    pub sha256: &'a str,
    pub width: u32,
    pub height: u32,
    pub faces: Vec<Crop>,
}

#[derive(Debug, serde::Serialize)]
pub struct Placement {
    pub source_index: usize,
    pub rect: RectMm,
    pub crop: Crop,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct GridRequest {
    profile: String,
    spreads: u32,
    minimum_ppi: f64,
    face_inset_mm: f64,
    sources: Vec<GridSource>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct GridSource {
    sha256: String,
    width: u32,
    height: u32,
    faces: Vec<Crop>,
}

/// Diagnostic layout request; not yet the canonical persisted Project format.
pub fn layout_json(input: &str) -> Result<String, String> {
    request_json(input, false)
}

/// Diagnostic preflighted painting plan, not a rendered PDF or persisted project.
pub fn composition_json(input: &str) -> Result<String, String> {
    request_json(input, true)
}

fn request_json(input: &str, composition: bool) -> Result<String, String> {
    if input.len() > 1_048_576 { return Err("Grid request exceeds 1 MiB".into()); }
    let request: GridRequest = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let size = match request.profile.as_str() {
        "12x12" => CanveraSize::Square12,
        "12x15L" => CanveraSize::Landscape12x15,
        "12x18L" => CanveraSize::Landscape12x18,
        _ => return Err("Unknown Canvera profile".into()),
    };
    let sources: Vec<Source> = request.sources.iter().map(|s| Source {
        sha256: &s.sha256, width: s.width, height: s.height, faces: s.faces.clone(),
    }).collect();
    if composition {
        let plan = crate::export_plan::compose_grid(&sources, size, request.spreads,
            request.minimum_ppi, request.face_inset_mm)?;
        return serde_json::to_string(&plan).map_err(|e| e.to_string());
    }
    let pages = layout_with_inset(&sources, size, request.spreads, request.minimum_ppi, request.face_inset_mm)?;
    serde_json::to_string(&pages).map_err(|e| e.to_string())
}

pub fn layout(sources: &[Source<'_>], size: CanveraSize, spreads: u32,
    minimum_ppi: f64) -> Result<Vec<Vec<Placement>>, &'static str> {
    layout_with_inset(sources, size, spreads, minimum_ppi, crate::safety::ASSUMED_SAFE_INSET_MM)
}

pub fn layout_with_inset(sources: &[Source<'_>], size: CanveraSize, spreads: u32,
    minimum_ppi: f64, face_inset_mm: f64) -> Result<Vec<Vec<Placement>>, &'static str> {
    let units = interior_units(spreads).map_err(|_| "Invalid spread count")?;
    if sources.len() < units.len() || sources.len() > units.len() * 2 {
        return Err("Baseline requires one or two distinct photos per upload unit");
    }
    let mut hashes = HashSet::new();
    for source in sources {
        if source.sha256.len() != 64 || !source.sha256.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {
            return Err("Invalid source SHA256");
        }
        if !hashes.insert(source.sha256) { return Err("Duplicate originals cannot fill separate slots"); }
    }
    let mut result = Vec::with_capacity(units.len());
    let mut cursor = 0;
    for (i, unit) in units.iter().enumerate() {
        let (width, height) = dimensions(size.template(), *unit).millimetres();
        let count = (sources.len() - cursor).div_ceil(units.len() - i);
        let horizontal = width > height;
        let gap = 6.0;
        // Images may extend to the canvas edge; text/face safety is a separate
        // preflight responsibility, not a blanket image inset.
        let w = if horizontal { (width - gap * (count - 1) as f64) / count as f64 } else { width };
        let h = if horizontal { height } else { (height - gap * (count - 1) as f64) / count as f64 };
        let mut placements = Vec::with_capacity(count);
        for slot in 0..count {
            let source = &sources[cursor];
            let crop = Crop::checked_grid_fit(source.width, source.height, w, h, &source.faces, minimum_ppi)?;
            let rect = RectMm { x: if horizontal { slot as f64 * (w + gap) } else { 0.0 },
                y: if horizontal { 0.0 } else { slot as f64 * (h + gap) }, width: w, height: h };
            let canvas = dimensions(size.template(), *unit);
            crate::safety::check_content(canvas, rect, crate::safety::ContentKind::Image, face_inset_mm)
                .map_err(|_| "Invalid grid frame or safety inset")?;
            for face in &source.faces {
                let projected = crop.project(*face, rect)?;
                crate::safety::check_content(canvas, projected, crate::safety::ContentKind::Face, face_inset_mm)
                    .map_err(|_| "Projected face falls outside the assumed safe area")?;
            }
            placements.push(Placement { source_index: cursor, crop, rect });
            cursor += 1;
        }
        result.push(placements);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_product_layout_covers_every_valid_photo_budget() {
        let hashes: Vec<String> = (0..82).map(|i| format!("{i:064x}")).collect();
        let sources: Vec<Source> = hashes.iter().map(|hash| Source {
            sha256: hash, width: 30000, height: 30000, faces: vec![],
        }).collect();
        for size in [CanveraSize::Square12, CanveraSize::Landscape12x15, CanveraSize::Landscape12x18] {
            for count in 41..=82 {
                let pages = layout(&sources[..count], size, 39, 200.0).unwrap();
                assert_eq!(pages.len(), 41);
                assert_eq!(pages.iter().flatten().map(|p| p.source_index).collect::<Vec<_>>(), (0..count).collect::<Vec<_>>());
                assert!(pages.iter().all(|p| (1..=2).contains(&p.len())));
            }
        }
    }
    #[test]
    fn retained_face_near_upload_edge_requires_safe_inset() {
        let hashes: Vec<String> = (0..3).map(|i| format!("{i:064x}")).collect();
        let sources: Vec<Source> = hashes.iter().map(|hash| Source {
            sha256: hash, width: 30000, height: 30000,
            faces: vec![Crop::new(0.001, 0.45, 0.02, 0.02).unwrap()],
        }).collect();
        assert_eq!(layout(&sources, CanveraSize::Square12, 1, 200.0).unwrap_err(),
            "Projected face falls outside the assumed safe area");
        assert!(layout_with_inset(&sources, CanveraSize::Square12, 1, 200.0, 0.0).is_ok());
        assert!(layout_with_inset(&sources, CanveraSize::Square12, 1, 200.0, f64::NAN).is_err());
    }
    #[test]
    fn mixed_units_preserve_input_order_without_repetition() {
        let hashes: Vec<String> = (0..5).map(|i| format!("{i:064x}")).collect();
        let sources: Vec<Source> = hashes.iter().map(|hash| Source { sha256: hash, width: 30000, height: 30000, faces: vec![] }).collect();
        let pages = layout(&sources, CanveraSize::Landscape12x15, 1, 200.0).unwrap();
        assert_eq!(pages.iter().map(Vec::len).collect::<Vec<_>>(), vec![2, 2, 1]);
        assert_eq!(pages.iter().flatten().map(|p| p.source_index).collect::<Vec<_>>(), vec![0,1,2,3,4]);
        assert!((pages[2][0].rect.width - 4394.0 * 25.4 / 300.0).abs() < 1e-9);
        assert!(layout(&sources[..2], CanveraSize::Landscape12x15, 1, 200.0).is_err());
    }
    #[test]
    fn duplicates_and_low_resolution_return_no_partial_layout() {
        let hash = "a".repeat(64);
        let sources: Vec<Source> = (0..3).map(|_| Source { sha256: &hash, width: 30000, height: 30000, faces: vec![] }).collect();
        assert!(layout(&sources, CanveraSize::Square12, 1, 200.0).is_err());
        let hashes: Vec<String> = (0..3).map(|i| format!("{i:064x}")).collect();
        let low: Vec<Source> = hashes.iter().map(|h| Source { sha256: h, width: 100, height: 100, faces: vec![] }).collect();
        assert!(layout(&low, CanveraSize::Square12, 1, 200.0).is_err());
    }
}
