//! The viewer's CPU grid: rectangular and circular, lines and points.
use super::*;
use crate::case_view::viewer_session::{GridLayout, GridRequest, ShaderGridRequest};
use martensite_tkservice::aspect::AspectGridDrawMode::{Lines, Points};

fn rect(step: (f64, f64), rotation: Option<f64>) -> GridRequest {
    GridRequest::rectangular(GridLayout {
        step: Some(step),
        rotation,
        ..GridLayout::default()
    })
}

fn circ(step: (f64, f64), rotation: Option<f64>) -> GridRequest {
    GridRequest::circular(GridLayout {
        step: Some(step),
        rotation,
        ..GridLayout::default()
    })
}

/// Rectangular grid: square, anisotropic, points and rotated; `rect_cpu()?`.
pub(crate) fn rect_cpu() -> Outcome {
    banner(
        &["Restored CPU rectangular grid path: V3dRectangularGrid + ActivateGrid"],
        66,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_grid(&rect((0.5, 0.5), Some(0.0)))?;
    let mut run = source_output::capture_case(driver, session, "rect_cpu_rect.png")?;
    run.scene.show_grid(&rect((0.5, 1.0), None))?;
    run.capture_additional("rect_cpu_rect_aniso.png")?;
    run.scene
        .show_grid(&rect((0.5, 0.5), None).drawn_as(Points))?;
    run.capture_additional("rect_cpu_rect_points.png")?;
    run.scene
        .show_grid(&rect((0.5, 0.5), Some(0.5)).drawn_as(Lines))?;
    run.capture_additional("rect_cpu_rect_rotated.png")?;
    finish(run)
}

/// Circular grid: coarse, fine, rotated and points; `circ_cpu()?`.
pub(crate) fn circ_cpu() -> Outcome {
    banner(
        &["Restored CPU circular grid path: V3dCircularGrid + ActivateGrid"],
        66,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_grid(&circ((0.5, 8.), None))?;
    let mut run = source_output::capture_case(driver, session, "circ_cpu_circ.png")?;
    run.scene.show_grid(&circ((1.0, 24.), None))?;
    run.capture_additional("circ_cpu_circ_fine.png")?;
    run.scene.show_grid(&circ((0.5, 12.), Some(0.4)))?;
    run.capture_additional("circ_cpu_circ_rotated.png")?;
    run.scene
        .show_grid(&circ((0.5, 8.), None).drawn_as(Points))?;
    run.capture_additional("circ_cpu_circ_points.png")?;
    finish(run)
}

/// Points at rectangular and circular intersections (OCC30979); `rect_points()?`.
pub(crate) fn rect_points() -> Outcome {
    banner(
        &["0030979: Aspect_GDM_Points renders dots at rectangular grid intersections"],
        66,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_grid(&rect((0.5, 0.5), None).drawn_as(Lines))?;
    let mut run = source_output::capture_case(driver, session, "rect_points_lines.png")?;
    run.scene
        .show_grid(&rect((0.5, 0.5), None).drawn_as(Points))?;
    run.capture_additional("rect_points_points.png")?;
    run.scene
        .show_grid(&circ((0.5, 8.), None).drawn_as(Points))?;
    run.capture_additional("rect_points_circ_points.png")?;
    finish(run)
}

/// CPU grid, then the shader grid, then the CPU grid again in one view; `mode_switch()?`.
pub(crate) fn mode_switch() -> Outcome {
    banner(
        &["Toggle between CPU grid and GPU shader grid in the same view"],
        66,
    );
    let (driver, session) = boxed_scene(SQUARE_400, [1., 2., 3.], 0.2)?;
    session.show_grid(&rect((0.5, 0.5), None))?;
    let mut run = source_output::capture_case(driver, session, "mode_switch_cpu.png")?;
    let shader = ShaderGridRequest {
        layout: GridLayout {
            step: Some((0.5, 0.5)),
            ..GridLayout::default()
        },
        ..ShaderGridRequest::default()
    };
    run.scene.show_shader_grid(&shader)?;
    run.capture_additional("mode_switch_shader.png")?;
    run.scene.show_grid(&rect((0.5, 0.5), None))?;
    run.capture_additional("mode_switch_cpu_again.png")?;
    finish(run)
}
