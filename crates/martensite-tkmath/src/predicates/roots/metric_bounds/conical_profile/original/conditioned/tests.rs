// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const SOURCE: [[f64; 2]; 4] = [[80., 0.], [80., 2.], [82., 4.], [84., 4.]];
fn execute<T>(body: impl FnOnce(&mut Arithmetic<'_>) -> T) -> T {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut never);
    body(&mut arithmetic)
}
fn interval() -> ConicalOriginalSpan<'static> {
    ConicalOriginalSpan {
        poles: &SOURCE,
        index: 1,
        level: 1,
    }
}
fn restricted(source: &[Vec<Interval>; 2], arithmetic: &mut Arithmetic<'_>) -> [Vec<Interval>; 2] {
    source
        .each_ref()
        .map(|axis| restrict(axis, 1, 1, arithmetic).unwrap())
}
#[test]
fn exact_full_restriction_yields_zero_and_retains_actual_coefficients() {
    execute(|arithmetic| {
        let source = admit_source(&SOURCE, arithmetic).unwrap();
        let child = restricted(&source, arithmetic);
        let (proof, retained) = displacement(&source, &child, interval(), arithmetic).unwrap();
        assert_eq!(retained.unwrap(), child);
        assert_eq!(proof.delta.upper, 0.into());
        assert_eq!(proof.derivative.upper, 0.into());
        assert!(proof.anchor.exact);
        assert_eq!(proof.anchor.turn_adjustment, Some(0));
    });
}
#[test]
fn one_interior_coefficient_mutation_cannot_use_endpoint_equality() {
    execute(|arithmetic| {
        let source = admit_source(&SOURCE, arithmetic).unwrap();
        let mut child = restricted(&source, arithmetic);
        child[0][1] = arithmetic.add(&child[0][1], &arithmetic.quantum()).unwrap();
        let (proof, _) = displacement(&source, &child, interval(), arithmetic).unwrap();
        assert!(proof.delta.upper > 0.into());
        assert!(proof.derivative.upper > 0.into());
        assert!(proof.anchor.exact);
    });
}
#[test]
fn identical_nonpoint_enclosures_do_not_establish_actual_curve_identity() {
    execute(|arithmetic| {
        let point = arithmetic.scalar(1.).unwrap();
        let rounded = Interval {
            lower: point.lower,
            upper: point.upper + 1,
        };
        assert!(!exact_coefficients(
            std::slice::from_ref(&rounded),
            std::slice::from_ref(&rounded)
        ));
    });
}
#[test]
fn branch_adjustment_survives_generic_nonidentical_source_fallback() {
    execute(|arithmetic| {
        let poles = [[-80., 0.], [-80., -1.]];
        let source = admit_source(&poles, arithmetic).unwrap();
        let child = admit_source(&[[-80., -0.125], [-80., -1.]], arithmetic).unwrap();
        let original = ConicalOriginalSpan {
            poles: &poles,
            index: 0,
            level: 0,
        };
        let (proof, _) = displacement(&source, &child, original, arithmetic).unwrap();
        assert_eq!(proof.anchor.turn_adjustment, Some(1));
        assert!(proof.delta.upper > 0.into());
    });
}
#[test]
fn exact_source_still_executes_real_restriction_and_cancel_checks() {
    let (source, child) = execute(|arithmetic| {
        let source = admit_source(&SOURCE, arithmetic).unwrap();
        let child = restricted(&source, arithmetic);
        (source, child)
    });
    let mut stop = || true;
    let mut arithmetic = Arithmetic::new(ArithmeticPolicy::default(), &mut stop);
    assert!(matches!(
        displacement(&source, &child, interval(), &mut arithmetic),
        Err(ConicalOriginalFailure::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}
