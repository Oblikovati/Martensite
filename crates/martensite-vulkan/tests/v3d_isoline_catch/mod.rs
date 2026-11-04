#[cfg(test)]
mod cpu;
use crate::dataset_shape::read_dataset_surface;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use std::collections::BTreeMap;
type BoxError = Box<dyn std::error::Error>;
const HEADING: &[&str] = &[
    "TODO OCC11111 ALL: An exception was caught",
    "TODO OCC11111 ALL: Faulty OCC22910",
    "TODO OCC11111 ALL: Error: The command cannot be built",
    "================",
    "0022910: F a i l u r e to compute iso-line for NURBS surface",
    "================",
    "",
];

fn actual_caught_isoline(
    surface: &Handle<dyn GeomSurface>,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    let authored_u = 0.5;
    surface.u_iso(authored_u)
}

fn report_isoline_catch(result: &Result<Handle<dyn GeomCurve>, Failure>) -> &'static str {
    match result {
        Ok(_) => "OK OCC22910",
        Err(_) => "Faulty OCC22910",
    }
}

fn missing_result_checkprops(bindings: &BTreeMap<&str, Handle<dyn GeomCurve>>) {
    println!("checkprops result -s 0");
    if !bindings.contains_key("result") {
        println!("Error: The command cannot be built");
    }
}

fn empty_result_view() -> Result<(), BoxError> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump("bug22910_1.png")?;
    run.finish()
}

/// Preserve the actual UIso catch and the authored u/result namespace distinction.
/// Example: `isoline_catch_view()?`.
pub(crate) fn isoline_catch_view() -> Result<(), BoxError> {
    banner(HEADING);
    let surface = read_dataset_surface("s.rle")?;
    let result = actual_caught_isoline(&surface);
    if let Err(error) = &result {
        println!("An exception was caught: {error}");
    }
    println!("{}", report_isoline_catch(&result));
    let mut bindings = BTreeMap::new();
    if let Ok(curve) = result {
        bindings.insert("u", curve);
    }
    missing_result_checkprops(&bindings);
    let outcome = empty_result_view();
    drop(bindings);
    outcome
}
