//! Native representations retain shared incidence owners; no sewing by proximity.
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Builder, Edge, Face, Shape, Vertex, Wire},
};
use martensite_tkernel::{
    ncollection::Array1,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve, GeomSurface};
use martensite_tkgeomalgo::geom_fill::{GeomFillBezierCurves, GeomFillFillingStyle};
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;

pub(super) fn curve3(
    poles: &super::angular::RationalPoles,
) -> Result<Handle<GeomBezierCurve>, Failure> {
    let count = i32::try_from(poles.points.len())
        .map_err(|_| Failure::new("rational pole count exceeds i32"))?;
    let mut points = Array1::with_bounds(1, count).map_err(array_failure)?;
    let mut weights = Array1::with_bounds(1, count).map_err(array_failure)?;
    for (i, [x, y, z]) in poles.points.iter().enumerate() {
        points
            .set_value(i as i32 + 1, Point3::new(*x, *y, *z))
            .map_err(array_failure)?;
        weights
            .set_value(i as i32 + 1, poles.weights[i])
            .map_err(array_failure)?;
    }
    Ok(Handle::new(GeomBezierCurve::with_weights(
        &points, &weights,
    )?))
}

pub(super) fn curve2(poles: &[[f64; 2]]) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let count =
        i32::try_from(poles.len()).map_err(|_| Failure::new("pcurve pole count exceeds i32"))?;
    let mut points = Array1::with_bounds(1, count).map_err(array_failure)?;
    for (i, [x, y]) in poles.iter().enumerate() {
        points
            .set_value(i as i32 + 1, Point2::new(*x, *y))
            .map_err(array_failure)?;
    }
    Ok(Handle::new(Geom2dBezierCurve::new(&points)?).into_cast())
}

pub(super) fn edge(curve: &Handle<GeomBezierCurve>, ends: [&Vertex; 2]) -> Result<Edge, Failure> {
    let curve: Handle<dyn GeomCurve> = curve.cast();
    let edge =
        BRepBuilderAPIMakeEdge::from_curve_vertices_params(&curve, ends[0], ends[1], 0., 1.)?
            .edge()?;
    // The walking skeleton has no O2/O3 certificate: keep conformance unset.
    BRepBuilder::default().same_parameter(&edge, false)?;
    Ok(edge)
}

pub(super) fn attach(
    edge: &Edge,
    curve: &Handle<dyn Geom2dCurve>,
    face: &Face,
    tolerance: f64,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    builder.update_edge_pcurve_on_face(edge, curve, face, tolerance)?;
    builder.update_edge_tolerance(edge, tolerance)?;
    builder.range_on_face(edge, face, 0., 1.)
}

pub(super) fn wire(edges: &[Shape]) -> Result<Wire, Failure> {
    let mut wire = Wire::default();
    Builder.make_wire(&mut wire);
    for edge in edges {
        Builder.add(&mut wire, edge)?;
    }
    wire.set_closed(true)?;
    Ok(wire)
}

pub(super) fn side(
    curves: &[Handle<GeomBezierCurve>; 2],
    edges: &[Edge; 2],
    connectors: &[Edge; 2],
    tolerance: f64,
) -> Result<Face, Failure> {
    let fill =
        GeomFillBezierCurves::with_two(&curves[0], &curves[1], GeomFillFillingStyle::StretchStyle)?;
    let surface: Handle<dyn GeomSurface> = fill.surface().cast();
    let mut face = Face::default();
    BRepBuilder::default().make_face_on_surface(&mut face, &surface, tolerance)?;
    side_pcurves(&face, edges, connectors, tolerance)?;
    let boundary = [
        edges[0].clone().into_shape(),
        connectors[1].clone().into_shape(),
        edges[1].reversed(),
        connectors[0].reversed(),
    ];
    let boundary = wire(&boundary)?;
    Builder.add(&mut face, &boundary)?;
    Ok(face)
}

fn side_pcurves(
    face: &Face,
    edges: &[Edge; 2],
    connectors: &[Edge; 2],
    tolerance: f64,
) -> Result<(), Failure> {
    let bindings = [
        (&edges[0], [[0., 0.], [1., 0.]]),
        (&edges[1], [[0., 1.], [1., 1.]]),
        (&connectors[0], [[0., 0.], [0., 1.]]),
        (&connectors[1], [[1., 0.], [1., 1.]]),
    ];
    for (edge, poles) in bindings {
        attach(edge, &curve2(&poles)?, face, tolerance)?;
    }
    Ok(())
}

fn array_failure(error: martensite_tkernel::ncollection::ArrayError) -> Failure {
    Failure::new(&format!("native pole array: {error:?}"))
}
