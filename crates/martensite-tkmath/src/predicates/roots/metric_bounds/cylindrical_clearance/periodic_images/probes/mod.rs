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
pub use error::{CylindricalProbeError, CylindricalProbeFailure, CylindricalProbeWorkError};
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
        cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalProbes, CylindricalProbeError> {
        self.certify_probes_with_work(requests, options, cancelled)
            .map_err(|error| error.cause)
    }

    /// Certify the same winding seeds and retain the arithmetic spent on refusal.
    /// Call with `periodic.certify_probes_with_work(&requests, options, || false)?`.
    pub fn certify_probes_with_work(
        &self,
        requests: &[CylindricalPeriodicProbe],
        options: CylindricalProbeOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalProbes, CylindricalProbeWorkError> {
        execute(self, requests, options, &mut cancelled).map_err(|(cause, operations)| {
            CylindricalProbeWorkError {
                operations,
                cause: CylindricalProbeError {
                    requested_options: options,
                    probe_count: requests.len(),
                    clearance: self.requested_clearance(),
                    cause,
                },
            }
        })
    }
}
fn execute(
    periodic: &CertifiedCylindricalPeriodicClearance,
    requests: &[CylindricalPeriodicProbe],
    options: CylindricalProbeOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<CertifiedCylindricalProbes, (CylindricalProbeFailure, u32)> {
    admit(requests.len(), options).map_err(|cause| (cause, 0))?;
    let mut arithmetic = arithmetic(options, cancelled);
    let probes = issue(periodic, requests, &mut arithmetic)
        .map_err(|cause| (cause, arithmetic.operations()))?;
    Ok(CertifiedCylindricalProbes {
        probes,
        clearance: periodic.requested_clearance(),
        options,
        operations: arithmetic.operations(),
    })
}

fn issue(
    periodic: &CertifiedCylindricalPeriodicClearance,
    requests: &[CylindricalPeriodicProbe],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Vec<CertifiedCylindricalProbe>> {
    arithmetic.check_cancelled()?;
    requests
        .iter()
        .copied()
        .enumerate()
        .map(|(index, request)| proof::certify(index, request, periodic, arithmetic))
        .collect()
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
