//! The source color expectations and all camera literals remain unchanged.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewParameters, ViewerSession},
    dataset_shape::read_dataset_shape,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn camera_parameters() -> ViewParameters {
    ViewParameters {
        scale: Some(276.456_580_489_041_4),
        proj: Some(camera_projection()),
        up: Some(camera_up()),
        at: Some([0.248127012715387, 0.109238834542233, -1.2607059785715]),
        ..Default::default()
    }
}
fn camera_projection() -> [f64; 3] {
    [
        -0.8895147442817688,
        -0.37965071201324463,
        0.254_220_604_896_545_4,
    ]
}
fn camera_up() -> [f64; 3] {
    [
        -0.055201318114995956,
        0.641_616_880_893_707_3,
        0.765_035_688_877_105_7,
    ]
}
pub(super) fn prepared(session: &mut ViewerSession) -> Result<(), Failure> {
    let shape = read_dataset_shape("PFS04041r1.brep")?;
    display_and_camera(session, &shape)
}
/// Restore the genuine input and execute both authored hole-color checks; `restored_hole_colors()?`.
pub(crate) fn restored_hole_colors() -> Outcome {
    heading();
    // Restore the shape before opening the view; keep that ordering without a second read.
    let shape = read_dataset_shape("PFS04041r1.brep")?;
    let mut run = SessionRun::new()?;
    display_and_camera(run.session(), &shape)?;
    run.session()
        .check_color(105, 275, [1., 0.764_705_002_307_891_8, 0.16078400611877441])?;
    hole_colors(run.session())?;
    run.dump("bug22701.png")?;
    run.finish()
}
fn display_and_camera(
    session: &mut ViewerSession,
    shape: &martensite_tkbrep::topods::Shape,
) -> Result<(), Failure> {
    session.display("result", shape, &DisplayRequest::default())?;
    session.fit()?;
    session.set_display_mode(1)?;
    session.fit()?;
    session.set_view_parameters(&camera_parameters())
}

fn heading() {
    banner(&[
        "============",
        "OCC22701: Regression in 6.5.1 - Wrong visualization",
        "============",
        "",
    ]);
}

fn hole_colors(session: &ViewerSession) -> Result<(), Failure> {
    session.check_color(
        112,
        248,
        [
            0.682_352_006_435_394_3,
            0.48235198855400085,
            0.078_431_002_795_696_26,
        ],
    )?;
    Ok(())
}
#[cfg(test)]
mod cpu {
    use super::*;
    use crate::case_view::SourceViewOutput;
    #[test]
    fn genuine_hole_shape_keeps_authored_camera_parameters() {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        prepared(&mut session).unwrap();
        assert!(session.object("result").is_some());
        assert_eq!(session.context().display_mode(), 1);
        let actual = session.view().view_base().scale();
        assert!((actual - 276.456_580_489_041_4).abs() < 1.0e-10);
        session.close();
    }
}
