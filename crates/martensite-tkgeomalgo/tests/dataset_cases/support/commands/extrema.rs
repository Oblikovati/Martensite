//! `extrema` and `proj` (GeometryTest_APICommands.cxx): GeomAPI extrema between curves and
//! surfaces and point projections, with the command's `ext_i` variables and its text.
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomSurface, GeomTrimmedCurve};
use martensite_tkgeomalgo::geom_api::{
    ExtremaCurveCurve, ExtremaCurveSurface, ExtremaSurfaceSurface, ProjectPointOnCurve,
    ProjectPointOnSurf,
};
use martensite_tkgeombase::extrema::ExtAlgo;
use martensite_tkmath::gp::{Direction3, Point3, Vector3};

/// The segment from `p1` to `p2`: `Geom_TrimmedCurve(Geom_Line(p1, p1p2), 0, |p1p2|)`.
pub fn segment(p1: Point3, p2: Point3) -> Result<Handle<dyn GeomCurve>, Failure> {
    let direction = Direction3::from_vector(Vector3::between(p1, p2))?;
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(p1, direction)).cast();
    let trimmed = GeomTrimmedCurve::new(&line, 0.0, p1.distance(&p2), true, true)?;
    Ok(Handle::new(trimmed).cast())
}

/// An `extrema` argument: a curve on its range or a surface on its bounds.
enum Operand {
    Curve(Handle<dyn GeomCurve>, (f64, f64)),
    Surface(Handle<dyn GeomSurface>, (f64, f64, f64, f64)),
}

fn operand(d: &Script, name: &str) -> Option<Operand> {
    let curve = d.curve(name);
    if let Some(c) = curve.get() {
        let range = (c.first_parameter(), c.last_parameter());
        return Some(Operand::Curve(curve, range));
    }
    let surface = d.surface(name);
    let bounds = surface.get()?.bounds();
    Some(Operand::Surface(surface, bounds))
}

/// The extrema found: the point pairs, or the distance of parallel operands.
struct Found {
    pairs: Vec<(Point3, Point3)>,
    parallel: Option<f64>,
}

/// `extrema a b`: `ext_i` segments (or `Extrema i is point : x y z`), or the infinite
/// solutions text.
pub fn extrema(d: &mut Script, first: &str, second: &str) -> ScriptStep<String> {
    d.command(&["extrema", first, second], |d| {
        let (Some(a), Some(b)) = (operand(d, first), operand(d, second)) else {
            return Ok(Reply::error(""));
        };
        let found = found_extrema(a, b)?;
        Ok(Reply::ok(publish_extrema(d, &found)?))
    })
}

fn found_extrema(a: Operand, b: Operand) -> Result<Found, Failure> {
    use Operand::{Curve, Surface};
    let mut pairs = Vec::new();
    let parallel = match (&a, &b) {
        (Curve(c1, r1), Curve(c2, r2)) => {
            let ex = ExtremaCurveCurve::with_ranges(c1, c2, *r1, *r2)?;
            if ex.extrema()?.is_parallel()? {
                Some(ex.lower_distance()?)
            } else {
                for j in 1..=ex.nb_extrema() {
                    pairs.push(ex.points(j)?);
                }
                None
            }
        }
        (Curve(c, r), Surface(s, b)) | (Surface(s, b), Curve(c, r)) => {
            let ex = ExtremaCurveSurface::with_ranges(c, s, (*r, (b.0, b.1), (b.2, b.3)))?;
            let curve_first = matches!(a, Curve(..));
            if ex.is_parallel()? {
                Some(ex.lower_distance()?)
            } else {
                for j in 1..=ex.nb_extrema() {
                    let (on_curve, on_surface) = ex.points(j)?;
                    pairs.push(if curve_first {
                        (on_curve, on_surface)
                    } else {
                        (on_surface, on_curve)
                    });
                }
                None
            }
        }
        (Surface(s1, b1), Surface(s2, b2)) => {
            let ex = ExtremaSurfaceSurface::with_bounds(s1, s2, *b1, *b2)?;
            if ex.is_parallel()? {
                Some(ex.lower_distance()?)
            } else {
                for j in 1..=ex.nb_extrema() {
                    pairs.push(ex.points(j)?);
                }
                None
            }
        }
    };
    Ok(Found { pairs, parallel })
}

fn publish_extrema(d: &mut Script, found: &Found) -> Result<String, Failure> {
    let mut text = String::new();
    if let Some(distance) = found.parallel {
        // Infinite solutions may come with no point in analytic curve/curve extrema.
        text.push_str(&format!(
            "Infinite number of extremas, distance = {}\n",
            g17(distance)
        ));
    } else if found.pairs.is_empty() {
        text.push_str("No solutions!\n");
    }
    for (j, (p1, p2)) in found.pairs.iter().enumerate() {
        let j = j + 1;
        if p1.distance(p2) < 1.0e-16 {
            text.push_str(&format!(
                "Extrema {j} is point : {} {} {}\n",
                g17(p1.x()),
                g17(p1.y()),
                g17(p1.z())
            ));
            continue;
        }
        let name = format!("ext_{j}");
        d.set(&name, segment(*p1, *p2)?);
        text.push_str(&format!("{name} "));
    }
    Ok(text)
}

/// `proj curve|surface x y z`: `ext_i` and the parameter text of each projection.
pub fn proj(d: &mut Script, target: &str, [x, y, z]: [f64; 3]) -> ScriptStep<String> {
    let texts = [x, y, z].map(g17);
    d.command(&["proj", target, &texts[0], &texts[1], &texts[2]], |d| {
        let p = Point3::new(x, y, z);
        let curve = d.curve(target);
        if let Some(c) = curve.get() {
            let range = (c.first_parameter(), c.last_parameter());
            return project_on_curve(d, p, &curve, range);
        }
        let surface = d.surface(target);
        let Some(s) = surface.get() else {
            return Ok(Reply::error(""));
        };
        let projection = ProjectPointOnSurf::with_bounds(p, &surface, s.bounds(), ExtAlgo::Grad)?;
        if !projection.is_done() {
            return Ok(Reply::ok("projection failed."));
        }
        let mut text = String::new();
        for i in 1..=projection.nb_points()? {
            let (u, v) = projection.parameters(i)?;
            let shown = show_solution(d, i, (p, projection.point(i)?), true)?;
            text.push_str(&format!("{shown} Parameters: {} {}\n", g17(u), g17(v)));
        }
        Ok(Reply::ok(text))
    })
}

fn project_on_curve(
    d: &mut Script,
    p: Point3,
    curve: &Handle<dyn GeomCurve>,
    (first, last): (f64, f64),
) -> Result<Reply, Failure> {
    let projection = ProjectPointOnCurve::with_range(p, curve, first, last)?;
    if projection.nb_points() == 0 {
        d.cout("No project point was found.\n");
        return Ok(Reply::ok(""));
    }
    let mut text = String::new();
    for i in 1..=projection.nb_points() {
        let u = projection.parameter(i)?;
        let shown = show_solution(d, i, (p, projection.point(i)?), false)?;
        text.push_str(&format!("{shown} parameter {i} = {}\n", g17(u)));
    }
    Ok(Reply::ok(text))
}

/// `showProjSolution`: `ext_i`, a segment to the projection or the projected point.
fn show_solution(
    d: &mut Script,
    i: i32,
    (p, p1): (Point3, Point3),
    on_surface: bool,
) -> Result<String, Failure> {
    let name = format!("ext_{i}");
    if p.distance(&p1) > precision::confusion() {
        d.set(&name, segment(p, p1)?);
        return Ok(format!("{name} "));
    }
    d.set(&name, p1);
    let kind = if on_surface { "surface" } else { "curve" };
    Ok(format!("{name}  Point on {kind} "))
}
