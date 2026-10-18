//! Transport-only checks keep the two existing readers distinct and exercise every adapter field.
use crate::construction_corpus::{
    common::Reader as OwnedReader, fixture_input::GeometryFixtureInput,
};
use crate::gb_probe::common::Reader as RetainedReader;
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3};

fn assert_scalar_fields(input: &mut impl GeometryFixtureInput) {
    assert_eq!(input.integer(), -7);
    assert_eq!(input.real(), 2.5);
    assert!(input.flag());
    assert_eq!(input.pnt2d(), Point2::new(3.0, 4.0));
    assert_eq!(input.pnt(), Point3::new(7.0, 8.0, 9.0));
    assert_eq!(input.real(), 88.0);
}
fn assert_direction_fields(input: &mut impl GeometryFixtureInput) {
    assert_eq!(input.dir2d(), Direction2::Y);
    assert_eq!(input.dir(), Direction3::Z);
    let frame = input.ax2d();
    assert_eq!(frame.location(), Point2::new(5.0, 6.0));
    assert_eq!(frame.direction(), Direction2::X);
}
fn assert_axis_fields(input: &mut impl GeometryFixtureInput) {
    let axis = input.ax1();
    assert_eq!(axis.location(), Point3::new(1.0, 2.0, 3.0));
    assert_eq!(axis.direction(), Direction3::Z);
    let frame = input.ax2();
    assert_eq!(frame.location(), Point3::default());
    assert_eq!(frame.direction(), Direction3::Z);
    assert_eq!(frame.x_direction(), Direction3::X);
    let frame = input.ax3();
    assert_eq!(frame.location(), Point3::new(4.0, 5.0, 6.0));
    assert_eq!(frame.direction(), Direction3::Z);
    assert_eq!(frame.x_direction(), Direction3::X);
    assert_eq!(input.real(), 77.0);
}
#[test]
fn reader_types_keep_distinct_identities() {
    assert_ne!(
        std::any::TypeId::of::<OwnedReader<'static>>(),
        std::any::TypeId::of::<RetainedReader<'static>>()
    );
}
#[test]
fn both_scalar_adapters_preserve_values_and_cursor() {
    let values = [-7.0, 2.5, 1.0, 3.0, 4.0, 7.0, 8.0, 9.0, 88.0];
    assert_scalar_fields(&mut OwnedReader::new(&values));
    assert_scalar_fields(&mut RetainedReader::new(&values));
}
#[test]
fn both_frame_adapters_preserve_values_and_cursor() {
    let values = [
        0.0, 1.0, 0.0, 0.0, 1.0, 5.0, 6.0, 1.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 4.0, 5.0, 6.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 77.0,
    ];
    let mut owned = OwnedReader::new(&values);
    assert_direction_fields(&mut owned);
    assert_axis_fields(&mut owned);
    let mut retained = RetainedReader::new(&values);
    assert_direction_fields(&mut retained);
    assert_axis_fields(&mut retained);
}

fn assert_linear_vector_reader(input: &mut impl GeometryFixtureInput) {
    let vector = crate::construction_corpus::linear_tokens::read_vector(input);
    assert_eq!((vector.lower(), vector.upper()), (-2, -1));
    assert_eq!([vector[-2], vector[-1]], [3.5, -0.0]);
    assert_eq!(input.real(), 99.0);
}
fn assert_multi_point_reader(input: &mut impl GeometryFixtureInput) {
    let points = crate::construction_corpus::app_par_containers::read_points(input).unwrap();
    assert_eq!(points[1].point(1).unwrap(), Point3::new(1.0, 2.0, 3.0));
    assert_eq!(input.real(), 99.0);
}
#[test]
fn both_readers_use_one_vector_builder_and_preserve_cursor() {
    let values = [-2.0, 2.0, 3.5, -0.0, 99.0];
    assert_linear_vector_reader(&mut OwnedReader::new(&values));
    assert_linear_vector_reader(&mut RetainedReader::new(&values));
}
#[test]
fn both_readers_use_one_multi_point_builder_and_preserve_cursor() {
    let values = [1.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 99.0];
    assert_multi_point_reader(&mut OwnedReader::new(&values));
    assert_multi_point_reader(&mut RetainedReader::new(&values));
}

fn assert_constraint_vectors(input: &mut impl GeometryFixtureInput) {
    use martensite_tkmath::gp::{Vector2, Vector3};
    let line = crate::construction_corpus::constraint_line_fixtures::read_line(input);
    let point = line.value(1).unwrap();
    assert_eq!(point.tang(1).unwrap(), Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(point.tang2d(2).unwrap(), Vector2::new(0.0, 1.0));
    assert_eq!(point.curv(1).unwrap(), Vector3::new(0.0, 2.0, 0.0));
    assert_eq!(point.curv2d(2).unwrap(), Vector2::new(3.0, 0.0));
    assert_eq!(input.real(), 99.0);
}
#[test]
fn both_readers_preserve_constraint_line_vectors_and_cursor() {
    let values = [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0, 4.0, 5.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
        3.0, 0.0, 99.0,
    ];
    assert_constraint_vectors(&mut OwnedReader::new(&values));
    assert_constraint_vectors(&mut RetainedReader::new(&values));
}

fn assert_directed_planar_frame(input: &mut impl GeometryFixtureInput) {
    let frame = input.ax22d();
    assert_eq!(frame.location(), Point2::new(3., 4.));
    assert_eq!(frame.x_direction(), Direction2::X);
    assert_eq!(frame.y_direction(), Direction2::Y);
    assert_eq!(input.real(), 99.);
}
#[test]
fn both_planar_frame_adapters_preserve_explicit_axis_fields_and_cursor() {
    let values = [3., 4., 1., 0., 0., 1., 99.];
    assert_directed_planar_frame(&mut OwnedReader::new(&values));
    assert_directed_planar_frame(&mut RetainedReader::new(&values));
}
fn assert_recursive_record_fields(input: &mut impl GeometryFixtureInput) {
    use crate::construction_corpus::geometry_record_fixtures as records;
    let line = records::curve(input).unwrap();
    assert_eq!(line.value(2.).unwrap(), Point3::new(3., 2., 3.));
    assert_eq!(input.real(), 99.);
    assert!(records::curve(input).unwrap().is_null());
    assert_eq!(input.real(), 88.);
}
#[test]
fn both_readers_share_recursive_record_builder_with_cursor_and_null_kind() {
    let values = [1., 1., 2., 3., 1., 0., 0., 99., 0., 88.];
    assert_recursive_record_fields(&mut OwnedReader::new(&values));
    assert_recursive_record_fields(&mut RetainedReader::new(&values));
}
