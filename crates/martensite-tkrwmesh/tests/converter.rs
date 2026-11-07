//! Axis and unit conversion on known values.
use martensite_tkernel::ncollection::Vec3;
use martensite_tkmath::gp::{self, Axis3, Transform3, Vector3, Xyz};
use martensite_tkrwmesh::rw_mesh::{
    RwMeshCoordinateSystem as System, RwMeshCoordinateSystemConverter as Converter,
};

fn y_up_to_z_up() -> Converter {
    let mut converter = Converter::new();
    converter.set_input_coordinate_system_kind(System::Y_UP);
    converter.set_output_coordinate_system_kind(System::Z_UP);
    converter
}

fn assert_xyz(value: Xyz, expected: [f64; 3]) {
    let found = [value.x(), value.y(), value.z()];
    for (a, b) in found.iter().zip(expected) {
        assert!((a - b).abs() < 1e-12, "{found:?}, expected {expected:?}");
    }
}

#[test]
fn a_new_converter_is_empty_and_changes_nothing() {
    let converter = Converter::new();
    assert!(converter.is_empty());
    assert_eq!(
        (
            converter.input_length_unit(),
            converter.output_length_unit()
        ),
        (-1.0, -1.0)
    );
    assert!(!converter.has_input_coordinate_system() && !converter.has_output_coordinate_system());
    let mut point = Xyz::new(1.0, 2.0, 3.0);
    converter.transform_position(&mut point);
    assert_xyz(point, [1.0, 2.0, 3.0]);
}

#[test]
fn y_up_goes_to_z_up_and_back() {
    let converter = y_up_to_z_up();
    assert!(!converter.is_empty());
    let mut point = Xyz::new(1.0, 2.0, 3.0);
    converter.transform_position(&mut point);
    assert_xyz(point, [1.0, -3.0, 2.0]);
    let mut normal = Vec3::new(0.0_f32, 1.0, 0.0);
    converter.transform_normal(&mut normal);
    assert!((normal.z - 1.0).abs() < 1e-6 && normal.y.abs() < 1e-6);
}

#[test]
fn units_scale_positions_and_translations_but_not_normals() {
    let mut converter = Converter::new();
    converter.set_input_length_unit(1.0);
    converter.set_output_length_unit(0.001);
    assert!(!converter.is_empty());
    let mut point = Xyz::new(1.0, 2.0, 3.0);
    converter.transform_position(&mut point);
    assert_xyz(point, [1000.0, 2000.0, 3000.0]);
    let mut trsf = Transform3::translation_vector(Vector3::new(1.0, 0.0, 0.0));
    converter.transform_transformation(&mut trsf);
    assert_xyz(trsf.translation_part(), [1000.0, 0.0, 0.0]);
    let mut normal = Vec3::new(0.0_f32, 0.0, 1.0);
    converter.transform_normal(&mut normal);
    assert_eq!(normal.z, 1.0);
}

#[test]
fn an_undefined_unit_or_system_disables_its_conversion() {
    let mut converter = Converter::new();
    converter.set_input_length_unit(1.0);
    assert!(converter.is_empty());
    converter.set_input_coordinate_system_kind(System::UNDEFINED);
    converter.set_output_coordinate_system_kind(System::Z_UP);
    assert!(!converter.has_input_coordinate_system() && converter.has_output_coordinate_system());
    assert!(converter.is_empty());
}

#[test]
fn explicit_frames_and_a_transformation_follow_the_systems() {
    let mut converter = Converter::new();
    let frame = Axis3::new(gp::origin(), gp::dy(), gp::dx()).unwrap();
    converter.set_input_coordinate_system(frame);
    converter.set_output_coordinate_system(Axis3::default());
    assert!(converter.has_input_coordinate_system());
    assert!(
        converter
            .input_coordinate_system()
            .direction()
            .is_equal(&gp::dy(), 1e-12)
    );
    assert!(
        converter
            .output_coordinate_system()
            .direction()
            .is_equal(&gp::dz(), 1e-12)
    );
    let mut trsf = Transform3::translation_vector(Vector3::new(0.0, 1.0, 0.0));
    converter.transform_transformation(&mut trsf);
    assert_xyz(trsf.translation_part(), [0.0, 0.0, 1.0]);
}

#[test]
fn the_same_system_on_both_sides_is_empty() {
    let mut converter = Converter::new();
    converter.init(Axis3::default(), 0.001, Axis3::default(), 0.001);
    assert!(converter.is_empty());
}

#[test]
fn standard_systems_are_the_declared_frames() {
    let z_up = Converter::standard_coordinate_system(System::Z_UP);
    let y_up = Converter::standard_coordinate_system(System::Y_UP);
    let undefined = Converter::standard_coordinate_system(System::UNDEFINED);
    assert!(z_up.direction().is_equal(&gp::dz(), 1e-12));
    assert!(y_up.direction().is_equal(&gp::dy(), 1e-12));
    assert!(undefined.direction().is_equal(&gp::dz(), 1e-12));
}
