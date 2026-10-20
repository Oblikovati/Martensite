//! The 2D curve commands the cases call: `2dintersect` and `2dextrema`
//! (GeomliteTest_API2dCommands.cxx, `Geom2dAPI_InterCurveCurve` and
//! `Geom2dAPI_ExtremaCurveCurve`) and `project` (GeometryTest_CurveCommands.cxx,
//! `GeomProjLib::Curve2d`).
use crate::support::brep_commands;
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use crate::support::session::Marker;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve};
use martensite_tkgeomalgo::geom2d_api::{ExtremaCurveCurve, InterCurveCurve};
use martensite_tkmath::gp::{Direction2, Point2, Vector2};

/// `2dextrema c1 c2`: `dist i: d  ext_i` per extremum, a point when the distance is
/// within `Precision::PConfusion()`, else the segment.
pub fn extrema2d(d: &mut Script, first: &str, second: &str) -> ScriptStep<String> {
    d.command(&["2dextrema", first, second], |d| {
        let (c1, c2) = (d.curve2d(first), d.curve2d(second));
        let (Some(a), Some(b)) = (c1.get(), c2.get()) else {
            return Ok(Reply::error("Syntax error: a curve is NULL"));
        };
        let (r1, r2) = (
            (a.first_parameter(), a.last_parameter()),
            (b.first_parameter(), b.last_parameter()),
        );
        let ex = ExtremaCurveCurve::new(&c1, &c2, r1.0, r1.1, r2.0, r2.1)?;
        let mut text = String::new();
        if ex.extrema()?.is_parallel()? {
            text.push_str(&format!(
                "Infinite number of extremas, distance = {}\n",
                g17(ex.lower_distance()?)
            ));
        } else if ex.nb_extrema() == 0 {
            text.push_str("No solutions!\n");
        }
        for i in 1..=ex.nb_extrema() {
            let (p1, p2) = ex.points(i)?;
            let distance = ex.distance(i)?;
            text.push_str(&format!("dist {i}: {}  ", g17(distance)));
            let name = format!("ext_{i}");
            if distance <= precision::p_confusion() {
                d.marker(Marker::PlaneCross(p1));
                d.set(&name, p1);
            } else {
                d.set(&name, segment2d(p1, p2)?);
            }
            text.push_str(&format!("{name}\n"));
        }
        Ok(Reply::ok(text))
    })
}

/// `Geom2d_TrimmedCurve(Geom2d_Line(p1, p1p2), 0, |p1p2|)`.
fn segment2d(p1: Point2, p2: Point2) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let direction = Direction2::from_vector(Vector2::between(p1, p2))?;
    let line: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(p1, direction)).cast();
    Ok(Handle::new(Geom2dTrimmedCurve::new(
        &line,
        0.,
        p1.distance(p2),
        true,
        true,
    )?)
    .cast())
}

/// `2dintersect c1 [c2] [-tol tol] [-state]`.
pub fn intersect2d(
    d: &mut Script,
    curves: &[&str],
    tolerance: Option<f64>,
    state: bool,
) -> ScriptStep<String> {
    let tol_text = tolerance.map(g17);
    let mut args = vec!["2dintersect"];
    args.extend(curves);
    if let Some(text) = tol_text.as_deref() {
        args.extend(["-tol", text]);
    }
    if state {
        args.push("-state");
    }
    d.command(&args, |d| {
        let handles: Vec<_> = curves.iter().map(|name| d.curve2d(name)).collect();
        let tolerance = tolerance.unwrap_or(0.001);
        let mut intersector = InterCurveCurve::new();
        match handles.as_slice() {
            [c1, c2] => intersector.init(c1, c2, tolerance)?,
            [c1] => intersector.init_self(c1, tolerance)?,
            _ => panic!("2dintersect of {curves:?}: expected one or two curves"),
        }
        let tool = intersector.intersector();
        if !tool.is_done() {
            return Ok(Reply::ok("Intersection failed\n"));
        }
        if tool.is_empty()? {
            return Ok(Reply::ok(""));
        }
        let mut text = String::new();
        for i in 1..=intersector.nb_points() {
            let p = intersector.point(i)?;
            let point = tool.point(i)?;
            text.push_str(&format!(
                "Intersection point {i} : {} {}\n",
                g17(p.x()),
                g17(p.y())
            ));
            text.push_str(&format!(
                "parameter on the fist: {} parameter on the second: {}\n",
                g17(point.param_on_first()),
                g17(point.param_on_second())
            ));
            if state {
                let kind = if point.transition_of_first().is_tangent()? {
                    "TOUCH"
                } else {
                    "INTERSECTION"
                };
                text.push_str(&format!("Intersection type: {kind}\n"));
            }
            d.marker(Marker::PlaneCross(p));
        }
        for i in 1..=intersector.nb_segments() {
            text.push_str(&format!("Segment #{i} found.\n"));
            let (s1, s2) = intersector.segment(i)?;
            d.marker(Marker::PlaneSegments(s1, s2));
        }
        Ok(Reply::ok(text))
    })
}

/// `project name curve3d surface`: `GeomProjLib::Curve2d` on the surface bounds.
pub fn project(d: &mut Script, name: &str, curve: &str, surface: &str) -> ScriptStep {
    d.command(&["project", name, curve, surface], |d| {
        let projected =
            brep_commands::project(&d.curve(curve), &d.surface(surface), precision::confusion());
        let Some(projected) = projected else {
            return Ok(Reply::error("Projection Failed\n"));
        };
        d.set(name, projected);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}
