// SPDX-License-Identifier: GPL-2.0-only
//! Hermite remainder sizing before any native fit or conformance proof is attempted.
use super::{algebra::*, *};

/// Choose a chart center and dyadic level from a fourth-derivative bound.
/// V1 uses fixed 256-bit planning arithmetic, independent of proof precision/work receipts.
/// Example: `let plan = plan_cylindrical_fit(query, || false)?;`.
pub fn plan_cylindrical_fit(
    query: CylindricalFitQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CylindricalFitPlan, CylindricalProfileError> {
    let options = CylindricalProfileOptions {
        fractional_bits: 256,
        max_operations: query.max_operations,
        ..Default::default()
    };
    execute(query, &mut cancelled).map_err(|cause| CylindricalProfileError {
        requested_options: options,
        requested_spatial_limit: query.spatial_limit,
        cause,
    })
}

fn execute(
    query: CylindricalFitQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CylindricalFitPlan> {
    admit(query)?;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 256,
            max_operations: query.max_operations,
        },
        cancelled,
    );
    planned_geometry(query, &mut arithmetic)
}

fn planned_geometry(
    query: CylindricalFitQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CylindricalFitPlan> {
    let source = source::developed(
        query.authored,
        query.interval,
        query.support_radius,
        query.angular_origin,
        arithmetic,
    )?;
    let range = hull(&source[0]);
    let center = arithmetic.upper_f64((&range.lower + &range.upper) / 2)?;
    let derivative = fourth_bound(&source[0], center, arithmetic)?;
    let radius = arithmetic.scalar(query.support_radius)?;
    let error = arithmetic.multiply(&radius, &derivative)?;
    let error = arithmetic.fraction(&error, 1, 192)?;
    select(query, center, error, arithmetic)
}

fn admit(query: CylindricalFitQuery<'_>) -> Checked<()> {
    if u16::from(query.interval.level()) + u16::from(query.max_level) > 63
        || query.max_operations > 1_000_000
    {
        return Err(CylindricalProfileFailure::FitPolicy {
            maximum_level: query.max_level,
            maximum_operations: query.max_operations,
        });
    }
    admission::count("authored poles", query.authored.len(), 4)?;
    admission::value("support radius", 0, query.support_radius, true)?;
    admission::value("angular origin", 0, query.angular_origin, false)?;
    admit_limit(query.spatial_limit)?;
    for (index, value) in query.authored.iter().flatten().enumerate() {
        admission::value("authored coordinate", index, *value, false)?;
    }
    Ok(())
}

fn admit_limit(limit: f64) -> Checked<()> {
    if !limit.is_finite() || limit < 0. {
        return Err(CylindricalProfileFailure::Value {
            role: "spatial limit",
            index: 0,
            value: limit,
        });
    }
    Ok(())
}

fn select(
    query: CylindricalFitQuery<'_>,
    center: f64,
    error: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CylindricalFitPlan> {
    let (level, error) = sized_error(query, error, arithmetic)?;
    Ok(CylindricalFitPlan::assemble(
        level,
        center,
        arithmetic.upper_f64(error.upper)?,
        query,
        arithmetic.operations(),
    ))
}

fn sized_error(
    query: CylindricalFitQuery<'_>,
    mut error: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(u8, Interval)> {
    let limit = arithmetic.scalar(query.spatial_limit)?;
    let allowance = arithmetic.fraction(&limit, 1, 2)?;
    let mut level = 0;
    while error.upper > allowance.lower {
        admit_level(level, &error, query, arithmetic)?;
        error = arithmetic.fraction(&error, 1, 16)?;
        level += 1;
    }
    Ok((level, error))
}

fn admit_level(
    level: u8,
    error: &Interval,
    query: CylindricalFitQuery<'_>,
    arithmetic: &Arithmetic<'_>,
) -> Checked<()> {
    if level == query.max_level {
        return Err(CylindricalProfileFailure::FitLevel {
            maximum: level,
            bound: arithmetic.upper_f64(error.upper.clone())?,
            requested: query.spatial_limit,
        });
    }
    Ok(())
}

fn fourth_bound(
    phase: &[Interval],
    center: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let tangent = tangent_bound(phase, center, arithmetic)?;
    let g = tangent_derivatives(&tangent, arithmetic)?;
    let d1 = derivative(phase, arithmetic)?;
    let d2 = derivative(&d1, arithmetic)?;
    let d3 = derivative(&d2, arithmetic)?;
    chain_bound(
        [magnitude(&d1), magnitude(&d2), magnitude(&d3)],
        g,
        arithmetic,
    )
}

fn tangent_bound(
    phase: &[Interval],
    center: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let c = arithmetic.scalar(center)?;
    let local = phase
        .iter()
        .map(|p| Ok(arithmetic.subtract(p, &c)?))
        .collect::<Checked<Vec<_>>>()?;
    let half = arithmetic.fraction(&magnitude(&local), 1, 2)?;
    let square = arithmetic.multiply(&half, &half)?;
    let square = arithmetic.fraction(&square, 1, 2)?;
    let one = arithmetic.scalar(1.)?;
    let cosine = arithmetic.subtract(&one, &square)?;
    let tangent = arithmetic.divide(&half, &cosine)?;
    Ok(point_upper(&tangent))
}

fn tangent_derivatives(t: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 3]> {
    let one = arithmetic.scalar(1.)?;
    let t2 = arithmetic.multiply(t, t)?;
    let a = arithmetic.add(&one, &t2)?;
    let g2 = arithmetic.multiply(t, &a)?;
    let g2 = arithmetic.fraction(&g2, 1, 2)?;
    let b = arithmetic.fraction(&t2, 3, 1)?;
    let b = arithmetic.add(&one, &b)?;
    let g3 = arithmetic.multiply(&a, &b)?;
    let g3 = arithmetic.fraction(&g3, 1, 4)?;
    let b = arithmetic.add(&one, &b)?;
    let b = arithmetic.multiply(t, &b)?;
    let g4 = arithmetic.multiply(&a, &b)?;
    Ok([g2, g3, arithmetic.fraction(&g4, 1, 2)?])
}

fn chain_bound(
    d: [Interval; 3],
    g: [Interval; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let m1_squared = arithmetic.multiply(&d[0], &d[0])?;
    let m1_fourth = arithmetic.multiply(&m1_squared, &m1_squared)?;
    let a = arithmetic.multiply(&g[2], &m1_fourth)?;
    let b = arithmetic.multiply(&m1_squared, &d[1])?;
    let b = arithmetic.multiply(&g[1], &b)?;
    let b = arithmetic.fraction(&b, 6, 1)?;
    let c = arithmetic.multiply(&d[1], &d[1])?;
    let c = arithmetic.multiply(&g[0], &c)?;
    let c = arithmetic.fraction(&c, 3, 1)?;
    let e = arithmetic.multiply(&d[0], &d[2])?;
    let e = arithmetic.multiply(&g[0], &e)?;
    let e = arithmetic.fraction(&e, 4, 1)?;
    let left = arithmetic.add(&a, &b)?;
    let right = arithmetic.add(&c, &e)?;
    Ok(arithmetic.add(&left, &right)?)
}
