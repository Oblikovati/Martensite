//! The geometry commands the cases call, on the Rust API; each names its command. Results are logged as the commands print them.
use super::log;
use super::printed::g17;
use super::restore::Restored;
use super::session::Session;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Shape},
};
use martensite_tkernel::{precision, standard::Handle, storage::format_general};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface, GeomTrimmedCurve};
use martensite_tkgeomalgo::{
    geom_api::{ExtremaCurveSurface, IntCS, ProjectPointOnSurf},
    geom2d_api::ExtremaCurveCurve as Extrema2d,
};
use martensite_tkgeombase::extrema::ExtAlgo;
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Line3, Point3, Vector3};

/// GeomliteTest_CurveCommands.cxx `line name x y z dx dy dz`.
pub fn line(point: [f64; 3], direction: [f64; 3]) -> Handle<dyn GeomCurve> {
    let [x, y, z] = point;
    let [dx, dy, dz] = direction;
    let lin = Line3::new(Point3::new(x, y, z), Direction3::new(dx, dy, dz).unwrap());
    Handle::new(GeomLine::from_lin(lin)).cast()
}

/// GeomliteTest_SurfaceCommands.cxx `plane name x y z nx ny nz ux uy uz`.
pub fn plane(point: [f64; 3], normal: [f64; 3], x_dir: [f64; 3]) -> Handle<dyn GeomSurface> {
    let [x, y, z] = point;
    let [nx, ny, nz] = normal;
    let [ux, uy, uz] = x_dir;
    let frame = Axis3::new(
        Point3::new(x, y, z),
        Direction3::new(nx, ny, nz).unwrap(),
        Direction3::new(ux, uy, uz).unwrap(),
    )
    .unwrap();
    Handle::new(GeomPlane::new(frame)).cast()
}

/// GeomliteTest_CurveCommands.cxx `cvalue curve u x y z`: the point.
pub fn cvalue(curve: &Handle<dyn GeomCurve>, u: f64) -> Point3 {
    curve.get().unwrap().value(u).unwrap()
}

/// GeometryTest_CurveCommands.cxx `intersect name curve surface`: GeomAPI_IntCS; the
/// set variables (segments then points, `name_i`, or `name` alone for a single result)
/// and the printed names; `Err("No intersections found!")` is the command's script error.
pub fn intersect_curve_surface(
    s: &mut dyn Session,
    name: &str,
    curve: &Handle<dyn GeomCurve>,
    surface: &Handle<dyn GeomSurface>,
) -> Result<Vec<(String, Restored)>, String> {
    let inters = IntCS::with_curve(curve, surface).map_err(|f| format!("{f:?}"))?;
    if !inters.is_done() {
        log::result("No intersections found!\n");
        return Err("No intersections found!".into());
    }
    let nb_lines = inters.nb_segments().unwrap();
    let nb_points = inters.nb_points().unwrap();
    let mut results: Vec<(String, Restored)> = Vec::new();
    if nb_lines + nb_points >= 2 {
        for i in 1..=nb_lines {
            results.push((format!("{name}_{i}"), inters.segment(i).unwrap().into()));
        }
        for i in nb_lines + 1..=nb_lines + nb_points {
            results.push((format!("{name}_{i}"), inters.point(i).unwrap().into()));
        }
    } else if nb_lines == 1 {
        results.push((name.to_string(), inters.segment(1).unwrap().into()));
    } else if nb_points == 1 {
        results.push((name.to_string(), inters.point(1).unwrap().into()));
    }
    let mut text = String::new();
    for (var, value) in &results {
        s.set(var, value);
        text.push_str(var);
        text.push(' ');
    }
    log::result(&text);
    Ok(results)
}

/// The `ext_i` segment of a solution: the line from `p1` towards `p2`, trimmed to the
/// distance.
fn ext_segment(p1: Point3, p2: Point3) -> Handle<dyn GeomCurve> {
    let direction = Direction3::from_vector(Vector3::between(p1, p2)).unwrap();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(p1, direction)).cast();
    let trimmed = GeomTrimmedCurve::new(&line, 0., p1.distance(&p2), true, true).unwrap();
    Handle::new(trimmed).cast()
}

/// GeometryTest_APICommands.cxx `extrema curve surface`: GeomAPI_ExtremaCurveSurface on
/// the curve range and the surface bounds; the `ext_i` segments (a coincident pair prints
/// `Extrema i is point : x y z`); the printed text.
pub fn extrema_curve_surface(
    s: &mut dyn Session,
    curve: &Handle<dyn GeomCurve>,
    surface: &Handle<dyn GeomSurface>,
) -> (String, Vec<Handle<dyn GeomCurve>>) {
    let c = curve.get().unwrap();
    let bounds = surface.get().unwrap().bounds();
    let ranges = (
        (c.first_parameter(), c.last_parameter()),
        (bounds.0, bounds.1),
        (bounds.2, bounds.3),
    );
    let ex = ExtremaCurveSurface::with_ranges(curve, surface, ranges).unwrap();
    let mut text = String::new();
    let mut segments = Vec::new();
    let parallel = ex.is_parallel().unwrap_or(false);
    if parallel {
        text.push_str(&format!(
            "Infinite number of extremas, distance = {}\n",
            g17(ex.lower_distance().unwrap())
        ));
    } else {
        let count = ex.nb_extrema();
        if count == 0 {
            text.push_str("No solutions!\n");
        }
        for j in 1..=count {
            let (p1, p2) = ex.points(j).unwrap();
            if p1.distance(&p2) < 1.0e-16 {
                text.push_str(&format!(
                    "Extrema {j} is point : {} {} {}\n",
                    g17(p1.x()),
                    g17(p1.y()),
                    g17(p1.z())
                ));
                continue;
            }
            let segment = ext_segment(p1, p2);
            s.set(&format!("ext_{j}"), &segment.clone().into());
            text.push_str(&format!("ext_{j} "));
            segments.push(segment);
        }
    }
    log::result(&text);
    (text, segments)
}

/// One `ext_i` solution of `proj`: the segment, or the point when the projection is the
/// point itself.
#[derive(Clone)]
pub enum ProjSolution {
    Segment(Handle<dyn GeomCurve>),
    Point(Point3),
}

/// GeometryTest_APICommands.cxx `proj surface x y z`: GeomAPI_ProjectPointOnSurf on the
/// surface bounds (Extrema_ExtAlgo_Grad); each solution `ext_i` (with ` Parameters: u v`
/// printed); `None` for `projection failed.`.
pub fn proj_surface(
    s: &mut dyn Session,
    surface: &Handle<dyn GeomSurface>,
    p: Point3,
) -> Option<Vec<ProjSolution>> {
    let bounds = surface.get().unwrap().bounds();
    let proj = ProjectPointOnSurf::with_bounds(p, surface, bounds, ExtAlgo::Grad).unwrap();
    if !proj.is_done() {
        log::result("projection failed.");
        return None;
    }
    let mut text = String::new();
    let mut solutions = Vec::new();
    for i in 1..=proj.nb_points().unwrap() {
        let p1 = proj.point(i).unwrap();
        let (u, v) = proj.parameters(i).unwrap();
        text.push_str(&format!("ext_{i} "));
        let solution = if p.distance(&p1) > precision::confusion() {
            let segment = ext_segment(p, p1);
            s.set(&format!("ext_{i}"), &segment.clone().into());
            ProjSolution::Segment(segment)
        } else {
            s.set(&format!("ext_{i}"), &p1.into());
            text.push_str(" Point on surface ");
            ProjSolution::Point(p1)
        };
        text.push_str(&format!(" Parameters: {} {}\n", g17(u), g17(v)));
        solutions.push(solution);
    }
    log::result(&text);
    Some(solutions)
}

/// BRepTest_CurveCommands.cxx `mk2dcurve name edge face`: the p-curve of the edge on the
/// face, trimmed to its range.
pub fn mk2dcurve_on_face(edge: &Shape, face: &Shape) -> Handle<dyn Geom2dCurve> {
    let edge = topods::edge(edge).expect("mk2dcurve: an edge");
    let face = topods::face(face).expect("mk2dcurve: a face");
    let found = BRepTool::curve_on_face(&edge, &face).unwrap();
    assert!(
        !found.pcurve.is_null(),
        "mk2dcurve: the edge has no p-curve on the face"
    );
    let trimmed = Geom2dTrimmedCurve::new(&found.pcurve, found.first, found.last, true, true);
    Handle::new(trimmed.unwrap()).cast()
}

/// GeomliteTest_API2dCommands.cxx `2dextrema c1 c2`: Geom2dAPI_ExtremaCurveCurve on the
/// curve ranges; the number of solutions it prints.
pub fn extrema2d(c1: &Handle<dyn Geom2dCurve>, c2: &Handle<dyn Geom2dCurve>) -> i32 {
    let (a, b) = (c1.get().unwrap(), c2.get().unwrap());
    let ex = Extrema2d::new(
        c1,
        c2,
        a.first_parameter(),
        a.last_parameter(),
        b.first_parameter(),
        b.last_parameter(),
    )
    .unwrap();
    let count = ex.nb_extrema();
    let mut text = String::new();
    for i in 1..=count {
        text.push_str(&format!(
            "dist {i}: {} ",
            format_general(ex.distance(i).unwrap(), 6)
        ));
    }
    log::result(&text);
    count
}

/// `dump name` of a point variable (the point dump at precision 15).
pub fn dump_point(name: &str, p: Point3) -> String {
    format!(
        "\n\n*********** Dump of {name} *************\nPoint : {}, {}, {}\n",
        format_general(p.x(), 15),
        format_general(p.y(), 15),
        format_general(p.z(), 15)
    )
}

/// The `Axis2` of a `circle name x y z r`: the point and the Z direction.
pub fn z_frame(point: [f64; 3]) -> Axis2 {
    let [x, y, z] = point;
    Axis2::new(Point3::new(x, y, z), Direction3::new(0., 0., 1.).unwrap()).unwrap()
}
