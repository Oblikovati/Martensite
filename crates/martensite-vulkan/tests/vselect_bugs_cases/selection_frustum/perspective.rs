//! Both complete perspective-grid cases keep their different camera and query order.
use super::grid::{display_frustum_grid, frustum_prefix_matches, frustum_selected_names};
use super::*;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;

const RECTANGLE_NAMES: &[&str] = &[
    "55", "6", "69", "18", "13", "67", "54", "68", "19", "12", "62", "5", "53", "60", "4", "61",
    "20", "11",
];
const POLYGON_NAMES: &[&str] = &[
    "67", "97", "47", "32", "12", "33", "49", "74", "54", "8", "50", "19", "9", "63", "83", "56",
    "39", "26", "66", "75", "18", "95", "81", "15", "89", "51", "46", "76", "57", "90", "82", "58",
    "65", "10", "11", "25", "62", "59", "14", "52", "40", "60", "68", "16", "55", "7", "64", "61",
    "88", "69", "53", "17", "96",
];
const REGION_POLYGON: &[(i32, i32)] = &[
    (70, 70),
    (0, 600),
    (200, 600),
    (200, 200),
    (600, 200),
    (600, 70),
];

const REGION_BANNER: &[&str] = &[
    "============",
    "OCC25230: Visualization - Test case for selection in perspective views",
    "Test is made using comparison of selected points with expected to be selected points.",
    "In case of mismatching test will report about an e r r o r of certain type:",
    "point selection, rectangle selection or polygon selection is broken.",
    "============",
    "",
];

const FRONT_BANNER: &[&str] = &[
    "============",
    "OCC25265: Perspective projection - selecting front point of two.",
    "When trying to select front point of two (lying on Z-coordinate) in a perspective",
    "view, the back one is snapped and selected.",
    "============",
    "",
];

/// Whole bug25230, including three independent late-reported comparisons; `perspective_grid_regions()?`.
pub(crate) fn perspective_grid_regions() -> Outcome {
    banner(REGION_BANNER);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    let bindings = display_frustum_grid(run.session())?;
    grid::frustum_region_camera(run.session())?;
    let checks = frustum_region_checks(run.session(), &bindings)?;
    for line in frustum_region_errors(checks) {
        println!("{line}");
    }
    run.dump("bug25230.png")?;
    run.finish()
}

/// Complete all three real queries before returning their source flags; `frustum_region_checks(session, bindings)?`.
pub(super) fn frustum_region_checks(
    session: &mut ViewerSession,
    bindings: &FrustumBindings,
) -> Result<[bool; 3], Failure> {
    session.select_pixels(&[(200, 200), (0, 0)], false)?;
    let rectangle =
        frustum_prefix_matches(&frustum_selected_names(session, bindings)?, RECTANGLE_NAMES);
    session.select_point(0, 0)?;
    session.select_point(27, 28)?;
    let point = frustum_selected_names(session, bindings)?
        .first()
        .is_some_and(|name| name == "6");
    session.select_pixels(REGION_POLYGON, false)?;
    let polygon =
        frustum_prefix_matches(&frustum_selected_names(session, bindings)?, POLYGON_NAMES);
    Ok([rectangle, point, polygon])
}

/// Keep independent source diagnostics in rectangle/point/polygon order; `frustum_region_errors([true; 3])`.
pub(super) fn frustum_region_errors(checks: [bool; 3]) -> Vec<String> {
    checks
        .into_iter()
        .zip(["rectangle", "point", "polygon"])
        .filter(|(passed, _)| !passed)
        .map(|(_, kind)| format!("Error : Perspective selection by {kind} is broken."))
        .collect()
}

/// Whole bug25265, including the post-selection dump camera; `perspective_front_point()?`.
pub(crate) fn perspective_front_point() -> Outcome {
    banner(FRONT_BANNER);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    let bindings = display_frustum_grid(run.session())?;
    grid::frustum_front_camera(run.session())?;
    run.session().select_point(204, 204)?;
    if !frustum_prefix_matches(&frustum_selected_names(run.session(), &bindings)?, &["24"]) {
        println!("Error : Perspective selection is broken.");
    }
    frustum_final_camera(run.session())?;
    run.dump("bug25265.png")?;
    run.finish()
}

/// Apply the complete camera after the front-point comparison; `frustum_final_camera(session)?`.
pub(super) fn frustum_final_camera(session: &ViewerSession) -> Result<(), Failure> {
    let parameters = ViewParameters {
        scale: Some(48.20),
        up: Some([-0.09, 0.94, -0.33]),
        at: Some([3.03, 2.70, 5.10]),
        eye: Some([16.40, 10.98, 24.59]),
        ..ViewParameters::default()
    };
    session.set_view_parameters(&parameters)
}
