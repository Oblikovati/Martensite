//! Exact datum sharing and raw evaluator descriptors are fixture input, not defaults.
use super::types::*;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::{
    gp::{Transform3, TransformForm},
    toploc::{Datum3, Location},
};
pub(super) fn datums(inputs: &[TransformInput]) -> Vec<Handle<Datum3>> {
    inputs
        .iter()
        .map(|input| Handle::new(Datum3::new(transform(*input))))
        .collect()
}
fn transform(input: TransformInput) -> Transform3 {
    let mut result = Transform3::translation(scalars(input.translation));
    result.set_form(TransformForm::from_native(input.form).unwrap());
    assert_transform(result, input);
    result
}
pub(super) fn location(input: LocationInput, datums: &[Handle<Datum3>]) -> Location {
    let mut result = Location::default();
    for (owner, power) in input.steps.iter().rev() {
        let item = Location::from_datum(datums[*owner].clone())
            .powered(*power)
            .unwrap();
        result = result.multiplied(&item).unwrap();
    }
    assert_transform(result.transformation(), input.effective);
    result
}
pub(super) fn assert_transform(actual: Transform3, expected: TransformInput) {
    assert_eq!(actual.form() as i32, expected.form);
    assert_eq!(actual.scale_factor().to_bits(), expected.scale);
    let offset = actual.translation_part();
    assert_eq!(
        [offset.x(), offset.y(), offset.z()].map(f64::to_bits),
        expected.translation
    );
    for row in 0..3 {
        for column in 0..3 {
            assert_eq!(
                actual
                    .matrix()
                    .value(row as i32 + 1, column as i32 + 1)
                    .unwrap()
                    .to_bits(),
                expected.matrix[row][column]
            );
        }
    }
}
