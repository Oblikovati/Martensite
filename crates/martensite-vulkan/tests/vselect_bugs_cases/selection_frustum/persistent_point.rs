//! The actual ranked point is rounded through source %g text before marker construction.
use super::*;

/// Whole bug26304, including row printing and both named-pixel checks; `persistent_ranked_points()?`.
pub(crate) fn persistent_ranked_points() -> Outcome {
    banner(&[
        "============",
        "0026304: Visualization - wrong calculation of point in SelectMgrRectangularFrustum::DetectedPoint",
        "============",
        "",
    ]);
    // VClear precedes VInit, so the isolated worker has no existing context to clear.
    let mut run = SessionRun::new()?;
    frustum_persistent_scene(run.session())?;
    frustum_ranked_marker(run.session(), (20, 200), "p1", &["b", "bz"])?;
    frustum_ranked_marker(run.session(), (48, 258), "p", &["b", "bz", "p1"])?;
    frustum_persistent_camera(run.session())?;
    frustum_point_pixels(run.session())?;
    frustum_dump_and_finish(run)
}

/// Fit the plain box before displaying the anchored transparent one; `frustum_persistent_scene(session)?`.
pub(super) fn frustum_persistent_scene(session: &mut ViewerSession) -> Outcome {
    let b = box_sized([100.; 3])?;
    let bz = box_sized([50.; 3])?;
    session.display("b", &b, &DisplayRequest::default().with_display_mode(1))?;
    session.fit()?;
    let request = DisplayRequest::zoom_persistent([0., -10., 50.]).with_display_mode(1);
    session.display("bz", &bz, &request)?;
    session.set_color("bz", Color::named(NamedColor::GREEN))?;
    session.set_transparency("bz", 0.5)?;
    Ok(())
}

/// Construct a real point from the printed ranked Point tokens; `frustum_ranked_marker(session, (20, 200), "p1", &["b", "bz"])?`.
pub(super) fn frustum_ranked_marker(
    session: &mut ViewerSession,
    pixel: (i32, i32),
    name: &str,
    names: &[&str],
) -> Outcome {
    session.select_point(pixel.0, pixel.1)?;
    let bindings = state_rows::frustum_bindings(session, names)?;
    let row = state_rows::frustum_entity_row(session, &bindings, 1)?;
    println!("{row}"); // findPoint prints precisely the row whose Point fields it consumes.
    let point = state_rows::frustum_point_from_row(&row)?;
    session.display_object(name, point_object(point)?)?;
    Ok(())
}

/// Apply the source camera before its yellow-pixel predicates; `frustum_persistent_camera(session)?`.
pub(super) fn frustum_persistent_camera(session: &ViewerSession) -> Result<(), Failure> {
    let parameters = ViewParameters {
        scale: Some(22.3),
        proj: Some([0.578415, -0.780275, 0.237924]),
        up: Some([-0.212025, 0.137838, 0.967495]),
        at: Some([10.887, 16.807, 36.2306]),
        ..ViewParameters::default()
    };
    session.set_view_parameters(&parameters)
}

fn frustum_point_pixels(session: &ViewerSession) -> Outcome {
    for (x, y) in [(103, 293), (29, 24)] {
        if session.pixel_color_name(x, y)? != "YELLOW" {
            println!("ERROR: Wrong point calculated");
        }
    }
    Ok(())
}

fn frustum_dump_and_finish(mut run: SessionRun) -> Outcome {
    run.dump("bug26304.png")?;
    let enabled = std::env::var_os("MARTENSITE_SELECTED_BOUNDARY_CONTROL").is_some();
    let observed = controls::selected_boundary::capture_selected_boundary(run.session(), enabled);
    let finished = run.finish(); // Observer refusal cannot bypass the original END/retirement.
    match (observed, finished) {
        (Err(observer), Err(end)) => Err(std::io::Error::other(format!(
            "selected boundary observer failed: {observer}; original END failed: {end}"
        ))
        .into()),
        (Err(error), _) | (_, Err(error)) => Err(error),
        _ => Ok(()),
    }
}
