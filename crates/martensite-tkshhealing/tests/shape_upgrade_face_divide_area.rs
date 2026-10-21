//! Native reference cases at the retained behavior 3d097a0328e71b8.
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::{message::ProgressRange, precision, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Plane3;
use martensite_tkshhealing::{
    shape_extend::{ShapeExtendMsgRegistrator, ShapeExtendStatus},
    shape_fix::ShapeFix,
    shape_upgrade::{
        FaceDivider, ShapeUpgradeFaceDivide, ShapeUpgradeFaceDivideArea, ShapeUpgradeSplitSurface,
    },
};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeFace,
    brep_check::BRepCheckAnalyzer,
};

fn configure(divider: &ShapeUpgradeFaceDivideArea, mode: usize) {
    if mode == 7 {
        divider.set_max_area(1.);
    }
    if mode == 1 {
        divider.set_max_area(2.);
    }
    if mode == 2 || mode == 3 {
        divider.set_max_area(-1.);
        divider.set_nb_parts(4);
    }
    if mode == 3 {
        divider.set_splitting_by_number(true);
        divider.set_numbers_uv_splits(2, 3).unwrap();
    }
    if mode == 4 || mode == 5 {
        divider.set_max_area(2.);
        divider.set_split_surface_tool(&if mode == 4 {
            Handle::default()
        } else {
            Handle::new(ShapeUpgradeSplitSurface::new())
        });
    }
    if mode == 6 {
        divider.set_max_area(-1.);
        divider.set_nb_parts(0);
    }
}
fn face_count(shape: &Shape) -> f64 {
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut count = 0.;
    while faces.more() {
        count += 1.;
        faces.next().unwrap();
    }
    count
}
fn fixture(mode: usize) -> martensite_tkbrep::topods::Face {
    if mode == 7 {
        use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
        let surface: Handle<dyn GeomSurface> = Handle::new(
            GeomSphericalSurface::new(martensite_tkmath::gp::Axis3::default(), 1.).unwrap(),
        )
        .cast();
        return BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 3., -1.5, 1.5), 1e-7)
            .unwrap()
            .face()
            .unwrap();
    }
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 2.))
        .unwrap()
        .face()
        .unwrap()
}
fn observe(mode: usize) -> Vec<f64> {
    let face = fixture(mode);
    let divider = ShapeUpgradeFaceDivideArea::new(face.clone());
    configure(&divider, mode);
    let changed = (&divider as &dyn FaceDivider).perform(999.).unwrap();
    if mode == 7 {
        compare_sphere_patches(&divider.result());
    }
    let mut row = vec![
        mode as f64,
        f64::from(changed),
        face_count(&divider.result()),
        f64::from(divider.context().is_null()),
        f64::from(divider.result().is_same(&face.into_shape())),
        divider.max_area(),
        f64::from(divider.nb_parts()),
    ];
    for flag in [
        ShapeExtendStatus::Done1,
        ShapeExtendStatus::Done2,
        ShapeExtendStatus::Done3,
        ShapeExtendStatus::Fail1,
        ShapeExtendStatus::Fail2,
        ShapeExtendStatus::Fail3,
    ] {
        row.push(f64::from(divider.status_flag(flag)));
    }
    row
}
#[test]
fn area_subdivision_matches_native_rows() {
    let reference = include_str!("fixtures/behavior/tkshhealing_face_divide_area.expected");
    for (mode, line) in reference
        .lines()
        .filter(|row| !row.starts_with("patch "))
        .enumerate()
    {
        let expected: Vec<f64> = line
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        let actual = observe(mode);
        assert_eq!(actual.len(), expected.len());
        for (column, (&found, &wanted)) in actual.iter().zip(&expected).enumerate() {
            assert!(
                found == wanted || (found - wanted).abs() < 1e-14,
                "mode {mode}, column {column}: {found}, expected {wanted}"
            );
        }
    }
}
#[test]
fn native_empty_constructors_and_area_defaults() {
    let base = ShapeUpgradeFaceDivide::empty();
    assert!(base.result().is_null());
    assert!(!base.perform(0.).unwrap());
    let divider = ShapeUpgradeFaceDivideArea::empty();
    assert!(divider.result().is_null());
    assert_eq!(divider.max_area(), precision::infinite());
    assert_eq!(divider.nb_parts(), 0);
    assert_eq!(divider.precision(), 1e-5);
    assert!(divider.context().is_null());
}
#[test]
fn grid_counts_preserve_native_nonpositive_behavior() {
    let divider = ShapeUpgradeFaceDivideArea::empty();
    divider.set_numbers_uv_splits(2, 3).unwrap();
    assert_eq!(divider.nb_parts(), 6);
    divider.set_numbers_uv_splits(-1, 3).unwrap();
    assert_eq!(divider.nb_parts(), 6);
    let error = divider.set_numbers_uv_splits(i32::MAX, 2).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
}

fn compare_sphere_patches(result: &Shape) {
    let reference = include_str!("fixtures/behavior/tkshhealing_face_divide_area.expected");
    let mut faces = Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    for line in reference.lines().filter(|row| row.starts_with("patch ")) {
        assert!(faces.more());
        let actual = patch_observation(faces.current());
        let expected: Vec<f64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        for (found, wanted) in actual.iter().zip(&expected) {
            assert!(
                (found - wanted).abs() < 1e-12,
                "patch {actual:?}, expected {expected:?}"
            );
        }
        faces.next().unwrap();
    }
    assert!(!faces.more());
}
fn patch_observation(shape: &Shape) -> [f64; 5] {
    let face = martensite_tkbrep::topods::face(shape).unwrap();
    let (u1, u2, v1, v2) =
        martensite_tkshhealing::shape_analysis::ShapeAnalysis::get_face_uv_bounds(&face).unwrap();
    let mut properties = martensite_tkgeombase::gprop::GProps::default();
    martensite_tktopalgo::brep_gprop::BRepGProp::surface_properties_eps(
        shape,
        &mut properties,
        1e-5,
        false,
    )
    .unwrap();
    [u1, u2, v1, v2, properties.mass()]
}

/// Native `tests/heal/split_two_numbers/A1..A3`: a hemisphere split by exact U/V counts.
#[test]
fn split_two_numbers_hemisphere_cases() {
    for (u_splits, v_splits, expected_faces) in [(3, 5, 15), (1, 5, 5), (5, 1, 5)] {
        let face = hemisphere(10.0);
        let divider = ShapeUpgradeFaceDivideArea::new(face);
        divider.set_splitting_by_number(true);
        // The native command derives the area threshold from the exact grid count.
        divider.set_max_area(-1.0);
        divider.set_numbers_uv_splits(u_splits, v_splits).unwrap();
        assert!(divider.perform(0.0).unwrap());
        let result = divider.result();
        assert_eq!(
            face_count(&result) as i32,
            expected_faces,
            "U={u_splits}, V={v_splits}"
        );
        finish_native_split_command(&result);
    }
}

fn hemisphere(radius: f64) -> martensite_tkbrep::topods::Face {
    use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(martensite_tkmath::gp::Axis3::default(), radius).unwrap())
            .cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0.0, std::f64::consts::TAU, 0.0, std::f64::consts::FRAC_PI_2),
        1e-7,
    )
    .unwrap()
    .face()
    .unwrap()
}

/// Native `tests/heal/split_number/A1,A2,A7,A8`: requested part counts on spheres.
#[test]
fn split_number_sphere_cases() {
    for (radius, v_min, v_max, parts, expected_faces) in [
        (10.0, 0.0, std::f64::consts::FRAC_PI_2, 10, 10),
        (10.0, 0.0, std::f64::consts::FRAC_PI_2, 100, 102),
        (10.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2, 10, 10),
        (10.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2, 100, 136),
    ] {
        let face = spherical_face(radius, v_min, v_max);
        let divider = ShapeUpgradeFaceDivideArea::new(face);
        divider.set_splitting_by_number(true);
        divider.set_max_area(-1.0);
        divider.set_nb_parts(parts);
        assert!(divider.perform(0.0).unwrap(), "parts={parts}");
        let result = divider.result();
        assert_eq!(face_count(&result) as i32, expected_faces, "parts={parts}");
        finish_native_split_command(&result);
    }
}

fn spherical_face(radius: f64, v_min: f64, v_max: f64) -> martensite_tkbrep::topods::Face {
    use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(martensite_tkmath::gp::Axis3::default(), radius).unwrap())
            .cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0.0, std::f64::consts::TAU, v_min, v_max),
        1e-7,
    )
    .unwrap()
    .face()
    .unwrap()
}

fn finish_native_split_command(result: &Shape) {
    let messages = Handle::new(ShapeExtendMsgRegistrator::new()).cast();
    ShapeFix::same_parameter(
        result,
        false,
        0.0,
        &ProgressRange::default(),
        &messages,
    )
    .unwrap();
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}
