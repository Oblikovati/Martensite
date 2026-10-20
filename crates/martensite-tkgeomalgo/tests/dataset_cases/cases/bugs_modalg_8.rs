//! `bugs_modalg_8` case bodies.
use crate::cases::script::banner;
use crate::support::commands::geometry::plane;
use crate::support::commands::planar::{intersect2d, project};
use crate::support::commands::topology::mkcurve;
use crate::support::commands::views::{checkview_screenshot, fit2d, v2d};
use crate::support::restore::restore;
use crate::support::script::{Script, ScriptStep};

pub fn bug32915(d: &mut Script) -> ScriptStep {
    banner(
        d,
        "=================================================",
        "0032915: Geom2dAPI_InterCurveCurve, The algorithm lost an intersection point.",
    );
    restore(d, "bug32915.brep", "e")?;
    mkcurve(d, "c3d", "e")?;
    plane(d, "p", &[0., 0., 1.])?;
    project(d, "c2d", "c3d", "p")?;
    let inter = intersect2d(d, &["c2d"], None, false)?;
    if inter.matches("Intersection point").count() != 29 {
        d.puts("Error: Wrong number of self-intersections (expected 29 points).");
    }
    v2d(d)?;
    fit2d(d)?;
    checkview_screenshot(d, "bug32915")
}
