//! Tighten the enclosure of the same cubic on two exact halves of its proof domain.
use super::*;
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::bisect;

pub(super) fn compare_with_proof(
    source: &Homogeneous,
    reference: &[Vec<Interval>; 2],
    limits: [f64; 2],
    proof: RationalReferenceProof,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Bounds> {
    let whole = accept(compare(source, reference, arithmetic)?, limits);
    match (proof, whole) {
        (RationalReferenceProof::BisectResidualV2, Err(Refusal::Exceeds { .. })) => {
            accept(composite(source, reference, arithmetic)?, limits)
        }
        (_, outcome) => outcome,
    }
}

fn composite(
    source: &Homogeneous,
    reference: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Bounds> {
    let halves = bisect(&source.weight, arithmetic)?;
    let denominators = positive_denominators(&halves, arithmetic)?;
    let mut totals = [arithmetic.zero(), arithmetic.zero()];
    for (axis, coefficients) in reference.iter().enumerate() {
        let [position, rate] = residual_terms(
            coefficients,
            &source.numerator[axis],
            &source.weight,
            arithmetic,
        )?;
        let position = upper(
            &position,
            [&denominators[0][0], &denominators[1][0]],
            arithmetic,
        )?;
        let rate = upper(
            &rate,
            [&denominators[0][1], &denominators[1][1]],
            arithmetic,
        )?;
        totals[0] = arithmetic.add(&totals[0], &position)?;
        totals[1] = arithmetic.add(&totals[1], &rate)?;
    }
    Ok(Bounds {
        position: arithmetic.upper_f64(totals[0].upper.clone())?,
        derivative: arithmetic.upper_f64(totals[1].upper.clone())?,
        operations: arithmetic.operations(),
    })
}

fn positive_denominators(
    parts: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[[Interval; 2]; 2]> {
    let pair =
        |intervals: &[Interval], arithmetic: &mut Arithmetic<'_>| -> Checked<[Interval; 2]> {
            let lo = bernstein::lower(intervals);
            let lower = Interval {
                lower: lo.clone(),
                upper: lo,
            };
            let squared = arithmetic.multiply(&lower, &lower)?;
            Ok([lower, squared])
        };
    Ok([pair(&parts[0], arithmetic)?, pair(&parts[1], arithmetic)?])
}

fn upper(
    numerator: &[Interval],
    denominator: [&Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let [left, right] = bisect(numerator, arithmetic)?;
    let left = arithmetic.divide(&algebra::magnitude(&left), denominator[0])?;
    let right = arithmetic.divide(&algebra::magnitude(&right), denominator[1])?;
    let maximum = left.upper.max(right.upper);
    Ok(Interval {
        lower: maximum.clone(),
        upper: maximum,
    })
}
