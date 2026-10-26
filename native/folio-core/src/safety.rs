//! Assumed, configurable face/text safety. No exclusion at a layflat fold.
use crate::Pixels;

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct RectMm {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum ContentKind {
    Image,
    Face,
    Text,
}

#[derive(Debug, PartialEq)]
pub enum SafetyError {
    InvalidGeometry,
    InvalidInset,
    OutsideUpload,
    OutsideAssumedSafeArea,
}

/// The 10mm default is an assumption, not a verified Canvera specification.
pub const ASSUMED_SAFE_INSET_MM: f64 = 10.0;

/// Validate positioned content against the exact upload canvas. The full image
/// may reach every edge; only faces and text receive the configurable inset.
/// Callers pass the visible image frame, not an oversized source crop rectangle.
pub fn check_content(
    canvas: Pixels,
    rect: RectMm,
    kind: ContentKind,
    inset_mm: f64,
) -> Result<(), SafetyError> {
    let (width, height) = canvas.millimetres();
    if canvas.width == 0 || canvas.height == 0
        || ![rect.x, rect.y, rect.width, rect.height].iter().all(|v| v.is_finite())
        || rect.width <= 0.0 || rect.height <= 0.0
    {
        return Err(SafetyError::InvalidGeometry);
    }
    if !inset_mm.is_finite() || inset_mm < 0.0
        || inset_mm * 2.0 >= width || inset_mm * 2.0 >= height
    {
        return Err(SafetyError::InvalidInset);
    }
    if rect.x < 0.0 || rect.y < 0.0
        || rect.x + rect.width > width || rect.y + rect.height > height
    {
        return Err(SafetyError::OutsideUpload);
    }
    if !matches!(kind, ContentKind::Image)
        && (rect.x < inset_mm || rect.y < inset_mm
            || rect.x + rect.width > width - inset_mm
            || rect.y + rect.height > height - inset_mm)
    {
        return Err(SafetyError::OutsideAssumedSafeArea);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CanveraSize;

    #[test]
    fn edge_to_edge_images_but_not_text_or_faces() {
        let canvas = CanveraSize::Landscape12x15.template().spread;
        let (width, height) = canvas.millimetres();
        let frame = RectMm { x: 0.0, y: 0.0, width, height };
        assert_eq!(check_content(canvas, frame, ContentKind::Image, 10.0), Ok(()));
        for kind in [ContentKind::Face, ContentKind::Text] {
            assert_eq!(check_content(canvas, frame, kind, 10.0), Err(SafetyError::OutsideAssumedSafeArea));
        }
    }

    #[test]
    fn fold_has_no_exclusion_and_inset_is_configurable() {
        let canvas = CanveraSize::Landscape12x15.template().spread;
        let (width, _) = canvas.millimetres();
        let face = RectMm { x: width / 2.0 - 5.0, y: 10.0, width: 10.0, height: 10.0 };
        assert_eq!(check_content(canvas, face, ContentKind::Face, 10.0), Ok(()));
        assert_eq!(check_content(canvas, face, ContentKind::Face, 11.0), Err(SafetyError::OutsideAssumedSafeArea));
        let edge = RectMm { x: 0.0, ..face };
        assert_eq!(check_content(canvas, edge, ContentKind::Text, 0.0), Ok(()));
    }

    #[test]
    fn malformed_numbers_cannot_bypass_preflight() {
        let canvas = CanveraSize::Square12.template().single;
        let valid = RectMm { x: 10.0, y: 10.0, width: 20.0, height: 20.0 };
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(check_content(canvas, RectMm { x, ..valid }, ContentKind::Image, 10.0), Err(SafetyError::InvalidGeometry));
        }
        for inset in [-1.0, f64::NAN, f64::INFINITY, 200.0] {
            assert_eq!(check_content(canvas, valid, ContentKind::Face, inset), Err(SafetyError::InvalidInset));
        }
        assert_eq!(check_content(canvas, RectMm { x: -0.01, ..valid }, ContentKind::Image, 10.0), Err(SafetyError::OutsideUpload));
        assert_eq!(check_content(canvas, RectMm { width: 0.0, ..valid }, ContentKind::Image, 10.0), Err(SafetyError::InvalidGeometry));
    }
}
