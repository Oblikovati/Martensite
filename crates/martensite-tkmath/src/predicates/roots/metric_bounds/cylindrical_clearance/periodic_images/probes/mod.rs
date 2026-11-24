// SPDX-License-Identifier: GPL-2.0-only
//! Rounded exact-winding seeds guarded by the existing literal/true periodic clearance.
mod binding;
mod error;
mod proof;
#[cfg(test)]
mod tests;
mod types;
use super::*;
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::{
    Arithmetic, ArithmeticPolicy, Interval,
};
pub use error::{CylindricalProbeError, CylindricalProbeFailure};
pub use types::*;
type Checked<T> = Result<T, CylindricalProbeFailure>;
impl CertifiedCylindricalPeriodicClearance {
    /// Prepare every required rounded winding seed under one arithmetic allowance.
    /// If the witness lies on a certified source boundary, literal clearance `delta+sigma`
    /// and strict rounding error below `delta` transfer its exact winding to the true image.
    /// This certifies rounding only; source incidence, winding and material topology are external.
    /// Example: `let seeds = periodic.certify_probes(&requests, options, || false)?;`.
    pub fn certify_probes(
        &self,
        requests: &[CylindricalPeriodicProbe],
        options: CylindricalProbeOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalProbes, CylindricalProbeError> {
        execute(self, requests, options, &mut cancelled).map_err(|cause| CylindricalProbeError {
            requested_options: options,
            probe_count: requests.len(),
            clearance: self.requested_clearance(),
            cause,
        })
    }
}
fn execute(
    periodic: &CertifiedCylindricalPeriodicClearance,
    requests: &[CylindricalPeriodicProbe],
    options: CylindricalProbeOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalProbes> {
    admit(requests.len(), options)?;
    let mut arithmetic = arithmetic(options, cancelled);
    arithmetic.check_cancelled()?;
    let mut probes = Vec::new();
    for (index, request) in requests.iter().enumerate() {
        probes.push(proof::certify(index, *request, periodic, &mut arithmetic)?);
    }
    Ok(CertifiedCylindricalProbes {
        probes,
        clearance: periodic.requested_clearance(),
        options,
        operations: arithmetic.operations(),
    })
}
fn arithmetic<'a>(
    options: CylindricalProbeOptions,
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
) -> Arithmetic<'a> {
    Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
        },
        cancelled,
    )
}

fn admit(count: usize, options: CylindricalProbeOptions) -> Checked<()> {
    if options.behavior_version != 1
        || !(8..=4096).contains(&options.fractional_bits)
        || options.max_operations > 1_000_000
        || options.max_probes > 65_536
    {
        return Err(CylindricalProbeFailure::Policy);
    }
    if count > options.max_probes as usize {
        return Err(CylindricalProbeFailure::Count);
    }
    Ok(())
}
