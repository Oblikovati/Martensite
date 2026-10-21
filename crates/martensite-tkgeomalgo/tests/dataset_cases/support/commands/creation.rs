//! The curve and surface creation commands of GeomliteTest the cases call: `circle` and
//! `ellipse` (2D forms), `bsplinecurve`, `2dbsplinecurve`, `cylinder` and the 2D `offset`.
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dOffsetCurve,
};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomCylindricalSurface, GeomSurface};
use martensite_tkmath::gp::{Axis3, Axis22d, Direction2, Direction3, Point2, Point3};

/// The planar frame of the 2D conic commands: the given center and the X direction.
fn planar_frame(x: f64, y: f64) -> Result<Axis22d, Failure> {
    Axis22d::new(Point2::new(x, y), Direction2::X, true)
}

/// The words of a command whose numbers the case writes as script text.
fn texts(values: &[f64]) -> Vec<String> {
    values.iter().copied().map(g17).collect()
}

fn command_args<'a>(name: &'a str, var: &'a str, texts: &'a [String]) -> Vec<&'a str> {
    [name, var]
        .into_iter()
        .chain(texts.iter().map(String::as_str))
        .collect()
}

/// `circle name x y r`: a 2D circle.
pub fn circle2d(d: &mut Script, name: &str, [x, y, radius]: [f64; 3]) -> ScriptStep {
    let words = texts(&[x, y, radius]);
    d.command(&command_args("circle", name, &words), |d| {
        let circle: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dCircle::new(planar_frame(x, y)?, radius)?).cast();
        d.set(name, circle);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `ellipse name x y major minor`: a 2D ellipse.
pub fn ellipse2d(d: &mut Script, name: &str, [x, y, major, minor]: [f64; 4]) -> ScriptStep {
    let words = texts(&[x, y, major, minor]);
    d.command(&command_args("ellipse", name, &words), |d| {
        let ellipse: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dEllipse::new(planar_frame(x, y)?, major, minor)?).cast();
        d.set(name, ellipse);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `cylinder name x y z dx dy dz ux uy uz r`: `Geom_CylindricalSurface(gp_Ax3, r)`.
pub fn cylinder(d: &mut Script, name: &str, values: [f64; 10]) -> ScriptStep {
    let words = texts(&values);
    d.command(&command_args("cylinder", name, &words), |d| {
        let v = &values;
        let frame = Axis3::new(
            Point3::new(v[0], v[1], v[2]),
            Direction3::new(v[3], v[4], v[5])?,
            Direction3::new(v[6], v[7], v[8])?,
        )?;
        let surface: Handle<dyn GeomSurface> =
            Handle::new(GeomCylindricalSurface::new(frame, v[9])?).cast();
        d.set(name, surface);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `offset name curve2d distance`: `Geom2d_OffsetCurve`.
pub fn offset2d(d: &mut Script, name: &str, curve: &str, distance: f64) -> ScriptStep {
    let text = g17(distance);
    d.command(&["offset", name, curve, &text], |d| {
        let basis = d.curve2d(curve);
        let offset: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dOffsetCurve::new(&basis, distance, false)?).cast();
        d.set(name, offset);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// The knots, multiplicities and pole count the `bsplinecurve` parsers read: `deg nbk
/// (knot mult)*`, then `Sigma - deg - 1` poles (non-periodic).
struct SplineHeader {
    degree: i32,
    knots: Vec<f64>,
    mults: Vec<i32>,
    poles: usize,
    next: usize,
}

/// `Script::Atoi`: the truncated real value.
fn atoi(value: f64) -> i32 {
    value as i32
}

fn spline_header(words: &[f64]) -> SplineHeader {
    let degree = atoi(words[0]);
    let count = atoi(words[1]) as usize;
    let knots: Vec<f64> = (0..count).map(|i| words[2 + 2 * i]).collect();
    let mults: Vec<i32> = (0..count).map(|i| atoi(words[3 + 2 * i])).collect();
    let sigma: i32 = mults.iter().sum();
    SplineHeader {
        degree,
        knots,
        mults,
        poles: (sigma - degree - 1) as usize,
        next: 2 + 2 * count,
    }
}

/// A one-based `NCollection_Array1` of `items`.
fn array<T: Default + Clone>(items: &[T]) -> Result<Array1<'static, T>, Failure> {
    let mut result = Array1::with_bounds(1, items.len() as i32)
        .map_err(|e| e.into_failure("NCollection_Array1"))?;
    for (i, item) in items.iter().enumerate() {
        result
            .set_value(i as i32 + 1, item.clone())
            .map_err(|e| e.into_failure("NCollection_Array1"))?;
    }
    Ok(result)
}

/// `bsplinecurve name deg nbknots knot mult ... x y z w ...` (the trailing words the
/// pole count does not reach are ignored, as natively).
pub fn bsplinecurve(d: &mut Script, name: &str, words: &[f64]) -> ScriptStep {
    let text = texts(words);
    d.command(&command_args("bsplinecurve", name, &text), |d| {
        let header = spline_header(words);
        let (mut poles, mut weights) = (Vec::new(), Vec::new());
        for i in 0..header.poles {
            let at = header.next + 4 * i;
            poles.push(Point3::new(words[at], words[at + 1], words[at + 2]));
            weights.push(words[at + 3]);
        }
        let curve = GeomBSplineCurve::with_weights(
            &array(&poles)?,
            &array(&weights)?,
            &array(&header.knots)?,
            &array(&header.mults)?,
            header.degree,
            false,
        )?;
        let curve: Handle<dyn GeomCurve> = Handle::new(curve).cast();
        d.set(name, curve);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `2dbsplinecurve name deg nbknots knot mult ... x y w ...`.
pub fn bsplinecurve2d(d: &mut Script, name: &str, words: &[f64]) -> ScriptStep {
    let text = texts(words);
    d.command(&command_args("2dbsplinecurve", name, &text), |d| {
        let header = spline_header(words);
        let (mut poles, mut weights) = (Vec::new(), Vec::new());
        for i in 0..header.poles {
            let at = header.next + 3 * i;
            poles.push(Point2::new(words[at], words[at + 1]));
            weights.push(words[at + 2]);
        }
        let curve = Geom2dBSplineCurve::with_weights(
            &array(&poles)?,
            &array(&weights)?,
            &array(&header.knots)?,
            &array(&header.mults)?,
            header.degree,
            false,
        )?;
        let curve: Handle<dyn Geom2dCurve> = Handle::new(curve).cast();
        d.set(name, curve);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}
