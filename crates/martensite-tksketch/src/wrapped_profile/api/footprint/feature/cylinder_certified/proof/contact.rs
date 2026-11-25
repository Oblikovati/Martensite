// SPDX-License-Identifier: GPL-2.0-only
//! Qualify the stock and far-cap contact collars on actual decoded ruled sides.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalStorage, CertifiedCylindricalTrim, CylindricalContactPrefixQuery,
    CylindricalRadialSense, CylindricalRadialSweepQuery, CylindricalTrimQuery, MetricBezier,
    certify_cylindrical_contact_prefix, certify_cylindrical_radial_sweep,
};
use std::collections::{BTreeMap, BTreeSet};

mod corner;
mod interval;
mod own;
mod windows;
use interval::{add, mul, positive};
use windows::Boundary;

#[derive(Clone, Copy)]
pub(super) struct Trace<'a> {
    pub(super) query: CylindricalTrimQuery<'a>,
    pub(super) proof: &'a CertifiedCylindricalTrim,
}

pub(super) struct Tube<'a> {
    /// Actual depth-zero and far-cap traces, regardless of radial ordering.
    pub(super) caps: [Trace<'a>; 2],
    pub(super) storage: &'a CertifiedCylindricalStorage,
    pub(super) directions: &'a [[f64; 2]],
    /// Lower bounds on the effective distance thresholds at the two cap contacts.
    pub(super) floor: [f64; 2],
}

pub(super) fn certify(
    attempt: Attempt,
    support: CylinderSupport,
    contact_upper: f64,
    tubes: &BTreeMap<WrappedHistoryKey, Tube<'_>>,
    joins: &BTreeMap<[WrappedHistoryKey; 2], [f64; 2]>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    let first = tubes
        .values()
        .next()
        .ok_or_else(|| missing(attempt, VALIDATION, "retained contact tubes"))?;
    require(
        attempt,
        VALIDATION,
        positive(contact_upper) && positive(support.radius),
        "finite positive contact and support radius",
    )?;
    check_join_census(attempt, tubes, joins)?;
    let mut boundaries = first
        .caps
        .map(|cap| Boundary::new(cap.query.radius, contact_upper));
    for tube in tubes.values() {
        attempt.poll(cancelled, VALIDATION)?;
        verify_tube(attempt, tube, support.normal, &mut boundaries, cancelled)?;
    }
    for (cap, boundary) in boundaries.into_iter().enumerate() {
        boundary.certify(attempt, support.radius, cap, tubes, joins, cancelled)?;
    }
    Ok(())
}

fn check_join_census(
    attempt: Attempt,
    tubes: &BTreeMap<WrappedHistoryKey, Tube<'_>>,
    joins: &BTreeMap<[WrappedHistoryKey; 2], [f64; 2]>,
) -> Checked<()> {
    let distinct = joins.keys().all(|[a, b]| a != b);
    let complete = (0..2).all(|end| {
        let found = joins.keys().map(|pair| pair[end]).collect::<BTreeSet<_>>();
        found.iter().eq(tubes.keys())
    });
    require(
        attempt,
        VALIDATION,
        distinct && joins.len() == tubes.len() && complete,
        "one incoming and one outgoing retained join for each contact tube",
    )
}

fn verify_tube(
    attempt: Attempt,
    tube: &Tube<'_>,
    normal: NormalSense,
    boundaries: &mut [Boundary; 2],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    let rails = tube.caps.map(|trace| trace.query.curve);
    let inward = normal == NormalSense::Inward;
    let ordered = if inward { [rails[1], rails[0]] } else { rails };
    let matched = tube
        .storage
        .matches_source(ordered, tube.storage.stored_radii());
    require(
        attempt,
        VALIDATION,
        matched && same_height(rails),
        "actual common-height side rails bound to storage proof",
    )?;
    let motion = ruling_motion(rails);
    for (cap, boundary) in boundaries.iter_mut().enumerate() {
        let sense = if inward ^ (cap == 1) {
            CylindricalRadialSense::Inward
        } else {
            CylindricalRadialSense::Outward
        };
        verify_end(attempt, tube, cap, sense, motion, boundary, cancelled)?;
    }
    Ok(())
}

fn verify_end(
    attempt: Attempt,
    tube: &Tube<'_>,
    cap: usize,
    sense: CylindricalRadialSense,
    motion: f64,
    boundary: &mut Boundary,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    let trace = tube.caps[cap];
    let far = tube.caps[1 - cap].query.curve;
    let error = trace.proof.spatial_residual_bound();
    let floor = tube.floor[cap];
    require(
        attempt,
        VALIDATION,
        positive(floor) && floor <= boundary.contact && error < floor,
        "actual seam gap below the retained contact continuation floor",
    )?;
    let prefix = radial_prefix(attempt, trace, far, sense, boundary.contact, cancelled)?;
    let bounds = EndBounds::new(tube, trace, sense, motion, error, &prefix);
    require(
        attempt,
        VALIDATION,
        own::qualify(bounds, trace.query, tube.directions, boundary.contact).is_some(),
        "positive contact Hessian on the full chart and the closed own trim",
    )?;
    boundary.include(attempt, trace, floor, bounds)?;
    Ok(())
}

fn radial_prefix(
    attempt: Attempt,
    trace: Trace<'_>,
    far: MetricBezier<'_, 3>,
    sense: CylindricalRadialSense,
    contact: f64,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalContactPrefix>
{
    let sweep = CylindricalRadialSweepQuery {
        rails: [trace.query.curve, far],
        axis_origin: [0.; 2],
        sense,
        minimum: 0.,
        options: Default::default(),
    };
    let radial = certify_math(attempt, "stored cylinder radial sweep", cancelled, |poll| {
        certify_cylindrical_radial_sweep(sweep, poll)
    })?;
    let prefix = CylindricalContactPrefixQuery {
        seam: trace.query,
        far_rail: far,
        sense,
        contact_allowance: contact,
        max_normalized_depth: 1.,
        options: Default::default(),
    };
    certify_math(
        attempt,
        "stored cylinder radial contact prefix",
        cancelled,
        |poll| certify_cylindrical_contact_prefix(prefix, &radial, trace.proof, poll),
    )
}

fn certify_math<T, E: std::fmt::Debug>(
    attempt: Attempt,
    query: &'static str,
    cancelled: &mut CancelPoll<'_>,
    run: impl FnOnce(&mut CancelPoll<'_>) -> Result<T, E>,
) -> Checked<T> {
    let mut observed = false;
    let result = run(&mut || {
        observed = observed || cancelled();
        observed
    });
    if observed {
        return Err(attempt.fail(VALIDATION, CylinderFeatureCause::Cancelled));
    }
    super::certify(attempt, VALIDATION, query, result)
}

#[derive(Clone, Copy)]
struct EndBounds {
    prefix: f64,
    rate: f64,
    motion: f64,
    error: f64,
    radius_floor: f64,
    radius_ceiling: f64,
    local_radius_error: f64,
    tangent_error: f64,
}

impl EndBounds {
    fn new(
        tube: &Tube<'_>,
        trace: Trace<'_>,
        sense: CylindricalRadialSense,
        motion: f64,
        error: f64,
        prefix: &martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalContactPrefix,
    ) -> Self {
        let radii = tube.storage.stored_radii();
        let radial = tube.storage.radial_distance_bounds();
        let index = usize::from(sense == CylindricalRadialSense::Inward);
        Self {
            prefix: prefix.normalized_depth_upper(),
            rate: prefix.signed_squared_radius_derivative_lower(),
            motion,
            error,
            radius_floor: (radii[0] - radial[0]).next_down(),
            radius_ceiling: add(radii[1], radial[1]),
            local_radius_error: add(
                radial[index],
                (trace.query.radius - radii[index]).abs().next_up(),
            ),
            tangent_error: tube.storage.scaled_ruling_bound(),
        }
    }
}

fn same_height([a, b]: [MetricBezier<'_, 3>; 2]) -> bool {
    a.poles.len() == b.poles.len()
        && a.poles
            .iter()
            .zip(b.poles)
            .all(|(a, b)| a[2].to_bits() == b[2].to_bits())
}

fn ruling_motion([a, b]: [MetricBezier<'_, 3>; 2]) -> f64 {
    a.poles
        .iter()
        .zip(b.poles)
        .map(|(a, b)| {
            let difference = |axis: usize| (a[axis] - b[axis]).abs().next_up();
            add(difference(0), difference(1))
        })
        .fold(0., f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_one_shot_cancel_inside_a_numeric_certificate_keeps_its_typed_cause() {
        let attempt = Attempt {
            attachment: FaceAttachmentId(1),
            revision: SupportRevision(2),
            kind: CylinderFeatureKind::Boss,
        };
        let mut polls = 0;
        let mut cancel = || {
            polls += 1;
            polls == 1
        };
        let error = certify_math(attempt, "corner test", &mut cancel, |poll| {
            assert!(poll());
            assert!(poll());
            Err::<(), _>("underlying arithmetic left the certificate incomplete")
        })
        .unwrap_err();
        assert_eq!(polls, 1);
        assert_eq!(error.stage, VALIDATION);
        assert!(matches!(*error.cause, CylinderFeatureCause::Cancelled));
    }
}
