// SPDX-License-Identifier: GPL-2.0-only
//! Finite complete image relations plus a certified infinite tail.
use super::*;
use crate::predicates::roots::DyadicBound;
mod planning;
mod probes;
pub use probes::*;
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
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalPeriodicClearance, CylindricalClearanceError> {
        self.certify_periodic_clearance_with_work(
            max_images,
            self.options.max_operations,
            cancelled,
        )
        .map_err(|error| error.cause)
    }

    /// Certify the same complete periodic relation under a caller's remaining arithmetic allowance.
    /// The proof uses the smaller of this allowance and the original boundary policy's limit.
    /// On refusal the error retains all arithmetic already consumed, including unfinished images.
    /// Example: `let proof = boundary.certify_periodic_clearance_with_work(16, remaining, || false)?;`.
    pub fn certify_periodic_clearance_with_work(
        &self,
        max_images: u32,
        max_operations: u32,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalPeriodicClearance, CylindricalClearanceWorkError> {
        let mut options = self.options;
        options.max_operations = options.max_operations.min(max_operations);
        proof::execute(self, max_images, options, &mut cancelled).map_err(|(cause, operations)| {
            CylindricalClearanceWorkError {
                cause: CylindricalClearanceError {
                    requested_clearance: self.clearance,
                    requested_options: options,
                    source_count: self.sources.len(),
                    cause,
                },
                operations,
            }
        })
    }
}

fn bound(integer: num_bigint::BigInt, bits: u16) -> DyadicBound {
    DyadicBound::new(integer, -i32::from(bits))
}
