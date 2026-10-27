//! Exact template geometry. Pixel dimensions are authoritative at 300 DPI.
//! Lab production approval is separate from agreement on these dimensions.

pub mod safety;
pub mod crop;
pub mod grid;
pub mod source_path;
pub mod export_plan;
pub mod publication;
pub mod decode;
pub mod color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixels {
    pub width: u32,
    pub height: u32,
}

impl Pixels {
    pub fn millimetres(self) -> (f64, f64) {
        (self.width as f64 * 25.4 / 300.0, self.height as f64 * 25.4 / 300.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanveraSize {
    Square12,
    Landscape12x15,
    Landscape12x18,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Template {
    pub single: Pixels,
    pub spread: Pixels,
    /// Proof only: the supplied cover has no verified sheet-count/spine mapping.
    pub cover_proof: Pixels,
}

impl CanveraSize {
    pub fn template(self) -> Template {
        let (single, spread, cover) = match self {
            Self::Square12 => ((3602, 3602), (7205, 3602), (8268, 4273)),
            Self::Landscape12x15 => ((4394, 3520), (8787, 3520), (9850, 4191)),
            Self::Landscape12x18 => ((5280, 3520), (10559, 3520), (11622, 4191)),
        };
        Template {
            single: Pixels { width: single.0, height: single.1 },
            spread: Pixels { width: spread.0, height: spread.1 },
            cover_proof: Pixels { width: cover.0, height: cover.1 },
        }
    }
}

impl Template {
    /// Retain half-pixel folds; never round or infer the spread from a single.
    pub fn fold_x(self) -> f64 {
        self.spread.width as f64 / 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteriorUnit {
    FirstSingle,
    Spread,
    LastSingle,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProfileError {
    InvalidSpreadCount,
    UnverifiedCoverSpine,
}

/// Provisional owner-approved structure, not a claim of lab acceptance.
/// Stress fixtures must be built separately and cannot bypass this product cap.
pub fn interior_units(spreads: u32) -> Result<Vec<InteriorUnit>, ProfileError> {
    if !(1..=39).contains(&spreads) {
        return Err(ProfileError::InvalidSpreadCount);
    }
    let mut units = Vec::with_capacity(spreads as usize + 2);
    units.push(InteriorUnit::FirstSingle);
    units.extend((0..spreads).map(|_| InteriorUnit::Spread));
    units.push(InteriorUnit::LastSingle);
    Ok(units)
}

pub fn dimensions(template: Template, unit: InteriorUnit) -> Pixels {
    match unit {
        InteriorUnit::FirstSingle | InteriorUnit::LastSingle => template.single,
        InteriorUnit::Spread => template.spread,
    }
}

/// No production cover until a verified lab spine table is implemented.
pub fn production_cover() -> Result<Pixels, ProfileError> {
    Err(ProfileError::UnverifiedCoverSpine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_is_not_double_single_width() {
        for size in [CanveraSize::Square12, CanveraSize::Landscape12x15, CanveraSize::Landscape12x18] {
            let t = size.template();
            assert_ne!(t.spread.width, 2 * t.single.width);
            assert_eq!(t.fold_x().fract(), 0.5);
        }
        assert_eq!(CanveraSize::Landscape12x15.template().fold_x(), 4393.5);
    }

    #[test]
    fn maximum_product_is_eighty_pages_with_two_singles() {
        let units = interior_units(39).unwrap();
        assert_eq!(units.len(), 41);
        assert_eq!(units.first(), Some(&InteriorUnit::FirstSingle));
        assert_eq!(units.last(), Some(&InteriorUnit::LastSingle));
        let pages: u32 = units.iter().map(|u| if *u == InteriorUnit::Spread { 2 } else { 1 }).sum();
        assert_eq!(pages, 80);
        assert_eq!(interior_units(40), Err(ProfileError::InvalidSpreadCount));
        assert_eq!(interior_units(u32::MAX), Err(ProfileError::InvalidSpreadCount));
        assert_eq!(interior_units(0), Err(ProfileError::InvalidSpreadCount));
    }

    #[test]
    fn mixed_units_keep_exact_pixels_and_cover_is_blocked() {
        let t = CanveraSize::Landscape12x15.template();
        assert_eq!(dimensions(t, InteriorUnit::FirstSingle), Pixels { width: 4394, height: 3520 });
        assert_eq!(dimensions(t, InteriorUnit::Spread), Pixels { width: 8787, height: 3520 });
        assert_eq!(production_cover(), Err(ProfileError::UnverifiedCoverSpine));
    }
}
