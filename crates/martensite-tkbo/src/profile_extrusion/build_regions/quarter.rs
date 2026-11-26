// SPDX-License-Identifier: GPL-2.0-only
//! Exact cardinal domain for the explicitly scoped circular edge constructor.
use super::{DECLINE, ProfileTopologyMode, Stage, cancelled_failure, placement};
use crate::profile_extrusion::{
    ArcSense, DeclineCause, PlanarProofWorkLimit, PlanarRegionWorkFailure, PlanarRegionWorkResult,
    ProfileCurve, ProfileCurveId, ProfileVertexId, Resolution, SketchProfile,
};

/// Certify a profile of lines and one positive quarter at the intrinsic chart origin.
///
/// The arc's endpoints must lie on the positive U and V axes at equal radius. Native
/// construction uses its known parameters and an operation-local modelling tolerance.
/// Example: `build_planar_cardinal_quarter_regions_with_work_and_cancel(&profile, resolution, work, || false)?`.
pub fn build_planar_cardinal_quarter_regions_with_work_and_cancel(
    profile: &SketchProfile,
    resolution: Resolution,
    requested: PlanarProofWorkLimit,
    mut cancelled: impl FnMut() -> bool + Send,
) -> PlanarRegionWorkResult {
    require_profile(profile, requested, &mut cancelled)?;
    placement::build_located(
        profile,
        resolution,
        requested,
        ProfileTopologyMode::ScopedCardinalQuarter,
        &mut cancelled,
    )
}

fn require_profile(
    profile: &SketchProfile,
    requested: PlanarProofWorkLimit,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), PlanarRegionWorkFailure> {
    let mut arc = None;
    for curve in &profile.curves {
        poll(requested, cancelled)?;
        match *curve {
            ProfileCurve::Line { .. } => {}
            ProfileCurve::Arc {
                id,
                start,
                end,
                center,
                sense,
            } if arc.is_none() && center == [0., 0.] && sense == ArcSense::CounterClockwise => {
                arc = Some((id, start, end));
            }
            _ => return Err(invalid(requested, curve.id())),
        }
    }
    poll(requested, cancelled)?;
    let Some((id, start, end)) = arc else {
        let failure = profile.curves.first().map_or_else(
            || {
                let decline = DECLINE.at(Stage::Admission, DeclineCause::EmptyProfile);
                PlanarRegionWorkFailure::new(requested, decline, None)
            },
            |curve| invalid(requested, curve.id()),
        );
        return Err(failure);
    };
    require_quarter_endpoints(profile, requested, (id, start, end), cancelled)
}

fn require_quarter_endpoints(
    profile: &SketchProfile,
    requested: PlanarProofWorkLimit,
    (id, start, end): (ProfileCurveId, ProfileVertexId, ProfileVertexId),
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), PlanarRegionWorkFailure> {
    let (mut first, mut last) = (None, None);
    for vertex in &profile.vertices {
        poll(requested, cancelled)?;
        if vertex.id == start {
            first = Some(vertex.point);
        } else if vertex.id == end {
            last = Some(vertex.point);
        }
    }
    match (first, last) {
        (Some([x, 0.]), Some([0., y])) if x.is_finite() && x > 0. && x == y => Ok(()),
        _ => Err(invalid(requested, id)),
    }
}

fn invalid(requested: PlanarProofWorkLimit, curve: ProfileCurveId) -> PlanarRegionWorkFailure {
    let cause = DeclineCause::InvalidCurve {
        curve,
        reason: "expected one positive circular quarter at the chart origin from +U to +V",
    };
    PlanarRegionWorkFailure::new(requested, DECLINE.at(Stage::Admission, cause), None)
}

fn poll(
    requested: PlanarProofWorkLimit,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), PlanarRegionWorkFailure> {
    if cancelled() {
        return Err(cancelled_failure(requested, Stage::Admission, None));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
