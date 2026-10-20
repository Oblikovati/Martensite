//! The checks the cases call: `xdistcs`, `checksection` and the check procedures
//! `checkreal` and `checklength`.
use super::geometry::length;
use super::variables::{isdraw, whatis};
use crate::support::numbers::{g, g17, script_real};
use crate::support::script::{Reply, Script, ScriptStep};
use crate::support::session::Marker;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_api::ProjectPointOnSurf;
use martensite_tkgeombase::extrema::ExtAlgo;

/// The distances `xdistcs` measured and the largest one.
pub struct Distances {
    pub samples: Vec<(f64, f64)>,
    pub max_distance: f64,
}

/// `xdistcs curve surface t1 t2 nbpoints [tol]`: the distance from `nbpoints` curve
/// points to the surface, with `Error in <curve>:` before each one above `tol`.
pub fn xdistcs(
    d: &mut Script,
    curve: &str,
    surface: &str,
    range: (f64, f64),
    count: usize,
    tolerance: f64,
) -> ScriptStep<Distances> {
    xdistcs_aliased(
        d,
        (curve, curve),
        (surface, surface),
        range,
        count,
        tolerance,
    )
}

/// `xdistcs` called through script `upvar` aliases: `(alias, variable)` pairs; the command
/// prints the alias and reads the variable.
pub fn xdistcs_aliased(
    d: &mut Script,
    (curve_alias, curve): (&str, &str),
    (surface_alias, surface): (&str, &str),
    (t1, t2): (f64, f64),
    count: usize,
    tolerance: f64,
) -> ScriptStep<Distances> {
    let texts = [g17(t1), g17(t2), count.to_string(), g17(tolerance)];
    let args = [
        "xdistcs",
        curve_alias,
        surface_alias,
        &texts[0],
        &texts[1],
        &texts[2],
        &texts[3],
    ];
    let mut measured = None;
    d.command(&args, |d| {
        let curve_name = curve_alias;
        let c = d.curve(curve);
        let Some(c) = c.get() else {
            return Ok(Reply::ok(format!("Error: {curve_name} is not a curve!\n")));
        };
        let s = d.surface(surface);
        if s.is_null() {
            return Ok(Reply::ok(format!(
                "Error: {surface_alias} is not a surface!\n"
            )));
        }
        let mut projection = ProjectPointOnSurf::new();
        let mut text = String::new();
        let mut distances = Distances {
            samples: Vec::new(),
            max_distance: 0.,
        };
        let mut max_param = 0.;
        let step = (t2 - t1) / (count as f64 - 1.);
        for i in 0..count {
            let t = if i == count - 1 {
                t2
            } else {
                t1 + i as f64 * step
            };
            let p = c.d0(t)?;
            projection.init_tolerance(p, &s, 1.0e-7, ExtAlgo::Grad)?;
            if !projection.is_done() {
                text.push_str("Error: GeomAPI_ProjectPointOnSurf failed\n");
                return Ok(Reply::ok(text));
            }
            let distance = projection.lower_distance()?;
            if distance > tolerance {
                text.push_str(&format!("Error in {curve_name}:"));
            }
            text.push_str(&format!(" T={}\tD={}\n", g(t, 6), g(distance, 6)));
            d.marker(Marker::SpacePlus(p));
            if distance > distances.max_distance {
                (max_param, distances.max_distance) = (t, distance);
            }
            distances.samples.push((t, distance));
        }
        text.push_str(&format!(
            "Max distance = {}\nParam = {}\n",
            g17(distances.max_distance),
            g17(max_param)
        ));
        measured = Some(distances);
        Ok(Reply::ok(text))
    })?;
    Ok(measured.unwrap_or(Distances {
        samples: Vec::new(),
        max_distance: 0.,
    }))
}

/// `checksection name`: ` nb alone Vertices : n` of the vertices met an odd number of
/// times (none for a variable that is not a shape).
pub fn checksection(d: &mut Script, name: &str) -> ScriptStep<usize> {
    let mut alone = 0;
    d.command(&["checksection", name], |d| {
        let shape = d.shape(name);
        alone = alone_vertices(&shape)?;
        Ok(Reply::ok(format!(" nb alone Vertices : {alone}\n\n")))
    })?;
    Ok(alone)
}

fn alone_vertices(shape: &martensite_tkbrep::topods::Shape) -> Result<usize, Failure> {
    if shape.is_null() {
        return Ok(0);
    }
    let mut vertices: Vec<martensite_tkbrep::topods::Shape> = Vec::new();
    for vertex in crate::support::brep_commands::explode_all(shape, ShapeEnum::Vertex) {
        match vertices.iter().position(|v| v.is_same(&vertex)) {
            Some(index) => {
                vertices.remove(index);
            }
            None => vertices.push(vertex),
        }
    }
    Ok(vertices.len())
}

/// `checkreal name value expected tol_abs tol_rel`.
pub fn checkreal(
    d: &mut Script,
    name: &str,
    value: f64,
    expected: f64,
    tol_abs: f64,
    tol_rel: f64,
) {
    let (value_text, expected_text) = (g17(value), g17(expected));
    if (value - expected).abs() > tol_abs + tol_rel * expected.abs() {
        d.puts(&format!(
            "Error: {name} = {value_text} is not equal to expected {expected_text}"
        ));
    } else {
        d.puts(&format!(
            "Check of {name} OK: value = {value_text}, expected = {expected_text}"
        ));
    }
}

/// `checklength shape -l expected [-eps epsilon]`, with `expected` as written in the
/// script.
pub fn checklength(
    d: &mut Script,
    shape: &str,
    expected: &str,
    epsilon: Option<&str>,
) -> ScriptStep {
    let options = match epsilon {
        Some(eps) => format!("-l {expected} -eps {eps}"),
        None => format!("-l {expected}"),
    };
    d.puts(&format!("checklength {shape} {options}"));
    // script's `||` evaluates `whatis` only for a defined variable.
    if !isdraw(d, shape)? || whatis(d, shape)?.contains(&format!("{shape} is a \n")) {
        d.puts("Error: The command cannot be built");
        return Ok(());
    }
    let epsilon = epsilon.map_or(1.0e-4, script_real);
    let text = length(d, shape, Some(epsilon))?;
    let measured_text = text.trim_end().rsplit(' ').next().expect("length text");
    let (mass, m) = (script_real(expected), script_real(measured_text));
    if m == 0. {
        d.puts("Error : The command is not valid. The length is 0.");
    }
    if mass > 0. {
        d.puts(&format!("The expected length is {expected}"));
    }
    if (mass != 0. && (mass - m).abs() / mass > 0.01) || (mass == 0. && m != 0.) {
        d.puts(&format!(
            "Error : The length of result shape is {measured_text}"
        ));
    }
    Ok(())
}
