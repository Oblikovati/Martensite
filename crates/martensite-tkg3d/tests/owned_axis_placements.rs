//! Exact Cartesian expectations for oriented axes and stored frame state.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{
    GeomAxis1Placement, GeomAxis2Placement, GeomAxisPlacement, GeomGeometry,
};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3, Transform3};

fn direction(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

fn assert_direction(actual: Direction3, expected: [f64; 3]) {
    assert_eq!([actual.x(), actual.y(), actual.z()], expected);
}

fn frame() -> GeomAxis2Placement {
    GeomAxis2Placement::from_directions(
        Point3::new(2., -4., 8.),
        direction(0., 0., 1.),
        direction(1., 0., 1.),
    )
    .unwrap()
}

fn assert_frame(value: &GeomAxis2Placement, main: [f64; 3], x: [f64; 3], y: [f64; 3]) {
    assert_direction(value.direction(), main);
    assert_direction(value.x_direction(), x);
    assert_direction(value.y_direction(), y);
}

#[test]
fn axis_reversal_copy_and_mutation_have_independent_storage() {
    let original =
        GeomAxis1Placement::from_location(Point3::new(2., -4., 8.), direction(0., 0., 1.));
    let reversed = original.reversed();
    let copied: Handle<GeomAxis1Placement> = original.copy().unwrap().cast();
    original.reverse();
    original.set_location(Point3::new(9., 8., 7.));
    assert_direction(original.direction(), [0., 0., -1.]);
    assert_direction(reversed.direction(), [0., 0., -1.]);
    assert_direction(copied.direction(), [0., 0., 1.]);
    assert_eq!(reversed.location(), Point3::new(2., -4., 8.));
    assert_eq!(copied.location(), Point3::new(2., -4., 8.));
    assert_eq!(original.angle(&*copied), std::f64::consts::PI);
}

#[test]
fn frame_projection_setters_and_axis_replacement_have_distinct_contracts() {
    let value = frame();
    assert_frame(&value, [0., 0., 1.], [1., 0., 0.], [0., 1., 0.]);
    value.set_x_direction(direction(0., 1., 1.)).unwrap();
    assert_frame(&value, [0., 0., 1.], [0., 1., 0.], [-1., 0., 0.]);
    value.set_y_direction(direction(1., 0., 1.)).unwrap();
    assert_frame(&value, [0., 0., 1.], [0., -1., 0.], [1., 0., 0.]);
    value.set_axis(Axis1::new(Point3::new(3., 5., 7.), direction(1., 0., 0.)));
    assert_frame(&value, [1., 0., 0.], [0., -1., 0.], [1., 0., 0.]);
    assert_eq!(value.location(), Point3::new(3., 5., 7.));
}

#[test]
fn failed_main_direction_setter_keeps_new_main_and_previous_transverse_axes() {
    let value = frame();
    let error = value.set_direction(direction(1., 0., 0.)).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ConstructionError");
    assert_frame(&value, [1., 0., 0.], [1., 0., 0.], [0., 1., 0.]);
    assert!(value.ax2().is_err());
    let copy: Handle<GeomAxis2Placement> = value.copy().unwrap().cast();
    value.set_ax2(Axis2::default());
    assert_frame(&copy, [1., 0., 0.], [1., 0., 0.], [0., 1., 0.]);
    assert_frame(&value, [0., 0., 1.], [1., 0., 0.], [0., 1., 0.]);
}

#[test]
fn failed_transverse_setters_and_constructor_reject_parallel_directions() {
    let value = frame();
    for result in [
        value.set_x_direction(direction(0., 0., 1.)),
        value.set_y_direction(direction(0., 0., -1.)),
    ] {
        assert_eq!(
            result.unwrap_err().exception_type(),
            "Standard_ConstructionError"
        );
        assert_frame(&value, [0., 0., 1.], [1., 0., 0.], [0., 1., 0.]);
    }
    let invalid = GeomAxis2Placement::from_directions(
        Point3::default(),
        direction(0., 0., 1.),
        direction(0., 0., -1.),
    );
    assert_eq!(
        invalid.err().unwrap().exception_type(),
        "Standard_ConstructionError"
    );
}

#[test]
fn negative_scale_reverses_single_axis_but_preserves_frame_main_handedness() {
    let one = GeomAxis1Placement::from_location(Point3::new(2., -4., 8.), direction(0., 0., 1.));
    let two = frame();
    let mut transform = Transform3::default();
    transform.set_scale(Point3::default(), -2.).unwrap();
    one.transform(transform).unwrap();
    two.transform(transform).unwrap();
    assert_eq!(one.location(), Point3::new(-4., 8., -16.));
    assert_eq!(two.location(), Point3::new(-4., 8., -16.));
    assert_direction(one.direction(), [0., 0., -1.]);
    assert_frame(&two, [0., 0., 1.], [-1., 0., 0.], [0., -1., 0.]);
}
