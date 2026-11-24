// SPDX-License-Identifier: GPL-2.0-only
//! Same-reference signed contour measure; topology and normal-map validity belong to the operation.
use super::subinterval::ContributionBounds;
use super::*;
use crate::predicates::roots::metric_bounds::{
    CylindricalFluxOptions, CylindricalFluxWork, CylindricalProfileOptions,
};
mod admission;
mod binding;
mod coefficients;
mod error;
mod execution;
mod quotient;
mod types;
pub use error::*;
pub use types::*;

/// Bound the ideal common-chart q/L normal-reference contour integral.
/// No closure, injectivity, orientation or body-validity claim accompanies this scalar.
/// Example: `let volume = certify_conical_reference_volume(query, || false)?;`.
pub fn certify_conical_reference_volume(
    query: ConicalReferenceVolumeQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedConicalReferenceVolume, ConicalReferenceVolumeError> {
    poll(&mut cancelled).map_err(|cause| {
        error::integration(
            query,
            None,
            (0, 0),
            ConicalReferenceVolumeTerm::Publication,
            cause.into(),
        )
    })?;
    admission::admit(query)?;
    let (coefficients, operations) = coefficients::compute(query, &mut cancelled)?;
    execution::integrate(query, &coefficients, operations, &mut cancelled)
}

#[cfg(test)]
mod tests;
