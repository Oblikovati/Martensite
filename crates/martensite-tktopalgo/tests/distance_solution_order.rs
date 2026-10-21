//! Original distance-command solution endpoint ordering regressions.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{Direction3, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
};
use martensite_tktopalgo::brep_extrema::BRepExtremaDistShapeShape;
use martensite_tktopalgo::brep_lib::BRepLibMakeEdge;

fn command_distance(first: &Shape, second: &Shape) -> Result<BRepExtremaDistShapeShape, Failure> {
    let mut distance = BRepExtremaDistShapeShape::new();
    distance.load_s1(first)?;
    distance.load_s2(second)?;
    distance.set_deflection(1e-7);
    distance.set_multi_thread(false);
    distance.perform(&ProgressRange::default())?;
    assert!(distance.is_done());
    Ok(distance)
}

fn fixture_array<T: Clone + Default>(values: &[T]) -> Array1<'static, T> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    array.as_mut_slice().clone_from_slice(values);
    array
}

fn original_quadratic_curve() -> Result<Handle<dyn GeomCurve>, Failure> {
    let poles = fixture_array(&[
        Point3::new(0., 2., 0.),
        Point3::new(1., 1., 0.),
        Point3::new(2., 2., 0.),
    ]);
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomBSplineCurve::with_weights(
        &poles,
        &fixture_array(&[1.; 3]),
        &fixture_array(&[0., 1.]),
        &fixture_array(&[3, 3]),
        2,
        false,
    )?)
    .cast();
    Ok(curve)
}

fn original_result_vertices(first: &Shape, second: &Shape) -> Result<Vec<Shape>, Failure> {
    let distance = command_distance(first, second)?;
    assert!(distance.value()? > 1e-9);
    let edge =
        BRepLibMakeEdge::from_points(distance.point_on_shape1(1)?, distance.point_on_shape2(1)?)?
            .edge()?;
    Ok(Explorer::new(&edge, ShapeEnum::Vertex, ShapeEnum::Shape)?.collect())
}

fn original_four_distances(first: &Shape, second: &Shape) -> Result<[f64; 4], Failure> {
    let vertices = original_result_vertices(first, second)?;
    assert_eq!(vertices.len(), 2);
    Ok([
        command_distance(&vertices[0], first)?.value()?,
        command_distance(&vertices[1], second)?.value()?,
        command_distance(&vertices[0], second)?.value()?,
        command_distance(&vertices[1], first)?.value()?,
    ])
}

#[test]
fn bug22758_quadratic_edge_solution_endpoints_are_not_swapped() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "distance_solution_order::bug22758_quadratic_edge_solution_endpoints_are_not_swapped",
        || Ok(bug22758_quadratic_edge_solution_endpoints_are_not_swapped_body()?),
    );
    Ok(())
}

fn bug22758_quadratic_edge_solution_endpoints_are_not_swapped_body() -> Result<(), Failure> {
    let edge = BRepBuilderAPIMakeEdge::from_curve(&original_quadratic_curve()?)?.edge()?;
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 1.))?.vertex()?;
    let [correct1, correct2, swapped1, swapped2] = original_four_distances(&edge, &vertex)?;
    assert!(!(correct1 > 1e-7 || correct2 > 1e-7));
    assert!(!(swapped1 < 0.1 || swapped2 < 0.1));
    Ok(())
}

#[test]
fn bug23051_bounded_plane_solution_preserves_original_combined_status() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "distance_solution_order::bug23051_bounded_plane_solution_preserves_original_combined_status",
        || Ok(bug23051_bounded_plane_solution_preserves_original_combined_status_body()?),
    );
    Ok(())
}

fn bug23051_bounded_plane_solution_preserves_original_combined_status_body() -> Result<(), Failure>
{
    let plane = Plane3::new(Point3::new(0., 0., 0.), Direction3::new(0., 0., 1.)?)?;
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(plane)).cast();
    let face = BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (-10., 10., -10., 10.), 1e-7)?
        .face()?;
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 1.))?.vertex()?;
    let [correct1, correct2, swapped1, swapped2] = original_four_distances(&face, &vertex)?;
    let status1 = correct1 > 1e-7 || correct2 > 1e-7;
    let status2 = swapped1 < 0.1 || swapped2 < 0.1;
    assert!(!(status1 && status2));
    Ok(())
}

const BUG30199_KNOTS: [f64; 8] = [
    0.,
    147.948545311472,
    227.205037349287,
    282.782457537237,
    341.526569446034,
    412.981618615062,
    493.370506013535,
    820.162497637298,
];

fn original_irregular_polyline_poles() -> Array1<'static, Point2> {
    fixture_array(&[
        Point2::new(70.4310501764176, -125.064069601289),
        Point2::new(202.351851590745, -192.039245703947),
        Point2::new(281.504332439341, -187.980144121968),
        Point2::new(310.594560443526, -140.62395898876),
        Point2::new(274.062646205713, -94.6208077364441),
        Point2::new(341.714339238701, -71.6192321052281),
        Point2::new(397.188727525752, -129.799688113598),
        Point2::new(70.4310501764176, -125.064069601289),
    ])
}

fn original_irregular_polyline_curve() -> Result<Handle<dyn GeomCurve>, Failure> {
    let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dBSplineCurve::with_weights(
        &original_irregular_polyline_poles(),
        &fixture_array(&[1.; 8]),
        &fixture_array(&BUG30199_KNOTS),
        &fixture_array(&[2, 1, 1, 1, 1, 1, 1, 2]),
        1,
        false,
    )?)
    .cast();
    geom_api::to_3d(&curve, &Plane3::default())
}

#[test]
fn bug30199_both_original_irregular_polyline_point_distances() -> Result<(), Failure> {
    crate::isolated_topology_case::run_isolated_case(
        "distance_solution_order::bug30199_both_original_irregular_polyline_point_distances",
        || Ok(bug30199_both_original_irregular_polyline_point_distances_body()?),
    );
    Ok(())
}

#[allow(clippy::excessive_precision)] // Preserve the original decimal inputs and reference distance.
fn bug30199_both_original_irregular_polyline_point_distances_body() -> Result<(), Failure> {
    let edge = BRepBuilderAPIMakeEdge::from_curve(&original_irregular_polyline_curve()?)?.edge()?;
    for (point, expected) in [
        (Point3::new(360., -127., 0.), 2.26048366458175),
        (
            Point3::new(199.49416090801449, -197.77438365880749, 0.),
            6.4076675475126184,
        ),
    ] {
        assert_original_point_distance(&edge, point, expected)?;
    }
    Ok(())
}

fn assert_original_point_distance(
    edge: &Shape,
    point: Point3,
    expected: f64,
) -> Result<(), Failure> {
    let vertex = BRepBuilderAPIMakeVertex::new(point)?.vertex()?;
    let distance = command_distance(edge, &vertex)?;
    let actual = distance.value()?;
    assert!(
        (actual - expected).abs() <= 1e-7,
        "distance {actual}, expected {expected} ±1e-7"
    );
    let result =
        BRepLibMakeEdge::from_points(distance.point_on_shape1(1)?, distance.point_on_shape2(1)?)?
            .edge()?;
    assert!(!result.is_null());
    Ok(())
}
