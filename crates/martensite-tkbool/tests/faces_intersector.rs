//! Face-pair state before intersection preserves the declared native contracts.
use martensite_tkbool::top_ope_brep::TopOpeBRepFacesIntersector;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;

#[test]
fn initial_pair_has_null_faces_and_no_restrictions() {
    let pair = TopOpeBRepFacesIntersector::new();
    assert!(!pair.is_done());
    assert!(pair.face(1).unwrap().is_null());
    assert!(pair.face(2).unwrap().is_null());
    assert_eq!(pair.restrictions().extent(), 0);
    assert!(!pair.is_restriction(&Shape::default()));
    assert_eq!(
        pair.get_tolerances(),
        (precision::confusion(), precision::confusion())
    );
}

#[test]
fn tolerance_override_retains_values_without_native_validation() {
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.force_tolerances(-1., f64::INFINITY);
    assert_eq!(pair.get_tolerances(), (-1., f64::INFINITY));
    pair.force_tolerances(2e-6, 3e-7);
    assert_eq!(pair.get_tolerances(), (2e-6, 3e-7));
    assert!(!pair.is_done());
}

#[test]
fn preperform_domain_queries_report_the_native_program_error() {
    let pair = TopOpeBRepFacesIntersector::new();
    assert!(pair.same_domain().is_err());
    assert!(pair.surfaces_same_oriented().is_err());
    for index in [i32::MIN, 0, 3, i32::MAX] {
        assert!(pair.face(index).is_err());
    }
}

#[test]
fn initial_traversal_is_unbound_and_retains_the_advanced_index() {
    let mut pair = TopOpeBRepFacesIntersector::new();
    assert!(pair.lines().is_null());
    assert_eq!(pair.nb_lines(), 0);
    pair.init_line().unwrap();
    assert!(!pair.more_line());
    assert_eq!(pair.current_line_index(), 1);
    assert!(pair.current_line().is_err());
    pair.next_line().unwrap();
    assert_eq!(pair.current_line_index(), 2);
    assert!(!pair.more_line());
    assert!(pair.change_line(0).is_err());
}

fn plane(z: f64) -> Shape {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(Point3::new(0., 0., z), Direction3::new(0., 0., 1.).unwrap()).unwrap(),
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape()
}

#[test]
fn disjoint_parallel_planes_complete_empty_and_retain_forced_tolerances() {
    let mut pair = TopOpeBRepFacesIntersector::new();
    assert!(!pair.is_empty().unwrap());
    pair.force_tolerances(2e-6, 3e-6);
    pair.perform(&plane(0.), &plane(1.)).unwrap();
    assert!(pair.is_done());
    assert!(pair.is_empty().unwrap());
    assert!(!pair.same_domain().unwrap());
    assert!(pair.surfaces_same_oriented().is_err());
    assert_eq!(pair.get_tolerances(), (2e-6, 3e-6));
    assert_eq!(pair.nb_lines(), 0);
}

#[test]
fn coincident_planes_preserve_surface_orientation_and_reset_traversal() {
    let mut pair = TopOpeBRepFacesIntersector::new();
    let first = plane(0.);
    pair.perform(&first, &first).unwrap();
    assert!(pair.is_done());
    assert!(pair.same_domain().unwrap());
    assert!(pair.surfaces_same_oriented().unwrap());
    assert_eq!(pair.get_tolerances(), (2e-7, 2e-7));
    assert!(pair.face(1).unwrap().is_same(&first));
    assert_eq!(pair.current_line_index(), 1);
    pair.perform(&Shape::default(), &first).unwrap_err();
    assert!(!pair.is_done());
    assert_eq!(pair.nb_lines(), 0);
    assert_eq!(pair.restrictions().extent(), 0);
}

#[test]
fn transverse_trimmed_planes_publish_one_segment_with_closed_form_endpoints() {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    let first = plane(0.);
    let vertical = Plane3::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap()).unwrap();
    let second = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        vertical,
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&first, &second).unwrap();
    assert!(pair.is_done());
    assert!(!pair.same_domain().unwrap());
    assert!(!pair.is_empty().unwrap());
    assert_eq!(pair.nb_lines(), 1);
    pair.init_line().unwrap();
    let line = pair.current_line().unwrap();
    assert_eq!(line.nb_vpoint(), 2);
    let a = line.vpoint(1).unwrap().value().unwrap();
    let b = line.vpoint(2).unwrap().value().unwrap();
    assert!((a.distance(&b) - 2.).abs() < 1e-12);
    for point in [a, b] {
        assert!(point.x().abs() < 1e-12);
        assert!(point.z().abs() < 1e-12);
        assert!((point.y().abs() - 1.).abs() < 1e-12);
    }
}

#[test]
fn shared_line_arrays_keep_native_bounds_alias_edits_and_failed_perform_identity() {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    let first = plane(0.);
    let other = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap()).unwrap(),
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&first, &other).unwrap();
    let shared = pair.lines();
    assert_eq!(shared.array1().lower(), 1);
    assert_eq!(shared.array1().upper(), 1);
    shared
        .change_array1()
        .change_value(1)
        .unwrap()
        .set_ok(false);
    pair.init_line().unwrap();
    assert!(!pair.more_line());
    assert_eq!(pair.current_line_index(), 2);
    shared.change_array1().change_value(1).unwrap().set_ok(true);
    pair.init_line().unwrap();
    assert!(pair.more_line());
    pair.change_line(1).unwrap().set_index(77);
    assert_eq!(shared.array1().value(1).unwrap().index(), 77);
    assert!(pair.change_line(i32::MIN).is_err());
    pair.perform(&Shape::default(), &first).unwrap_err();
    assert!(pair.lines() == shared);
    assert_eq!(pair.nb_lines(), 0);
    assert_eq!(pair.change_line(1).unwrap().index(), 77);
    pair.perform(&first, &first).unwrap();
    assert_eq!(pair.lines().array1().lower(), 0);
    assert_eq!(pair.lines().array1().upper(), 0);
    assert_eq!(pair.change_line(0).unwrap().index(), 0);
}

#[test]
fn face_intersection_operation_and_line_records_are_send_sync() {
    fn require<T: Send + Sync>() {}
    require::<TopOpeBRepFacesIntersector>();
    require::<martensite_tkbool::top_ope_brep::TopOpeBRepLineInter>();
}

#[test]
fn coincident_boundary_segments_record_their_topological_restrictions() {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    let first = plane(0.);
    let perpendicular = Plane3::new(
        Point3::new(0., -1., 0.),
        Direction3::new(0., 1., 0.).unwrap(),
    )
    .unwrap();
    let second = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        perpendicular,
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&first, &second).unwrap();
    assert!(pair.is_done());
    // The native face intersector preserves both the geometric line and its restriction.
    assert_eq!(pair.nb_lines(), 2);
    assert_eq!(pair.change_line(1).unwrap().nb_vpoint(), 4);
    assert_eq!(pair.change_line(2).unwrap().nb_vpoint(), 4);
    assert_eq!(pair.restrictions().extent(), 1);
    let edge = pair.restrictions().find_key(1).unwrap().clone();
    assert!(pair.is_restriction(&edge));
}

#[test]
fn coincident_surfaces_with_opposite_normals_report_opposed_orientation() {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    let reversed_plane =
        Plane3::new(Point3::default(), Direction3::new(0., 0., -1.).unwrap()).unwrap();
    let other = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        reversed_plane,
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&plane(0.), &other).unwrap();
    assert!(pair.same_domain().unwrap());
    assert!(!pair.surfaces_same_oriented().unwrap());
}

#[test]
fn a_second_face_cast_failure_retains_the_last_surface_context_until_preparation() {
    use martensite_tkmath::gp::{Direction3, Plane3, Point3};
    let old_first = plane(0.);
    let vertical = Plane3::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap()).unwrap();
    let second = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
        vertical,
        (-1., 1., -1., 1.),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape();
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&old_first, &second).unwrap();
    let replacement = plane(5.);
    let wrong_kind = martensite_tkbrep::topexp::Explorer::new(
        &old_first,
        martensite_tkg3d::top_abs::ShapeEnum::Edge,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )
    .unwrap()
    .current()
    .clone();
    pair.perform(&replacement, &wrong_kind).unwrap_err();
    assert!(!pair.is_done());
    assert!(pair.face(1).unwrap().is_same(&replacement));
    assert!(pair.face(2).unwrap().is_same(&second));
    pair.prepare_lines().unwrap();
    assert_eq!(pair.nb_lines(), 1);
    let point = pair
        .change_line(1)
        .unwrap()
        .vpoint(1)
        .unwrap()
        .value()
        .unwrap();
    assert!(point.z().abs() < 1e-12);
    assert!(!pair.is_done());
}
