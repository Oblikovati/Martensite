// SPDX-License-Identifier: GPL-2.0-only
//! Cold full-rectangle cap bounds; selected filled material is independently rebound by TKBO.
use super::*;
mod capture;
mod pairs;
pub(super) struct CapCapture {
    pub(super) source: ConicalActualCapSnapshot,
    pub(super) u: Interval,
    pub(super) level: Interval,
}
pub(super) fn certify(
    query: &ConicalActualPatchQuery<'_>,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<Vec<CapCapture>> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    let mut result = Vec::new();
    for cap in query.caps {
        position.clear();
        position.cap(cap.carrier.id);
        result.push(capture::cap(cap, sine, cosine, &period, arithmetic)?);
    }
    position.clear();
    pairs::certify(&result, &period, arithmetic)?;
    Ok(result)
}
