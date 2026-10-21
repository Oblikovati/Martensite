//! `perf_modalg` case bodies ("GeomAPI_ExtremaCurveCurve hangs on parallel b-spline
//! curves").
use crate::cases::script::banner;
use crate::support::commands::extrema::extrema;
use crate::support::commands::topology::mkcurve;
use crate::support::process::cpulimit;
use crate::support::restore::restore;
use crate::support::script::{Reply, Script, ScriptStep};

/// The extrema of the parallel B-spline edges under `cpulimit 20`, timed by `dchrono`.
fn parallel_extrema(d: &mut Script, suffix: &str) -> ScriptStep {
    banner(d, "============", "OCC26184");
    restore(d, &format!("bug26184_Curve_Extrema_1_{suffix}.brep"), "a1")?;
    restore(d, &format!("bug26184_Curve_Extrema_2_{suffix}.brep"), "a2")?;
    mkcurve(d, "c1", "a1")?;
    mkcurve(d, "c2", "a2")?;
    d.command(&["cpulimit", "20"], |d| {
        cpulimit(20);
        d.cout("CPU and elapsed time limit set to 20 seconds\n");
        Ok(Reply::ok(""))
    })?;
    d.command(&["dchrono", "h", "restart"], |_| Ok(Reply::ok("")))?;
    let started = std::time::Instant::now();
    extrema(d, "c1", "c2")?;
    let seconds = started.elapsed().as_secs_f64();
    d.command(&["dchrono", "h", "stop", "counter", "extrema"], |_| {
        Ok(Reply::ok(format!("COUNTER extrema: {seconds}\n")))
    })?;
    Ok(())
}

pub fn bug26184_1(d: &mut Script) -> ScriptStep {
    parallel_extrema(d, "12971")
}

pub fn bug26184_2(d: &mut Script) -> ScriptStep {
    parallel_extrema(d, "13767")
}
