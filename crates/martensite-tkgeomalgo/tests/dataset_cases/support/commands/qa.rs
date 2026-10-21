//! The diagnostic commands and the `lowalgos_intss` setup procedure `CheckIntersectionResult` the cases call.
use super::checks::xdistcs_aliased;
use super::geometry::bounds;
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};
use martensite_tkgeomalgo::int_patch::curvature_radius_of_inters_line;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
use martensite_tkmath::gp::Direction3;

/// `BUC60843 result c1 c2 par1 [par2]`: `Geom2dGcc_Lin2d2TanIter` from a
/// circle and a curve (one start parameter) or two curves (two start parameters), at
/// `Precision::Angular()`; the tangent line as `result`.
pub fn buc60843(
    d: &mut Script,
    result: &str,
    [first, second]: [&str; 2],
    start: &[f64],
) -> ScriptStep {
    use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
    use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
    use martensite_tkgeomalgo::gcc_ent::{Position, QualifiedCirc};
    use martensite_tkgeomalgo::geom2d_gcc::{Lin2d2TanIter, QCurve};
    let texts: Vec<String> = start.iter().copied().map(g17).collect();
    let args: Vec<&str> = ["BUC60843", result, first, second]
        .into_iter()
        .chain(texts.iter().map(String::as_str))
        .collect();
    d.command(&args, |d| {
        let (c1, c2) = (d.curve2d(first), d.curve2d(second));
        let qualified = |c: &Handle<dyn Geom2dCurve>| {
            QCurve::new(&Geom2dAdaptorCurve::with_curve(c)?, Position::Unqualified)
        };
        let tolerance = precision::angular();
        let circle = c1.cast::<Geom2dCircle>();
        let solver = match circle.get() {
            Some(circle) => {
                let from = QualifiedCirc::new(circle.circ2d()?, Position::Unqualified);
                Lin2d2TanIter::circle_curve(from, &qualified(&c2)?, start[0], tolerance)?
            }
            None => Lin2d2TanIter::new(
                &qualified(&c1)?,
                &qualified(&c2)?,
                start[0],
                start[1],
                tolerance,
            )?,
        };
        if !solver.is_done() {
            return Ok(Reply::ok("Faulty : tangent line is not done.\n"));
        }
        let line: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dLine::from_lin2d(solver.this_solution()?)).cast();
        d.set(result, line);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `QACheckBends curve`: `Error: The curve <c> is possible to have a bend at parameter p`
/// wherever the unit tangent turns by more than acos(0.8) between 1000 steps.
pub fn qa_check_bends(d: &mut Script, curve: &str) -> ScriptStep {
    d.command(&["QACheckBends", curve], |d| {
        let handle = d.curve(curve);
        let Some(c) = handle.get() else {
            return Ok(Reply::ok(format!(" {curve} : NULL curve\n")));
        };
        let (u1, u2) = (c.first_parameter(), c.last_parameter());
        if precision::is_infinite(u1) || precision::is_infinite(u2) {
            return Ok(Reply::ok(format!("Infinite interval  : {}  {}\n", g17(u1), g17(u2))));
        }
        let delta = (u2 - u1) / 1000.;
        let mut previous = Direction3::from_vector(c.d1(u1)?.1)?;
        let mut text = String::new();
        let mut p = u1;
        while p <= u2 {
            let current = Direction3::from_vector(c.d1(p)?.1)?;
            if previous.dot(current) < 0.8 {
                text.push_str(&format!(
                    "Error: The curve {curve} is possible to have a bend at parameter {}. Please check carefully \n",
                    g17(p)
                ));
            }
            previous = current;
            p += delta;
        }
        Ok(Reply::ok(text))
    })?;
    Ok(())
}

/// `OCC29807 s1 s2 u1 v1 u2 v2`: `IntPatch_PointLine::CurvatureRadiusOfIntersLine` at
/// the common point of the two surface points.
pub fn occ29807(d: &mut Script, [s1, s2]: [&str; 2], uv: [f64; 4]) -> ScriptStep<String> {
    let texts = uv.map(g17);
    let args = [
        "OCC29807", s1, s2, &texts[0], &texts[1], &texts[2], &texts[3],
    ];
    d.command(&args, |d| {
        let adapted = |s: Handle<dyn GeomSurface>| -> Result<Adaptor3dSurfaceHandle, martensite_tkernel::standard::Failure> {
            Ok(Handle::new(GeomAdaptorSurface::with_surface(&s)?).cast())
        };
        let (a1, a2) = (adapted(d.surface(s1))?, adapted(d.surface(s2))?);
        let p1 = a1.get().expect("first adaptor").value(uv[0], uv[1])?;
        let p2 = a2.get().expect("second adaptor").value(uv[2], uv[3])?;
        if p1.square_distance(&p2) > precision::square_confusion() {
            return Ok(Reply::error(
                "Error. True intersection point must be specified. Please check parameters: u1 v1 u2 v2.\n",
            ));
        }
        let mut point = IntSurfPntOn2S::new();
        let middle = (p1.xyz() + p2.xyz()) * 0.5;
        point.set_value_all(middle.into(), uv[0], uv[1], uv[2], uv[3]);
        let radius = curvature_radius_of_inters_line(&a1, &a2, &point)?;
        Ok(Reply::ok(format!("Radius of curvature is {}\n", g17(radius))))
    })
}

/// `OCC23952intersect nbsol s1 s2`: `GeomInt_IntSS` of the same two surfaces in 100
/// threads at once, each expecting `nbsol` lines.
pub fn occ23952intersect(d: &mut Script, expected: i32, [s1, s2]: [&str; 2]) -> ScriptStep {
    let count = expected.to_string();
    d.command(&["OCC23952intersect", &count, s1, s2], |d| {
        let (a, b) = (d.surface(s1), d.surface(s2));
        let outcomes: Vec<String> = std::thread::scope(|scope| {
            let threads: Vec<_> = (0..100)
                .map(|_| scope.spawn(|| thread_lines(&a, &b, expected)))
                .collect();
            threads
                .into_iter()
                .map(|t| t.join().expect("intersection thread"))
                .collect()
        });
        let mut text = String::new();
        for (i, outcome) in outcomes.iter().enumerate() {
            d.cout(outcome);
            if !outcome.ends_with(": OK\n") {
                text.push_str(&format!(
                    "Error: wrong number of intersections in thread {i}!\n"
                ));
            }
        }
        Ok(Reply::ok(text))
    })?;
    Ok(())
}

/// `GeomIntSSTest`: what one thread prints.
fn thread_lines(a: &Handle<dyn GeomSurface>, b: &Handle<dyn GeomSurface>, expected: i32) -> String {
    let mut inter = GeomIntIntSS::new();
    let done = inter
        .perform(a, b, precision::confusion(), GeomIntIntSSApprox::default())
        .is_ok();
    if !done || !inter.is_done() {
        return "An intersection is not done!\n".into();
    }
    let lines = inter.nb_lines();
    if lines == expected {
        format!("Number of Lines:{lines}: OK\n")
    } else {
        format!("Number of Lines:{lines}: Error, must be {expected}\n")
    }
}

/// `CheckIntersectionResult s1 s2 curves nbpoints tol1 tol2` (lowalgos/intss/begin).
pub fn check_intersection_result(
    d: &mut Script,
    [s1, s2]: [&str; 2],
    curves: &[String],
    count: usize,
    tolerances: [f64; 2],
) -> ScriptStep {
    for curve in curves {
        d.puts(&format!("** Check of {curve} **"));
        let (u1, u2) = bounds(d, curve, ["U1", "U2"])?;
        if u2 - u1 < 1.0e-9 {
            d.puts(&format!("Error: Wrong range of {curve}"));
        }
        xdistcs_aliased(
            d,
            ("aCurve", curve),
            ("s1", s1),
            (u1, u2),
            count,
            tolerances[0],
        )?;
        xdistcs_aliased(
            d,
            ("aCurve", curve),
            ("s2", s2),
            (u1, u2),
            count,
            tolerances[1],
        )?;
    }
    Ok(())
}
