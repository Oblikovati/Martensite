// SPDX-License-Identifier: GPL-2.0-only
//! Exhaust all possibly active adjacent contacts on one decoded periodic lift.
use super::corner::{self, Arm};
use super::interval::{Range, prefix};
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::certify_cylindrical_trim;
use std::f64::consts::TAU;

pub(super) struct Boundary {
    radius: f64,
    contact: f64,
    floor: f64,
    angular: [f64; 2],
    seam: f64,
    prefix: f64,
    motion: f64,
    radial_error: f64,
    tangent_error: f64,
}

impl Boundary {
    pub(super) fn new(radius: f64, contact: f64) -> Self {
        Self {
            radius,
            contact,
            floor: f64::INFINITY,
            angular: [f64::INFINITY, f64::NEG_INFINITY],
            seam: 0.,
            prefix: 0.,
            motion: 0.,
            radial_error: 0.,
            tangent_error: 0.,
        }
    }

    pub(super) fn include(
        &mut self,
        attempt: Attempt,
        trace: Trace<'_>,
        floor: f64,
        bounds: EndBounds,
    ) -> Checked<()> {
        require(
            attempt,
            VALIDATION,
            trace.query.radius.to_bits() == self.radius.to_bits(),
            "identical actual radius for all traces of one cap",
        )?;
        self.floor = self.floor.min(floor);
        self.seam = self.seam.max(bounds.error);
        self.prefix = self.prefix.max(bounds.prefix);
        self.motion = self.motion.max(bounds.motion);
        self.radial_error = self.radial_error.max(bounds.local_radius_error);
        self.tangent_error = self.tangent_error.max(bounds.tangent_error);
        for &[angle, _] in trace.query.pcurve {
            self.angular[0] = self.angular[0].min(angle);
            self.angular[1] = self.angular[1].max(angle);
        }
        Ok(())
    }

    pub(super) fn certify(
        &self,
        attempt: Attempt,
        support_radius: f64,
        cap: usize,
        tubes: &BTreeMap<WrappedHistoryKey, Tube<'_>>,
        joins: &BTreeMap<[WrappedHistoryKey; 2], [f64; 2]>,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let pad = self.projected_reach().ok_or_else(|| {
            missing(
                attempt,
                VALIDATION,
                "finite projected contact windows clear of the cylinder axis",
            )
        })?;
        let phase_width = mul(self.radius, (self.angular[1] - self.angular[0]).next_up());
        let period = (self.radius * TAU.next_down()).next_down();
        require(
            attempt,
            VALIDATION,
            add(phase_width, pad) < period,
            "no other periodic image can reach the decoded contact windows",
        )?;
        let adjacent: Vec<_> = joins.iter().collect();
        work::ordered(
            attempt,
            VALIDATION,
            &adjacent,
            cancelled,
            |&(&pair, &[angle, z]), poll| {
                let [Some(a), Some(b)] = pair.map(|key| tubes.get(&key)) else {
                    return Err(missing(
                        attempt,
                        VALIDATION,
                        "both retained tubes of each directed join",
                    ));
                };
                let direction = [angle * support_radius / self.radius, z];
                self.certify_join(attempt, [a, b], cap, direction, pad, poll)
            },
        )?;
        Ok(())
    }

    fn projected_reach(&self) -> Option<f64> {
        let radial_floor = (self.radius - self.radial_error).next_down();
        let minimum = (radial_floor - mul(self.prefix, self.motion)).next_down();
        if !positive(minimum) {
            return None;
        }
        // A common-height ruling is radial up to the certified tangential error.
        // Unlike Euclidean travel, its radial movement does not move the chart point.
        let tangential =
            (mul(mul(self.prefix, self.tangent_error), self.radius) / minimum).next_up();
        let pad = add(
            add(self.project(self.contact)?, self.project(self.seam)?),
            tangential,
        );
        positive(pad).then_some(pad)
    }

    fn project(&self, distance: f64) -> Option<f64> {
        let minimum = (self.radius - distance).next_down();
        positive(minimum).then(|| (mul(distance, self.radius) / minimum).next_up())
    }

    fn certify_join(
        &self,
        attempt: Attempt,
        tubes: [&Tube<'_>; 2],
        cap: usize,
        direction: [f64; 2],
        pad: f64,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let queries = tubes.map(|tube| tube.caps[cap].query);
        let (anchor, uv) = shared_corner(queries).ok_or_else(|| {
            missing(
                attempt,
                VALIDATION,
                "one exact shared three-dimensional and UV contact corner",
            )
        })?;
        let norm = add(direction[0].abs(), direction[1].abs());
        let bound = mul(norm, pad);
        let windows = clipped_windows(queries, self.radius, direction, bound).ok_or_else(|| {
            missing(
                attempt,
                VALIDATION,
                "complete positively directed adjacent trim windows",
            )
        })?;
        let gap = anchor_gap(attempt, queries[0], anchor, uv, cancelled)?;
        let far = tubes.map(|tube| tube.caps[1 - cap].query.curve);
        self.verify_arms(attempt, queries, far, windows, anchor, gap)
    }

    fn verify_arms(
        &self,
        attempt: Attempt,
        queries: [CylindricalTrimQuery<'_>; 2],
        far: [MetricBezier<'_, 3>; 2],
        windows: [f64; 2],
        anchor: [f64; 3],
        gap: f64,
    ) -> Checked<()> {
        for i in 0..2 {
            let arm = Arm {
                near: queries[i].curve,
                far: far[i],
                target: queries[1 - i].pcurve,
                radius: self.radius,
                windows: [windows[i], windows[1 - i]],
                depth: self.prefix,
                reverse: i == 0,
                anchor,
                gap,
                floor: self.floor,
            };
            require(
                attempt,
                VALIDATION,
                corner::qualify(arm).is_some(),
                "directed retraction of each adjacent cap contact into its actual corner",
            )?;
        }
        Ok(())
    }
}

fn shared_corner([a, b]: [CylindricalTrimQuery<'_>; 2]) -> Option<([f64; 3], [f64; 2])> {
    let anchor = *a.curve.poles.last()?;
    let uv = *a.pcurve.last()?;
    let same_space = anchor.map(f64::to_bits) == b.curve.poles.first()?.map(f64::to_bits);
    let same_uv = uv.map(f64::to_bits) == b.pcurve.first()?.map(f64::to_bits);
    (same_space && same_uv).then_some((anchor, uv))
}

fn anchor_gap(
    attempt: Attempt,
    query: CylindricalTrimQuery<'_>,
    anchor: [f64; 3],
    uv: [f64; 2],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<f64> {
    let query = CylindricalTrimQuery {
        curve: MetricBezier {
            poles: &[anchor; 2],
            weights: None,
        },
        pcurve: &[uv; 2],
        ..query
    };
    let proof = certify_math(
        attempt,
        "actual corner seam point and UV lift",
        cancelled,
        |poll| certify_cylindrical_trim(query, poll),
    )?;
    Ok(proof.spatial_residual_bound())
}

fn clipped_windows(
    [a, b]: [CylindricalTrimQuery<'_>; 2],
    radius: f64,
    direction: [f64; 2],
    bound: f64,
) -> Option<[f64; 2]> {
    let [first, second] = [(a.pcurve, true), (b.pcurve, false)]
        .map(|(curve, reverse)| clip(curve, radius, direction, bound, reverse));
    Some([first?, second?])
}

fn clip(
    controls: &[[f64; 2]],
    radius: f64,
    direction: [f64; 2],
    bound: f64,
    reverse: bool,
) -> Option<f64> {
    let slope = own::projection_margin(controls, radius, direction);
    if !positive(slope) || !positive(bound) {
        return None;
    }
    // The two outgoing projected travels add. Thus a contact with projected
    // distance at most bound has each individual travel below the same bound.
    let mut upper = (bound / slope).next_up().min(1.);
    let projected = corner_projection(controls, radius, direction, reverse)?;
    let mut lower = 0.;
    for _ in 0..12 {
        let mid = (lower + upper) / 2.;
        let [value] = *prefix(projected.clone(), mid, reverse).last()?;
        if value.0[0] > bound {
            upper = mid;
        } else {
            lower = mid;
        }
    }
    positive(upper).then_some(upper)
}

fn corner_projection(
    controls: &[[f64; 2]],
    radius: f64,
    direction: [f64; 2],
    reverse: bool,
) -> Option<Vec<[Range; 1]>> {
    let (anchor, direction) = if reverse {
        (*controls.last()?, direction.map(|q| -q))
    } else {
        (*controls.first()?, direction)
    };
    Some(
        controls
            .iter()
            .map(|p| {
                let angle = Range::point(p[0])
                    .difference(Range::point(anchor[0]))
                    .scale(radius)
                    .scale(direction[0]);
                let height = Range::point(p[1])
                    .difference(Range::point(anchor[1]))
                    .scale(direction[1]);
                [angle.sum(height)]
            })
            .collect(),
    )
}
