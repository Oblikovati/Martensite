//! The regression case `bugs_moddata_3::bug29827`.
//! The original compare command checks IsSame and then IsEqual after Nullify.
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::{gp::Transform3, toploc::Location};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;

fn translated_box(sizes: [f64; 3], translation: [f64; 3]) -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(sizes[0], sizes[1], sizes[2]).unwrap();
    let shape = maker.solid().unwrap().into_shape();
    let location = Location::new(Transform3::translation(translation));
    shape.moved(&location, true).unwrap()
}

#[test]
fn bug29827_nullified_translated_boxes_are_same_and_equal() {
    crate::isolated_primitive_case::run_primitive_case(
        "nullify_location::bug29827_nullified_translated_boxes_are_same_and_equal",
        bug29827_nullified_translated_boxes_are_same_and_equal_body,
    );
}

fn bug29827_nullified_translated_boxes_are_same_and_equal_body() {
    let mut first = translated_box([1.0, 2.0, 1.0], [0.0, 1.0, 1.0]);
    let mut second = translated_box([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
    first.nullify();
    second.nullify();
    assert!(first.is_same(&second), "same shapes");
    assert!(first.is_equal(&second), "equal shapes");
}
