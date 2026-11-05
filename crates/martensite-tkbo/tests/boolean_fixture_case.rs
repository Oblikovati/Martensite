//! Preserve original fixture validity, operation and area assertion ordering.
#[path = "common_operation/mod.rs"]
pub(super) mod common_operation;
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
#[path = "boolean_area_assertion.rs"]
mod boolean_area_assertion;
pub use boolean_area_assertion::check_area;
/// Preserve an explicit source checkshape; `check_shape(&shape)`.
pub(super) fn check_shape(shape: &Shape) {
    assert!(
        BRepCheckAnalyzer::new(shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}
/// Replay unchanged fixture assertions; `execute(fixtures, Operation::Common, 1., true)`.
#[allow(dead_code)] // Prepared-operation cases reuse only the original assertion helpers.
pub fn execute(
    fixtures: [(&[u8], ShapeEnum); 2],
    kind: Operation,
    expected: f64,
    validity_first: bool,
) -> Shape {
    // No-op observation keeps all old fixture callers on the original algorithm and defaults.
    execute_observed(fixtures, kind, expected, validity_first, |_| {})
}

fn restored_checked(bytes: &[u8], kind: ShapeEnum) -> Shape {
    let shape = common_operation::restored_shape(bytes, kind);
    check_shape(&shape);
    shape
}

fn checked_result(result: Shape, expected: f64, validity_first: bool) -> Shape {
    if validity_first {
        check_shape(&result);
    }
    check_area(&result, expected);
    if !validity_first {
        check_shape(&result);
    }
    result
}

/// Observe reports/history from the actual operation before guards/properties; `execute_observed(inputs,kind,area,false,inspect)`.
pub fn execute_observed(
    fixtures: [(&[u8], ShapeEnum); 2],
    kind: Operation,
    expected: f64,
    validity_first: bool,
    observe: impl FnOnce(&mut BOPAlgoBOP),
) -> Shape {
    let first = restored_checked(fixtures[0].0, fixtures[0].1);
    let second = restored_checked(fixtures[1].0, fixtures[1].1);
    let result = common_operation::direct_with_fuzzy_observed(
        &first,
        &second,
        kind,
        precision::confusion(),
        observe,
    )
    .0;
    checked_result(result, expected, validity_first)
}
