// SPDX-License-Identifier: GPL-2.0-only
//! Coordinate charts, derivative intervals and whole-domain speed witnesses.
use super::{
    AngularChartQuantity as Quantity, AngularEquationCertificate, AngularEquationError as Error,
    AngularEquationMode as Mode, Arithmetic, MAX_DEPTH, Net, R,
    interval::{bounds, derivative, identically_zero, positive_lower, speed_lower},
    refusal::Checked,
    series::{self, FACTORIAL, Series, WorldJet},
};
use crate::predicates::roots::certificate_arithmetic::{
    bounded_dyadic::Interval, circular_constants,
};
use num_bigint::Sign;

pub(super) fn domain(
    mode: Mode,
    rows: &[Net; 4],
    depth: u8,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<AngularEquationCertificate> {
    let input = range_laws(rows, arithmetic)?;
    let leaf = match chart(mode, &input, arithmetic)? {
        Some(quantity) => {
            check_chart_endpoints(quantity, &rows[0], arithmetic)?;
            Err(Error::ChartUnproven(quantity))
        }
        None if stationary_endpoint(&rows[1]) => Err(Error::Singular),
        None => certify_leaf(mode, &input, arithmetic),
    };
    let reason = match leaf {
        Ok(certificate) => return Ok(certificate),
        Err(reason @ (Error::ChartUnproven(_) | Error::RegularityUnproven)) => reason,
        Err(other) => return Err(other),
    };
    if depth == MAX_DEPTH {
        return Err(reason);
    }
    let [left, right] = bisect(rows);
    let left = domain(mode, &left, depth + 1, arithmetic)?;
    let right = domain(mode, &right, depth + 1, arithmetic)?;
    Ok(merge(left, right))
}

fn check_chart_endpoints(
    quantity: Quantity,
    position: &Net,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<()> {
    let axis = match quantity {
        Quantity::Radius => &position.0[0],
        Quantity::Inclination => &position.0[1],
    };
    let ends = [&axis[0], &axis[axis.len() - 1]];
    if ends.iter().any(|value| !value.sign().is_gt()) {
        return Err(Error::ChartViolation(quantity));
    }
    if quantity == Quantity::Inclination {
        let pi = circular_constants::pi(arithmetic)?;
        for value in ends {
            if arithmetic.rational(value)?.lower >= pi.upper {
                return Err(Error::ChartViolation(quantity));
            }
        }
    }
    Ok(())
}

fn stationary_endpoint(row: &Net) -> bool {
    row.0.iter().all(|axis| axis[0].sign().is_eq())
        || row.0.iter().all(|axis| axis[axis.len() - 1].sign().is_eq())
}

fn certify_leaf(
    mode: Mode,
    input: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<AngularEquationCertificate> {
    let (jet, rates) = world(mode, input, arithmetic)?;
    if rates.iter().all(identically_zero) {
        return Err(Error::Singular);
    }
    let speed = speed_lower(
        rates.iter().map(positive_lower).max().unwrap_or_default(),
        arithmetic,
    )?;
    let mut coordinate_bounds = [[[0.; 2]; 3]; 5];
    for (order, output) in coordinate_bounds.iter_mut().enumerate() {
        for (series, bound) in jet.iter().zip(output) {
            *bound = bounds(&derivative(series, order, arithmetic)?, arithmetic)?;
        }
    }
    Ok(AngularEquationCertificate {
        coordinate_bounds,
        speed_lower: speed,
    })
}

fn chart(
    mode: Mode,
    input: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Option<Quantity>> {
    let radius = &input[0][0];
    if radius.upper.sign() != Sign::Plus {
        return Err(Error::ChartViolation(Quantity::Radius));
    }
    if radius.lower.sign() != Sign::Plus {
        return Ok(Some(Quantity::Radius));
    }
    if mode == Mode::Cylindrical {
        return Ok(None);
    }
    let inclination = &input[1][0];
    let pi = circular_constants::pi(arithmetic)?;
    if inclination.upper.sign() != Sign::Plus || inclination.lower >= pi.upper {
        return Err(Error::ChartViolation(Quantity::Inclination));
    }
    let strict = inclination.lower.sign() == Sign::Plus && inclination.upper < pi.lower;
    Ok((!strict).then_some(Quantity::Inclination))
}

pub(super) fn range_jet(
    mode: Mode,
    rows: &[Net; 4],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<WorldJet> {
    let input = range_laws(rows, arithmetic)?;
    world(mode, &input, arithmetic).map(|(jet, _)| jet)
}

pub(super) fn point_jet(
    mode: Mode,
    rows: &[Net; 4],
    normalized: &R,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<WorldJet> {
    let mut input = zeros(arithmetic);
    for (order, row) in rows.iter().enumerate() {
        for (axis, value) in row.value(normalized).iter().enumerate() {
            input[axis][order] = rational_taylor(value, order, arithmetic)?;
        }
    }
    world(mode, &input, arithmetic).map(|(jet, _)| jet)
}

fn range_laws(rows: &[Net; 4], arithmetic: &mut Arithmetic<'_>) -> Checked<WorldJet> {
    let mut input = zeros(arithmetic);
    for (order, row) in rows.iter().enumerate() {
        for (axis, controls) in row.0.iter().enumerate() {
            let [lower, upper] = controls
                .iter()
                .fold([&controls[0]; 2], |[lo, hi], v| [lo.min(v), hi.max(v)]);
            input[axis][order] = Interval {
                lower: rational_taylor(lower, order, arithmetic)?.lower,
                upper: rational_taylor(upper, order, arithmetic)?.upper,
            };
        }
    }
    Ok(input)
}

fn rational_taylor(value: &R, order: usize, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let value = arithmetic.rational(value)?;
    Ok(arithmetic.fraction(&value, 1, FACTORIAL[order])?)
}

fn zeros(arithmetic: &Arithmetic<'_>) -> WorldJet {
    std::array::from_fn(|_| std::array::from_fn(|_| arithmetic.zero()))
}

fn world(
    mode: Mode,
    input: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(WorldJet, [Interval; 3])> {
    match mode {
        Mode::Cylindrical => cylindrical(input, arithmetic),
        Mode::Spherical => spherical(input, arithmetic),
    }
}

fn cylindrical(
    [radius, azimuth, axial]: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(WorldJet, [Interval; 3])> {
    let jet = rotated_plane(radius, azimuth, axial.clone(), arithmetic)?;
    let rates = [
        radius[1].clone(),
        arithmetic.multiply(&radius[0], &azimuth[1])?,
        axial[1].clone(),
    ];
    Ok((jet, rates))
}

fn spherical(
    [radius, inclination, azimuth]: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(WorldJet, [Interval; 3])> {
    let [cosine, sine] = series::trig(inclination, arithmetic)?;
    let radial_plane = series::multiply(radius, &sine, arithmetic)?;
    let z = series::multiply(radius, &cosine, arithmetic)?;
    let jet = rotated_plane(&radial_plane, azimuth, z, arithmetic)?;
    let rates = [
        radius[1].clone(),
        arithmetic.multiply(&radius[0], &inclination[1])?,
        arithmetic.multiply(&radial_plane[0], &azimuth[1])?,
    ];
    Ok((jet, rates))
}

fn rotated_plane(
    radius: &Series,
    azimuth: &Series,
    z: Series,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<WorldJet> {
    let [cosine, sine] = series::trig(azimuth, arithmetic)?;
    Ok([
        series::multiply(radius, &cosine, arithmetic)?,
        series::multiply(radius, &sine, arithmetic)?,
        z,
    ])
}

pub(super) fn bisect(rows: &[Net; 4]) -> [[Net; 4]; 2] {
    let [[l0, r0], [l1, r1], [l2, r2], [l3, r3]] = rows.each_ref().map(Net::bisect);
    [[l0, l1, l2, l3], [r0, r1, r2, r3]]
}

fn merge(
    mut a: AngularEquationCertificate,
    b: AngularEquationCertificate,
) -> AngularEquationCertificate {
    for (a, b) in a
        .coordinate_bounds
        .iter_mut()
        .flatten()
        .zip(b.coordinate_bounds.into_iter().flatten())
    {
        *a = [a[0].min(b[0]), a[1].max(b[1])];
    }
    a.speed_lower = a.speed_lower.min(b.speed_lower);
    a
}
