//! Quadratic difference factorization excludes the diagonal without an approximate root.
use super::super::bezier_pair::hull_separation::cross;
use super::super::certificate_arithmetic::ExactRatio;
use super::super::sketch_incidence::spans::closed_bridge::ProjectedQuadraticSpan;
use super::types::ClosedExtrusionProjectionContact;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum SelfSpanWitness {
    ProperParabola,
    StrictMonotonicity { axis: usize },
}

pub(super) fn certify_span(
    span: &ProjectedQuadraticSpan,
    index: usize,
) -> Result<SelfSpanWitness, ClosedExtrusionProjectionContact> {
    let a = span.powers.each_ref().map(|p| p[2].clone());
    let b = span.powers.each_ref().map(|p| p[1].clone());
    if cross(&a, &b).sign() != Ordering::Equal {
        return Ok(SelfSpanWitness::ProperParabola);
    }
    for axis in 0..2 {
        if monotone(&span.powers[axis], span.range) {
            return Ok(SelfSpanWitness::StrictMonotonicity { axis });
        }
    }
    if a.iter().chain(&b).all(|v| v.sign() == Ordering::Equal) {
        return Err(ClosedExtrusionProjectionContact::CollapsedSpan { span: index });
    }
    Err(ClosedExtrusionProjectionContact::BacktrackingSpan { span: index })
}

fn monotone(powers: &[ExactRatio; 3], range: [f64; 2]) -> bool {
    let slope = powers[2].multiply(&ExactRatio::from_finite(2.));
    let signs = range.map(|t| {
        slope
            .multiply(&ExactRatio::from_finite(t))
            .add(&powers[1])
            .sign()
    });
    signs.iter().all(|s| *s != Ordering::Less) && signs.iter().any(|s| *s == Ordering::Greater)
        || signs.iter().all(|s| *s != Ordering::Greater)
            && signs.iter().any(|s| *s == Ordering::Less)
}
