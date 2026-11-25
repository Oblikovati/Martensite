// SPDX-License-Identifier: GPL-2.0-only
//! Reuse only identical retained trigonometric inputs; each depth keeps its own residual.
use super::*;

pub(super) struct CapturedTrace {
    uv: [Vec<Interval>; 2],
    trig: [Vec<Interval>; 2],
    angle: [Vec<Interval>; 2],
}
#[derive(Clone, Copy)]
pub(super) struct Reuse<'a, 'b> {
    pub(super) query: ConicalTrimQuery<'a>,
    pub(super) captured: &'b CapturedTrace,
}
impl Reuse<'_, '_> {
    fn same_uv(self, query: ConicalTrimQuery<'_>) -> bool {
        self.query.pcurve.len() == query.pcurve.len()
            && self
                .query
                .pcurve
                .iter()
                .flatten()
                .map(|v| v.to_bits())
                .eq(query.pcurve.iter().flatten().map(|v| v.to_bits()))
    }
    fn same_angle(self, query: ConicalTrimQuery<'_>) -> bool {
        self.query.carrier.semi_angle.to_bits() == query.carrier.semi_angle.to_bits()
    }
}

pub(super) fn residual(
    query: ConicalTrimQuery<'_>,
    reuse: Option<Reuse<'_, '_>>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([f64; 3], CapturedTrace)> {
    let uv = captured_uv(query, reuse, arithmetic)?;
    let stored = source::stored(query.curve, arithmetic)?;
    let weight = proof::minimum_weight(&stored);
    let captured = prepare(query, uv, reuse, arithmetic)?;
    let bounds = residual_bounds(query, &captured, &stored, &weight, arithmetic)?;
    Ok((bounds, captured))
}

pub(super) fn ranged_residual(
    query: ConicalTrimQuery<'_>,
    [space, chart]: [[f64; 2]; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let uv = narrowed(capture_planar(query.pcurve, arithmetic)?, chart, arithmetic)?;
    let stored = narrowed(source::stored(query.curve, arithmetic)?, space, arithmetic)?;
    let weight = proof::minimum_weight(&stored);
    let captured = prepare(query, uv, None, arithmetic)?;
    residual_bounds(query, &captured, &stored, &weight, arithmetic)
}

fn captured_uv(
    query: ConicalTrimQuery<'_>,
    reuse: Option<Reuse<'_, '_>>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    if let Some(reuse) = reuse.filter(|prior| prior.same_uv(query)) {
        return Ok(reuse.captured.uv.clone());
    }
    Ok(capture_planar(query.pcurve, arithmetic)?)
}

fn prepare(
    query: ConicalTrimQuery<'_>,
    uv: [Vec<Interval>; 2],
    reuse: Option<Reuse<'_, '_>>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CapturedTrace> {
    let trig = match reuse.filter(|prior| prior.same_uv(query)) {
        Some(prior) => prior.captured.trig.clone(),
        None => trigonometric::enclose(&uv[0], arithmetic)?,
    };
    let angle = match reuse.filter(|prior| prior.same_angle(query)) {
        Some(prior) => prior.captured.angle.clone(),
        None => capture_angle(query.carrier.semi_angle, arithmetic)?,
    };
    Ok(CapturedTrace { uv, trig, angle })
}

fn capture_angle(angle: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<[Vec<Interval>; 2]> {
    let angle = arithmetic.scalar(angle)?;
    trigonometric::enclose(&[angle], arithmetic)
}

// A downward numerator share admits a genuine full hull; the final divided sum
// remains authoritative regardless of whether refinement was necessary.
fn residual_bounds(
    query: ConicalTrimQuery<'_>,
    captured: &CapturedTrace,
    stored: &[Vec<Interval>; 4],
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let [radius, height] = meridian(query.carrier, captured, arithmetic)?;
    let scaled = polynomial::product(&stored[3], &radius, arithmetic)?;
    let cutoff = numerator_cutoff(query.spatial_limit, weight, arithmetic)?;
    let xy = proof::scaled_xy_residual_conditioned(
        &scaled,
        stored,
        &captured.trig,
        weight,
        Some(&cutoff),
        arithmetic,
    )?;
    let height =
        proof::height_residual_conditioned(&height, stored, weight, Some(&cutoff), arithmetic)?;
    let total = arithmetic.add(&xy, &height)?;
    Ok([
        arithmetic.upper_f64(xy.upper)?,
        arithmetic.upper_f64(height.upper)?,
        arithmetic.upper_f64(total.upper)?,
    ])
}

fn meridian(
    carrier: ConicalTrimCarrier,
    captured: &CapturedTrace,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let [cosine, sine] = &captured.angle;
    let radius = affine(&captured.uv[1], &sine[0], carrier.radius, arithmetic)?;
    positive_radius(&radius, carrier.radius, arithmetic)?;
    let height = affine(
        &captured.uv[1],
        &cosine[0],
        carrier.axial_origin,
        arithmetic,
    )?;
    Ok([radius, height])
}

fn numerator_cutoff(
    limit: f64,
    weight: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let requested = arithmetic.scalar(limit)?;
    let numerator = arithmetic.multiply(&requested, weight)?;
    let share = arithmetic.fraction(&numerator, 1, 4)?;
    Ok(Interval {
        lower: share.lower.clone(),
        upper: share.lower,
    })
}

#[cfg(test)]
#[path = "capture/requested_bounds_tests.rs"]
mod requested_bounds_tests;
