//! The topology and geometry commands the cases call, on the Rust API.
//! Each function names the command it follows.

use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools,
    topexp::Explorer,
    topods::{self, Compound, Shape, ShapeIterator},
    toptools::MapOfShape,
};
use martensite_tkernel::{precision, standard::Handle, storage::IoStream};
use martensite_tkg2d::{
    geom2d::{Geom2dCurve, Geom2dTrimmedCurve},
    geom2d_adaptor::Geom2dAdaptorCurve,
};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    geom::{
        GeomBSplineCurve, GeomBezierCurve, GeomCurve, GeomOffsetSurface, GeomSurface,
        GeomTrimmedCurve,
    },
    geom_adaptor::GeomAdaptorCurve,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkgeombase::{
    gcpnts::{AbscissaPoint, TangentialCriteria, TangentialDeflection},
    geom_proj_lib, geom_tools,
};
use std::io::{Read, Seek, SeekFrom};

/// `explode name`: the direct sub-shapes, in iteration order.
pub fn explode(shape: &Shape) -> Vec<Shape> {
    let mut parts = Vec::new();
    let mut iterator = ShapeIterator::new(shape, true, true);
    while let Ok(child) = iterator.value() {
        parts.push(child.clone());
        iterator.next();
    }
    parts
}

/// `explode name type`: the sub-shapes of `kind` in exploration order, each
/// once (the map is seeded with the shape itself).
pub fn explode_type(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut parts = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            parts.push(explorer.current().clone());
        }
        explorer.next().unwrap();
    }
    parts
}

/// BRepTest_CurveCommands.cxx `mkcurve c edge`: the edge's 3D curve trimmed to its
/// range, transformed by the composed location.
pub fn mkcurve(edge: &Shape) -> Handle<dyn GeomCurve> {
    let edge = topods::edge(edge).expect("mkcurve: an edge");
    let found = BRepTool::curve_with_location(&edge).unwrap();
    assert!(!found.curve.is_null(), "mkcurve: the edge has no 3d curve");
    let trimmed = GeomTrimmedCurve::new(&found.curve, found.first, found.last, true, true);
    let trimmed: Handle<dyn GeomCurve> = Handle::new(trimmed.unwrap()).cast();
    let copy = trimmed
        .get()
        .unwrap()
        .transformed(found.location.transformation());
    copy.unwrap().cast()
}

/// BRepTest_SurfaceCommands.cxx `mksurface s face`: the face's surface transformed by
/// the composed location.
pub fn mksurface(face: &Shape) -> Handle<dyn GeomSurface> {
    let face = topods::face(face).expect("mksurface: a face");
    let (surface, location) = BRepTool::surface_with_location(&face).unwrap();
    let copy = surface
        .get()
        .unwrap()
        .transformed(location.transformation());
    copy.unwrap().cast()
}

/// GeometryTest_CurveCommands.cxx `project result c3d surf [-t tol]`:
/// `GeomProjLib::Curve2d` over the curve range and the surface bounds; `None` is the
/// command's "Projection Failed".
pub fn project(
    curve: &Handle<dyn GeomCurve>,
    surface: &Handle<dyn GeomSurface>,
    tolerance: f64,
) -> Option<Handle<dyn Geom2dCurve>> {
    let bounds = surface.get().unwrap().bounds();
    let (projected, _) =
        geom_proj_lib::curve2d_bounds_tolerance(curve, surface, bounds, tolerance).unwrap();
    (!projected.is_null()).then_some(projected)
}

/// `dump name`: the banner, then `print` at precision 15.
fn dump_text(name: &str, print: impl FnOnce(&mut IoStream)) -> String {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    print(&mut stream);
    let mut device = stream.into_device().expect("in-memory dump stream");
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut body = String::new();
    device.read_to_string(&mut body).expect("dump text");
    format!("\n\n*********** Dump of {name} *************\n{body}")
}

/// `dump name` of a 2D curve (GeomTools_Curve2dSet::PrintCurve2d).
pub fn dump_curve2d(name: &str, curve: &Handle<dyn Geom2dCurve>) -> String {
    dump_text(name, |stream| {
        geom_tools::dump_curve2d(curve, stream).unwrap()
    })
}

/// `dump name` of a 3D curve (GeomTools_CurveSet::PrintCurve).
pub fn dump_curve(name: &str, curve: &Handle<dyn GeomCurve>) -> String {
    dump_text(name, |stream| {
        geom_tools::dump_curve(curve, stream).unwrap()
    })
}

/// `dump name` of a surface (GeomTools_SurfaceSet::PrintSurface).
pub fn dump_surface(name: &str, surface: &Handle<dyn GeomSurface>) -> String {
    dump_text(name, |stream| {
        geom_tools::dump_surface(surface, stream).unwrap()
    })
}

/// GeomliteTest_SurfaceCommands.cxx `trim result c` (three words): the basis curve of a
/// trimmed curve, the curve itself otherwise; the variable shares the handle.
pub fn trim_to_basis(curve: &Handle<dyn GeomCurve>) -> Handle<dyn GeomCurve> {
    let trimmed = curve.cast::<GeomTrimmedCurve>();
    match trimmed.get() {
        Some(trimmed) => trimmed.basis_curve(),
        None => curve.clone(),
    }
}

/// GeomliteTest_CurveCommands.cxx `segment name f l`: `Segment(f, l, PConfusion)` of a
/// B-spline curve (a Bezier curve segments without tolerance); any other curve makes the
/// command fail, which the cases see as a script error.
pub fn segment(curve: &Handle<dyn GeomCurve>, first: f64, last: f64) -> Result<(), String> {
    if let Some(bspline) = curve.cast::<GeomBSplineCurve>().get() {
        return bspline
            .segment(first, last, precision::p_confusion())
            .map_err(|failure| format!("{failure:?}"));
    }
    if let Some(bezier) = curve.cast::<GeomBezierCurve>().get() {
        return bezier
            .segment(first, last)
            .map_err(|failure| format!("{failure:?}"));
    }
    Err("segment: not a Bezier or B-spline curve".into())
}

/// GeomliteTest_SurfaceCommands.cxx `offset name surface distance`:
/// `new Geom_OffsetSurface(GS, dist)` (C0 basis checked).
pub fn offset_surface(
    surface: &Handle<dyn GeomSurface>,
    distance: f64,
) -> Result<Handle<dyn GeomSurface>, String> {
    let offset = GeomOffsetSurface::new(surface, distance, false);
    let offset = offset.map_err(|failure| format!("{failure:?}"))?;
    Ok(Handle::new(offset).cast())
}

/// BRepTest_SurfaceCommands.cxx `pcurve face`: the p-curve of every explored edge of the
/// FORWARD face (`face_1`, `face_2`, ...), trimmed to the edge range unless the
/// non-periodic curve is already shorter than it; an edge without p-curve prints
/// `Error`.
pub fn pcurve(face: &Shape) -> Vec<Handle<dyn Geom2dCurve>> {
    let mut forward = face.clone();
    forward.set_orientation(Orientation::Forward);
    let face = topods::face(&forward).expect("pcurve: a face");
    explode_all(&forward, ShapeEnum::Edge)
        .iter()
        .enumerate()
        .map(|(i, edge)| {
            let edge = topods::edge(edge).unwrap();
            let found = BRepTool::curve_on_face(&edge, &face).unwrap();
            assert!(
                !found.pcurve.is_null(),
                "Error: Edge {} does not have pcurve",
                i + 1
            );
            pcurve_variable(found.pcurve, found.first, found.last)
        })
        .collect()
}

fn pcurve_variable(c: Handle<dyn Geom2dCurve>, f: f64, l: f64) -> Handle<dyn Geom2dCurve> {
    let curve = c.get().unwrap();
    let (mut fr, mut lr, mut periodic) = (
        curve.first_parameter(),
        curve.last_parameter(),
        curve.is_periodic(),
    );
    if let Some(trimmed) = c.cast::<Geom2dTrimmedCurve>().get() {
        let basis = trimmed.basis_curve();
        let basis = basis.get().unwrap();
        (fr, lr, periodic) = (
            basis.first_parameter(),
            basis.last_parameter(),
            basis.is_periodic(),
        );
    }
    let tolerance = precision::p_confusion();
    if !periodic && (fr - f > tolerance || l - lr > tolerance) {
        return c;
    }
    Handle::new(Geom2dTrimmedCurve::new(&c, f, l, true, true).unwrap()).cast()
}

/// Every sub-shape of `kind` in exploration order, repeats included (TopExp_Explorer).
pub fn explode_all(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut parts = Vec::new();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        parts.push(explorer.current().clone());
        explorer.next().unwrap();
    }
    parts
}

/// GeometryTest_CurveCommands.cxx `crvtpoints r c defl angle minpoints`: the number of
/// points of GCPnts_TangentialDeflection on the GeomAdaptor_Curve.
pub fn crvtpoints_count(
    curve: &Handle<dyn GeomCurve>,
    deflection: f64,
    angle: f64,
    min_points: i32,
) -> i32 {
    let adaptor = GeomAdaptorCurve::with_curve(curve).unwrap();
    let adaptor: &dyn Adaptor3dCurve = &adaptor;
    let mut criteria = TangentialCriteria::new(angle, deflection);
    criteria.minimum_of_points = min_points;
    TangentialDeflection::with_curve(adaptor, criteria)
        .unwrap()
        .nb_points()
}

/// `tclean [-force] name`: `BRepTools::Clean` of a compound holding the shape.
pub fn tclean(shape: &Shape, force: bool) {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder.add(&mut compound, shape).unwrap();
    brep_tools::clean(&compound, force).unwrap();
}

/// GeomliteTest_CurveCommands.cxx `length c tol` for a 2D curve.
pub fn length2d(curve: &Handle<dyn Geom2dCurve>, tolerance: f64) -> f64 {
    let adaptor = Geom2dAdaptorCurve::with_curve(curve).unwrap();
    AbscissaPoint::length_with_tolerance(&adaptor, tolerance).unwrap()
}

/// GeomliteTest_CurveCommands.cxx `length c tol` for a 3D curve.
pub fn length3d(curve: &Handle<dyn GeomCurve>, tolerance: f64) -> f64 {
    let adaptor = GeomAdaptorCurve::with_curve(curve).unwrap();
    AbscissaPoint::length_with_tolerance(&adaptor, tolerance).unwrap()
}
