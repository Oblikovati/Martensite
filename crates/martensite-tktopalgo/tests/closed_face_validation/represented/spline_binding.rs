//! Every stored coefficient and evaluator layout participates in the input binding.
use super::types::*;
use martensite_tkg2d::geom2d::StoredPlanarSplineInput;
use martensite_tkg3d::geom::StoredSplineInput;
struct Fields<'a, const D: usize> {
    poles: &'a [[f64; D]],
    weights: &'a [f64],
    knots: &'a [f64],
    multiplicities: &'a [i32],
    flat: &'a [f64],
    domain: [f64; 2],
    degree: i32,
    periodic: bool,
    rational: bool,
    evaluation_override: bool,
    lower: [i32; 5],
}
trait ReadSpline<const D: usize> {
    fn fields(&self) -> Fields<'_, D>;
}
impl ReadSpline<3> for StoredSplineInput {
    fn fields(&self) -> Fields<'_, 3> {
        Fields {
            poles: self.poles(),
            weights: self.weights(),
            knots: self.knots(),
            multiplicities: self.multiplicities(),
            flat: self.flat_knots(),
            domain: self.domain(),
            degree: self.degree(),
            periodic: self.periodic(),
            rational: self.rational(),
            evaluation_override: self.evaluation_override(),
            lower: self.lower_bounds(),
        }
    }
}
impl ReadSpline<2> for StoredPlanarSplineInput {
    fn fields(&self) -> Fields<'_, 2> {
        Fields {
            poles: self.poles(),
            weights: self.weights(),
            knots: self.knots(),
            multiplicities: self.multiplicities(),
            flat: self.flat_knots(),
            domain: self.domain(),
            degree: self.degree(),
            periodic: self.periodic(),
            rational: self.rational(),
            evaluation_override: self.evaluation_override(),
            lower: self.lower_bounds(),
        }
    }
}
pub(super) fn spline3(actual: &StoredSplineInput, expected: SplineInput<3>) {
    compare(actual.fields(), expected)
}
pub(super) fn spline2(actual: &StoredPlanarSplineInput, expected: SplineInput<2>) {
    compare(actual.fields(), expected)
}
fn compare<const D: usize>(actual: Fields<'_, D>, expected: SplineInput<D>) {
    assert_eq!(
        actual
            .poles
            .iter()
            .map(|p| p.map(f64::to_bits))
            .collect::<Vec<_>>(),
        expected.poles
    );
    assert_eq!(bits(actual.weights), expected.weights);
    assert_eq!(bits(actual.knots), expected.knots);
    assert_eq!(actual.multiplicities, expected.multiplicities);
    assert_eq!(bits(actual.flat), expected.flat_knots);
    assert_eq!(actual.domain.map(f64::to_bits), expected.domain);
    assert_eq!(
        (
            actual.degree,
            actual.periodic,
            actual.rational,
            actual.evaluation_override,
            actual.lower
        ),
        (
            expected.degree,
            expected.periodic,
            expected.rational,
            expected.evaluation_override,
            expected.lower_bounds
        )
    );
}
fn bits(input: &[f64]) -> Vec<u64> {
    input.iter().copied().map(f64::to_bits).collect()
}
