//! Literal camera projection and close-all commands retain five distinct mandatory captures.
use super::*;
use crate::case_view::viewer_session::DisplayRequest;
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::graphic3d::{
    Graphic3dCameraFocusType, Graphic3dCameraIodType, Graphic3dCameraProjection as Projection,
};
use martensite_tkv3d::v3d::V3dStereoDumpOptions as Stereo;

/// Run every authored camera dump and genuine namespace retirement; `camera_dumps()?`.
pub(crate) fn camera_dumps() -> Outcome {
    banner(&["========", "OCC24001: Camera dump test", "========"]);
    let shape = crate::primitive_shapes::box_at([0.; 3], [1., 2., 3.])?;
    let mut scene = NamedViewerScene::new(driver_factory::QualifiedVulkanFactory);
    initialize_box(&mut scene, "View1", &shape)?;
    configure_camera(&scene)?;
    scene
        .active_session()?
        .set_projection(Projection::Orthographic)?;
    scene.active_session()?.fit()?;
    let mut errors = Vec::new();
    let mut captured = stereo_capture::first(scene, "bug24001_ortho.png", &mut errors)?;
    initial_dumps(&mut captured, &mut errors)?;
    reopen_and_dump(&mut captured, &shape, &mut errors)?;
    stereo_capture::finish(captured, errors)
}

pub(super) fn initialize_box<F: SourceDriverFactory>(
    scene: &mut NamedViewerScene<F>,
    name: &str,
    shape: &Shape,
) -> Outcome {
    if name.is_empty() {
        scene.initialize_default()?;
    } else {
        scene.initialize(name)?;
    }
    scene
        .active_session_mut()?
        .display("b", shape, &DisplayRequest::default())?;
    Ok(())
}

pub(super) fn configure_camera<F: SourceDriverFactory>(scene: &NamedViewerScene<F>) -> Outcome {
    let camera = scene.current.camera();
    let camera = camera
        .get()
        .ok_or("source camera parameters require actual current camera")?;
    camera.set_fov_y(45.);
    camera.set_iod(Graphic3dCameraIodType::Relative, 0.05);
    camera.set_z_focus(Graphic3dCameraFocusType::Relative, 1.);
    scene.current.redraw();
    Ok(())
}

fn initial_dumps(
    captured: &mut stereo_capture::CapturedScene,
    errors: &mut Vec<Box<dyn Error>>,
) -> Outcome {
    captured
        .scene
        .active_session()?
        .set_projection(Projection::Perspective)?;
    captured.scene.active_session()?.fit()?;
    stereo_capture::additional(captured, "bug24001_persp.png", Stereo::Mono, errors)?;
    let session = captured.scene.active_session()?;
    session.set_projection(Projection::Stereo)?;
    stereo_capture::additional(captured, "bug24001_stereoR.png", Stereo::RightEye, errors)?;
    stereo_capture::additional(captured, "bug24001_stereoL.png", Stereo::LeftEye, errors)
}

fn reopen_and_dump(
    captured: &mut stereo_capture::CapturedScene,
    shape: &Shape,
    errors: &mut Vec<Box<dyn Error>>,
) -> Outcome {
    // Setting stereo changes only the current camera, not future driver caps.
    let session = captured.scene.active_session()?;
    session.set_projection(Projection::Stereo)?;
    captured.scene.close_all()?;
    initialize_box(&mut captured.scene, "", shape)?;
    let session = captured.scene.active_session()?;
    session.set_projection(Projection::Stereo)?;
    captured.scene.active_session()?.fit()?;
    stereo_capture::additional(
        captured,
        "bug24001_afterSwitch.png",
        Stereo::RightEye,
        errors,
    )
}
