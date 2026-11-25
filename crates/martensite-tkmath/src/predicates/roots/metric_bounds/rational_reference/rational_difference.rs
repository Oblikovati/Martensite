//! Bound two rational representatives without raising the derivative product degree.
use super::*;

pub(super) fn transport(
    source: &Homogeneous,
    target: &Homogeneous,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<(Bounds, f64)> {
    let mut proof = compare(source, target, arithmetic)?;
    let speed = speed_upper(target, arithmetic)?;
    // The closing homotopy segments cancel at shared exact and literal knots.
    let area = ((speed + proof.derivative).next_up() * proof.position).next_up();
    proof.operations = arithmetic.operations();
    if area.is_finite() {
        Ok((proof, area))
    } else {
        Err(Refusal::Limits {
            position: proof.position,
            derivative: speed,
        })
    }
}

pub(super) fn compare(
    source: &Homogeneous,
    target: &Homogeneous,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Bounds> {
    let denominator = bernstein::product(&source.weight, &target.weight, arithmetic)?;
    let minimum = bernstein::lower(&denominator);
    let floor = Interval {
        lower: minimum.clone(),
        upper: minimum,
    };
    let rate_scale = algebra::magnitude(&derivative(&denominator, arithmetic)?);
    let [value, rate] = residual(source, target, arithmetic)?;
    let position = arithmetic.divide(&value, &floor)?;
    // (F/S)' = F'/S - (F/S)(S'/S). Bounding each factor avoids degree 4d-1.
    let correction = arithmetic.multiply(&position, &rate_scale)?;
    let derivative = arithmetic.add(&rate, &correction)?;
    let derivative = arithmetic.divide(&derivative, &floor)?;
    Ok(Bounds {
        position: arithmetic.upper_f64(position.upper)?,
        derivative: arithmetic.upper_f64(derivative.upper)?,
        operations: arithmetic.operations(),
    })
}

fn residual(
    source: &Homogeneous,
    target: &Homogeneous,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Interval; 2]> {
    let mut value = arithmetic.zero();
    let mut rate = arithmetic.zero();
    for (original, rounded) in source.numerator.iter().zip(&target.numerator) {
        let a = bernstein::product(original, &target.weight, arithmetic)?;
        let b = bernstein::product(rounded, &source.weight, arithmetic)?;
        let diff = bernstein::difference(&a, &b, arithmetic)?;
        value = arithmetic.add(&value, &algebra::magnitude(&diff))?;
        let slope = algebra::magnitude(&derivative(&diff, arithmetic)?);
        rate = arithmetic.add(&rate, &slope)?;
    }
    Ok([value, rate])
}

pub(super) fn speed_upper(source: &Homogeneous, arithmetic: &mut Arithmetic<'_>) -> Checked<f64> {
    let min = bernstein::lower(&source.weight);
    let floor = Interval {
        lower: min.clone(),
        upper: min,
    };
    let squared = arithmetic.multiply(&floor, &floor)?;
    let dw = derivative(&source.weight, arithmetic)?;
    let mut sum = arithmetic.zero();
    for coefficients in &source.numerator {
        let dn = derivative(coefficients, arithmetic)?;
        let first = bernstein::product(&dn, &source.weight, arithmetic)?;
        let second = bernstein::product(coefficients, &dw, arithmetic)?;
        let residual = bernstein::difference(&first, &second, arithmetic)?;
        sum = arithmetic.add(&sum, &algebra::magnitude(&residual))?;
    }
    let upper = arithmetic.divide(&sum, &squared)?;
    Ok(arithmetic.upper_f64(upper.upper)?)
}
