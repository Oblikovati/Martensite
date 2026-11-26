// SPDX-License-Identifier: GPL-2.0-only
//! Certified Bernstein orthogonality root isolation for B-spline and Bézier spans (#462).
use super::GGExtPC;
use crate::extrema::StationaryNature;
use crate::extrema::conic_classification::certified_minimum;
use crate::extrema::curve_tool::{BezierPoles, ExtremaCurveTool, SplineKnots};
use crate::extrema::gp_space::{GpPoint, GpVector};
use crate::gcpnts::PoleCurve;
use martensite_tkernel::standard::Failure;

const MAX_SUBDIVISION_DEPTH: u32 = 56;
const MAX_SUBDIVISION_NODES: usize = 8192;
const MAX_BERNSTEIN_DEGREE: usize = 60;

/// Isolates and certifies all interior stationary points of `C(u)` on a Bézier curve (#462).
pub(super) fn perform_bezier_certified<T: ExtremaCurveTool>(
    extrema: &mut GGExtPC<'_, T>,
    point: T::Point,
    curve: &T::Curve,
    bezier: &T::Bezier,
) -> Result<(), Failure> {
    let poles = bezier.homogeneous_poles();
    let rational = bezier.is_bezier_rational();
    let (u0, u1) = (T::first_parameter(curve)?, T::last_parameter(curve)?);
    let span_len = u1 - u0;
    if !span_len.is_finite() || span_len <= 0.0 {
        extrema.done = true;
        return Ok(());
    }
    let clipped_lo = extrema.uinf.max(u0);
    let clipped_hi = extrema.usup.min(u1);
    if clipped_hi <= clipped_lo {
        extrema.done = true;
        return Ok(());
    }
    let t0 = ((clipped_lo - u0) / span_len).clamp(0.0, 1.0);
    let t1 = ((clipped_hi - u0) / span_len).clamp(0.0, 1.0);
    let segment = trim_homogeneous_bezier(&to_weighted(&poles), t0, t1);
    let brackets = isolate_segment_roots(point, &segment, rational)?;
    record_certified_roots(extrema, point, curve, (clipped_lo, clipped_hi), &brackets)?;
    extrema.done = true;
    Ok(())
}

/// Isolates and certifies all stationary points across active B-spline spans (#462).
/// Hoists pole and flat-knot extraction once per curve instead of per span.
pub(super) fn perform_bspline_spans_certified<T: ExtremaCurveTool>(
    extrema: &mut GGExtPC<'_, T>,
    point: T::Point,
    curve: &T::Curve,
    spline: &T::BSpline,
    spans: &super::bspline::KnotSpans,
) -> Result<(), Failure> {
    let degree = spline.pole_degree() as usize;
    let poles = spline.homogeneous_poles();
    let flat_knots = spline.flat_knots();
    let periodic = spline.is_spline_periodic();
    let rational = spline.is_spline_rational();
    for index in spans.first..spans.last {
        let span_bounds = (spans.knot(index)?, spans.knot(index + 1)?);
        let clipped = spans.span(index, (extrema.uinf, extrema.usup), false)?;
        let (k_lo, k_hi) = (span_bounds.0 - spans.jump, span_bounds.1 - spans.jump);
        let span_len = k_hi - k_lo;
        if !span_len.is_finite() || span_len <= 0.0 || clipped.1 <= clipped.0 {
            continue;
        }
        let Ok(weighted) =
            extract_bspline_span_bezier(&poles, &flat_knots, degree, k_lo, k_hi, periodic)
        else {
            continue;
        };
        let t0 = ((clipped.0 - span_bounds.0) / span_len).clamp(0.0, 1.0);
        let t1 = ((clipped.1 - span_bounds.0) / span_len).clamp(0.0, 1.0);
        let segment = trim_homogeneous_bezier(&weighted, t0, t1);
        if let Ok(brackets) = isolate_segment_roots(point, &segment, rational) {
            let _ = record_certified_roots(extrema, point, curve, clipped, &brackets);
        }
    }
    Ok(())
}

fn to_weighted(poles: &[[f64; 4]]) -> Vec<[f64; 4]> {
    poles
        .iter()
        .map(|&[x, y, z, w]| [x * w, y * w, z * w, w])
        .collect()
}

fn extract_bspline_span_bezier(
    poles: &[[f64; 4]],
    flat_knots: &[f64],
    degree: usize,
    u_lo: f64,
    u_hi: f64,
    periodic: bool,
) -> Result<Vec<[f64; 4]>, Failure> {
    let tol = (u_hi - u_lo).abs() * 1e-12 + 1e-15; // tol:knot-span-lookup
    let span_idx = find_flat_span_index(flat_knots, degree, u_lo, tol).ok_or_else(|| {
        incomplete_failure("could not locate B-spline knot span in flat knot sequence")
    })?;
    let n_poles = poles.len();
    let mut ctrl: Vec<[f64; 4]> = (0..=degree)
        .map(|j| {
            let raw = span_idx - degree + j;
            let idx = if periodic {
                raw % n_poles
            } else {
                raw.min(n_poles - 1)
            };
            let [x, y, z, w] = poles[idx];
            [x * w, y * w, z * w, w]
        })
        .collect();
    let mut knots: Vec<f64> = (0..2 * degree)
        .map(|j| flat_knots[span_idx - degree + 1 + j])
        .collect();
    insert_knot_to_multiplicity(&mut ctrl, &mut knots, degree, u_lo, tol, true);
    insert_knot_to_multiplicity(&mut ctrl, &mut knots, degree, u_hi, tol, false);
    Ok(ctrl)
}

fn find_flat_span_index(flat_knots: &[f64], degree: usize, u_lo: f64, tol: f64) -> Option<usize> {
    let limit = flat_knots.len().checked_sub(degree + 1)?;
    if degree >= limit {
        return None;
    }
    let slice = &flat_knots[degree..=limit];
    let pos = slice.partition_point(|&k| k <= u_lo + tol);
    if pos == 0 || pos >= slice.len() {
        return None;
    }
    let idx = degree + pos - 1;
    (idx < limit && flat_knots[idx] <= u_lo + tol && flat_knots[idx + 1] > u_lo + tol)
        .then_some(idx)
}

fn insert_knot_to_multiplicity(
    ctrl: &mut [[f64; 4]],
    knots: &mut [f64],
    degree: usize,
    u: f64,
    tol: f64,
    left_end: bool,
) {
    let mut mult = knots.iter().filter(|&&k| (k - u).abs() <= tol).count();
    while mult < degree {
        if left_end {
            for j in 0..(degree - mult) {
                let k_left = knots[j];
                let k_right = knots[j + degree];
                let alpha = safe_ratio(u - k_left, k_right - k_left);
                ctrl[j] = lerp4(ctrl[j], ctrl[j + 1], alpha);
            }
            for j in 0..(degree - 1) {
                knots[j] = knots[j + 1];
            }
            knots[degree - 1] = u;
        } else {
            for j in ((mult + 1)..=degree).rev() {
                let k_left = knots[j - 1];
                let k_right = knots[j - 1 + degree];
                let alpha = safe_ratio(u - k_left, k_right - k_left);
                ctrl[j] = lerp4(ctrl[j - 1], ctrl[j], alpha);
            }
            for j in ((degree + 1)..(2 * degree)).rev() {
                knots[j] = knots[j - 1];
            }
            knots[degree] = u;
        }
        mult += 1;
    }
}

fn trim_homogeneous_bezier(ctrl: &[[f64; 4]], t0: f64, t1: f64) -> Vec<[f64; 4]> {
    let mut work = ctrl.to_vec();
    if t1 < 1.0 {
        let (left, _) = split_bezier4(&work, t1);
        work = left;
    }
    if t0 > 0.0 && t1 > 0.0 {
        let (_, right) = split_bezier4(&work, (t0 / t1).clamp(0.0, 1.0));
        work = right;
    }
    work
}

fn split_bezier4(ctrl: &[[f64; 4]], t: f64) -> (Vec<[f64; 4]>, Vec<[f64; 4]>) {
    let n = ctrl.len();
    let mut work = ctrl.to_vec();
    let mut left = Vec::with_capacity(n);
    let mut right = vec![[0.0; 4]; n];
    left.push(work[0]);
    right[n - 1] = work[n - 1];
    for r in 1..n {
        for i in 0..(n - r) {
            work[i] = lerp4(work[i], work[i + 1], t);
        }
        left.push(work[0]);
        right[n - 1 - r] = work[n - 1 - r];
    }
    (left, right)
}

fn isolate_segment_roots<P: GpPoint>(
    query: P,
    weighted: &[[f64; 4]],
    rational: bool,
) -> Result<Vec<(f64, f64)>, Failure> {
    let q = [
        query.coordinate(0),
        if P::DIMENSION > 1 {
            query.coordinate(1)
        } else {
            0.0
        },
        if P::DIMENSION > 2 {
            query.coordinate(2)
        } else {
            0.0
        },
    ];
    let poly = if rational {
        rational_orthogonality_bernstein(weighted, q)?
    } else {
        polynomial_orthogonality_bernstein(weighted, q)?
    };
    isolate_bernstein_roots(&poly)
}

fn polynomial_orthogonality_bernstein(
    weighted: &[[f64; 4]],
    q: [f64; 3],
) -> Result<Vec<f64>, Failure> {
    let d = weighted.len().saturating_sub(1);
    if d == 0 || 2 * d - 1 > MAX_BERNSTEIN_DEGREE {
        return Err(incomplete_failure(
            "spline degree exceeds certified Bernstein bound",
        ));
    }
    let diff: Vec<[f64; 3]> = weighted
        .iter()
        .map(|&[x, y, z, _]| [x - q[0], y - q[1], z - q[2]])
        .collect();
    let deriv: Vec<[f64; 3]> = (0..d)
        .map(|j| {
            [
                weighted[j + 1][0] - weighted[j][0],
                weighted[j + 1][1] - weighted[j][1],
                weighted[j + 1][2] - weighted[j][2],
            ]
        })
        .collect();
    Ok(bernstein_dot_product(&diff, &deriv))
}

fn rational_orthogonality_bernstein(
    weighted: &[[f64; 4]],
    q: [f64; 3],
) -> Result<Vec<f64>, Failure> {
    let d = weighted.len().saturating_sub(1);
    if d == 0 || 3 * d - 1 > MAX_BERNSTEIN_DEGREE {
        return Err(incomplete_failure(
            "rational spline degree exceeds certified Bernstein bound",
        ));
    }
    let num: Vec<[f64; 3]> = weighted.iter().map(|&[x, y, z, _]| [x, y, z]).collect();
    let w: Vec<f64> = weighted.iter().map(|&[_, _, _, w]| w).collect();
    let a: Vec<[f64; 3]> = weighted
        .iter()
        .map(|&[x, y, z, wi]| [x - wi * q[0], y - wi * q[1], z - wi * q[2]])
        .collect();
    let num_p: Vec<[f64; 3]> = (0..d)
        .map(|j| {
            [
                num[j + 1][0] - num[j][0],
                num[j + 1][1] - num[j][1],
                num[j + 1][2] - num[j][2],
            ]
        })
        .collect();
    let w_p: Vec<f64> = (0..d).map(|j| w[j + 1] - w[j]).collect();
    let t_vec = bernstein_vec_diff_mul(&num_p, &w, &num, &w_p);
    Ok(bernstein_dot_product(&a, &t_vec))
}

fn bernstein_vec_diff_mul(
    num_p: &[[f64; 3]],
    w: &[f64],
    num: &[[f64; 3]],
    w_p: &[f64],
) -> Vec<[f64; 3]> {
    let d1 = num_p.len() - 1;
    let d2 = w.len() - 1;
    let m = d1 + d2;
    let mut out = vec![[0.0; 3]; m + 1];
    for i in 0..=d1 {
        let ci = binomial(d1, i);
        for j in 0..=d2 {
            let factor = ci * binomial(d2, j) / binomial(m, i + j);
            for axis in 0..3 {
                out[i + j][axis] += factor * (num_p[i][axis] * w[j] - num[j][axis] * w_p[i]);
            }
        }
    }
    out
}

fn bernstein_dot_product(a: &[[f64; 3]], b: &[[f64; 3]]) -> Vec<f64> {
    let da = a.len() - 1;
    let db = b.len() - 1;
    let m = da + db;
    let mut out = vec![0.0; m + 1];
    for i in 0..=da {
        let ci = binomial(da, i);
        for j in 0..=db {
            let factor = ci * binomial(db, j) / binomial(m, i + j);
            let dot = a[i][0] * b[j][0] + a[i][1] * b[j][1] + a[i][2] * b[j][2];
            out[i + j] += factor * dot;
        }
    }
    out
}

fn isolate_bernstein_roots(coeffs: &[f64]) -> Result<Vec<(f64, f64)>, Failure> {
    let scale = coeffs.iter().fold(0.0_f64, |acc, &v| acc.max(v.abs()));
    if !scale.is_finite() {
        return Err(incomplete_failure("non-finite orthogonality polynomial"));
    }
    if scale == 0.0 {
        // Constant-distance or degenerate span: endpoints capture the extremum without failing.
        return Ok(vec![(0.0, 0.0), (1.0, 1.0)]);
    }
    let normalized: Vec<f64> = coeffs.iter().map(|&v| v / scale).collect();
    let mut stack = vec![(0.0_f64, 1.0_f64, normalized.clone(), 0_u32)];
    let mut roots = Vec::new();
    let end_tol = 1e-11; // tol:normalized-bernstein-endpoint
    if normalized[0].abs() <= end_tol {
        roots.push((0.0, 0.0));
    }
    if normalized[normalized.len() - 1].abs() <= end_tol {
        roots.push((1.0, 1.0));
    }
    let mut visited = 0_usize;
    while let Some((lo, hi, poly, depth)) = stack.pop() {
        visited += 1;
        if visited > MAX_SUBDIVISION_NODES {
            return Err(incomplete_failure(
                "Bernstein subdivision budget exhausted before certifying all roots",
            ));
        }
        let variations = count_sign_variations(&poly);
        let has_zero = poly.contains(&0.0);
        if variations == 0 && !has_zero {
            continue;
        }
        let m = poly.len() - 1;
        if variations == 1 && poly[0] * poly[m] < 0.0 {
            roots.push((lo, hi));
            continue;
        }
        if depth >= MAX_SUBDIVISION_DEPTH || (hi - lo) <= 1e-13 {
            if poly[0] * poly[m] <= 0.0 || has_zero {
                roots.push((lo, hi));
                continue;
            }
            if variations >= 2 {
                return Err(incomplete_failure(
                    "unresolved closely-spaced stationary roots below subdivision limit",
                ));
            }
            continue;
        }
        let mid = 0.5 * (lo + hi);
        let (left, right) = split_bernstein_scalar(&poly);
        stack.push((mid, hi, right, depth + 1));
        stack.push((lo, mid, left, depth + 1));
    }
    roots.sort_by(|a, b| a.0.total_cmp(&b.0));
    Ok(roots)
}

fn count_sign_variations(coeffs: &[f64]) -> usize {
    let mut prev_sign = 0_i8;
    let mut count = 0_usize;
    for &c in coeffs {
        let sign = if c > 0.0 {
            1
        } else if c < 0.0 {
            -1
        } else {
            0
        };
        if sign != 0 {
            if prev_sign != 0 && sign != prev_sign {
                count += 1;
            }
            prev_sign = sign;
        }
    }
    count
}

fn split_bernstein_scalar(coeffs: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n = coeffs.len();
    let mut work = coeffs.to_vec();
    let mut left = Vec::with_capacity(n);
    let mut right = vec![0.0; n];
    left.push(work[0]);
    right[n - 1] = work[n - 1];
    for r in 1..n {
        for i in 0..(n - r) {
            work[i] = 0.5 * (work[i] + work[i + 1]);
        }
        left.push(work[0]);
        right[n - 1 - r] = work[n - 1 - r];
    }
    (left, right)
}

fn record_certified_roots<T: ExtremaCurveTool>(
    extrema: &mut GGExtPC<'_, T>,
    query: T::Point,
    curve: &T::Curve,
    (u_start, u_end): (f64, f64),
    brackets: &[(f64, f64)],
) -> Result<(), Failure> {
    let span = u_end - u_start;
    for &(s_lo, s_hi) in brackets {
        let lo = u_start + s_lo * span;
        let hi = u_start + s_hi * span;
        let Some(u) = refine_stationary_root::<T>(query, curve, lo, hi, extrema.tol_u)? else {
            continue;
        };
        let u = extrema.periodic_parameter(curve, u)?;
        if u < extrema.uinf - extrema.tol_u || u > extrema.usup + extrema.tol_u {
            continue;
        }
        let (pos, d1, d2) = T::d2(curve, u)?;
        let sq_dist = query.square_distance_to(&pos);
        let nature = match certified_minimum(query, u, (pos, d1, d2)) {
            Ok(is_min) => StationaryNature::from(is_min),
            Err(error) => StationaryNature::Unresolved(error),
        };
        // Avoid adding uncertified roots when supplementing sampled spans (#462).
        if matches!(nature, StationaryNature::Unresolved(_)) {
            continue;
        }
        extrema.add_classified_sol(u, pos, sq_dist, nature);
    }
    Ok(())
}

fn refine_stationary_root<T: ExtremaCurveTool>(
    query: T::Point,
    curve: &T::Curve,
    mut lo: f64,
    mut hi: f64,
    tol_u: f64,
) -> Result<Option<f64>, Failure> {
    let mut f_lo = ortho_eval::<T>(query, curve, lo)?;
    if (hi - lo).abs() == 0.0 {
        let (_, d1) = T::d1(curve, lo)?;
        let scale = d1.norm().max(1.0);
        return Ok((f_lo.abs() <= 1e-9 * scale).then_some(lo));
    }
    let f_hi = ortho_eval::<T>(query, curve, hi)?;
    let bracketed = f_lo * f_hi < 0.0;
    let mut u = 0.5 * (lo + hi);
    let stop = tol_u.clamp(1e-14, 1e-12);
    for _ in 0..64 {
        let (pos, d1, d2) = T::d2(curve, u)?;
        let diff = query.vector_to(&pos);
        let f = diff.dot_with(&d1);
        let df = d1.dot_with(&d1) + diff.dot_with(&d2);
        if bracketed {
            if f * f_lo <= 0.0 {
                hi = u;
            } else {
                lo = u;
                f_lo = f;
            }
        }
        if (hi - lo).abs() <= stop {
            break;
        }
        let newton = if df.abs() > 0.0 { u - f / df } else { f64::NAN };
        u = if newton.is_finite() && newton > lo && newton < hi {
            newton
        } else {
            0.5 * (lo + hi)
        };
    }
    let (pos, d1) = T::d1(curve, u)?;
    let diff = query.vector_to(&pos);
    let f_final = diff.dot_with(&d1);
    let scale = (diff.norm() * d1.norm()).max(1.0);
    if !bracketed && f_final.abs() > 1e-8 * scale {
        return Ok(None);
    }
    Ok(Some(u))
}

fn ortho_eval<T: ExtremaCurveTool>(
    query: T::Point,
    curve: &T::Curve,
    u: f64,
) -> Result<f64, Failure> {
    let (pos, d1) = T::d1(curve, u)?;
    Ok(query.vector_to(&pos).dot_with(&d1))
}

fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut result = 1.0_f64;
    for i in 1..=k {
        result = result * ((n - k + i) as f64) / (i as f64);
    }
    result
}

fn lerp4(a: [f64; 4], b: [f64; 4], t: f64) -> [f64; 4] {
    let s = 1.0 - t;
    [
        s * a[0] + t * b[0],
        s * a[1] + t * b[1],
        s * a[2] + t * b[2],
        s * a[3] + t * b[3],
    ]
}

fn safe_ratio(num: f64, den: f64) -> f64 {
    if den.abs() > 0.0 { num / den } else { 0.0 }
}

fn incomplete_failure(detail: &str) -> Failure {
    Failure::with_message("Extrema_IncompleteSplineExtrema", detail)
}
