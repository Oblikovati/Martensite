//! `intersect` (GeometryTest_CurveCommands.cxx `intersection`): `GeomInt_IntSS` for two
//! surfaces, `GeomAPI_IntCS` for a curve and a surface, with the command's variable naming.
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkgeomalgo::geom_api::IntCS;
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};

/// The optional arguments of `intersect result s1 s2 [u1 v1 u2 v2] [tolerance]`.
#[derive(Clone, Copy, Default)]
pub struct IntersectOptions {
    /// The starting point `u1 v1 u2 v2`.
    pub start: Option<[f64; 4]>,
    /// The tolerance (`Precision::Confusion()` without it).
    pub tolerance: Option<f64>,
}

/// `intersect result a b`: the names it prints and sets, in order.
pub fn intersect(d: &mut Script, result: &str, first: &str, second: &str) -> ScriptStep<String> {
    intersect_with(d, result, [first, second], IntersectOptions::default())
}

/// `intersect result a b [u1 v1 u2 v2] [tolerance]`.
pub fn intersect_with(
    d: &mut Script,
    result: &str,
    [first, second]: [&str; 2],
    options: IntersectOptions,
) -> ScriptStep<String> {
    let mut texts: Vec<String> = options.start.into_iter().flatten().map(g17).collect();
    texts.extend(options.tolerance.map(g17));
    let args: Vec<&str> = ["intersect", result, first, second]
        .into_iter()
        .chain(texts.iter().map(String::as_str))
        .collect();
    d.command(&args, |d| {
        let s1 = d.surface(first);
        let s2 = d.surface(second);
        if s2.is_null() {
            return Ok(Reply::error(""));
        }
        if s1.is_null() {
            let c1 = d.curve(first);
            if c1.is_null() {
                return Ok(Reply::error(""));
            }
            return curve_surface(d, result, &c1, &s2);
        }
        surface_surface(d, result, [&s1, &s2], options)
    })
}

fn surface_surface(
    d: &mut Script,
    result: &str,
    [s1, s2]: [&Handle<dyn GeomSurface>; 2],
    options: IntersectOptions,
) -> Result<Reply, Failure> {
    let tolerance = options.tolerance.unwrap_or(precision::confusion());
    let mut inters = GeomIntIntSS::new();
    let approx = GeomIntIntSSApprox::default();
    match options.start {
        None => inters.perform(s1, s2, tolerance, approx)?,
        Some(uv) => inters.perform_from_start(s1, s2, tolerance, uv, approx)?,
    }
    if !inters.is_done() {
        return Ok(Reply::error("No intersections found!\n"));
    }
    let mut text = String::new();
    let count = inters.nb_lines();
    for i in 1..=count {
        let name = if count >= 2 {
            format!("{result}_{i}")
        } else {
            result.into()
        };
        text.push_str(&format!("{name} "));
        d.set(&name, inters.line(i)?);
    }
    for i in 1..=inters.nb_points()? {
        let name = format!("{result}_p_{i}");
        text.push_str(&format!("{name} "));
        d.set(&name, inters.point(i)?);
    }
    Ok(Reply::ok(text))
}

/// `intersect result s1 s2 U1F U1L V1F V1L U2F U2L V2F V2L`: both surfaces bounded.
pub fn intersect_bounded(
    d: &mut Script,
    result: &str,
    [first, second]: [&str; 2],
    bounds: [f64; 8],
) -> ScriptStep<String> {
    let texts = bounds.map(g17);
    let args: Vec<&str> = ["intersect", result, first, second]
        .into_iter()
        .chain(texts.iter().map(String::as_str))
        .collect();
    d.command(&args, |d| {
        let bounded = |surface: &Handle<dyn GeomSurface>,
                       b: &[f64]|
         -> Result<Handle<GeomAdaptorSurface>, Failure> {
            Ok(Handle::new(GeomAdaptorSurface::with_bounds(
                surface,
                (b[0], b[1], b[2], b[3]),
            )?))
        };
        let a1 = bounded(&d.surface(first), &bounds[..4])?;
        let a2 = bounded(&d.surface(second), &bounds[4..])?;
        let mut inters = GeomIntIntSS::new();
        inters.perform_adaptors(
            &a1,
            &a2,
            precision::confusion(),
            GeomIntIntSSApprox::default(),
        )?;
        if !inters.is_done() {
            return Ok(Reply::error("No intersections found!\n"));
        }
        let mut text = String::new();
        let count = inters.nb_lines();
        for i in 1..=count {
            let name = if count >= 2 {
                format!("{result}_{i}")
            } else {
                result.into()
            };
            text.push_str(&format!("{name} "));
            d.set(&name, inters.line(i)?);
        }
        Ok(Reply::ok(text))
    })
}

fn curve_surface(
    d: &mut Script,
    result: &str,
    c1: &Handle<dyn GeomCurve>,
    s2: &Handle<dyn GeomSurface>,
) -> Result<Reply, Failure> {
    let inters = IntCS::with_curve(c1, s2)?;
    if !inters.is_done() {
        return Ok(Reply::error("No intersections found!\n"));
    }
    let (lines, points) = (inters.nb_segments()?, inters.nb_points()?);
    let mut text = String::new();
    if lines + points >= 2 {
        if lines >= 1 {
            d.cout("   Lines: \n");
        }
        for i in 1..=lines {
            let name = format!("{result}_{i}");
            text.push_str(&format!("{name} "));
            d.set(&name, inters.segment(i)?.cast::<dyn GeomCurve>());
        }
        if points >= 1 {
            d.cout("   Points: \n");
        }
        // The points continue the numbering of the segments and are read with that index.
        for i in lines + 1..=lines + points {
            let name = format!("{result}_{i}");
            text.push_str(&format!("{name} "));
            d.set(&name, inters.point(i)?);
        }
    } else if lines == 1 {
        text.push_str(&format!("{result} "));
        d.set(result, inters.segment(1)?.cast::<dyn GeomCurve>());
    } else if points == 1 {
        text.push_str(&format!("{result} "));
        d.set(result, inters.point(1)?);
    }
    Ok(Reply::ok(text))
}
