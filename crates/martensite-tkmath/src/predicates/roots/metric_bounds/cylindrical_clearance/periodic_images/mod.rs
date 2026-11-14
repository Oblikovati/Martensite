// SPDX-License-Identifier: GPL-2.0-only
//! Finite complete image relations plus a certified infinite tail.
use super::*;
use crate::predicates::roots::DyadicBound;
mod planning;
mod proof;
mod types;
pub use types::*;

impl CertifiedCylindricalClearance {
    /// Certify boundary clearance for every nonzero true cylindrical image.
    /// Original operation/domain/node limits cover planning and all images together.
    /// At most 1024 positive images may be requested; zero admits only the slab case.
    /// Example: `let periodic = clearance.certify_periodic_clearance(16, || false)?;`.
    pub fn certify_periodic_clearance(
        &self,
        max_images: u32,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalPeriodicClearance, CylindricalClearanceError> {
        proof::execute(self, max_images, &mut cancelled).map_err(|cause| {
            CylindricalClearanceError {
                requested_clearance: self.clearance,
                requested_options: self.options,
                source_count: self.sources.len(),
                cause,
            }
        })
    }
}

fn bound(integer: num_bigint::BigInt, bits: u16) -> DyadicBound {
    DyadicBound::new(integer, -i32::from(bits))
}
