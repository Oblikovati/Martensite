//! Projection and wireframe regression helpers returning their diagnostic text.
use super::printed::g17;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{self, Face, Shape},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkshhealing::shape_construct::ShapeConstructProjectCurveOnSurface;
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFixEdgeProjAux, ShapeFixWireframe};

type Projected = Result<Handle<dyn Geom2dCurve>, martensite_tkernel::standard::Failure>;

/// ShapeConstructProjectCurveOnSurface on `surface` at Precision::Confusion(), projecting
/// `curve` over `range`: the pcurve (null when not created), or the failure it raised.
fn project(
    curve: &Handle<dyn GeomCurve>,
    surface: &Handle<dyn GeomSurface>,
    range: (f64, f64),
) -> Projected {
    let projector = ShapeConstructProjectCurveOnSurface::new();
    projector.init(surface, precision::confusion())?;
    let mut pcurve = Handle::default();
    projector.perform(curve, range, &mut pcurve, (-1., -1.))?;
    Ok(pcurve)
}

/// `OCC24008 curve surface`: the projection over the curve's parameter range; prints
/// `Exception was caught` for a raised failure. A null pcurve makes the command fail
/// (`PCurve was not created`, status 1), which aborts the script.
pub fn occ24008(curve: &Handle<dyn GeomCurve>, surface: &Handle<dyn GeomSurface>) -> String {
    let c = curve.get().unwrap();
    match project(curve, surface, (c.first_parameter(), c.last_parameter())) {
        Ok(pcurve) if pcurve.is_null() => panic!("OCC24008: PCurve was not created"),
        Ok(_) => String::new(),
        Err(_) => "Exception was caught\n".into(),
    }
}

/// `OCC26930 surface curve start end`: the projection over `[start, end]` must be a
/// Geom2d_Line (a caught failure leaves the pcurve null).
pub fn occ26930(
    surface: &Handle<dyn GeomSurface>,
    curve: &Handle<dyn GeomCurve>,
    start: f64,
    end: f64,
) -> String {
    let pcurve = project(curve, surface, (start, end)).unwrap_or_default();
    if pcurve.is_null() {
        "Error: pcurve is null\n".into()
    } else if !pcurve.cast::<Geom2dLine>().is_null() {
        "Pcurve is line: OK\n".into()
    } else {
        "Error: PCurve is not line\n".into()
    }
}

/// `OCC24370 edge pcurve surface precision`: the pcurve put on a face of the surface
/// over its own range, then ShapeFixEdgeProjAux; prints `isFirstDone isLastDone first
/// last` (`%.17g` reals, 0 when not done).
pub fn occ24370(
    edge: &Shape,
    pcurve: &Handle<dyn Geom2dCurve>,
    surface: &Handle<dyn GeomSurface>,
    tolerance: f64,
) -> String {
    let edge = topods::edge(edge).expect("OCC24370: an edge");
    let builder = BRepBuilder::default();
    let mut face = Face::default();
    let confusion = precision::confusion();
    builder
        .make_face_on_surface(&mut face, surface, confusion)
        .unwrap();
    builder
        .update_edge_pcurve_on_face(&edge, pcurve, &face, confusion)
        .unwrap();
    let c = pcurve.get().unwrap();
    builder
        .range_on_face(&edge, &face, c.first_parameter(), c.last_parameter())
        .unwrap();
    let projection = ShapeFixEdgeProjAux::with_face_edge(&face, &edge);
    projection.compute(tolerance).unwrap();
    let (first_done, last_done) = (projection.is_first_done(), projection.is_last_done());
    let first = if first_done {
        projection.first_param()
    } else {
        0.
    };
    let last = if last_done {
        projection.last_param()
    } else {
        0.
    };
    format!(
        "{} {} {} {} \n",
        i32::from(first_done),
        i32::from(last_done),
        g17(first),
        g17(last)
    )
}

/// `OCC7570 shape`: ShapeFixWireframe dropping small edges at precision 1e+6 and limit
/// angle 0.01; it prints nothing.
pub fn occ7570(shape: &Shape) {
    let tool = ShapeFixWireframe::from_shape(shape);
    tool.set_mode_drop_small_edges(true);
    tool.set_precision(1.0e+6);
    tool.set_limit_angle(0.01);
    tool.fix_small_edges()
        .unwrap_or_else(|failure| panic!("OCC7570: {failure:?}"));
}
