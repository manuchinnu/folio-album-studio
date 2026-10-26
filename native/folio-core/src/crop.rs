//! Validated normalized crop at the JSON boundary. Coordinates reference the
//! orientation-corrected source image, matching the retained reference model.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "CropInput")]
pub struct Crop {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CropInput {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl TryFrom<CropInput> for Crop {
    type Error = &'static str;
    fn try_from(v: CropInput) -> Result<Self, Self::Error> {
        Self::new(v.x, v.y, v.w, v.h)
    }
}

impl Crop {
    /// PDF image placement matrix for a full source image, clipped to `frame`.
    /// Source crop coordinates are top-left normalized; PDF uses bottom-left.
    /// The caller must apply the frame clip before painting the image.
    pub fn pdf_placement(self, frame: crate::safety::RectMm, page_height_mm: f64)
        -> Result<([f64; 6], [f64; 4]), &'static str> {
        let values = [frame.x, frame.y, frame.width, frame.height, page_height_mm];
        if values.iter().any(|v| !v.is_finite()) || frame.width <= 0.0
            || frame.height <= 0.0 || page_height_mm <= 0.0 {
            return Err("Invalid PDF placement geometry");
        }
        let pt = 72.0 / 25.4;
        let left = frame.x * pt;
        let bottom = (page_height_mm - frame.y - frame.height) * pt;
        let width = frame.width * pt;
        let height = frame.height * pt;
        let image_width = width / self.w;
        let image_height = height / self.h;
        let matrix = [image_width, 0.0, 0.0, image_height,
            left - self.x * image_width,
            bottom - (1.0 - self.y - self.h) * image_height];
        let clip = [left, bottom, width, height];
        if matrix.iter().chain(clip.iter()).any(|v| !v.is_finite()) {
            return Err("PDF placement overflow");
        }
        Ok((matrix, clip))
    }

    /// Map a fully retained source face to its physical position in the frame.
    pub fn project(self, face: Self, frame: crate::safety::RectMm)
        -> Result<crate::safety::RectMm, &'static str> {
        if !self.contains(face) { return Err("Face is outside the retained crop"); }
        if ![frame.x, frame.y, frame.width, frame.height].iter().all(|v| v.is_finite())
            || frame.width <= 0.0 || frame.height <= 0.0 {
            return Err("Invalid placement frame");
        }
        Ok(crate::safety::RectMm {
            x: frame.x + (face.x - self.x) / self.w * frame.width,
            y: frame.y + (face.y - self.y) / self.h * frame.height,
            width: face.w / self.w * frame.width,
            height: face.h / self.h * frame.height,
        })
    }

    pub fn contains(self, other: Self) -> bool {
        other.x >= self.x && other.y >= self.y
            && other.x + other.w <= self.x + self.w
            && other.y + other.h <= self.y + self.h
    }

    /// Baseline grid placement: fail rather than silently clipping a known face
    /// or relying on output upsampling to meet the native-resolution threshold.
    pub fn checked_grid_fit(source_width: u32, source_height: u32,
        frame_width_mm: f64, frame_height_mm: f64, faces: &[Self],
        minimum_ppi: f64) -> Result<Self, &'static str> {
        if !minimum_ppi.is_finite() || minimum_ppi <= 0.0 {
            return Err("Minimum PPI must be positive and finite");
        }
        let crop = Self::centered_cover(source_width, source_height, frame_width_mm, frame_height_mm)?;
        if crop.effective_ppi(source_width, source_height, frame_width_mm, frame_height_mm)? < minimum_ppi {
            return Err("Insufficient native resolution for grid placement");
        }
        if faces.iter().any(|face| !crop.contains(*face)) {
            return Err("Centered crop would cut a protected face; change layout or select an alternate");
        }
        Ok(crop)
    }

    /// Centered cover-fit fallback for deterministic grid placement. Face-aware
    /// composition will use the same validated crop boundary, not this fallback.
    pub fn centered_cover(source_width: u32, source_height: u32,
        frame_width: f64, frame_height: f64) -> Result<Self, &'static str> {
        if source_width == 0 || source_height == 0
            || !frame_width.is_finite() || !frame_height.is_finite()
            || frame_width <= 0.0 || frame_height <= 0.0 {
            return Err("Source and frame dimensions must be positive and finite");
        }
        let source_aspect = source_width as f64 / source_height as f64;
        let frame_aspect = frame_width / frame_height;
        if !frame_aspect.is_finite() || frame_aspect <= 0.0 {
            return Err("Frame aspect ratio overflow or underflow");
        }
        if source_aspect > frame_aspect {
            let w = frame_aspect / source_aspect;
            Self::new((1.0 - w) / 2.0, 0.0, w, 1.0)
        } else {
            let h = source_aspect / frame_aspect;
            Self::new(0.0, (1.0 - h) / 2.0, 1.0, h)
        }
    }

    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Result<Self, &'static str> {
        if ![x, y, w, h].iter().all(|v| v.is_finite())
            || x < 0.0 || y < 0.0 || w < 0.0001 || h < 0.0001
            || w > 1.0 || h > 1.0 || x + w > 1.0 || y + h > 1.0
        {
            return Err("Crop must be finite, nonempty and inside the normalized source");
        }
        Ok(Self { x, y, w, h })
    }

    /// Effective resolution after cropping, before any output upsampling.
    pub fn effective_ppi(self, source_width: u32, source_height: u32,
        placed_width_mm: f64, placed_height_mm: f64) -> Result<f64, &'static str> {
        if source_width == 0 || source_height == 0
            || !placed_width_mm.is_finite() || !placed_height_mm.is_finite()
            || placed_width_mm <= 0.0 || placed_height_mm <= 0.0 {
            return Err("Source and placement dimensions must be positive and finite");
        }
        let ppi = (source_width as f64 * self.w * 25.4 / placed_width_mm)
            .min(source_height as f64 * self.h * 25.4 / placed_height_mm);
        if !ppi.is_finite() { return Err("Resolution overflow"); }
        Ok(ppi)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pdf_crop_maps_source_corners_to_clip_without_vertical_flip() {
        use crate::safety::RectMm;
        // Asymmetric crop catches top/bottom inversion that centered crops hide.
        let crop = super::Crop::new(0.1, 0.2, 0.5, 0.3).unwrap();
        let frame = RectMm { x: 25.4, y: 50.8, width: 50.8, height: 25.4 };
        let (m, clip) = crop.pdf_placement(frame, 254.0).unwrap();
        let close = |a: f64, b: f64| assert!((a-b).abs() < 1e-9, "{a} != {b}");
        for (a,b) in clip.into_iter().zip([72.0, 504.0, 144.0, 72.0]) { close(a,b); }
        // PDF image coordinates invert the source top-left Y convention.
        close(m[0] * 0.1 + m[4], clip[0]);
        close(m[0] * 0.6 + m[4], clip[0] + clip[2]);
        close(m[3] * (1.0 - 0.2) + m[5], clip[1] + clip[3]);
        close(m[3] * (1.0 - 0.5) + m[5], clip[1]);
        assert!(crop.pdf_placement(frame, f64::NAN).is_err());
        assert!(crop.pdf_placement(RectMm { width: f64::MAX, ..frame }, 254.0).is_err());
        assert!(crop.pdf_placement(RectMm { height: 0.0, ..frame }, 254.0).is_err());
    }

    use super::*;
    #[test]
    fn grid_fit_blocks_face_clipping_and_low_native_resolution() {
        let central = Crop::new(0.4, 0.4, 0.2, 0.2).unwrap();
        let edge = Crop::new(0.0, 0.4, 0.1, 0.2).unwrap();
        assert!(Crop::checked_grid_fit(6000, 3000, 100.0, 100.0, &[central], 200.0).is_ok());
        assert!(Crop::checked_grid_fit(6000, 3000, 100.0, 100.0, &[edge], 200.0).is_err());
        assert!(Crop::checked_grid_fit(300, 300, 100.0, 100.0, &[], 200.0).is_err());
        assert!(Crop::checked_grid_fit(6000, 3000, 100.0, 100.0, &[], f64::NAN).is_err());
        // Equality at the resolution threshold is allowed.
        assert!(Crop::checked_grid_fit(2000, 2000, 254.0, 254.0, &[], 200.0).is_ok());
    }
    #[test]
    fn cover_fit_preserves_aspect_and_center_across_orientations() {
        for (sw, sh, fw, fh) in [(4000, 3000, 200.0, 100.0),
            (3000, 4000, 100.0, 200.0), (8000, 1000, 100.0, 100.0),
            (3000, 3000, 100.0, 100.0)] {
            let c = Crop::centered_cover(sw, sh, fw, fh).unwrap();
            let cropped_aspect = sw as f64 * c.w / (sh as f64 * c.h);
            assert!((cropped_aspect - fw / fh).abs() < 1e-9);
            assert!((c.x + c.w / 2.0 - 0.5).abs() < 1e-9);
            assert!((c.y + c.h / 2.0 - 0.5).abs() < 1e-9);
            assert!(c.w == 1.0 || c.h == 1.0);
        }
        assert!(Crop::centered_cover(0, 100, 1.0, 1.0).is_err());
        assert!(Crop::centered_cover(100, 100, f64::MAX, f64::MIN_POSITIVE).is_err());
        assert!(Crop::centered_cover(100, 100, f64::NAN, 1.0).is_err());
    }
    #[test]
    fn json_rejects_unknown_missing_wrong_type_and_outside_crop() {
        for json in [r#"{"x":0,"y":0,"w":1,"h":1,"extra":1}"#,
            r#"{"x":0,"y":0,"w":1}"#, r#"{"x":"0","y":0,"w":1,"h":1}"#,
            r#"{"x":0.9,"y":0,"w":0.2,"h":1}"#,
            r#"{"x":0,"y":0,"w":0,"h":1}"#] {
            assert!(serde_json::from_str::<Crop>(json).is_err(), "{json}");
        }
        let crop: Crop = serde_json::from_str(r#"{"x":0,"y":0,"w":1,"h":1}"#).unwrap();
        let roundtrip = serde_json::to_string(&crop).unwrap();
        assert!(serde_json::from_str::<Crop>(&roundtrip).is_ok());
        assert!(Crop::new(f64::NAN, 0.0, 1.0, 1.0).is_err());
    }

    #[test]
    fn crop_reduces_effective_resolution_without_credit_for_upsampling() {
        let full = Crop::new(0.0, 0.0, 1.0, 1.0).unwrap();
        let half = Crop::new(0.25, 0.0, 0.5, 1.0).unwrap();
        assert!((full.effective_ppi(3000, 3000, 254.0, 254.0).unwrap() - 300.0).abs() < 1e-9);
        assert!((half.effective_ppi(3000, 3000, 254.0, 254.0).unwrap() - 150.0).abs() < 1e-9);
        assert!(full.effective_ppi(0, 3000, 254.0, 254.0).is_err());
        assert!(full.effective_ppi(3000, 3000, f64::NAN, 254.0).is_err());
    }
}
