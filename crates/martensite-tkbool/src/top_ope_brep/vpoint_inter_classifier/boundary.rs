//! The slow boundary pass validates spatial incidence before accepting UV proximity.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Edge, Face},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkgeomalgo::geom_api::ProjectPointOnCurve;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tktopalgo::brep_class::BRepClassFaceClassifier;

pub(super) fn slow_boundary(
    point: Point3,
    uv: Point2,
    slow: &mut BRepClassFaceClassifier,
    face: &Face,
) -> Result<State, Failure> {
    let parameter = slow.edge_parameter()?;
    let edge = slow.edge()?.edge().clone();
    let (curve, first, last) = BRepTool::curve(&edge)?;
    if curve.is_null() || same_vertex(point, &edge)? {
        return Ok(State::On);
    }
    let projection = ProjectPointOnCurve::with_curve(point, &curve)?;
    let distance = closest_parameter_distance(&projection, parameter, last - first)?;
    // The native negated >= accepts unordered values; replacing it with < would not.
    if projection.nb_points() != 0
        && !matches!(
            distance.partial_cmp(&BRepTool::edge_tolerance(&edge)?),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
    {
        return Ok(State::On);
    }
    refine_uv(uv, parameter, &edge, face, slow)
}

fn same_vertex(point: Point3, edge: &Edge) -> Result<bool, Failure> {
    for vertex in Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)? {
        let vertex = topods::vertex(&vertex)?;
        if point.is_equal(
            &BRepTool::pnt(&vertex)?,
            BRepTool::vertex_tolerance(&vertex)?,
        ) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn refine_uv(
    uv: Point2,
    parameter: f64,
    edge: &Edge,
    face: &Face,
    slow: &mut BRepClassFaceClassifier,
) -> Result<State, Failure> {
    let pcurve = BRepTool::curve_on_face(edge, face)?;
    if pcurve.pcurve.is_null() {
        return Ok(State::Out);
    }
    let limit = uv.distance(pcurve.pcurve.value(parameter)?) / 3.0;
    slow.perform(face, &uv, limit, false, 0.1)?;
    Ok(if slow.state() == State::In {
        State::In
    } else {
        State::Out
    })
}

fn closest_parameter_distance(
    projection: &ProjectPointOnCurve,
    parameter: f64,
    mut difference: f64,
) -> Result<f64, Failure> {
    let mut distance = 0.0;
    for index in 1..=projection.nb_points() {
        let current = (projection.parameter(index)? - parameter).abs();
        if current < difference {
            difference = current;
            distance = projection.distance(index)?;
        }
    }
    Ok(distance)
}
