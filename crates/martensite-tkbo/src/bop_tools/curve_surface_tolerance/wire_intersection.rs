use super::wires::EdgeSample;
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorCurve,
    topods::{Shape, Vertex},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, geom::GeomSurface};
use martensite_tkgeomalgo::{
    geom2d_int::GInter,
    int_res2d::{Domain, IntersectionPoint},
};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
pub(super) fn intersection_distance(
    vertex: &Vertex,
    surface: &Handle<dyn GeomSurface>,
    first: &EdgeSample,
    second: &EdgeSample,
    lengths: &mut Vec<(Shape, f64)>,
) -> Result<f64, Failure> {
    if first.edge.is_same(&second.edge) {
        return Ok(0.);
    }
    let mut intersection = GInter::default();
    intersection.perform(
        &first.adaptor,
        &domain(first)?,
        &second.adaptor,
        &domain(second)?,
        1e-10,
        1e-10,
    )?;
    if !intersection.is_done() {
        return Ok(0.);
    }
    let points = intersection_points(&intersection)?;
    if points.is_empty() {
        return Ok(0.);
    }
    // MapEdgeLength is deliberately called on the FIRST edge twice; keep that threshold.
    let limit = martensite_cmath::powi(edge_length(first, lengths)? * 0.3, 2);
    distance_from_intersections(vertex, surface, first, second, &points, limit)
}
fn domain(edge: &EdgeSample) -> Result<Domain, Failure> {
    let (first, last) = edge.range;
    Ok(Domain::bounded(
        edge.adaptor.d0(first)?,
        first,
        1e-10,
        edge.adaptor.d0(last)?,
        last,
        1e-10,
    ))
}
fn intersection_points(intersection: &GInter) -> Result<Vec<IntersectionPoint>, Failure> {
    let mut points = Vec::new();
    for index in 1..=intersection.nb_segments()? {
        let segment = intersection.segment(index)?;
        points.extend([segment.first_point()?, segment.last_point()?]);
    }
    for index in 1..=intersection.nb_points()? {
        points.push(intersection.point(index)?);
    }
    Ok(points)
}
fn edge_length(edge: &EdgeSample, lengths: &mut Vec<(Shape, f64)>) -> Result<f64, Failure> {
    if let Some((_, length)) = lengths.iter().find(|(shape, _)| shape.is_same(&edge.edge)) {
        return Ok(*length);
    }
    let length = if BRepTool::degenerated(&edge.edge)? {
        0.
    } else {
        AbscissaPoint::length(&BRepAdaptorCurve::with_edge(&edge.edge)? as &dyn Adaptor3dCurve)?
    };
    lengths.push((edge.edge.clone().into_shape(), length));
    Ok(length)
}
fn distance_from_intersections(
    vertex: &Vertex,
    surface: &Handle<dyn GeomSurface>,
    first: &EdgeSample,
    second: &EdgeSample,
    points: &[IntersectionPoint],
    limit: f64,
) -> Result<f64, Failure> {
    let vertex_point = BRepTool::pnt(vertex)?;
    let mut maximum = 0.;
    for point in points {
        if !near_vertex_end(first, point.param_on_first())
            || !near_vertex_end(second, point.param_on_second())
        {
            continue;
        }
        let uv = point.value();
        let distance = vertex_point.square_distance(&surface.value(uv.x(), uv.y())?);
        if distance > maximum && distance < limit {
            maximum = distance;
        }
    }
    Ok(maximum)
}
fn near_vertex_end(edge: &EdgeSample, parameter: f64) -> bool {
    let (first, last) = edge.range;
    if parameter < first || parameter > last {
        return false;
    }
    edge.closed || (parameter - edge.parameter).abs() <= (last - first) / 2.
}
