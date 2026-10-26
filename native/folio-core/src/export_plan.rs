//! Geometry-proof export plan. No lab bleed/spine approval is implied.
use crate::{dimensions, interior_units, CanveraSize, InteriorUnit, ProfileError};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCommand {
    pub source_index: usize,
    pub matrix: [f64; 6],
    pub clip: [f64; 4],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionPlan {
    pub geometry: ProofPlan,
    pub images: Vec<Vec<ImageCommand>>,
}

/// Preflight the whole grid before returning any painting commands. Source
/// indices refer to the supplied source order; image decoding is still separate.
pub fn compose_grid(sources: &[crate::grid::Source<'_>], size: CanveraSize,
    spreads: u32, minimum_ppi: f64, face_inset_mm: f64)
    -> Result<CompositionPlan, &'static str> {
    let geometry = geometry_proof(size, spreads).map_err(|_| "Invalid spread count")?;
    let pages = crate::grid::layout_with_inset(sources, size, spreads, minimum_ppi, face_inset_mm)?;
    let mut images = Vec::with_capacity(pages.len());
    for (page, unit) in pages.iter().zip(&geometry.units) {
        let mut commands = Vec::with_capacity(page.len());
        for placement in page {
            let (matrix, clip) = placement.crop.pdf_placement(placement.rect,
                unit.height_px as f64 * 25.4 / 300.0)?;
            commands.push(ImageCommand { source_index: placement.source_index, matrix, clip });
        }
        images.push(commands);
    }
    Ok(CompositionPlan { geometry, images })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUnit {
    pub index: usize,
    pub kind: &'static str,
    pub jpeg: String,
    pub width_px: u32,
    pub height_px: u32,
    pub media_box: [f64; 4],
    pub trim_box: [f64; 4],
    pub bleed_box: [f64; 4],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProofPlan {
    pub profile: &'static str,
    pub dpi: u32,
    pub physical_pages: u32,
    pub structure_provisional: bool,
    pub production_cover_allowed: bool,
    pub proof_only: bool,
    pub units: Vec<ExportUnit>,
}

/// Equal boxes intentionally describe a zero-additional-bleed geometry proof.
/// Production exports require a separately verified lab box policy.
pub fn geometry_proof(size: CanveraSize, spreads: u32) -> Result<ProofPlan, ProfileError> {
    let units = interior_units(spreads)?.into_iter().enumerate().map(|(i, unit)| {
        let pixels = dimensions(size.template(), unit);
        // 72 points/inch at 300 pixels/inch, without rounding through millimetres.
        let bounds = [0.0, 0.0, pixels.width as f64 * 72.0 / 300.0,
            pixels.height as f64 * 72.0 / 300.0];
        ExportUnit {
            index: i + 1,
            kind: match unit { InteriorUnit::FirstSingle => "firstSingle",
                InteriorUnit::Spread => "spread", InteriorUnit::LastSingle => "lastSingle" },
            jpeg: format!("{:03}.jpg", i + 1),
            width_px: pixels.width, height_px: pixels.height,
            media_box: bounds, trim_box: bounds, bleed_box: bounds,
        }
    }).collect();
    Ok(ProofPlan {
        profile: match size { CanveraSize::Square12 => "12x12",
            CanveraSize::Landscape12x15 => "12x15L", CanveraSize::Landscape12x18 => "12x18L" },
        dpi: 300, physical_pages: spreads * 2 + 2,
        structure_provisional: true, production_cover_allowed: false, proof_only: true, units,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn composition_keeps_mixed_geometry_and_rejects_bad_sources() {
        let hashes: Vec<_> = (0..4).map(|i| format!("{i:064x}")).collect();
        let mut sources: Vec<_> = hashes.iter().map(|hash| crate::grid::Source {
            sha256: hash, width: 30000, height: 30000, faces: vec![],
        }).collect();
        let plan = compose_grid(&sources, CanveraSize::Landscape12x15, 1, 200.0, 10.0).unwrap();
        assert_eq!(plan.images.len(), 3);
        let mut indices = Vec::new();
        for (commands, unit) in plan.images.iter().zip(&plan.geometry.units) {
            for command in commands {
                indices.push(command.source_index);
                let [x,y,w,h] = command.clip;
                assert!(x >= -1e-9 && y >= -1e-9);
                assert!(x+w <= unit.media_box[2]+1e-9);
                assert!(y+h <= unit.media_box[3]+1e-9);
                assert!(command.matrix.iter().all(|v| v.is_finite()));
            }
        }
        assert_eq!(indices, vec![0,1,2,3]);
        sources[2].width = 10;
        assert!(compose_grid(&sources, CanveraSize::Landscape12x15, 1, 200.0, 10.0).is_err());
    }
    #[test]
    fn matches_independent_frozen_maximum_product() {
        // This fixture is reviewed input, never generated from this plan.
        let expected: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/audit/canvera-12x15L-80page-proof.json")).unwrap();
        let plan = geometry_proof(CanveraSize::Landscape12x15, 39).unwrap();
        assert_eq!(plan.units.len(), expected["units"].as_array().unwrap().len());
        for (unit, frozen) in plan.units.iter().zip(expected["units"].as_array().unwrap()) {
            assert_eq!(unit.jpeg, frozen["jpeg"].as_str().unwrap());
            assert_eq!(serde_json::json!([unit.width_px, unit.height_px]), frozen["pixels"]);
            for (actual, key) in [(unit.media_box, "mediaBox"), (unit.trim_box, "trimBox"), (unit.bleed_box, "bleedBox")] {
                for (a, e) in actual.iter().zip(frozen[key].as_array().unwrap()) {
                    assert!((a - e.as_f64().unwrap()).abs() < 1e-9);
                }
            }
        }
        assert!(plan.proof_only);
        assert!(!plan.production_cover_allowed);
        assert!(geometry_proof(CanveraSize::Landscape12x15, 40).is_err());
    }
}
