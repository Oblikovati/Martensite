// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::cmp::Ordering;
pub(super) fn certify(
    index: usize,
    request: CylindricalPeriodicProbe,
    periodic: &CertifiedCylindricalPeriodicClearance,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CertifiedCylindricalProbe> {
    let translation = binding::image(periodic, request.turn)?;
    let (point, rounding) = receipt(index, request, translation.offset(), arithmetic)?;
    margin(index, &rounding, periodic.requested_clearance())?;
    Ok(CertifiedCylindricalProbe {
        request,
        point,
        rounding,
        translation: translation.clone(),
    })
}
fn receipt(
    index: usize,
    request: CylindricalPeriodicProbe,
    offset: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([f64; 2], DyadicBound)> {
    finite(index, request.point)?;
    let point = rounded_point(request, offset);
    finite(index, point)?;
    let expected = literal_point(request, offset, arithmetic)?;
    let actual = arithmetic.scalar(point[0])?;
    let difference = arithmetic.subtract(&actual, &expected)?;
    let rounding = DyadicBound::new(
        difference.magnitude(),
        -i32::from(arithmetic.fractional_bits()),
    );
    Ok((point, rounding))
}
fn margin(index: usize, rounding: &DyadicBound, clearance: f64) -> Checked<()> {
    if rounding.compare_f64(clearance) != Ok(Ordering::Less) {
        return Err(CylindricalProbeFailure::Uncertified {
            index,
            rounding: rounding.clone(),
        });
    }
    Ok(())
}

fn finite(index: usize, point: [f64; 2]) -> Checked<()> {
    if !point.iter().all(|v| v.is_finite()) {
        return Err(CylindricalProbeFailure::Point { index, point });
    }
    Ok(())
}
fn rounded_point(request: CylindricalPeriodicProbe, offset: f64) -> [f64; 2] {
    let x = match request.sense {
        CylindricalProbeSense::Add => request.point[0] + offset,
        CylindricalProbeSense::Subtract => request.point[0] - offset,
    };
    [x, request.point[1]]
}
fn literal_point(
    request: CylindricalPeriodicProbe,
    offset: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let point = arithmetic.scalar(request.point[0])?;
    let shift = arithmetic.scalar(offset)?;
    Ok(match request.sense {
        CylindricalProbeSense::Add => arithmetic.add(&point, &shift)?,
        CylindricalProbeSense::Subtract => arithmetic.subtract(&point, &shift)?,
    })
}
