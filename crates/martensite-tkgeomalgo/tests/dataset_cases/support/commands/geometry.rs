//! The geometry commands of GeomliteTest and GeometryTest the cases call: `plane`, `trim`,
//! `trimu`, `trimv`, `bounds`, `cvalue`, `svalue`, `length`, `to2d`, `to3d`, `project`.
use crate::support::brep_commands::{length2d, length3d, trim_to_basis};
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{
    GeomCurve, GeomPlane, GeomRectangularTrimmedSurface, GeomSurface, GeomTrimmedCurve,
};
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3, xoy};

/// `plane name x y z [dx dy dz [ux uy uz]]` (`anasurface`): `Geom_Plane(gp_Ax3)`.
pub fn plane(d: &mut Script, name: &str, values: &[f64]) -> ScriptStep {
    let texts: Vec<String> = values.iter().copied().map(g17).collect();
    let args: Vec<&str> = ["plane", name]
        .into_iter()
        .chain(texts.iter().map(String::as_str))
        .collect();
    d.command(&args, |d| {
        let point = |i: usize| Point3::new(values[i], values[i + 1], values[i + 2]);
        let direction = |i: usize| Direction3::new(values[i], values[i + 1], values[i + 2]);
        let frame = match values.len() {
            0 => Axis3::new(
                Point3::new(0., 0., 0.),
                Direction3::new(0., 0., 1.)?,
                Direction3::new(1., 0., 0.)?,
            )?,
            3 => Axis3::new(
                point(0),
                Direction3::new(0., 0., 1.)?,
                Direction3::new(1., 0., 0.)?,
            )?,
            6 => Axis3::from_normal(point(0), direction(3)?)?,
            9 => Axis3::new(point(0), direction(3)?, direction(6)?)?,
            count => panic!("plane with {count} values; expected 0, 3, 6 or 9"),
        };
        let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
        d.set(name, plane);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `trim name curve u1 u2`: `Geom_TrimmedCurve(curve, u1, u2)` (or the 2D one).
pub fn trim_curve(d: &mut Script, name: &str, curve: &str, (u1, u2): (f64, f64)) -> ScriptStep {
    let (t1, t2) = (g17(u1), g17(u2));
    d.command(&["trim", name, curve, &t1, &t2], |d| {
        let c3d = d.curve(curve);
        if !c3d.is_null() {
            let trimmed: Handle<dyn GeomCurve> =
                Handle::new(GeomTrimmedCurve::new(&c3d, u1, u2, true, true)?).cast();
            d.set(name, trimmed);
            return Ok(Reply::ok(""));
        }
        let c2d = d.curve2d(curve);
        if c2d.is_null() {
            return Ok(Reply::error(""));
        }
        let trimmed: Handle<dyn Geom2dCurve> =
            Handle::new(Geom2dTrimmedCurve::new(&c2d, u1, u2, true, true)?).cast();
        d.set(name, trimmed);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `trim name surface u1 u2 v1 v2`: `Geom_RectangularTrimmedSurface`.
pub fn trim_surface(d: &mut Script, name: &str, surface: &str, uv: [f64; 4]) -> ScriptStep {
    let texts = uv.map(g17);
    let args = [
        "trim", name, surface, &texts[0], &texts[1], &texts[2], &texts[3],
    ];
    d.command(&args, |d| {
        let basis = d.surface(surface);
        let trimmed = GeomRectangularTrimmedSurface::new(
            &basis,
            (uv[0], uv[1]),
            (uv[2], uv[3]),
            (true, true),
        )?;
        let trimmed: Handle<dyn GeomSurface> = Handle::new(trimmed).cast();
        d.set(name, trimmed);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `trimu`/`trimv name surface p1 p2`: one-direction rectangular trim.
pub fn trim_one_direction(
    d: &mut Script,
    u_trim: bool,
    name: &str,
    surface: &str,
    (p1, p2): (f64, f64),
) -> ScriptStep {
    let (t1, t2) = (g17(p1), g17(p2));
    let command = if u_trim { "trimu" } else { "trimv" };
    d.command(&[command, name, surface, &t1, &t2], |d| {
        let basis = d.surface(surface);
        if basis.is_null() {
            return Ok(Reply::error(""));
        }
        let trimmed =
            GeomRectangularTrimmedSurface::with_one_direction(&basis, p1, p2, u_trim, true)?;
        let trimmed: Handle<dyn GeomSurface> = Handle::new(trimmed).cast();
        d.set(name, trimmed);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `trim name geometry` (three words): the basis of a trimmed curve or surface, or the
/// geometry itself; the new variable shares the handle.
pub fn untrim(d: &mut Script, name: &str, geometry: &str) -> ScriptStep {
    d.command(&["trim", name, geometry], |d| {
        let curve = d.curve(geometry);
        if !curve.is_null() {
            d.set(name, trim_to_basis(&curve));
            return Ok(Reply::ok(""));
        }
        let surface = d.surface(geometry);
        let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
        let basis = match trimmed.get() {
            Some(trimmed) => trimmed.basis_surface(),
            None => surface,
        };
        d.set(name, basis);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `dval expression`: the value the case computed from the numeric variables, printed
/// `%.17g` (the round trip the script's later uses of the text see).
pub fn dval(d: &mut Script, expression: &str, value: f64) -> ScriptStep<f64> {
    let text = d.command(&["dval", expression], |_| Ok(Reply::ok(g17(value))))?;
    Ok(crate::support::numbers::script_real(&text))
}

/// `bounds curve u1 u2`: the parameter range as numeric variables.
pub fn bounds(d: &mut Script, curve: &str, names: [&str; 2]) -> ScriptStep<(f64, f64)> {
    let mut range = (0., 0.);
    d.command(&["bounds", curve, names[0], names[1]], |d| {
        let c3d = d.curve(curve);
        range = match c3d.get() {
            Some(c) => (c.first_parameter(), c.last_parameter()),
            None => {
                let c2d = d.curve2d(curve);
                let Some(c) = c2d.get() else {
                    return Ok(Reply::error(""));
                };
                (c.first_parameter(), c.last_parameter())
            }
        };
        d.set(names[0], range.0);
        d.set(names[1], range.1);
        Ok(Reply::ok(""))
    })?;
    Ok(range)
}

/// `cvalue curve u x y z`: the point at `u` as numeric variables.
pub fn cvalue(d: &mut Script, curve: &str, u: f64, names: [&str; 3]) -> ScriptStep<Point3> {
    let text = g17(u);
    let mut point = Point3::new(0., 0., 0.);
    d.command(
        &["cvalue", curve, &text, names[0], names[1], names[2]],
        |d| {
            let c = d.curve(curve);
            let Some(c) = c.get() else {
                return Ok(Reply::error(""));
            };
            point = c.d0(u)?;
            for (name, value) in names.iter().zip([point.x(), point.y(), point.z()]) {
                d.set(name, value);
            }
            Ok(Reply::ok(""))
        },
    )?;
    Ok(point)
}

/// The values `svalue surface u v x y z dux duy duz dvx dvy dvz` stores.
pub struct SurfaceValue {
    pub point: Point3,
    pub du: [f64; 3],
    pub dv: [f64; 3],
}

/// `svalue surface u v` with the point and first derivatives as numeric variables
/// named `names` (`x y z dux duy duz dvx dvy dvz`).
pub fn svalue(
    d: &mut Script,
    surface: &str,
    (u, v): (f64, f64),
    names: [&str; 9],
) -> ScriptStep<SurfaceValue> {
    let (tu, tv) = (g17(u), g17(v));
    let args: Vec<&str> = ["svalue", surface, &tu, &tv]
        .into_iter()
        .chain(names)
        .collect();
    let mut found = None;
    d.command(&args, |d| {
        let s = d.surface(surface);
        let d1 = s.get().expect("svalue surface").d1(u, v)?;
        let (p, du, dv) = (d1.point, d1.d1u, d1.d1v);
        let values = [
            p.x(),
            p.y(),
            p.z(),
            du.x(),
            du.y(),
            du.z(),
            dv.x(),
            dv.y(),
            dv.z(),
        ];
        for (name, value) in names.iter().zip(values) {
            d.set(name, value);
        }
        found = Some(SurfaceValue {
            point: p,
            du: [values[3], values[4], values[5]],
            dv: [values[6], values[7], values[8]],
        });
        Ok(Reply::ok(""))
    })?;
    Ok(found.expect("svalue result"))
}

/// `length curve [tol]`: `The length <curve> is <L>`.
pub fn length(d: &mut Script, curve: &str, tolerance: Option<f64>) -> ScriptStep<String> {
    let tol_text = tolerance.map(g17);
    let mut args = vec!["length", curve];
    args.extend(tol_text.as_deref());
    let tolerance = tolerance.unwrap_or(precision::confusion());
    d.command(&args, |d| {
        let c3d = d.curve(curve);
        let c2d = d.curve2d(curve);
        let measured = if !c3d.is_null() {
            length3d(&c3d, tolerance)
        } else if !c2d.is_null() {
            length2d(&c2d, tolerance)
        } else {
            return Ok(Reply::error(format!("{curve}is not a curve\n")));
        };
        Ok(Reply::ok(format!(
            "The length {curve} is {}\n",
            g17(measured)
        )))
    })
}

/// `to2d name curve3d` on the default XOY plane (`GeomAPI::To2d`).
pub fn to2d(d: &mut Script, name: &str, curve: &str) -> ScriptStep {
    d.command(&["to2d", name, curve], |d| {
        let c = d.curve(curve);
        if c.is_null() {
            return Ok(Reply::error(""));
        }
        let projected = geom_api::to_2d(&c, &xoy_plane()?)?;
        d.set(name, projected);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `to3d name curve2d` on the default XOY plane (`GeomAPI::To3d`).
pub fn to3d(d: &mut Script, name: &str, curve: &str) -> ScriptStep {
    d.command(&["to3d", name, curve], |d| {
        let c = d.curve2d(curve);
        if c.is_null() {
            return Ok(Reply::error(""));
        }
        let lifted = geom_api::to_3d(&c, &xoy_plane()?)?;
        d.set(name, lifted);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// The default `gp_Pln` (`XOY`) of `to2d`/`to3d`.
fn xoy_plane() -> Result<Plane3, Failure> {
    Ok(GeomPlane::new(Axis3::from_axis2(xoy())).pln())
}
