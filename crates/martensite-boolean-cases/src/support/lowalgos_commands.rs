//! The commands of `tests/lowalgos`: the surfaces of a script, `intersect` with its answer, the
//! curve loops of the `intss` scripts and the checks they call. A surface, a curve and a point are
//! plain values in a case body; the session only shows them under the script's names.
use super::{
    bop_session::BopSession,
    bopcurves::BoundCurves,
    script_error::script_error,
    session::{Drawing, Session, ViewStep},
    surface_intersection::xdistcs,
    tolerance_check::tolerance_report,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge};
use martensite_tkernel::{
    standard::Handle,
    storage::{IoStream, format_general},
};
use martensite_tkg3d::{
    geom::{
        GeomBSplineSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
        GeomSphericalSurface, GeomToroidalSurface,
    },
    geom_adaptor::GeomAdaptorCurve,
};
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};

pub mod shape_commands;
pub use shape_commands::*;

pub use super::surface_intersection::{Curve, Surface};

fn point(at: [f64; 3]) -> Point3 {
    Point3::new(at[0], at[1], at[2])
}

fn direction(along: [f64; 3]) -> Direction3 {
    Direction3::new(along[0], along[1], along[2])
        .unwrap_or_else(|failure| panic!("direction {along:?}: {failure:?}"))
}

/// The frame of a script's `x y z dx dy dz ux uy uz`; `frame([0.; 3], [0., 0., 1.], [1., 0., 0.])`.
pub fn frame(at: [f64; 3], normal: [f64; 3], x: [f64; 3]) -> Axis3 {
    Axis3::new(point(at), direction(normal), direction(x))
        .unwrap_or_else(|failure| panic!("frame {at:?} {normal:?} {x:?}: {failure:?}"))
}

/// `plane name x y z dx dy dz ux uy uz`, bound; `plane(s, "p", [0.; 3], [0., 0., 1.], [1., 0., 0.])`.
pub fn plane(s: &mut dyn Session, name: &str, at: [f64; 3], n: [f64; 3], x: [f64; 3]) -> Surface {
    let surface: Surface = Handle::new(GeomPlane::new(frame(at, n, x))).cast();
    s.bind_drawing(name, Drawing::Surface(&surface));
    surface
}

/// `cylinder name x y z dx dy dz ux uy uz radius`, bound.
/// Example: `cylinder(s, "c", ([0.; 3], [1., 0., 0.], [0., 1., 0.]), 100.)`.
pub fn cylinder(s: &mut dyn Session, name: &str, at: Placed, radius: f64) -> Surface {
    let surface = GeomCylindricalSurface::new(frame(at.0, at.1, at.2), radius)
        .unwrap_or_else(|failure| panic!("cylinder {name} {radius}: {failure:?}"));
    bound_surface(s, name, Handle::new(surface).cast())
}

/// `torus name x y z dx dy dz ux uy uz major minor`, bound.
/// Example: `torus(s, "t", ([0.; 3], [0., 0., 1.], [1., 0., 0.]), 23., 10.)`.
pub fn torus(s: &mut dyn Session, name: &str, at: Placed, major: f64, minor: f64) -> Surface {
    let surface = GeomToroidalSurface::new(frame(at.0, at.1, at.2), major, minor)
        .unwrap_or_else(|failure| panic!("torus {name} {major} {minor}: {failure:?}"));
    bound_surface(s, name, Handle::new(surface).cast())
}

/// `cone name x y z dx dy dz ux uy uz degrees radius`, bound.
/// Example: `cone(s, "c", ([0.; 3], [0., 0., 1.], [1., 0., 0.]), 5., 0.25)`.
pub fn cone(s: &mut dyn Session, name: &str, at: Placed, degrees: f64, radius: f64) -> Surface {
    let angle = degrees * (std::f64::consts::PI / 180.0);
    let surface = GeomConicalSurface::new(frame(at.0, at.1, at.2), angle, radius)
        .unwrap_or_else(|failure| panic!("cone {name} {degrees} {radius}: {failure:?}"));
    bound_surface(s, name, Handle::new(surface).cast())
}

/// `sphere name x y z radius`, bound; `sphere(s, "ss1", [0., 0., 0.], 6.5)`.
pub fn sphere(s: &mut dyn Session, name: &str, at: [f64; 3], radius: f64) -> Surface {
    let placed = Axis3::new(point(at), Direction3::Z, Direction3::X)
        .unwrap_or_else(|failure| panic!("sphere {name} {at:?}: {failure:?}"));
    let surface = GeomSphericalSurface::new(placed, radius)
        .unwrap_or_else(|failure| panic!("sphere {name} {radius}: {failure:?}"));
    bound_surface(s, name, Handle::new(surface).cast())
}

/// A script's `x y z dx dy dz ux uy uz`.
pub type Placed = ([f64; 3], [f64; 3], [f64; 3]);

/// The surface is the variable `name`; `bound_surface(s, "s1", surface)`.
pub fn bound_surface(s: &mut dyn Session, name: &str, surface: Surface) -> Surface {
    s.bind_drawing(name, Drawing::Surface(&surface));
    surface
}

/// `mksurface name face`: a copy of the face's surface with its location.
/// Example: `mksurface(s, "s1", "f1")`.
pub fn mksurface(s: &mut dyn Session, name: &str, face: &str) -> Surface {
    let shape = s
        .shape(face)
        .unwrap_or_else(|| panic!("mksurface {name}: expected the bound face {face}"));
    let face =
        topods::face(shape).unwrap_or_else(|failure| panic!("mksurface {face:?}: {failure:?}"));
    let surface =
        BRepTool::surface(&face).unwrap_or_else(|failure| panic!("mksurface {name}: {failure:?}"));
    bound_surface(s, name, surface)
}

/// The kind of drawable a `.draw` file says it holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SavedSurface {
    /// `GeometrySurface`.
    Any,
    /// `GeometryBSplineSurface`: the record read must be one as well.
    BSpline,
}

impl SavedSurface {
    fn header(self) -> &'static str {
        match self {
            SavedSurface::Any => "DrawTrSurf_Surface",
            SavedSurface::BSpline => "DrawTrSurf_BSplineSurface",
        }
    }
}

/// `restore [locate_data_file file] name` of a saved surface; a file that is not there is the
/// script error of the native lookup. Example: `restore_surface(s, "a.draw", "s1", SavedSurface::Any)`.
pub fn restore_surface(s: &mut dyn Session, file: &str, name: &str, kind: SavedSurface) -> Surface {
    let bytes = super::restore::input_bytes(file);
    let end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .unwrap_or_else(|| panic!("restore {file}: expected a header word"));
    let header = String::from_utf8_lossy(&bytes[..end]);
    assert_eq!(header, kind.header(), "restore {file}: drawable kind");
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    let surface = martensite_tkgeombase::geom_tools::read_surface(&mut stream);
    assert!(!surface.is_null(), "restore {file}: expected a surface");
    if kind == SavedSurface::BSpline {
        assert!(
            !surface.cast::<GeomBSplineSurface>().is_null(),
            "restore {file}: expected a BSpline surface"
        );
    }
    bound_surface(s, name, surface)
}

/// What `intersect` left: its curves under their final names, and its isolated points.
pub struct SurfaceCurves {
    /// The curves, as `res_1`, `res_2`, ...
    pub curves: Vec<Curve>,
    /// `res_1`, `res_2`, ...: also after a single `res` is renamed.
    pub names: Vec<String>,
    /// `res_p_1`, `res_p_2`, ...
    pub points: Vec<Point3>,
}

fn performed_intersection(
    first: &Surface,
    second: &Surface,
    tolerance: f64,
) -> Option<GeomIntIntSS> {
    let approximation = GeomIntIntSSApprox::default();
    let operation = GeomIntIntSS::with_surfaces(first, second, tolerance, approximation).ok()?;
    if operation.is_done() {
        return Some(operation);
    }
    println!("No intersections found!");
    None
}

fn intersection_answer(name: &str, lines: usize, points: usize) -> String {
    let mut words: Vec<String> = match lines {
        0 => Vec::new(),
        1 => vec![name.to_owned()],
        _ => (1..=lines).map(|i| format!("{name}_{i}")).collect(),
    };
    words.extend((1..=points).map(|i| format!("{name}_p_{i}")));
    words.join(" ")
}

/// `intersect name first second [tolerance]`: its error stops the script.
/// Example: `intersect(s, "res", &s1, &s2, 1.0e-7)`.
pub fn intersect(
    s: &mut dyn Session,
    name: &str,
    first: &Surface,
    second: &Surface,
    tolerance: f64,
) -> SurfaceCurves {
    intersect_caught(s, name, first, second, tolerance)
        .unwrap_or_else(|| script_error("intersect: No intersections found!"))
}

/// `catch {intersect name first second [tolerance]}`: writes what the command returns and binds
/// the curves; `None` is the error the script catches.
/// Example: `intersect_caught(s, "res", &s1, &s2, 1.0e-7)`.
pub fn intersect_caught(
    s: &mut dyn Session,
    name: &str,
    first: &Surface,
    second: &Surface,
    tolerance: f64,
) -> Option<SurfaceCurves> {
    let operation = performed_intersection(first, second, tolerance)?;
    let curves: Vec<Curve> = (1..=operation.nb_lines())
        .map(|i| operation.line(i).unwrap())
        .collect();
    let count = operation.nb_points().unwrap();
    let points: Vec<Point3> = (1..=count).map(|i| operation.point(i).unwrap()).collect();
    println!("{}", intersection_answer(name, curves.len(), points.len()));
    Some(bound_intersection(s, name, curves, points))
}

fn bound_intersection(
    s: &mut dyn Session,
    name: &str,
    curves: Vec<Curve>,
    points: Vec<Point3>,
) -> SurfaceCurves {
    let names: Vec<String> = (1..=curves.len()).map(|i| format!("{name}_{i}")).collect();
    for (curve, label) in curves.iter().zip(&names) {
        s.bind_drawing(label, Drawing::Curve(curve));
    }
    for (index, at) in points.iter().enumerate() {
        s.bind_drawing(&format!("{name}_p_{}", index + 1), Drawing::Point(*at));
    }
    SurfaceCurves {
        curves,
        names,
        points,
    }
}

/// The names of `curves` for a view command; `name_refs(&names)`.
pub fn name_refs(names: &[String]) -> Vec<&str> {
    names.iter().map(String::as_str).collect()
}

/// `bounds curve U1 U2`; `curve_range(&curve)`.
pub fn curve_range(curve: &Curve) -> [f64; 2] {
    let geometry = curve
        .get()
        .unwrap_or_else(|| panic!("bounds: expected a curve"));
    [geometry.first_parameter(), geometry.last_parameter()]
}

/// `mkedge ee curve`, bound under `ee`, then `baddobjects ee`; `add_curve_edge(s, &mut bop, &curve)`.
pub fn add_curve_edge(s: &mut dyn Session, bop: &mut BopSession, curve: &Curve) {
    super::surface_intersection::edge_of(s, "ee", curve);
    bop.add_object(s, "ee");
}

/// What one `xdistcs` loop of a script varies: the samples, its limit on each surface, and the
/// smallest curve range it accepts.
#[derive(Clone, Copy, Debug)]
pub struct SampledDistances {
    /// `nbpoints` of `xdistcs`.
    pub samples: usize,
    /// The script's limit for the first surface, then for the second.
    pub limits: [f64; 2],
    /// The script's `dval U2-U1` bound.
    pub shortest_range: f64,
}

/// The script's loop over `res_N`: `bounds`, the range check, both `xdistcs` and `mkedge ee`.
/// Example: `checked_curves(s, &mut bop, &answer, [&s1, &s2], &distances)`.
pub fn checked_curves(
    s: &mut dyn Session,
    bop: &mut BopSession,
    answer: &SurfaceCurves,
    surfaces: [&Surface; 2],
    distances: &SampledDistances,
) {
    for (curve, name) in answer.curves.iter().zip(&answer.names) {
        let range = curve_range(curve);
        if range[1] - range[0] < distances.shortest_range {
            println!("Error: Wrong curve's range!");
        }
        xdistcs(
            name,
            curve,
            surfaces[0],
            range,
            distances.samples,
            distances.limits[0],
        );
        xdistcs(
            name,
            curve,
            surfaces[1],
            range,
            distances.samples,
            distances.limits[1],
        );
        add_curve_edge(s, bop, curve);
    }
}

/// `CheckIntersectionResult`, the procedure of the grid's `begin`.
/// Example: `check_intersection_result(&answer, [&s1, &s2], 100, [2.0e-7, 2.0e-7])`.
pub fn check_intersection_result(
    answer: &SurfaceCurves,
    surfaces: [&Surface; 2],
    samples: usize,
    limits: [f64; 2],
) {
    for (curve, name) in answer.curves.iter().zip(&answer.names) {
        println!("** Check of {name} **");
        let range = curve_range(curve);
        if range[1] - range[0] < 1.0e-9 {
            println!("Error: Wrong range of {name}");
        }
        xdistcs("aCurve", curve, surfaces[0], range, samples, limits[0]);
        xdistcs("aCurve", curve, surfaces[1], range, samples, limits[1]);
    }
}

/// `checkoverlapedges first second tolerance`: writes and returns its answer; a degenerated edge
/// is the command's error, which stops the script.
/// Example: `checkoverlapedges(("e1", &one), ("e2", &two), 5.0e-5)`.
pub fn checkoverlapedges(
    first: (&str, &Edge),
    second: (&str, &Edge),
    tolerance: f64,
) -> &'static str {
    for (name, edge) in [first, second] {
        if BRepTool::degenerated(edge).unwrap() {
            println!("{name} is degenerated");
            script_error(&format!("checkoverlapedges: {name} is degenerated"));
        }
    }
    let answer = if super::surface_intersection::edges_overlap(first.1, second.1, tolerance) {
        "Edges are overlapping completely"
    } else {
        "Edges are not overlapped"
    };
    println!("{answer}");
    answer
}

/// `checkmaxtol name -min_tol limit`: `tolerance name`, and an `Error` above the limit, whose text
/// the script gives. Example: `checkmaxtol_within(s, "result", "2.0e-7")`.
pub fn checkmaxtol_within(s: &dyn Session, name: &str, limit: &str) {
    let printed = tolerance_report(s, name);
    let value: f64 = printed
        .parse()
        .unwrap_or_else(|cause| panic!("checkmaxtol {name} {printed:?}: {cause}"));
    let cap: f64 = limit
        .parse()
        .unwrap_or_else(|cause| panic!("checkmaxtol {name} -min_tol {limit:?}: {cause}"));
    if cap != 0. && value > cap {
        println!(
            "Error: tolerance of \"{name}\" ({printed}) is greater than checking tolerance ({limit})"
        );
    }
}

/// `length curve eps`, as written: its value; `curve_length("c_1", &curve, 1.0e-4)`.
pub fn curve_length(name: &str, curve: &Curve, epsilon: f64) -> String {
    let adaptor = GeomAdaptorCurve::with_curve(curve)
        .unwrap_or_else(|failure| panic!("length {name}: {failure:?}"));
    let length = AbscissaPoint::length_with_tolerance(&adaptor, epsilon)
        .unwrap_or_else(|failure| panic!("length {name}: {failure:?}"));
    let printed = format_general(length, 17);
    println!("The length {name} is {printed}");
    printed
}

/// `checklength name -l expected [-eps epsilon]`; `checklength("c_1", &c, "5.03", 1.0e-4)`.
pub fn checklength(name: &str, curve: &Curve, expected: &str, epsilon: f64) {
    let printed = curve_length(name, curve, epsilon);
    let parsed = |text: &str| -> f64 {
        text.parse()
            .unwrap_or_else(|cause| panic!("checklength {name} {text:?}: {cause}"))
    };
    let (measured, wanted) = (parsed(&printed), parsed(expected));
    if measured == 0. {
        println!("Error : The command is not valid. The length is 0.");
    }
    if wanted > 0. {
        println!("The expected length is {expected}");
    }
    if (wanted != 0. && (wanted - measured).abs() / wanted > 0.01)
        || (wanted == 0. && measured != 0.)
    {
        println!("Error : The length of result shape is {printed}");
    }
}

/// What `QACheckBends` sampled: how many parameters, and those it wrote an `Error` about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BendScan {
    /// The parameters sampled.
    pub samples: usize,
    /// Each flagged parameter, as printed.
    pub flagged: Vec<String>,
}

/// `QACheckBends curve`: an `Error` at each sampled turn sharper than the cosine 0.8.
/// Example: `qa_check_bends("c_1", &curve)`.
pub fn qa_check_bends(name: &str, curve: &Curve) -> BendScan {
    let geometry = curve.get().unwrap_or_else(|| panic!("{name}: NULL curve"));
    let (first, last) = (geometry.first_parameter(), geometry.last_parameter());
    let delta = (last - first) / 1000.;
    let along = |at: f64| {
        let (_, derivative) = geometry
            .d1(at)
            .unwrap_or_else(|failure| panic!("QACheckBends {name} {at}: {failure:?}"));
        Direction3::from_vector(derivative)
            .unwrap_or_else(|failure| panic!("QACheckBends {name} {at}: {failure:?}"))
    };
    let mut scan = BendScan {
        samples: 0,
        flagged: Vec::new(),
    };
    let mut previous = along(first);
    let mut at = first;
    // Native repeats the addition, so the parameters and what they print keep its rounding.
    while at <= last {
        let current = along(at);
        scan.samples += 1;
        if previous.dot(current) < 0.8 {
            let printed = format_general(at, 17);
            println!(
                "Error: The curve {name} is possible to have a bend at parameter {printed}. Please check carefully "
            );
            scan.flagged.push(printed);
        }
        previous = current;
        at += delta;
    }
    scan
}

/// `cylinder name x y z dx dy dz radius`, bound; `cylinder_from_normal(s, "c", [0.; 3], [0., 0., 1.], 10.)`.
pub fn cylinder_from_normal(
    s: &mut dyn Session,
    name: &str,
    at: [f64; 3],
    normal: [f64; 3],
    radius: f64,
) -> Surface {
    let placed = Axis3::from_normal(point(at), direction(normal))
        .unwrap_or_else(|failure| panic!("cylinder {name} {at:?} {normal:?}: {failure:?}"));
    let surface = GeomCylindricalSurface::new(placed, radius)
        .unwrap_or_else(|failure| panic!("cylinder {name} {radius}: {failure:?}"));
    bound_surface(s, name, Handle::new(surface).cast())
}

/// The script's title between its two rules, and `puts ""`.
pub fn banner(width: usize, title: &str) {
    let rule = "=".repeat(width);
    println!("{rule}");
    println!("{title}");
    println!("{rule}");
    println!();
}

/// `don c_*`, with what else the script names.
pub fn donly_curves(s: &mut dyn Session, log: &BoundCurves, others: &[&str]) {
    let mut names = name_refs(&log.names);
    names.extend_from_slice(others);
    s.view(ViewStep::Donly(&names));
}

#[test]
fn intersect_names_a_single_curve_without_its_number() {
    assert_eq!(intersection_answer("res", 1, 1), "res res_p_1");
    assert_eq!(intersection_answer("res", 2, 0), "res_1 res_2");
    assert_eq!(intersection_answer("res", 0, 2), "res_p_1 res_p_2");
}
