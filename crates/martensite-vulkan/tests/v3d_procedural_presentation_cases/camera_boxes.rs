//! The frustum is computed from the active camera, then viewed from two later directions.
use super::*;
use crate::primitive_shapes::box_at;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkv3d::{ais::AisCameraFrustum, v3d::V3dTypeOfOrientation};
const BANNER: &[&str] = &[
    "=============",
    "0030412: Visualization, TKV3d - add presentation of camera frustum",
    "=============",
];

/// Display the source lattice under one deferred-update scope; `authored_boxes(session)?`.
pub(super) fn authored_boxes(session: &mut ViewerSession) -> Result<(), Failure> {
    session.without_updates(|session| {
        for i in 0..5 {
            display_lattice_plane(session, i)?;
        }
        Ok(())
    })
}
fn display_lattice_plane(session: &mut ViewerSession, i: i32) -> Result<(), Failure> {
    for j in 0..5 {
        display_lattice_row(session, i, j)?;
    }
    Ok(())
}
fn display_lattice_row(session: &mut ViewerSession, i: i32, j: i32) -> Result<(), Failure> {
    for k in 0..5 {
        display_lattice_box(session, [i, j, k])?;
    }
    Ok(())
}

fn display_lattice_box(session: &mut ViewerSession, [i, j, k]: [i32; 3]) -> Result<(), Failure> {
    let shape = box_at(
        [3. * f64::from(i), 3. * f64::from(j), 3. * f64::from(k)],
        [1.; 3],
    )?;
    let name = format!("b{i}{j}{k}");
    session.display(
        &name,
        &shape,
        &DisplayRequest::default().with_display_mode(1),
    )
}

/// Snapshot the actual fitted/zoomed camera before later view changes; `frustum_scene(session)?`.
pub(super) fn frustum_scene(
    session: &mut ViewerSession,
) -> Result<Handle<AisCameraFrustum>, Failure> {
    session.clear()?;
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    println!("Creating 125 boxes...");
    authored_boxes(session)?;
    session.fit()?;
    session.zoom(1.5)?;
    let frustum = AisCameraFrustum::new()?;
    frustum.set_camera_frustum(&session.view().camera())?;
    session.display_object("cam", frustum.clone().cast())?;
    Ok(frustum)
}

fn frustum_camera(
    run: &mut SessionRun,
    direction: V3dTypeOfOrientation,
    image: &'static str,
) -> Outcome {
    run.session().set_standard_projection(direction)?;
    run.session().fit()?;
    run.dump(image)
}

/// Complete all 125 boxes, the real frustum snapshot, and both mandatory images; `camera_frustum_boxes()?`.
pub(crate) fn camera_frustum_boxes() -> Outcome {
    banner(BANNER);
    let mut run = SessionRun::new()?;
    frustum_scene(run.session())?;
    frustum_camera(
        &mut run,
        V3dTypeOfOrientation::ZUP_RIGHT,
        "bug30412_cam_right.png",
    )?;
    frustum_camera(
        &mut run,
        V3dTypeOfOrientation::ZUP_AXO_RIGHT,
        "bug30412_cam_axo.png",
    )?;
    run.finish()
}
