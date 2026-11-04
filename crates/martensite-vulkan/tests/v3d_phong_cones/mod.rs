//! The complete source uses the ordinary 409 view and the existing actual Phong program.
#[cfg(test)]
mod cpu;
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::aspect::AspectTypeOfDeflection;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Execute every meshing, transform, display and program command before the mandatory image; `phong_cones()?`.
pub(crate) fn phong_cones() -> Outcome {
    banner(&[
        "===========",
        "OCC21753: Poor display of shading on cones due to Gouraud shading limitations",
        "===========",
    ]);
    let mut run = SessionRun::new()?;
    prepare_source(run.session())?;
    run.session().fit()?;
    run.dump("bug21753.png")?;
    run.finish()
}

fn prepare_source(session: &mut ViewerSession) -> Outcome {
    source_defaults(session);
    println!("Displaying two equal cones, one with Phong shader");
    let gouraud = source_cone()?;
    display_shaded(session, "p_gouraud", &gouraud)?;
    let mut copy = BRepBuilderAPICopy::with_shape(&gouraud, true, false)?;
    let copy = copy.shape()?.clone();
    let phong = crate::primitive_shapes::translated(&copy, [200., 0., 0.])?;
    display_shaded(session, "p_phong", &phong)?;
    attach_phong(session)
}

fn source_defaults(session: &ViewerSession) {
    let drawer = session.context().default_drawer();
    drawer.set_type_of_deflection(AspectTypeOfDeflection::Absolute);
    drawer.set_maximal_chordial_deviation(10.);
    session.set_default_deviation_coefficient(0.1);
}

fn source_cone() -> Result<martensite_tkbrep::topods::Shape, Failure> {
    let cone = crate::primitive_shapes::cone(10., 100., 100.)?;
    crate::primitive_shapes::rotated_about(&cone, [0.; 3], [0., 1., 0.], -90.)
}

fn display_shaded(
    session: &mut ViewerSession,
    name: &str,
    shape: &martensite_tkbrep::topods::Shape,
) -> Result<(), Failure> {
    session.display(name, shape, &DisplayRequest::default())?;
    session.set_object_display_mode(name, 1)
}

fn attach_phong(session: &ViewerSession) -> Outcome {
    let object = session
        .object("p_phong")
        .ok_or("source shader object=p_phong; expected displayed cone")?;
    crate::object_fill_program::assign(session.context(), &[object], || {
        Handle::new(martensite_vulkan::phong_shading_program())
    })?;
    session.context().update_current_viewer();
    Ok(())
}
