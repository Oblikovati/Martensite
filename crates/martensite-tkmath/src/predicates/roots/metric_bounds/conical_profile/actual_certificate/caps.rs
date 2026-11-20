// SPDX-License-Identifier: GPL-2.0-only
//! Cold full-rectangle cap bounds; selected filled material is independently rebound by TKBO.
use super::capture::SideCapture;
use super::*;
mod atlas;
mod capture;
mod pairs;
pub(super) struct CapCapture {
    pub(super) source: ConicalActualCapSnapshot,
    pub(super) u: Interval,
    pub(super) level: Interval,
    extent: CapExtent,
}
enum CapExtent {
    Slant(Interval),
    Atlas(Box<[Interval; 3]>),
    Legacy,
}
pub(super) fn certify(
    query: &ConicalActualPatchQuery<'_>,
    sides: &[SideCapture],
    sine: &Interval,
    cosine: &Interval,
    bound_slant: bool,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Outcome<Vec<CapCapture>> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    let atlas = query.sides[0].domain.version() == ConicalChartDomainVersion::AtlasMemberV5;
    let mut result = Vec::new();
    for cap in query.caps {
        position.clear();
        position.cap(cap.carrier.id);
        result.push(capture::cap(
            cap,
            sine,
            cosine,
            &period,
            bound_slant,
            atlas,
            arithmetic,
        )?);
    }
    position.clear();
    if atlas {
        atlas::certify(&result, sides, arithmetic)?;
    } else {
        pairs::certify(&result, &period, arithmetic)?;
    }
    Ok(result)
}
