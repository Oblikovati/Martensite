//! differential coverage for `ShapeUpgradeClosedFaceDivide`.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Axis3;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis, BRepPrimAPIMakeSphere,
};
use martensite_tkshhealing::{
    shape_extend::ShapeExtendStatus,
    shape_upgrade::{FaceDivider, ShapeUpgradeClosedFaceDivide},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[test]
fn cylinder_seam_uses_native_default_split_count()
-> Result<(), martensite_tkernel::standard::Failure> {
    let face = BRepPrimAPIMakeCylinder::new(2.0, 5.0).face()?;
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    assert_eq!(divider.nb_split_points(), 1);
    assert!(divider.perform(0.0)?);
    assert_native_row("cylinder_seam_default", &divider, true, 1, 1);
    Ok(())
}

#[test]
fn cylinder_seam_honors_multiple_requested_cut_points()
-> Result<(), martensite_tkernel::standard::Failure> {
    let face = BRepPrimAPIMakeCylinder::new(2.0, 5.0).face()?;
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    divider.set_nb_split_points(2);
    assert_eq!(divider.nb_split_points(), 2);
    assert!(divider.perform(0.0)?);
    assert_native_row("cylinder_seam_two_points", &divider, true, 2, 2);
    Ok(())
}

#[test]
fn trait_dispatch_uses_the_closed_face_splitter()
-> Result<(), martensite_tkernel::standard::Failure> {
    let face = BRepPrimAPIMakeCylinder::new(2.0, 5.0).face()?;
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    divider.set_nb_split_points(0);
    assert_eq!(divider.nb_split_points(), 1);
    let policy: &dyn FaceDivider = &divider;
    assert!(policy.perform(0.0)?);
    assert_native_row("cylinder_seam_default", &divider, true, 1, 1);
    Ok(())
}

#[test]
fn sphere_uses_geometric_closure_fallback() -> Result<(), martensite_tkernel::standard::Failure> {
    let face = BRepPrimAPIMakeSphere::new(2.0)?.face()?;
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    assert!(divider.perform(0.0)?);
    assert_native_row("sphere_geometric_closure", &divider, true, 3, 1);
    Ok(())
}

#[test]
fn geometric_closure_uses_configured_precision() -> Result<(), martensite_tkernel::standard::Failure>
{
    let face = near_closed_face()?;
    let default_precision = ShapeUpgradeClosedFaceDivide::new(face.clone());
    assert!(!default_precision.perform(0.0)?);
    assert_native_row(
        "near_closed_default_precision",
        &default_precision,
        false,
        4,
        1,
    );
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    divider.set_precision(0.1);
    assert!(divider.perform(0.0)?);
    assert_native_row("near_closed_precision", &divider, true, 5, 1);
    Ok(())
}

fn near_closed_face()
-> Result<martensite_tkbrep::topods::Face, martensite_tkernel::standard::Failure> {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(Axis3::default(), 2.0)?).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0.0, std::f64::consts::TAU - 0.01, -1.0, 1.0),
        1e-7,
    )
    .unwrap()
    .face()
}

#[test]
fn direct_split_on_uninitialized_result_sets_fail3() {
    let divider = ShapeUpgradeClosedFaceDivide::empty();
    assert!(!divider.split_surface(0.0).unwrap());
    assert!(divider.status_flag(ShapeExtendStatus::Fail3));
    let actual = measured_row("empty_result_fail3", &divider, false);
    let expected = include_str!("fixtures/behavior/shape_upgrade_closed_face_divide.expected");
    assert_eq!(actual, expected.lines().next().unwrap());
}

#[test]
fn closed_face_divider_keeps_native_parent_rtti() {
    let divider = Handle::new(ShapeUpgradeClosedFaceDivide::empty());
    assert_eq!(
        divider.dynamic_type().unwrap().name(),
        "ShapeUpgrade_ClosedFaceDivide"
    );
    assert!(divider.is_kind_name("ShapeUpgrade_FaceDivide"));
}

fn assert_native_row(
    name: &str,
    divider: &ShapeUpgradeClosedFaceDivide,
    changed: bool,
    row: usize,
    split_points: i32,
) {
    let actual = measured_row(name, divider, changed);
    let expected = include_str!("fixtures/behavior/shape_upgrade_closed_face_divide.expected");
    assert_eq!(actual, expected.lines().nth(row).unwrap());
    assert_eq!(split_points, divider.nb_split_points());
}

fn measured_row(name: &str, divider: &ShapeUpgradeClosedFaceDivide, changed: bool) -> String {
    let result = divider.result();
    format!(
        "{name} changed={} faces={} edges={} seams={} done2={} fail2={} fail3={}",
        usize::from(changed),
        count(&result, ShapeEnum::Face),
        count(&result, ShapeEnum::Edge),
        count_seams(&result),
        usize::from(divider.status_flag(ShapeExtendStatus::Done2)),
        usize::from(divider.status_flag(ShapeExtendStatus::Fail2)),
        usize::from(divider.status_flag(ShapeExtendStatus::Fail3)),
    )
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    Explorer::new(shape, kind, ShapeEnum::Shape)
        .unwrap()
        .count()
}

// A literal zero hid seam occurrences that the original native probe actually counts.
fn count_seams(shape: &Shape) -> usize {
    let mut seams = 0;
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let face = topods::face(&face).unwrap();
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            seams += usize::from(
                BRepTool::is_closed_on_face(&topods::edge(&edge).unwrap(), &face).unwrap(),
            );
        }
    }
    seams
}

#[test]
fn doubly_periodic_torus_opens_both_seams_with_the_requested_grid() {
    for points in [1, 2] {
        assert_torus_grid(points);
    }
}

fn assert_torus_grid(points: i32) {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
    let face = BRepPrimAPIMakeTorus::new(3., 1.).unwrap().face().unwrap();
    let divider = ShapeUpgradeClosedFaceDivide::new(face);
    divider.set_nb_split_points(points);
    assert!(divider.perform(0.).unwrap());
    let result = divider.result();
    let cells = usize::try_from((points + 1) * (points + 1)).unwrap();
    assert_eq!(count(&result, ShapeEnum::Face), cells);
    assert_eq!(count(&result, ShapeEnum::Edge), 4 * cells);
    assert_eq!(count_seams(&result), 0);
    assert!(divider.status_flag(ShapeExtendStatus::Done2));
    assert!(!divider.status_flag(ShapeExtendStatus::Fail2));
    assert_torus_intervals(&result, points);
}

fn assert_torus_intervals(result: &Shape, points: i32) {
    let expected_span = std::f64::consts::TAU / f64::from(points + 1);
    for face in Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape).unwrap() {
        let face = topods::face(&face).unwrap();
        let (u0, u1, v0, v1) =
            martensite_tkshhealing::shape_analysis::ShapeAnalysis::get_face_uv_bounds(&face)
                .unwrap();
        for span in [u1 - u0, v1 - v0] {
            assert!((span - expected_span).abs() <= martensite_tkernel::precision::p_confusion());
        }
    }
}

#[test]
fn absent_splitter_returns_before_result_validation_without_failure_status() {
    let divider = ShapeUpgradeClosedFaceDivide::empty();
    divider.set_split_surface_tool(&Handle::default());
    assert!(!divider.split_surface(0.).unwrap());
    assert!(!divider.status_flag(ShapeExtendStatus::Fail3));
    assert!(!divider.status_flag(ShapeExtendStatus::Done));
    assert!(divider.result().is_null());
}

#[test]
fn seam_measurement_detects_the_original_single_and_double_periodic_faces() {
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
    let cylinder = BRepPrimAPIMakeCylinder::new(2., 5.).face().unwrap();
    let torus = BRepPrimAPIMakeTorus::new(3., 1.).unwrap().face().unwrap();
    assert_eq!(count_seams(&cylinder.into_shape()), 2);
    assert_eq!(count_seams(&torus.into_shape()), 4);
}
