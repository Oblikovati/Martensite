//! Unconditional source Boolean operations with distinct property, view-branch and suite-end checks.
#[path = "boolean_fixture_case.rs"]
mod boolean_fixture_case;
pub use boolean_fixture_case::check_area;
use boolean_fixture_case::common_operation;
pub use common_operation::{direct, restored_shape};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;
/// Preserve source restore/restore/bfuse/checkprops; `checked_fuse(fixtures, 43552.5)`.
#[allow(dead_code)] // Primitive source cases call checked_fuse_shapes directly.
pub fn checked_fuse(fixtures: [(&[u8], ShapeEnum); 2], expected: f64) -> Shape {
    checked_fixture_operation(fixtures, Operation::Fuse, false, expected)
}
/// Restore in source order, then choose operand roles; `checked_fixture_operation(fixtures, Operation::Cut, true, 32.9469)`.
pub fn checked_fixture_operation(
    fixtures: [(&[u8], ShapeEnum); 2],
    operation: Operation,
    reverse: bool,
    expected: f64,
) -> Shape {
    let first = restored_shape(fixtures[0].0, fixtures[0].1);
    let second = restored_shape(fixtures[1].0, fixtures[1].1);
    let (argument, tool) = if reverse {
        (&second, &first)
    } else {
        (&first, &second)
    };
    checked_operation(argument, tool, operation, expected)
}
/// Fuse original procedural operands with source checkprops; `checked_fuse_shapes(a, b, 2199.11)`.
#[allow(dead_code)] // Fixture cohorts use checked_fixture_operation; procedural cases use this.
pub fn checked_fuse_shapes(first: &Shape, second: &Shape, expected: f64) -> Shape {
    checked_operation(first, second, Operation::Fuse, expected)
}
/// Run a direct source operation and its area check; `checked_operation(a, b, Operation::Cut, 330507.)`.
pub fn checked_operation(
    first: &Shape,
    second: &Shape,
    operation: Operation,
    expected: f64,
) -> Shape {
    let result = direct(first, second, operation);
    check_area(&result, expected);
    result
}
/// Pin checkview's independently observed nonempty result branch; `check_view_result(&shape)`.
#[allow(dead_code)] // Empty-result cases check zero mass and fallback operands separately.
pub fn check_view_result(result: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result, &mut properties, false, false).unwrap();
    let text = format_general(properties.mass(), 6);
    let mass: f64 = text
        .parse()
        .expect("sprops mass: expected numeric six-digit text");
    assert_ne!(
        mass, 0.,
        "checkview -s: expected native nonzero result branch, got {text}"
    );
}
/// Preserve boolean/end validity after saved view; `check_end(&shape)`.
pub fn check_end(result: &Shape) {
    boolean_fixture_case::check_shape(result);
}
