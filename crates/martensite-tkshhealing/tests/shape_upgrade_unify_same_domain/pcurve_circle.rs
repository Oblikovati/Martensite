use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomCircle, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::{
    Axis2, Axis22d, Circle2, Circle3, Direction2, Direction3, Point2, Point3,
};
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
    BRepBuilderAPIMakeWire,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

type PCurveObservation = (CurveType, (f64, f64), Point2, Point2);

#[test]
fn circular_uv_arcs_merge_on_nonplanar_bspline_surface() {
    let input = paraboloid_circle_face().unwrap();
    let before = edge_pcurve_observations(&input).unwrap();
    assert_eq!(before.len(), 2);
    assert!(before.iter().all(|row| row.0 == CurveType::Circle));

    let unifier = ShapeUpgradeUnifySameDomain::new(&input, true, false, false);
    unifier.build().unwrap();

    let result = unifier.shape();
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let after = edge_pcurve_observations(&result).unwrap();
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].0, CurveType::BSplineCurve);
    assert_eq!(after[0].1, (0.0, std::f64::consts::TAU));
    assert_point(after[0].2, Point2::new(-0.5, 0.0));
    assert_point(after[0].3, Point2::new(-0.5, 0.0));
}

fn paraboloid_circle_face() -> Result<Shape, martensite_tkernel::standard::Failure> {
    let surface = paraboloid_surface()?;
    let wire = circle_arc_wire(&surface)?;
    let face = BRepBuilderAPIMakeFace::from_surface_wire(&surface, &wire, true)?;
    Ok(face.face()?.into_shape())
}

fn paraboloid_surface() -> Result<Handle<dyn GeomSurface>, martensite_tkernel::standard::Failure> {
    let poles = paraboloid_poles()?;
    let (knots, multiplicities) = quadratic_domain()?;
    Ok(Handle::new(GeomBSplineSurface::new(
        &poles,
        (&knots, &multiplicities),
        (&knots, &multiplicities),
        (2, 2),
        (false, false),
    )?)
    .cast())
}

fn paraboloid_poles() -> Result<Array2<'static, Point3>, martensite_tkernel::standard::Failure> {
    let values = [
        Point3::new(-1.0, -1.0, 1.75),
        Point3::new(-1.0, 0.0, -0.25),
        Point3::new(-1.0, 1.0, 1.75),
        Point3::new(0.0, -1.0, -0.25),
        Point3::new(0.0, 0.0, -2.25),
        Point3::new(0.0, 1.0, -0.25),
        Point3::new(1.0, -1.0, 1.75),
        Point3::new(1.0, 0.0, -0.25),
        Point3::new(1.0, 1.0, 1.75),
    ];
    let mut poles = Array2::with_bounds(1, 3, 1, 3).map_err(|error| {
        martensite_tkernel::standard::Failure::new(&format!(
            "invalid quadratic pole grid {error:?}"
        ))
    })?;
    poles.as_mut_slice().copy_from_slice(&values);
    Ok(poles)
}

fn quadratic_domain()
-> Result<(Array1<'static, f64>, Array1<'static, i32>), martensite_tkernel::standard::Failure> {
    let knots = Array1::from_shared(&[-1.0, 1.0], 1);
    let multiplicities = Array1::from_shared(&[3, 3], 1);
    Ok((knots, multiplicities))
}

fn circle_arc_wire(
    surface: &Handle<dyn GeomSurface>,
) -> Result<martensite_tkbrep::topods::Wire, martensite_tkernel::standard::Failure> {
    let curve = circle_curve()?;
    let (start, end) = circle_vertices()?;
    let (edge1, edge2) = circle_arcs(&curve, &start, &end)?;
    let pcurve = circle_pcurve()?;
    store_circle_arc_pcurves(&edge1, &edge2, &pcurve, surface)?;
    let mut builder = BRepBuilderAPIMakeWire::new();
    builder.add_edge(&edge1)?;
    builder.add_edge(&edge2)?;
    builder.wire()
}

fn circle_vertices() -> Result<
    (
        martensite_tkbrep::topods::Vertex,
        martensite_tkbrep::topods::Vertex,
    ),
    martensite_tkernel::standard::Failure,
> {
    let start = BRepBuilderAPIMakeVertex::new(Point3::new(0.5, 0.0, 0.0))?.vertex()?;
    let end = BRepBuilderAPIMakeVertex::new(Point3::new(-0.5, 0.0, 0.0))?.vertex()?;
    Ok((start, end))
}

fn circle_arcs(
    curve: &Handle<dyn martensite_tkg3d::geom::GeomCurve>,
    start: &martensite_tkbrep::topods::Vertex,
    end: &martensite_tkbrep::topods::Vertex,
) -> Result<(Edge, Edge), martensite_tkernel::standard::Failure> {
    let first = circle_arc(curve, start, end, (0.0, std::f64::consts::PI))?;
    let second = circle_arc(
        curve,
        end,
        start,
        (std::f64::consts::PI, std::f64::consts::TAU),
    )?;
    Ok((first, second))
}

fn store_circle_arc_pcurves(
    first: &Edge,
    second: &Edge,
    curve: &Handle<dyn Geom2dCurve>,
    surface: &Handle<dyn GeomSurface>,
) -> Result<(), martensite_tkernel::standard::Failure> {
    store_circle_pcurve(first, curve, surface, (0.0, std::f64::consts::PI))?;
    store_circle_pcurve(
        second,
        curve,
        surface,
        (std::f64::consts::PI, std::f64::consts::TAU),
    )
}

fn circle_curve()
-> Result<Handle<dyn martensite_tkg3d::geom::GeomCurve>, martensite_tkernel::standard::Failure> {
    let axis = Axis2::new(Point3::default(), Direction3::Z)?;
    let circle = Circle3::new(axis, 0.5)?;
    Ok(Handle::new(GeomCircle::from_circ(circle)).cast())
}

fn circle_arc(
    curve: &Handle<dyn martensite_tkg3d::geom::GeomCurve>,
    start: &martensite_tkbrep::topods::Vertex,
    end: &martensite_tkbrep::topods::Vertex,
    range: (f64, f64),
) -> Result<Edge, martensite_tkernel::standard::Failure> {
    BRepBuilderAPIMakeEdge::from_curve_vertices_params(curve, start, end, range.0, range.1)?.edge()
}

fn circle_pcurve() -> Result<Handle<dyn Geom2dCurve>, martensite_tkernel::standard::Failure> {
    let axis = Axis22d::new(Point2::default(), Direction2::X, true)?;
    Ok(Handle::new(Geom2dCircle::from_circ2d(Circle2::new(axis, 0.5)?)).cast())
}

fn store_circle_pcurve(
    edge: &Edge,
    curve: &Handle<dyn Geom2dCurve>,
    surface: &Handle<dyn GeomSurface>,
    range: (f64, f64),
) -> Result<(), martensite_tkernel::standard::Failure> {
    let builder = BRepBuilder::default();
    builder.update_edge_pcurve(
        edge,
        curve,
        surface,
        &Default::default(),
        precision::confusion(),
        None,
    )?;
    builder.range_on_surface(edge, surface, &Default::default(), range.0, range.1)
}

fn edge_pcurve_observations(
    shape: &Shape,
) -> Result<Vec<PCurveObservation>, martensite_tkernel::standard::Failure> {
    let face = first_face(shape)?;
    let mut result = Vec::new();
    for candidate in Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)? {
        let edge = martensite_tkbrep::topods::edge(&candidate)?;
        result.push(edge_pcurve_observation(&edge, &face)?);
    }
    Ok(result)
}

fn first_face(shape: &Shape) -> Result<Face, martensite_tkernel::standard::Failure> {
    let faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?.collect::<Vec<_>>();
    let first = faces
        .first()
        .ok_or_else(|| martensite_tkernel::standard::Failure::new("shape has no face"))?;
    martensite_tkbrep::topods::face(first)
}

fn edge_pcurve_observation(
    edge: &Edge,
    face: &Face,
) -> Result<PCurveObservation, martensite_tkernel::standard::Failure> {
    let pcurve = BRepTool::curve_on_face(edge, face)?;
    let adaptor = martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve::with_curve(&pcurve.pcurve)?;
    Ok((
        adaptor.get_type()?,
        (pcurve.first, pcurve.last),
        pcurve.pcurve.eval_d0(pcurve.first)?,
        pcurve.pcurve.eval_d0(pcurve.last)?,
    ))
}

fn assert_point(point: Point2, expected: Point2) {
    assert!(
        (point.x() - expected.x()).abs() < 1.0e-12,
        "x coordinate {point:?} does not match {expected:?}"
    );
    assert!(
        (point.y() - expected.y()).abs() < 1.0e-12,
        "y coordinate {point:?} does not match {expected:?}"
    );
}
