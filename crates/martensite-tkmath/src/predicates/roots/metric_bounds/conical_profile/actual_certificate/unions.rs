// SPDX-License-Identifier: GPL-2.0-only
//! A whole concatenated rectangle is injective under the coupled f/D inequalities.
use super::*;
use capture::SideCapture;
pub(super) fn certify(
    pair: &ConicalActualAdjacent,
    a: &SideCapture,
    b: &SideCapture,
    barriers: &barriers::Barriers,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    arithmetic.certificate_step()?;
    shared(pair, a, b)?;
    let signs = [
        if pair.u_slots[0] == 1 { 1. } else { -1. },
        if pair.u_slots[1] == 0 { 1. } else { -1. },
    ];
    let tangent = chord(pair, a, b)
        .into_iter()
        .map(|v| arithmetic.scalar(v))
        .collect::<Checked<Vec<_>>>()?;
    let first = projected(a, &tangent, signs[0], sine, cosine, arithmetic)?;
    let last = projected(b, &tangent, signs[1], sine, cosine, arithmetic)?;
    coupled(
        &first,
        &last,
        (&a.du_error, &b.du_error),
        &barriers.dv_lower,
        arithmetic,
    )
}
fn chord(pair: &ConicalActualAdjacent, a: &SideCapture, b: &SideCapture) -> [f64; 3] {
    let first = if pair.u_slots[0] == 1 {
        0
    } else {
        a.source.rows[0].len() - 1
    };
    let last = if pair.u_slots[1] == 0 {
        b.source.rows[0].len() - 1
    } else {
        0
    };
    std::array::from_fn(|axis| b.source.rows[0][last][axis] - a.source.rows[0][first][axis])
}
fn shared(pair: &ConicalActualAdjacent, a: &SideCapture, b: &SideCapture) -> Outcome<()> {
    if pair.u_slots.iter().any(|v| *v > 1) {
        return Err(error::binding(
            "shared U slots",
            pair.u_slots.iter().map(|v| u64::from(*v)).collect(),
            vec![0, 1],
        ));
    }
    same_family(a, b)?;
    for row in 0..2 {
        shared_row(pair, a, b, row)?;
    }
    Ok(())
}
fn projected(
    side: &SideCapture,
    a: &[Interval],
    sign: f64,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<[Interval; 2]> {
    let [ux, uy] = unit_circle(side, arithmetic)?;
    let (radial, tangent) = directions(a, &ux, &uy, arithmetic)?;
    let mut du = ideal_du(side, a, &radial, &tangent, sine, cosine, arithmetic)?;
    if sign < 0. {
        du = numeric::neg(&du);
    }
    let radial = arithmetic.multiply(cosine, &radial)?;
    let axial = arithmetic.multiply(sine, &a[2])?;
    let dv = arithmetic.subtract(&radial, &axial)?;
    let dv = arithmetic.multiply(&side.depth_gap, &dv)?;
    let norm = arithmetic.add(&numeric::magnitude(&a[0]), &numeric::magnitude(&a[1]))?;
    let norm = arithmetic.add(&norm, &numeric::magnitude(&a[2]))?;
    Ok([
        widen(&du, &norm, &side.derivative_error, arithmetic)?,
        widen(&dv, &norm, &side.ruling_error, arithmetic)?,
    ])
}
fn directions(
    a: &[Interval],
    ux: &Interval,
    uy: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<(Interval, Interval)> {
    let x = arithmetic.multiply(&a[0], ux)?;
    let y = arithmetic.multiply(&a[1], uy)?;
    let radial = arithmetic.add(&x, &y)?;
    let x = arithmetic.multiply(&a[1], ux)?;
    let y = arithmetic.multiply(&a[0], uy)?;
    Ok((radial, arithmetic.subtract(&x, &y)?))
}
fn ideal_du(
    side: &SideCapture,
    a: &[Interval],
    radial: &Interval,
    tangent: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let radial = arithmetic.multiply(sine, radial)?;
    let axial = arithmetic.multiply(cosine, &a[2])?;
    let slant = arithmetic.add(&radial, &axial)?;
    let first = arithmetic.multiply(&side.slant_rate, &slant)?;
    let second = arithmetic.multiply(&side.radius, &side.angle_rate)?;
    let second = arithmetic.multiply(&second, tangent)?;
    Ok(arithmetic.add(&first, &second)?)
}
fn widen(
    value: &Interval,
    norm: &Interval,
    error: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let error = arithmetic.multiply(norm, error)?;
    let signed = Interval {
        lower: -error.upper.clone(),
        upper: error.upper,
    };
    Ok(arithmetic.add(value, &signed)?)
}

fn coupled(
    first: &[Interval; 2],
    last: &[Interval; 2],
    errors: (&Interval, &Interval),
    dv: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Interval> {
    let lower = first[0].lower.clone().min(last[0].lower.clone());
    let lower = Interval {
        lower: lower.clone(),
        upper: lower,
    };
    numeric::positive("whole-union f_u", &lower, arithmetic)?;
    let lateral = numeric::max_magnitude(&[first[1].clone(), last[1].clone()]);
    let transverse = numeric::max_magnitude(&[errors.0.clone(), errors.1.clone()]);
    let diagonal = arithmetic.multiply(&lower, &numeric::lower_point(dv))?;
    let cross = arithmetic.multiply(&lateral, &transverse)?;
    let margin = arithmetic.subtract(&diagonal, &cross)?;
    numeric::positive("whole-union a*d-b*e", &margin, arithmetic)?;
    Ok(margin)
}

fn same_family(a: &SideCapture, b: &SideCapture) -> Outcome<()> {
    if a.source.member != b.source.member || a.source.caps != b.source.caps {
        let actual = vec![b.source.member.0, b.source.caps[0].0, b.source.caps[1].0];
        let expected = vec![a.source.member.0, a.source.caps[0].0, a.source.caps[1].0];
        return Err(error::binding(
            "shared ruling member/caps",
            actual,
            expected,
        ));
    }
    Ok(())
}
fn shared_row(
    pair: &ConicalActualAdjacent,
    a: &SideCapture,
    b: &SideCapture,
    row: usize,
) -> Outcome<()> {
    let ia = endpoint(pair.u_slots[0], a.source.rows[row].len());
    let ib = endpoint(pair.u_slots[1], b.source.rows[row].len());
    let actual = b.source.rows[row][ib].map(f64::to_bits);
    let expected = a.source.rows[row][ia].map(f64::to_bits);
    if actual != expected {
        return Err(error::binding(
            "exact actual shared ruling",
            actual.to_vec(),
            expected.to_vec(),
        ));
    }
    Ok(())
}
fn endpoint(slot: u8, count: usize) -> usize {
    if slot == 0 { 0 } else { count - 1 }
}
fn unit_circle(side: &SideCapture, arithmetic: &mut Arithmetic<'_>) -> Outcome<[Interval; 2]> {
    let one = arithmetic.scalar(1.)?;
    let square = numeric::square(&side.p, arithmetic)?;
    let ux = arithmetic.subtract(&one, &square)?;
    let ux = arithmetic.divide(&ux, &side.angle_denominator)?;
    let uy = arithmetic.fraction(&side.p, 2, 1)?;
    let uy = arithmetic.divide(&uy, &side.angle_denominator)?;
    Ok([ux, uy])
}
