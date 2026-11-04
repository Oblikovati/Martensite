//! No raster epsilon replaces the original cyan-pixel checks at increasing eye distance.
#[cfg(test)]
mod cpu;
mod planes;
use crate::{
    case_view::viewer_session::{ViewParameters, ViewerSession},
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
type Outcome = Result<(), Box<dyn std::error::Error>>;
/// Execute both authored frustum modes and every eye-distance check; `far_plane_projection()?`.
pub(crate) fn far_plane_projection() -> Outcome {
    heading();
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    for (culling, word) in [(false, "OFF"), (true, "ON")] {
        run.session()
            .set_projection(Graphic3dCameraProjection::Orthographic)?;
        run.session().set_frustum_culling(culling)?;
        if !test_projection(run.session())? {
            println!("Error: 2D projection test failed: view frustum culling is {word}");
        }
    }
    run.finish_without_image()
}
fn heading() {
    banner(&[
        "============",
        "CR25760: Visualization - precision factor added to ZNear, ZFar in method ZFitAll() of Graphic3d_Camera is not enough",
        "============",
        "",
    ]);
}
fn test_projection(session: &mut ViewerSession) -> Result<bool, Failure> {
    let min_z = martensite_cmath::powi(-10.0_f64, 8);
    let max_z = min_z + 1000.;
    planes::display(session, min_z, max_z)?;
    let mut z = max_z + 1.;
    while z <= 1.0e10 {
        if !check_eye(session, z, min_z, max_z)? {
            return Ok(false);
        }
        z = z.abs() * 1.2;
    }
    Ok(true)
}
fn check_eye(session: &mut ViewerSession, z: f64, min_z: f64, max_z: f64) -> Result<bool, Failure> {
    session.set_view_parameters(&ViewParameters {
        eye: Some([0., 0., z]),
        ..Default::default()
    })?;
    for (x, y) in [(135, 204), (204, 187)] {
        session.move_cursor(x, y)?;
        if !session.check_color(x, y, [0., 1., 1.])? {
            failure_parameters(session, z, min_z, max_z);
            return Ok(false);
        }
    }
    Ok(true)
}
fn failure_parameters(session: &ViewerSession, z: f64, min_z: f64, max_z: f64) {
    println!("Error: 2D projection test failed with the following parameters:");
    let camera = session.view().camera();
    query_view(session);
    println!(
        "ZNear: {}\nZFar: {}\n\nz    : {z}\nmin_z: {min_z}\nmax_z: {max_z}",
        camera.z_near(),
        camera.z_far()
    );
}

fn query_view(session: &ViewerSession) {
    let camera = session.view().camera();
    println!(
        "Scale: {}\nAspect: {}\nProj: {:?}",
        camera.scale(),
        camera.aspect(),
        session.view().view_base().proj()
    );
    println!(
        "Up: {:?}\nAt: {:?}\nEye: {:?}",
        camera.up(),
        camera.center(),
        camera.eye()
    );
    query_matrices(session);
}
fn query_matrices(session: &ViewerSession) {
    let camera = session.view().camera();
    println!(
        "OrientMat: {:?}\nProjMat: {:?}",
        camera.orientation_matrix().data(),
        camera.projection_matrix().data()
    );
}
