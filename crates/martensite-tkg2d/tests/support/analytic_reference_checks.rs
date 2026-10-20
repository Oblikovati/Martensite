//! Fixed primitive words remove the shared-transcendental expected-side dependency (#201).
use super::{
    AnalyticFamily, DerivativeJet, actual_derivatives, analytic_fixture, assert_derivatives,
    check_lower_evaluators, global_derivatives,
};
#[path = "analytic_reference_words.rs"]
mod words;

fn reference_jet(family: AnalyticFamily, row: [u64; 5], scale: f64) -> DerivativeJet {
    let [_, sine, cosine, sinh, cosh] = row.map(f64::from_bits);
    if matches!(family, AnalyticFamily::Hyperbola) {
        let even = [2.0 * scale * cosh, scale * sinh];
        let odd = [2.0 * scale * sinh, scale * cosh];
        return [even, odd, even, odd];
    }
    let major = if matches!(family, AnalyticFamily::Ellipse) {
        2.0 * scale
    } else {
        scale
    };
    let even = [major * cosine, scale * sine];
    let odd = [-major * sine, scale * cosine];
    [even, odd, even.map(|value| -value), odd.map(|value| -value)]
}

fn check_reference_grid(family: AnalyticFamily, sense: bool, scale: f64) -> usize {
    let curve = analytic_fixture(family, scale, sense);
    for (index, row) in words::ANALYTIC_REFERENCE_WORDS.into_iter().enumerate() {
        let parameter = f64::from(i32::try_from(index).unwrap() - 16) / 8.0;
        assert_eq!(
            parameter.to_bits(),
            row[0],
            "expected the original exact dyadic parameter"
        );
        let expected = global_derivatives(reference_jet(family, row, scale), sense);
        let label = format!("independent {family:?}, direct={sense}, scale={scale}, u={parameter}");
        assert_derivatives(
            actual_derivatives(curve.as_ref(), parameter),
            expected,
            &label,
        );
        check_lower_evaluators(curve.as_ref(), parameter, expected, &label);
    }
    33
}

fn check_reference_orientations(family: AnalyticFamily) -> usize {
    let mut count = 0;
    for sense in [false, true] {
        for scale in [0.125, 1.0, 8.0, 128.0] {
            count += check_reference_grid(family, sense, scale);
        }
    }
    count
}

#[test]
fn analytic_jets_match_independent_exact_input_primitive_references() {
    let mut count = 0;
    for family in [
        AnalyticFamily::Circle,
        AnalyticFamily::Ellipse,
        AnalyticFamily::Hyperbola,
    ] {
        count += check_reference_orientations(family);
    }
    assert_eq!(count, 792);
}
