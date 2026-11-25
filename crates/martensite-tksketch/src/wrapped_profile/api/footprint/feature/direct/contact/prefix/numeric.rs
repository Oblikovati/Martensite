//! Prove the allowed contact paths within the qualified stock chart.
use super::*;

pub(super) fn positive(x: f64) -> bool {
    x.is_finite() && x > 0.
}
pub(super) fn magnitude(x: f64) -> bool {
    x.is_finite() && x >= 0.
}
pub(super) fn maximum(values: impl Iterator<Item = f64>) -> f64 {
    values.fold(0., f64::max)
}
pub(super) fn minimum(values: impl Iterator<Item = f64>) -> f64 {
    values.fold(f64::INFINITY, f64::min)
}
pub(super) fn vertex_guard(input: &Input<'_>) -> f64 {
    maximum(input.sides.iter().flat_map(|side| side.vertex_corner_upper))
}
pub(super) fn max_wall(input: &Input<'_>) -> f64 {
    maximum(input.sides.iter().map(|side| side.xyz_anchor_pad_upper))
}

pub(super) struct Thresholds {
    pub(super) lo: [f64; 4],
    pub(super) hi: [f64; 4],
}
impl Thresholds {
    pub(super) fn min(&self) -> f64 {
        minimum(self.lo.into_iter())
    }
    pub(super) fn max(&self) -> f64 {
        maximum(self.hi.into_iter())
    }
}

pub(super) fn thresholds(input: &Input<'_>) -> Result<Thresholds, Failure> {
    if !positive(input.fuzzy)
        || !positive(input.collar_threshold)
        || input.sides.is_empty()
        || input.stock.is_empty()
    {
        return Err(Failure::InvalidScalar);
    }
    let mut result = Thresholds {
        lo: [f64::INFINITY; 4],
        hi: [0.; 4],
    };
    for face in input.stock {
        for side in input.sides {
            let half = input.fuzzy / 2.;
            let pair = |tool| (face.tolerance + half) + (tool + half);
            let ranges = [
                [pair(side.face_tolerance); 2],
                [pair(side.rail_tolerance); 2],
                side.ruling_tolerance.map(pair),
                side.vertex_tolerance
                    .map(|tol| (face.tolerance + tol) + input.fuzzy),
            ];
            if !positive(face.tolerance)
                || [
                    side.face_tolerance,
                    side.rail_tolerance,
                    side.ruling_tolerance[0],
                    side.ruling_tolerance[1],
                    side.vertex_tolerance[0],
                    side.vertex_tolerance[1],
                ]
                .into_iter()
                .any(|x| !positive(x))
                || ranges
                    .iter()
                    .flatten()
                    .any(|x| !positive(*x) || *x > input.collar_threshold)
            {
                return Err(Failure::InvalidScalar);
            }
            for (i, values) in ranges.into_iter().enumerate() {
                result.lo[i] = result.lo[i].min(minimum(values.into_iter()));
                result.hi[i] = result.hi[i].max(maximum(values.into_iter()));
            }
        }
    }
    Ok(result)
}

#[derive(Clone, Copy)]
pub(super) struct Convex {
    pub(super) moving_hessian: f64,
    pub(super) vertex_hessian: f64,
    pub(super) angular_metric_lower: f64,
}

pub(super) fn convexity(input: &Input<'_>, geo: &Geometry) -> Result<Convex, Failure> {
    let c = input.cone;
    if !positive(c.radius)
        || !(0. < c.sin[0] && c.sin[0] <= c.sin[1] && c.sin[1] < 1.)
        || !(0. <= c.slant[0] && c.slant[0] < c.slant[1] && c.slant[1].is_finite())
        || !c.chart_angle.into_iter().all(f64::is_finite)
    {
        return Err(Failure::InvalidScalar);
    }
    validate_side_metrics(input.sides)?;
    let speed = I::p(maximum(input.sides.iter().map(|s| s.speed_upper)));
    let depth = I::p(maximum(input.sides.iter().map(|s| s.depth_upper)));
    let sew = I::p(maximum(input.sides.iter().map(|s| s.sew_upper)));
    let gain = I::p(minimum(
        input.sides.iter().map(|s| s.signed_depth_rate_lower),
    ));
    let guarded = I::p(vertex_guard(input));
    let stock_min = I::p(c.radius) + I::p(c.slant[0]) * I::p(c.sin[0]);
    let stock_max = I::p(c.radius) + I::p(c.slant[1]) * I::p(c.sin[1]);
    // A common conservative radius (the smaller of the two near-point radii).
    let tool_min =
        I::p(c.radius) + I::p(geo.ranges.lo[1]) * I::p(c.sin[0]) - speed * depth - sew - guarded;
    let minimum_radius = I::p(tool_min.0.min(stock_min.0));
    let angular = minimum_radius * (I::p(7.) / I::p(11.));
    if !positive(angular.0) {
        return Err(Failure::Pad);
    }
    let tau = I::p(input.collar_threshold) + guarded;
    let pad = [0, 1].map(|i| {
        I::p(maximum(
            input.sides.iter().map(|side| side.uv_anchor_pad_upper[i]),
        ))
    });
    let box_pad = [tau / I::p(angular.0) + pad[0], tau + pad[1]];
    check_box(geo.ranges, c, box_pad)?;
    let gamma = gain - speed * (box_pad[0] + pad[0]);
    if !positive(gamma.0) {
        return Err(Failure::NonConvex);
    }
    let b = I::p(1.) + speed / gamma;
    let gram = I::p(1.)
        / (I::p(1.) / gamma.square() + b.square() * (I::p(1.) + I::p(1.) / stock_min.square()));
    let residual = speed * depth + sew + guarded + stock_max * box_pad[0] + box_pad[1];
    let error = residual * (stock_max + I::p(c.sin[1]));
    let moving = gram - error;
    let fixed = I::p(stock_min.0.min(1.)).square() - error;
    if !positive(moving.0) || !positive(fixed.0) {
        return Err(Failure::NonConvex);
    }
    Ok(Convex {
        moving_hessian: moving.0,
        vertex_hessian: fixed.0,
        angular_metric_lower: angular.0,
    })
}

pub(super) fn validate_side_metrics(sides: &[Side]) -> Result<(), Failure> {
    for s in sides {
        let p = [
            s.depth_upper,
            s.signed_depth_rate_lower,
            s.speed_upper,
            s.sew_upper,
            s.own_margin_lower,
        ];
        let m = [
            s.own_ratio_upper,
            s.xyz_anchor_pad_upper,
            s.uv_anchor_pad_upper[0],
            s.uv_anchor_pad_upper[1],
            s.vertex_corner_upper[0],
            s.vertex_corner_upper[1],
        ];
        if p.into_iter().any(|x| !positive(x))
            || m.into_iter().any(|x| !magnitude(x))
            || s.depth_upper >= 1.
            || s.sew_upper > s.xyz_anchor_pad_upper
        {
            return Err(Failure::InvalidScalar);
        }
    }
    Ok(())
}

pub(super) fn check_box(ranges: Ranges, cone: Cone, pad: [I; 2]) -> Result<(), Failure> {
    let low = [cone.chart_angle[0], cone.slant[0]];
    let high = [cone.chart_angle[1], cone.slant[1]];
    for i in 0..2 {
        let lo = I::p(ranges.lo[i]) - pad[i];
        let hi = I::p(ranges.hi[i]) + pad[i];
        if !lo.valid() || !hi.valid() || lo.0 <= low[i] || high[i] <= hi.1 {
            return Err(Failure::Pad);
        }
    }
    // The angular chord estimate is for a principal sheet with width less than pi.
    let width = (I::p(cone.chart_angle[1]) - I::p(cone.chart_angle[0])).1;
    if width.partial_cmp(&std::f64::consts::PI) != Some(std::cmp::Ordering::Less) {
        return Err(Failure::Pad);
    }
    Ok(())
}

pub(super) fn foreign_upper(census_weight: f64, available_metric: f64, hit: f64) -> f64 {
    let ratio = (I::p(census_weight) / I::p(available_metric)).1.max(1.);
    (I::p(ratio) * I::p(hit)).1
}

pub(super) fn verify_separations(
    input: &Input<'_>,
    geo: &Geometry,
    c: Convex,
    metric: metric::Bounds,
    hit: f64,
    threshold: f64,
) -> Result<(), Failure> {
    let s = input.separation;
    let lower = [
        s.all_nonincident_lower,
        s.all_cross_contour_lower,
        s.all_foreign_face_lower,
        s.native_rim_lower,
        s.native_periodic_lower,
        s.endpoint_window_threshold,
        s.weighted_angle_scale,
    ];
    if lower.into_iter().any(|x| !positive(x)) || !positive(hit) {
        return Err(Failure::InvalidScalar);
    }
    let pairs = geo
        .loops
        .checked_mul(geo.loops.saturating_sub(1))
        .ok_or(Failure::Census)?
        / 2;
    if pairs != s.excluded_loop_pairs {
        return Err(Failure::Census);
    }
    let foreign = foreign_upper(metric.angle_scale, c.angular_metric_lower, hit);
    if foreign >= s.endpoint_window_threshold
        || foreign >= s.all_nonincident_lower.min(metric.nonincident)
        || foreign >= s.all_cross_contour_lower.min(metric.cross_loop)
        || foreign >= s.all_foreign_face_lower.min(metric.foreign)
        || threshold >= s.native_rim_lower
        || threshold >= s.native_periodic_lower
    {
        return Err(Failure::Separation);
    }
    Ok(())
}

pub(super) fn own_cores(input: &Input<'_>) -> Result<f64, Failure> {
    let mut upper = 0_f64;
    for side in input.sides {
        let e = I::p(side.sew_upper) + I::p(maximum(side.vertex_corner_upper.into_iter()));
        // Only the displacement term in the earlier ratio increases for a fixed stored point.
        // Scaling the entire ratio by the increased collar threshold safely includes it.
        let relative = I::p(1.)
            + I::p(maximum(side.vertex_corner_upper.into_iter())) / I::p(input.collar_threshold);
        let q = I::p((I::p(side.own_ratio_upper) * relative).1);
        let mu = I::p(1.) - I::p(2.) * q - q.square();
        let margin = I::p(mu.0.min(side.own_margin_lower));
        if !positive(margin.0) {
            return Err(Failure::Core);
        }
        let core = e * (I::p(1.) + (I::p(1.) + q).square() / margin);
        if !core.valid() {
            return Err(Failure::Core);
        }
        upper = upper.max(core.1);
    }
    Ok(upper)
}

pub(super) fn incident_cores(input: &Input<'_>, geo: &Geometry) -> Result<f64, Failure> {
    let windows = validate_windows(input, geo)?;
    let mut seen = BTreeSet::new();
    let guard = vertex_guard(input);
    let mut upper = 0_f64;
    for a in input.arms {
        if a.source_end > 1
            || windows.get(&a.pair) != Some(&a.window_fraction.to_bits())
            || !seen.insert((a.pair, a.source_end))
        {
            return Err(Failure::Identity);
        }
        upper = upper.max(incident_core(*a, guard)?);
    }
    if seen.len() != 2 * windows.len() {
        return Err(Failure::Census);
    }
    Ok(upper)
}

pub(super) fn validate_windows(
    input: &Input<'_>,
    geo: &Geometry,
) -> Result<BTreeMap<Incidence, u64>, Failure> {
    let mut supplied = BTreeMap::new();
    for w in input.windows {
        if !positive(w.fraction)
            || w.fraction >= 1.
            || supplied.insert(w.pair, w.fraction.to_bits()).is_some()
        {
            return Err(Failure::Identity);
        }
    }
    if supplied.keys().copied().collect::<BTreeSet<_>>() != geo.incident {
        return Err(Failure::Census);
    }
    Ok(supplied)
}

pub(super) fn incident_core(a: ArmProof, guard: f64) -> Result<f64, Failure> {
    if !positive(a.linear_min_lower)
        || !positive(a.linear_sum_upper)
        || [
            a.quadratic_upper,
            a.derivative_remainder_upper,
            a.anchor_upper,
        ]
        .into_iter()
        .any(|n| !magnitude(n))
    {
        return Err(Failure::InvalidScalar);
    }
    let m = I::p(a.linear_min_lower);
    let l = I::p(a.linear_sum_upper);
    let q = I::p(a.quadratic_upper);
    let r = I::p(a.derivative_remainder_upper);
    let e = I::p(a.anchor_upper) + I::p(guard);
    if (m * I::p(2.)).0 < (q + r).1 {
        return Err(Failure::Core);
    }
    let margin = m.square() - m * (q + r) - q * r;
    if !positive(margin.0) {
        return Err(Failure::Core);
    }
    let radius = I::p(2.) * e * (l + r) / margin;
    let center = e + (l + q) * radius;
    if !center.valid() || radius.1 >= 1. {
        return Err(Failure::Core);
    }
    Ok(center.1)
}
