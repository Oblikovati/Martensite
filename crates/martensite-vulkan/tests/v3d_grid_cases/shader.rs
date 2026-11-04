//! The shader grid's rectangular, polar, bounded and option variants.
use super::*;
use crate::case_view::viewer_session::{GridLayout, ShaderGridRequest};
use martensite_tkservice::aspect::{AspectGridDrawMode, AspectGridType};

/// A shader grid with `step` and optional rotation; `stepped((0.5, 0.5), None)`.
pub(super) fn stepped(step: (f64, f64), rotation: Option<f64>) -> ShaderGridRequest {
    let layout = GridLayout {
        step: Some(step),
        rotation,
        ..GridLayout::default()
    };
    ShaderGridRequest {
        layout,
        ..ShaderGridRequest::default()
    }
}

/// Square, anisotropic and rotated rectangular shader grids; `rect_gpu()?`.
pub(crate) fn rect_gpu() -> Outcome {
    banner(
        &["0030979: vgrid -type gpu renders the rectangular grid via the shader"],
        66,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_shader_grid(&stepped((0.5, 0.5), Some(0.0)))?;
    let mut run = source_output::capture_case(driver, session, "rect_gpu_rect.png")?;
    run.scene.show_shader_grid(&stepped((0.5, 1.0), None))?;
    run.capture_additional("rect_gpu_rect_aniso.png")?;
    run.scene
        .show_shader_grid(&stepped((0.5, 0.5), Some(0.5)))?;
    run.capture_additional("rect_gpu_rect_rotated.png")?;
    finish(run)
}

/// Polar shader grids bounded by a radius; `circ_gpu()?`.
pub(crate) fn circ_gpu() -> Outcome {
    banner(
        &["0030979: vgrid -type gpu with circular options uses the polar shader"],
        66,
    );
    let polar = |step, radius, rotation| ShaderGridRequest {
        radius: Some(radius),
        ..stepped(step, rotation)
    };
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_shader_grid(&polar((0.5, 8.), 5.0, None))?;
    let mut run = source_output::capture_case(driver, session, "circ_gpu_circ.png")?;
    run.scene.show_shader_grid(&polar((1.0, 24.), 10.0, None))?;
    run.capture_additional("circ_gpu_circ_fine.png")?;
    run.scene
        .show_shader_grid(&polar((0.5, 12.), 5.0, Some(0.4)))?;
    run.capture_additional("circ_gpu_circ_rotated.png")?;
    finish(run)
}

/// Rotation, points and a wedge on the shader grid; `gpu_options()?`.
#[allow(clippy::approx_constant)] // The original passes these literal arc bounds.
pub(crate) fn gpu_options() -> Outcome {
    banner(
        &["0030979: vgrid -type gpu keeps step, mode, rotation and disc/wedge"],
        66,
    );
    let (driver, session) = boxed_scene(SQUARE_400, [2., 2., 0.2], 0.25)?;
    session.show_shader_grid(&stepped((0.5, 1.0), Some(0.4)))?;
    let mut run = source_output::capture_case(driver, session, "gpu_options_rotated.png")?;
    let points = ShaderGridRequest {
        mode: Some(AspectGridDrawMode::Points),
        ..stepped((0.5, 1.0), None)
    };
    run.scene.show_shader_grid(&points)?;
    run.capture_additional("gpu_options_points.png")?;
    let sector = ShaderGridRequest {
        radius: Some(2.0),
        arc: Some((0.0, 1.5707)),
        ..stepped((0.5, 0.5), None)
    };
    run.scene.show_shader_grid(&sector)?;
    run.capture_additional("gpu_options_sector.png")?;
    finish(run)
}

/// Bounded rectangular shader grids, including a Z offset; `bounded_rect()?`.
pub(crate) fn bounded_rect() -> Outcome {
    banner(
        &["0030979: Bounded rectangular GPU grid (-size X Y clips the area)"],
        66,
    );
    let bounded = |size, z_offset| ShaderGridRequest {
        kind: Some(AspectGridType::Rectangular),
        size: Some(size),
        z_offset,
        ..stepped((0.5, 0.5), None)
    };
    let (driver, session) = boxed_scene(SQUARE_400, [3., 3., 0.1], 0.5)?;
    session.show_shader_grid(&bounded((4., 4.), None))?;
    let mut run = source_output::capture_case(driver, session, "bounded_rect_full_square.png")?;
    run.scene.show_shader_grid(&bounded((4., 8.), None))?;
    run.capture_additional("bounded_rect_strip_x.png")?;
    run.scene
        .show_shader_grid(&bounded((4., 4.), Some(-0.01)))?;
    run.capture_additional("bounded_rect_zoffset.png")?;
    finish(run)
}

/// A bounded polar shader grid: disc, half, quarter and wrap-around arcs; `bounded_circ()?`.
#[allow(clippy::approx_constant)] // The original passes these literal arc bounds.
pub(crate) fn bounded_circ() -> Outcome {
    banner(
        &["0030979: Bounded circular GPU grid (-radius, -arc support)"],
        66,
    );
    let disc = |arc| ShaderGridRequest {
        kind: Some(AspectGridType::Circular),
        radius: Some(2.5),
        arc,
        ..stepped((0.25, 16.), None)
    };
    let (driver, session) = boxed_scene(SQUARE_400, [1., 1., 0.1], 0.3)?;
    session.show_shader_grid(&disc(None))?;
    let mut run = source_output::capture_case(driver, session, "bounded_circ_disc.png")?;
    run.scene.show_shader_grid(&disc(Some((0.0, 3.14159))))?;
    run.capture_additional("bounded_circ_halfcircle.png")?;
    run.scene.show_shader_grid(&disc(Some((0.0, 1.5707))))?;
    run.capture_additional("bounded_circ_quarter.png")?;
    run.scene.show_shader_grid(&disc(Some((2.356, -2.356))))?;
    run.capture_additional("bounded_circ_wraparound.png")?;
    finish(run)
}
