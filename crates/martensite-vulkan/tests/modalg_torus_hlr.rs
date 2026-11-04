//! A horn-torus segment uses the authored explicit HLR eye, direction and up frame.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    hidden_lines::{HiddenLineAlgorithm, visible_lines},
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tkv3d::ais::AisColoredShape;
type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(super) fn torus() -> Result<Shape, Failure> {
    let degrees = 2.35 * 180. / std::f64::consts::PI;
    let mut maker =
        BRepPrimAPIMakeTorus::with_angle(17.37, 17.37, degrees * std::f64::consts::PI / 180.)?;
    Ok(maker.shape()?.clone())
}
pub(super) fn projected(shape: &Shape) -> Result<Shape, Failure> {
    // gpAx2::SetYDirection(up=+Z) with direction=-Y establishes +X as the right axis.
    let frame = Axis2::with_x_direction(
        Point3::new(0., 50., 0.),
        Direction3::new(0., -50., 0.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    visible_lines(shape, frame, HiddenLineAlgorithm::Exact)
}
/// Display the actual input and the complete colored HLR result, without adding a dump; `torus_normal_status()?`.
pub(crate) fn torus_normal_status() -> Outcome {
    banner(&[
        "============================================",
        "OCC29405: Modeling Algorithms - Check normal define status in HLRBRep_Data",
        "============================================",
        "",
    ]);
    let shape = torus()?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("t", &shape, &DisplayRequest::default())?;
    // VComputeHLR reads the active view frame before parsing the explicit frame that replaces it.
    let _initial_frame = run.session().projection_frame()?;
    let result = projected(&shape)?;
    display_result(run.session(), &result)?;
    run.finish_without_image()
}
fn display_result(session: &mut ViewerSession, result: &Shape) -> Result<(), Failure> {
    let colored = AisColoredShape::new(result);
    session.bind_object("t2", colored.cast())?;
    session.context().display(&colored.cast(), true)
}
#[cfg(test)]
mod cpu {
    use super::*;
    #[test]
    fn torus_explicit_frame_executes_actual_exact_hlr() {
        let result = projected(&torus().unwrap()).unwrap();
        assert!(!result.is_null());
    }
}
