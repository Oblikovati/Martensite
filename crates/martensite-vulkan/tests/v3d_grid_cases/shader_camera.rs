//! The shader grid under camera changes: projection, pan, rotation, plane and zoom adaptation.
use super::shader::stepped;
use super::*;
use crate::case_view::viewer_session::{GridLayout, ShaderGridRequest};
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;

/// The second grid of ortho/persp: background, no axes, blue, offset origin, adaptive, thin.
fn styled() -> Result<ShaderGridRequest, Box<dyn Error>> {
    Ok(ShaderGridRequest {
        background: Some(true),
        draw_axis: Some(false),
        color: Some(Color::new(0., 0., 1., ColorSpace::Rgb)?),
        layout: GridLayout {
            origin: Some((1., 1.)),
            ..GridLayout::default()
        },
        view_adaptive: Some(true),
        line_thickness: Some(0.05),
        ..ShaderGridRequest::default()
    })
}

fn projected(
    projection: Graphic3dCameraProjection,
    images: [&'static str; 2],
    title: &str,
) -> Outcome {
    banner(
        &["0030979: Visualization - shader-based GPU grid", title],
        50,
    );
    let (driver, mut session) = axonometric_view(DEFAULT)?;
    shaded_box(&mut session, [1., 2., 3.])?;
    session.set_projection(projection)?;
    session.fit()?;
    session.zoom(0.1)?;
    session.show_shader_grid(&ShaderGridRequest::default())?;
    let mut run = source_output::capture_case(driver, session, images[0])?;
    run.scene.hide_grid()?;
    run.scene.show_shader_grid(&styled()?)?;
    run.capture_additional(images[1])?;
    finish(run)
}

/// Default and styled shader grids in orthographic projection; `ortho()?`.
pub(crate) fn ortho() -> Outcome {
    projected(
        Graphic3dCameraProjection::Orthographic,
        ["ortho_0.png", "ortho_1.png"],
        "Orthographic projection",
    )
}

/// Default and styled shader grids in perspective projection; `persp()?`.
pub(crate) fn persp() -> Outcome {
    projected(
        Graphic3dCameraProjection::Perspective,
        ["persp_0.png", "persp_1.png"],
        "Perspective projection",
    )
}

fn background_grid() -> ShaderGridRequest {
    ShaderGridRequest {
        background: Some(true),
        ..ShaderGridRequest::default()
    }
}

/// Write the shader block the view submits for its grid next to the capture `image`
/// (`<image>.grid.json`), the geometry from which the harness proves exact axis ties
/// (the recording tool); `write_grid_geometry(&run.scene, "a.png")?`.
fn write_grid_geometry(session: &ViewerSession, image: &str) -> Outcome {
    use martensite_tkservice::graphic3d::{
        Graphic3dCView as _, Graphic3dDataStructureManager as _,
    };
    let source = session.view().get().ok_or("a V3d view")?;
    let view: Handle<martensite_vulkan::Graphic3dRenderView> = source.view().cast();
    let camera = view.camera();
    let camera = camera.get().ok_or("a view camera")?;
    let window = view.window();
    let (width, height) = window.get().ok_or("a view window")?.size();
    let block = view
        .shader_grid()
        .uniforms(camera, &camera.orientation_matrix_f());
    let list = |values: &mut dyn Iterator<Item = String>| values.collect::<Vec<_>>().join(", ");
    let rows = list(
        &mut block
            .rows
            .iter()
            .map(|row| format!("[{}]", list(&mut row.iter().map(|v| format!("{v:?}"))))),
    );
    let flags = list(
        &mut block
            .flags
            .iter()
            .map(|row| format!("[{}]", list(&mut row.iter().map(|v| v.to_string())))),
    );
    let inverse = list(&mut block.projection_inverse.iter().map(|v| format!("{v:?}")));
    let json = format!(
        "{{\"projection_inverse\": [{inverse}], \"rows\": [{rows}], \"flags\": [{flags}], \"size\": [{width}, {height}]}}\n"
    );
    std::fs::write(
        source_output::capture_path(&format!("{image}.grid.json"))?,
        json,
    )?;
    Ok(())
}

/// The background shader grid stays fixed while the view pans; `gpu_pan()?`.
pub(crate) fn gpu_pan() -> Outcome {
    banner(&["0030979: shader GPU grid stays fixed during panning"], 54);
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_shader_grid(&background_grid())?;
    let mut run = source_output::capture_case(driver, session, "gpu_pan_before.png")?;
    write_grid_geometry(&run.scene, "gpu_pan_before.png")?;
    run.scene.pan(80, 40)?;
    run.capture_additional("gpu_pan_after.png")?;
    write_grid_geometry(&run.scene, "gpu_pan_after.png")?;
    finish(run)
}

/// The background shader grid stays fixed while the view rotates; `gpu_rotate()?`.
pub(crate) fn gpu_rotate() -> Outcome {
    banner(
        &["0030979: shader GPU grid stays fixed during rotation"],
        55,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.show_shader_grid(&background_grid())?;
    let mut run = source_output::capture_case(driver, session, "gpu_rotate_before.png")?;
    write_grid_geometry(&run.scene, "gpu_rotate_before.png")?;
    run.scene.rotate(0., 0., 0.5)?;
    run.capture_additional("gpu_rotate_after.png")?;
    write_grid_geometry(&run.scene, "gpu_rotate_after.png")?;
    finish(run)
}

fn bounded_without_axes() -> ShaderGridRequest {
    ShaderGridRequest {
        size: Some((100., 100.)),
        draw_axis: Some(false),
        ..ShaderGridRequest::default()
    }
}

/// A bounded grid in perspective, before and after rotation; `gpu_perspective_nomirror()?`.
pub(crate) fn gpu_perspective_nomirror() -> Outcome {
    banner(
        &["GPU shader grid keeps foreground plane semantics in perspective view"],
        66,
    );
    let (driver, mut session) = axonometric_view(SQUARE_400)?;
    shaded_box(&mut session, [10., 10., 10.])?;
    session.fit()?;
    session.show_shader_grid(&bounded_without_axes())?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    let mut run =
        source_output::capture_case(driver, session, "gpu_perspective_nomirror_persp.png")?;
    run.scene.rotate(0., 0., 0.5)?;
    run.capture_additional("gpu_perspective_nomirror_rotated.png")?;
    finish(run)
}

/// The shader grid follows a YZ privileged plane; `gpu_plane()?`.
pub(crate) fn gpu_plane() -> Outcome {
    banner(
        &[
            "0030979: shader GPU grid follows non-default privileged",
            "plane (YZ plane: normal +X).",
        ],
        59,
    );
    let (driver, session) = boxed_scene(DEFAULT, [1., 2., 3.], 0.2)?;
    session.set_privileged_plane([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let green = ShaderGridRequest {
        color: Some(Color::new(0.2, 0.8, 0.3, ColorSpace::Rgb)?),
        ..ShaderGridRequest::default()
    };
    session.show_shader_grid(&green)?;
    let run = source_output::capture_case(driver, session, "gpu_plane_yz.png")?;
    finish(run)
}

/// The shader grid shows in an empty view and after a display without fit; `gpu_visibility()?`.
pub(crate) fn gpu_visibility() -> Outcome {
    banner(
        &["GPU shader grid is visible without scene objects and without vfit"],
        66,
    );
    let (driver, session) = axonometric_view(SQUARE_400)?;
    session.show_shader_grid(&bounded_without_axes())?;
    let mut run = source_output::capture_case(driver, session, "gpu_visibility_empty.png")?;
    shaded_box(&mut run.scene, [10., 10., 10.])?;
    run.capture_additional("gpu_visibility_display_no_vfit.png")?;
    finish(run)
}

/// View-adaptive cell spacing through zoom out, zoom in and a polar grid; `view_adaptive()?`.
pub(crate) fn view_adaptive() -> Outcome {
    banner(
        &["GPU grid -viewAdaptive 1: cell spacing tracks camera zoom"],
        66,
    );
    let adaptive = ShaderGridRequest {
        view_adaptive: Some(true),
        ..ShaderGridRequest::default()
    };
    let (driver, mut session) = axonometric_view(SQUARE_400)?;
    shaded_box(&mut session, [1., 1., 1.])?;
    session.fit()?;
    session.show_shader_grid(&adaptive)?;
    let mut run = source_output::capture_case(driver, session, "view_adaptive_fit.png")?;
    run.scene.zoom(0.1)?;
    run.capture_additional("view_adaptive_zoom_out.png")?;
    run.scene.zoom(100.)?;
    run.capture_additional("view_adaptive_zoom_in.png")?;
    run.scene.zoom(0.0001)?;
    run.scene.show_shader_grid(&ShaderGridRequest {
        view_adaptive: Some(true),
        ..stepped((0.5, 24.), None)
    })?;
    run.capture_additional("view_adaptive_circ.png")?;
    finish(run)
}
