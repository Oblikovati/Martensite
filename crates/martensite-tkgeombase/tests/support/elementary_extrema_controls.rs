//! Exact closed forms supplement the retained baseline without numerical tolerances.
use martensite_tkgeombase::extrema::{
    ElementType, ExtElC, ExtElC2d, ExtElCS, ExtElSS, ExtPElC, ExtPElC2d, ExtPElS, POnCurv,
    POnCurv2d, POnSurf, POnSurfParams,
};
use martensite_tkmath::gp::{Direction2, Direction3, Line2, Line3, Plane3, Point2, Point3};

#[test]
fn point_parameters_and_metadata_keep_exact_owned_values() {
    let point = Point3::new(2., 3., 4.);
    let mut curve = POnCurv::default();
    curve.set_values(5., point);
    assert_eq!((curve.parameter(), *curve.value()), (5., point));
    let mut curve2 = POnCurv2d::default();
    curve2.set_values(6., Point2::new(2., 3.));
    assert_eq!(
        (curve2.parameter(), *curve2.value()),
        (6., Point2::new(2., 3.))
    );
    let mut surface = POnSurf::default();
    surface.set_parameters(7., 8., point);
    assert_eq!((surface.parameter(), *surface.value()), ((7., 8.), point));
}
#[test]
fn surface_metadata_keeps_distance_kind_and_indices() {
    let mut params = POnSurfParams::new(7., 8., Point3::new(2., 3., 4.));
    params.set_sqr_distance(16.);
    params.set_element_type(ElementType::Face);
    params.set_indices(9, 10);
    assert_eq!(
        (
            params.get_sqr_distance(),
            params.get_element_type(),
            params.get_indices()
        ),
        (16., ElementType::Face, (9, 10))
    );
}
#[test]
fn projection_on_x_axis_has_orthogonal_remainder_4_squared_plus_5_squared() {
    let line = Line3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    let result = ExtPElC::with_line(Point3::new(3., 4., 5.), line, 0., -10., 10.);
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert!(result.is_min(1).unwrap());
    assert_eq!(result.square_distance(1).unwrap(), 41.);
    let point = result.point(1).unwrap();
    assert_eq!(
        (point.parameter(), *point.value()),
        (3., Point3::new(3., 0., 0.))
    );
}
#[test]
fn planar_projection_on_x_axis_has_squared_distance_16() {
    let line = Line2::new(Point2::new(0., 0.), Direction2::new(1., 0.).unwrap());
    let result = ExtPElC2d::with_line(Point2::new(3., 4.), line, 0., -10., 10.);
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 16.);
    let point = result.point(1).unwrap();
    assert_eq!(
        (point.parameter(), *point.value()),
        (3., Point2::new(3., 0.))
    );
}
#[test]
fn skew_orthogonal_axes_have_the_fixed_squared_height_separation() {
    let first = Line3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    let second = Line3::new(
        Point3::new(0., 0., 2.),
        Direction3::new(0., 1., 0.).unwrap(),
    );
    let result = ExtElC::lines(first, second, 0.);
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 4.);
    let (a, b) = result.points(1).unwrap();
    assert_eq!(
        (*a.value(), *b.value()),
        (Point3::new(0., 0., 0.), Point3::new(0., 0., 2.))
    );
}
#[test]
fn perpendicular_planar_axes_intersect_at_the_origin() {
    let first = Line2::new(Point2::new(0., 0.), Direction2::new(1., 0.).unwrap());
    let second = Line2::new(Point2::new(0., 0.), Direction2::new(0., 1.).unwrap());
    let result = ExtElC2d::lines(first, second, 0.).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 0.);
    let (a, b) = result.points(1).unwrap();
    assert_eq!(
        (*a.value(), *b.value()),
        (Point2::new(0., 0.), Point2::new(0., 0.))
    );
}
#[test]
fn projection_on_xy_plane_removes_only_height() {
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let result = ExtPElS::with_plane(Point3::new(2., 3., 4.), plane, 0.);
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 16.);
    assert_eq!(*result.point(1).unwrap().value(), Point3::new(2., 3., 0.));
}
#[test]
fn horizontal_line_above_xy_plane_has_squared_height_separation() {
    let line = Line3::new(
        Point3::new(0., 0., 2.),
        Direction3::new(1., 0., 0.).unwrap(),
    );
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let mut result = ExtElCS::new();
    result.perform_line_plane(&line, &plane);
    assert!(result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 4.);
}
#[test]
fn nonparallel_line_plane_contract_reports_done_without_an_extremum() {
    let line = Line3::new(
        Point3::new(0., 0., 2.),
        Direction3::new(0., 0., 1.).unwrap(),
    );
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let mut result = ExtElCS::new();
    result.perform_line_plane(&line, &plane);
    assert!(result.is_done());
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 0);
}
#[test]
fn parallel_planes_have_the_squared_normal_separation() {
    let first = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let second = Plane3::new(
        Point3::new(0., 0., 2.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let result = ExtElSS::planes(first, second);
    assert!(result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 4.);
}
