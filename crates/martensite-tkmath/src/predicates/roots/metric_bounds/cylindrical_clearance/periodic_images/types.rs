// SPDX-License-Identifier: GPL-2.0-only
use super::*;

/// One literal translation with a certified error from the true `turn * 2πR`.
#[derive(Clone, Debug)]
pub struct CylindricalPeriodicTranslation {
    pub(super) turn: u32,
    pub(super) offset: f64,
    pub(super) error: DyadicBound,
    pub(super) comparison: f64,
}
impl CylindricalPeriodicTranslation {
    /// Positive image number; negative images follow from ordered-pair symmetry; `shift.turn()`.
    pub fn turn(&self) -> u32 {
        self.turn
    }
    /// Exact supplied binary64 S translation; `shift.offset()`.
    pub fn offset(&self) -> f64 {
        self.offset
    }
    /// Outward distance from the true image translation; `shift.translation_error_bound()`.
    pub fn translation_error_bound(&self) -> &DyadicBound {
        &self.error
    }
    /// Strengthened literal-image comparison margin; `shift.comparison_clearance()`.
    pub fn comparison_clearance(&self) -> f64 {
        self.comparison
    }
}

/// Complete ordered boundary relation for one literal translated family.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalImage {
    pub(super) translation: CylindricalPeriodicTranslation,
    pub(super) certificate: CertifiedCylindricalClearance,
}
impl CertifiedCylindricalImage {
    /// True/literal shift accounting; `image.translation()`.
    pub fn translation(&self) -> &CylindricalPeriodicTranslation {
        &self.translation
    }
    /// Effective comparison and work receipts; `image.clearance_certificate()`.
    pub fn clearance_certificate(&self) -> &CertifiedCylindricalClearance {
        &self.certificate
    }
}

/// Boundary clearance for every nonzero true periodic image.
/// This certifies boundaries; disconnected material requires a separate containment proof.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalPeriodicClearance {
    pub(super) source: CertifiedCylindricalClearance,
    pub(super) images: Vec<CertifiedCylindricalImage>,
    pub(super) enclosure: PeriodicEnclosure,
    pub(super) work: PeriodicWork,
}

#[derive(Clone, Debug)]
pub(super) struct PeriodicEnclosure {
    pub width: DyadicBound,
    pub period: [DyadicBound; 2],
    pub tail: DyadicBound,
    pub margin: DyadicBound,
}

#[derive(Clone)]
pub(super) struct PeriodicWork {
    pub image_limit: u32,
    pub operations: u32,
    pub domains: u32,
    pub nodes: u32,
    pub operation_limit: Option<u32>,
}

impl std::fmt::Debug for PeriodicWork {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut work = out.debug_struct("PeriodicWork");
        work.field("image_limit", &self.image_limit)
            .field("operations", &self.operations)
            .field("domains", &self.domains)
            .field("nodes", &self.nodes);
        if let Some(limit) = self.operation_limit {
            work.field("operation_limit", &limit);
        }
        work.finish()
    }
}

impl CertifiedCylindricalPeriodicClearance {
    /// Bind the original boundary relation, chart, clearance and exclusions; work policy is separate.
    /// Example: `assert!(periodic.matches_boundary(&clearance));`.
    pub fn matches_boundary(&self, other: &CertifiedCylindricalClearance) -> bool {
        self.source.sources == other.sources
            && self.source.radius.to_bits() == other.radius.to_bits()
            && self.source.origin.to_bits() == other.origin.to_bits()
            && self.source.clearance.to_bits() == other.clearance.to_bits()
            && self.source.scope == other.scope
            && self.source.excluded == other.excluded
    }
    /// Every required positive image, including self-source pairs; `proof.images()`.
    pub fn images(&self) -> &[CertifiedCylindricalImage] {
        &self.images
    }
    /// Every image from this number onward is excluded by the enclosing slab; `proof.tail_start()`.
    pub fn tail_start(&self) -> u32 {
        self.images.len() as u32 + 1
    }
    /// Certified gap for the first excluded slab image; `proof.tail_separation_lower_bound()`.
    pub fn tail_separation_lower_bound(&self) -> &DyadicBound {
        &self.enclosure.tail
    }
    /// Guaranteed physical boundary margin for every image; `proof.separation_lower_bound()`.
    pub fn separation_lower_bound(&self) -> &DyadicBound {
        &self.enclosure.margin
    }
    /// Original physical margin; `proof.requested_clearance()`.
    pub fn requested_clearance(&self) -> f64 {
        self.source.clearance_lower_bound()
    }
    /// Outward true circumference; `proof.circumference_bounds()`.
    pub fn circumference_bounds(&self) -> &[DyadicBound; 2] {
        &self.enclosure.period
    }
    /// Conservative developed source width; `proof.width_upper_bound()`.
    pub fn width_upper_bound(&self) -> &DyadicBound {
        &self.enclosure.width
    }
    /// Original positive-image allowance; `proof.requested_max_images()`.
    pub fn requested_max_images(&self) -> u32 {
        self.work.image_limit
    }
    /// Total planning and image arithmetic under one allowance; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.work.operations
    }
    /// Total visited image domains; `proof.domain_visits()`.
    pub fn domain_visits(&self) -> u32 {
        self.work.domains
    }
    /// Total cached image curve nodes charged across images; `proof.curve_nodes()`.
    pub fn curve_nodes(&self) -> u32 {
        self.work.nodes
    }
    /// Original aggregate limits; effective per-image remainders appear in each image receipt.
    /// Example: `proof.requested_options()`.
    pub fn requested_options(&self) -> CylindricalClearanceOptions {
        let mut options = self.source.requested_options();
        if let Some(limit) = self.work.operation_limit {
            options.max_operations = limit;
        }
        options
    }
    /// Bind the complete proof to original authored inputs and retained O2 chart; `proof.matches_profiles(&sources)`.
    pub fn matches_profiles(&self, sources: &[CylindricalProfileSource<'_>]) -> bool {
        self.source.matches_profiles(sources)
    }
}
