//! Shared owned operation dispatch for the CLI and direct numeric corpus tests.
mod common;
pub mod helix_brep;
pub mod helix_geom;
pub mod helix_geom_builders;
pub mod placement;
mod reader;
mod sink;

/// Execute one numeric request without process I/O; `execute_request("hx_coil 0")`.
pub fn execute_request(line: &str) -> Result<String, String> {
    let mut fields = line.split_whitespace();
    let Some(operation) = fields.next() else {
        return Ok(String::new());
    };
    let values: Vec<f64> = fields
        .map(|field| field.parse().expect("numeric record field"))
        .collect();
    dispatch(operation, &values)
}

fn dispatch(operation: &str, values: &[f64]) -> Result<String, String> {
    let mut out = sink::Sink::default();
    match operation {
        "hx_placement" => placement::placement_case(&mut out, values),
        "hx_curve" => helix_geom::curve_case(&mut out, values),
        "hx_appr_curve3d" => helix_geom::appr_curve3d_case(&mut out, values),
        "hx_appr_helix" => helix_geom::appr_helix_case(&mut out, values),
        "hx_coil" => helix_geom_builders::coil_case(&mut out, values),
        "hx_helix" => helix_geom_builders::helix_case(&mut out, values),
        "hxb_build" => helix_brep::build_case(&mut out, values),
        _ => return Err(format!("Unknown probe operation: {operation}")),
    }
    Ok(out.finish())
}
