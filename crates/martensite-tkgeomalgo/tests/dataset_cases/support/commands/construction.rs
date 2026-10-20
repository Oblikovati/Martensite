//! The construction commands of GeometryTest the cases call: `tuyau` and `fillcurves`
//! (GeometryTest_SurfaceCommands.cxx), `interpol` and `cirtang`
//! (GeometryTest_ConstraintCommands.cxx).
use crate::support::dataset::located_bytes;
use crate::support::numbers::g17;
use crate::support::script::{Reply, Script, ScriptStep};
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomSurface};
use martensite_tkgeomalgo::geom_api;
use martensite_tkgeomalgo::geom_fill::{
    GeomFillBSplineCurves, GeomFillFillingStyle, GeomFillPipe, GeomFillTrihedron,
};
use martensite_tkgeomalgo::geom2d_gcc::{self, Circ2d3Tan};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::Point3;

/// `tuyau result path radius|section`: a pipe of constant radius (when the third word
/// is no curve) or of constant section, approximated at 1e-4 with C2 continuity.
pub fn tuyau(d: &mut Script, result: &str, path: &str, section: &str) -> ScriptStep<String> {
    d.command(&["tuyau", result, path, section], |d| {
        let spine = d.curve(path);
        if spine.is_null() {
            return Ok(Reply::error(""));
        }
        let mut pipe = GeomFillPipe::new();
        pipe.set_generate_particular_case(true);
        let first = d.curve(section);
        if first.is_null() {
            let radius: f64 = section.parse().expect("tuyau radius");
            pipe.init_radius(&spine, radius)?;
        } else {
            pipe.init_section(&spine, &first, GeomFillTrihedron::IsCorrectedFrenet)?;
        }
        pipe.perform(1.0e-4, false, Shape::C2, 11, 30)?;
        if !pipe.is_done() {
            return Ok(Reply::ok("Error: GeomFill_Pipe cannot make a surface\n"));
        }
        let accuracy = pipe.error_on_surf();
        let surface: Handle<dyn GeomSurface> = pipe.surface().cast();
        d.set(result, surface);
        Ok(Reply::ok(format!(
            "Accuracy of approximation = {}\n",
            g17(accuracy)
        )))
    })
}

/// `fillcurves result c1 c2 c3 c4`: the Coons filling of four curves converted to
/// rational C1 B-splines.
pub fn fillcurves(d: &mut Script, result: &str, curves: [&str; 4]) -> ScriptStep {
    let args = [
        "fillcurves",
        result,
        curves[0],
        curves[1],
        curves[2],
        curves[3],
    ];
    d.command(&args, |d| {
        let mut splines: Vec<Handle<GeomBSplineCurve>> = Vec::new();
        for name in curves {
            let curve = d.curve(name);
            if curve.is_null() {
                return Ok(Reply::error(""));
            }
            splines.push(geom_convert::curve_to_bspline_curve(
                &curve,
                ParameterisationType::RationalC1,
            )?);
        }
        let style = GeomFillFillingStyle::CoonsStyle;
        let filling = GeomFillBSplineCurves::with_four(
            &splines[0],
            &splines[1],
            &splines[2],
            &splines[3],
            style,
        )?;
        let surface: Handle<dyn GeomSurface> = filling.surface().cast();
        d.set(result, surface);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `interpol name file`: the interpolation (tolerance 1e-5) of the
/// points of a `nbpoints 3d x y z ...` file.
pub fn interpol(d: &mut Script, name: &str, file: &str) -> ScriptStep {
    let bytes = located_bytes(file);
    d.command(&["interpol", name, file], |d| {
        let text = String::from_utf8(bytes).expect("interpol: a text file");
        let mut words = text.split_whitespace();
        let count: usize = words
            .next()
            .and_then(|w| w.parse().ok())
            .expect("interpol: the point count");
        assert_eq!(words.next(), Some("3d"), "interpol {file}: a 3d point file");
        let mut real = || -> f64 {
            words
                .next()
                .and_then(|w| w.parse().ok())
                .expect("interpol: a coordinate")
        };
        let points: Vec<Point3> = (0..count)
            .map(|_| Point3::new(real(), real(), real()))
            .collect();
        let mut interpolator = geom_api::Interpolate::new(&points_array(&points)?, false, 1.0e-5)?;
        interpolator.perform()?;
        if interpolator.is_done() {
            let curve: Handle<dyn GeomCurve> = interpolator.curve()?.cast();
            d.set(name, curve);
        }
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// A one-based `NCollection_HArray1` of `points`.
fn points_array(points: &[Point3]) -> Result<Handle<HArray1<Point3>>, Failure> {
    let mut array = Array1::with_bounds(1, points.len() as i32)
        .map_err(|e| e.into_failure("NCollection_Array1"))?;
    for (i, point) in points.iter().enumerate() {
        array
            .set_value(i as i32 + 1, *point)
            .map_err(|e| e.into_failure("NCollection_Array1"))?;
    }
    Ok(Handle::new(HArray1::from_owned(array)))
}

/// `cirtang result -c c1 -c c2 -c c3`: `Geom2dGcc_Circ2d3Tan` of three unqualified
/// curves; `result_i` per solution.
pub fn cirtang_curves(d: &mut Script, result: &str, curves: [&str; 3]) -> ScriptStep<String> {
    let args = [
        "cirtang", result, "-c", curves[0], "-c", curves[1], "-c", curves[2],
    ];
    d.command(&args, |d| {
        let mut qualified = Vec::new();
        for name in curves {
            let curve: Handle<dyn Geom2dCurve> = d.curve2d(name);
            if curve.is_null() {
                return Ok(Reply::error(format!("Error: {name} is not a curve\n")));
            }
            qualified.push(geom2d_gcc::unqualified(&Geom2dAdaptorCurve::with_curve(
                &curve,
            )?)?);
        }
        let solver = Circ2d3Tan::new(
            &qualified[0],
            &qualified[1],
            &qualified[2],
            precision::confusion(),
            [0.0; 3],
        )?;
        let mut text = String::from("Solution of type C-C-C is: ");
        if !solver.is_done() {
            text.push_str("Circ2d3Tan Not done");
            return Ok(Reply::error(text));
        }
        for i in 1..=solver.nb_solutions() {
            let name = format!("{result}_{i}");
            let circle: Handle<dyn Geom2dCurve> =
                Handle::new(Geom2dCircle::from_circ2d(solver.this_solution(i)?)).cast();
            d.set(&name, circle);
            text.push_str(&format!("{name} "));
        }
        Ok(Reply::ok(text))
    })
}
