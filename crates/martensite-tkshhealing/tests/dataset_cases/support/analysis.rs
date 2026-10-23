//! The scenario analysis commands of the bugs/moddata_3 cases on the Rust API: `gproject`,
//! `gbounding`, `trinfo`, `mk2dcurve`, `splitc12d`, `tocanon` and the pole lines of a dump.
use martensite_tkbrep::{
    brep::{BRepCurveRepresentation, BRepTool, access::edge_tshape},
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    geom::{GeomCurve, GeomSurface},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
    top_abs::ShapeEnum,
};
use martensite_tkgeombase::{
    bnd_lib::Add2dCurve,
    geom_convert::{ConvType, CurveToAnaCurve},
    geom2d_convert,
    proj_lib::CompProjectedCurve,
};
use martensite_tkmath::{
    bnd::BoundingBox2, convert::ParameterisationType, geom_abs::Shape as Continuity,
};

use super::commands::explode_all;

/// The approximation errors `gproject` prints for its first result curve.
pub struct GprojectErrors {
    pub u2d: f64,
    pub v2d: f64,
    pub e3d: f64,
}

/// `gproject r c s tol3d -3d 1 -c continuity`:
/// ProjLib_CompProjectedCurve(tol3d, S, C, -1) with the continuity and 3D results.
pub fn gproject_errors(
    curve: &Handle<dyn GeomCurve>,
    surface: &Handle<dyn GeomSurface>,
    tol3d: f64,
    continuity: Continuity,
) -> GprojectErrors {
    let hcur: Handle<dyn Adaptor3dCurve> =
        Handle::new(GeomAdaptorCurve::with_curve(curve).unwrap()).cast();
    let hsur: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(surface).unwrap()).cast();
    let projector = CompProjectedCurve::with_tol3d(tol3d, &hsur, &hcur, -1.0).unwrap();
    projector.set_continuity(continuity);
    projector.set_proj3d(true);
    projector.perform().unwrap();
    assert!(projector.nb_curves() >= 1, "gproject gave no curve");
    assert!(
        !projector.result_is_point(1).unwrap(),
        "gproject gave a point"
    );
    GprojectErrors {
        u2d: projector.get_result_2d_u_approx_error(1).unwrap(),
        v2d: projector.get_result_2d_v_approx_error(1).unwrap(),
        e3d: projector.get_result_3d_approx_error(1).unwrap(),
    }
}

/// `gbounding curve2d`: BndLib_Add2dCurve::Add over the curve
/// range at Precision::Confusion, as `xmin ymin xmax ymax`.
pub fn gbounding2d(curve: &Handle<dyn Geom2dCurve>) -> [f64; 4] {
    let c = curve.get().unwrap();
    let mut bounds = BoundingBox2::default();
    let (first, last) = (c.first_parameter(), c.last_parameter());
    Add2dCurve::add_curve_range(curve, first, last, precision::confusion(), &mut bounds).unwrap();
    bounds.get().unwrap()
}

/// `trinfo shape`: the counts it prints, as text.
pub fn trinfo(shape: &Shape) -> String {
    let (mut faces, mut triangles, mut nodes, mut deflection) = (0, 0, 0, 0.0_f64);
    for face in explode_all(shape, ShapeEnum::Face) {
        faces += 1;
        let face = topods::face(&face).unwrap();
        let (triangulation, _) = BRepTool::triangulation(&face, 0).unwrap();
        if let Some(triangulation) = triangulation.get() {
            triangles += triangulation.nb_triangles();
            nodes += triangulation.nb_nodes();
            deflection = deflection.max(triangulation.deflection());
        }
    }
    let polygons = polygons_on_triangulation(shape);
    format!(
        "\nThis shape contains {faces} faces.\n {triangles} triangles.\n {nodes} nodes.\n \
         {polygons} polygons on triangulation.\nMaximal deflection {deflection}\n"
    )
}

fn polygons_on_triangulation(shape: &Shape) -> usize {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).unwrap();
    let mut count = 0;
    for i in 1..=edges.extent() {
        let edge = edges.find_key(i).unwrap();
        for representation in edge_tshape(edge, "trinfo").unwrap().curves() {
            let representation: Option<&dyn BRepCurveRepresentation> = representation.get();
            if representation.is_some_and(|r| r.is_polygon_on_triangulation()) {
                count += 1;
            }
        }
    }
    count
}

/// `mk2dcurve c edge index`: the `index`-th p-curve of the
/// edge, trimmed to its range.
pub fn mk2dcurve_by_index(edge: &Shape, index: i32) -> Handle<dyn Geom2dCurve> {
    let edge = topods::edge(edge).unwrap();
    let found = BRepTool::curve_on_surface_by_index(&edge, index).unwrap();
    assert!(!found.pcurve.is_null(), "edge has no 2d curve");
    let trimmed = Geom2dTrimmedCurve::new(&found.pcurve, found.first, found.last, true, true);
    Handle::new(trimmed.unwrap()).cast()
}

/// `splitc12d c 0`: Geom2dConvert::CurveToBSplineCurve
/// then C0BSplineToC1BSplineCurve with the default tolerance 1e-5.
pub fn splitc12d(curve: &Handle<dyn Geom2dCurve>) -> Result<(), String> {
    let c = curve.get().unwrap();
    let (f, l) = (c.first_parameter(), c.last_parameter());
    assert!(
        !precision::is_infinite(f) && !precision::is_infinite(l),
        "Infinite curves"
    );
    let parameterisation = ParameterisationType::TgtThetaOver2;
    let mut bs = geom2d_convert::curve_to_bspline_curve(curve, parameterisation)
        .map_err(|failure| format!("{failure:?}"))?;
    geom2d_convert::c0_bspline_to_c1_bspline_curve(&mut bs, 1.0e-5)
        .map_err(|failure| format!("{failure:?}"))
}

/// `tocanon r c [tol]` on a curve: whether
/// CurveToAnaCurve (Simplest) converts it; false prints `Conversion failed`.
pub fn tocanon_curve(curve: &Handle<dyn GeomCurve>, tolerance: f64) -> bool {
    let mut conversion = CurveToAnaCurve::with_curve(curve);
    conversion.set_conv_type(ConvType::Simplest);
    let c = curve.get().unwrap();
    let range = (c.first_parameter(), c.last_parameter());
    let mut result = Handle::default();
    let mut new_range = (0.0, 0.0);
    conversion
        .convert_to_analytical(tolerance, &mut result, range, &mut new_range)
        .unwrap()
}

/// The pole lines ` i : u, v` of a 2D B-spline dump, read with the cases' regexp
/// ` +${i} : +([num]+), +([num]+)`: the first match for each index.
pub fn dump_pole(dump: &str, index: usize) -> (f64, f64) {
    let label = format!(" {index} : ");
    let mut rest = dump;
    while let Some(at) = rest.find(&label) {
        let after = rest[at + label.len()..].trim_start_matches(' ');
        if let Some(pole) = parse_pair(after) {
            return pole;
        }
        rest = &rest[at + 1..];
    }
    panic!("dump has no pole {index}");
}

fn parse_pair(text: &str) -> Option<(f64, f64)> {
    let (u, rest) = text.split_once(',')?;
    let v = rest.trim_start_matches(' ').split([' ', '\n']).next()?;
    Some((u.parse().ok()?, v.parse().ok()?))
}

/// `regexp {Degree +(n), +(n) Poles, +(n)}` of a B-spline dump: the pole count.
pub fn dump_nb_poles(dump: &str) -> usize {
    let after = dump.split_once("Degree").expect("Degree in dump").1;
    let poles = after.split_once(',').unwrap().1.trim_start();
    poles.split(' ').next().unwrap().parse().unwrap()
}
