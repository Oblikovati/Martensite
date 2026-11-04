//! Values pass through the existing typed grid, view-parameter and depth-range adapters.
use crate::{
    case_view::viewer_session::{
        DisplayRequest, GridLayout, GridRequest, ViewParameters, ViewerSession,
    },
    primitive_shapes::{cylinder, translated},
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::AspectGridDrawMode;
// VGrid passes -rotAngle directly to the service in radians; do not reinterpret the literal as degrees.
const SOURCE_CONVERSION_GRID_ROTATION: f64 = 45.;
pub(super) fn detection_scene(session: &mut ViewerSession) -> Result<(), Failure> {
    let layout = GridLayout {
        origin: Some((-10., 0.)),
        step: Some((5., 5.)),
        rotation: Some(SOURCE_CONVERSION_GRID_ROTATION),
    };
    session.show_grid(&GridRequest::circular(layout).drawn_as(AspectGridDrawMode::Points))?;
    let shape = translated(&cylinder(5., 10.)?, [100., 0., 0.])?;
    session.display("c", &shape, &DisplayRequest::default())?;
    session.fit()
}
pub(super) fn first(session: &ViewerSession) -> Result<(), Failure> {
    let parameters = ViewParameters {
        scale: Some(2.5),
        proj: Some([0., 1., 0.]),
        up: Some([0., 0., 1.]),
        eye: Some([0.5, 100.5, 0.5]),
        at: Some([0.5, 0.5, 0.5]),
    };
    session.set_view_parameters(&parameters)?;
    session.set_z_range(0., 150.)
}
pub(super) fn second(session: &ViewerSession) -> Result<(), Failure> {
    let layout = GridLayout {
        origin: Some((10., 10.)),
        step: Some((1., 1.)),
        rotation: Some(SOURCE_CONVERSION_GRID_ROTATION),
    };
    session.show_grid(&GridRequest::rectangular(layout))?;
    let parameters = ViewParameters {
        scale: Some(49.504950495049506),
        proj: Some([0., 0., 1.]),
        up: Some([0., 1., 0.]),
        eye: Some([25., 5., 125.]),
        at: Some([25., 5., 45.]),
    };
    session.set_view_parameters(&parameters)?;
    session.set_auto_z_fit(false)?;
    session.set_z_range(0., 150.)
}
