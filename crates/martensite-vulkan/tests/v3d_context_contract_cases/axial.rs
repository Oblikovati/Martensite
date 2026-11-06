use super::named_pixel;
use crate::{
    case_view::viewer_session::ViewerSession,
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
use std::error::Error;

/// Preserve VScale/SetAxialScale: the following authored operation draws; `axial_scale(session,[1.;3])?`.
pub(super) fn axial_scale(session: &ViewerSession, scale: [f64; 3]) -> Result<(), Failure> {
    session
        .view()
        .view_base()
        .set_axial_scale(scale[0], scale[1], scale[2])
}

/// Replay the complete authored regression; `axial()?`.
pub(crate) fn axial() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "OCC11615: Anisotropic scaling in 3D view",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let session = run.session();
    session.display("b", &box_sized([10.; 3])?, &Default::default())?;
    session.fit()?;
    axial_scale(session, [1.; 3])?;
    session.set_line_width("b", 5.)?;
    yellow_samples(session, &[(165, 384), (340, 283), (70, 79)])?;
    axial_scale(session, [0.5, 1.5, 0.7])?;
    session.fit()?;
    yellow_samples(session, &[(80, 390), (390, 218), (25, 167)])?;
    run.dump("bug11615.png")?;
    run.finish()
}

fn yellow_samples(session: &ViewerSession, points: &[(i32, i32)]) -> Result<(), Failure> {
    println!();
    for &(x, y) in points {
        named_pixel(session, x, y, "YELLOW", "Error : color is not yellow")?;
    }
    Ok(())
}
