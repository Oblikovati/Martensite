//! Real command sequencing is observed through the existing named CPU rendering fake.
use super::*;
use crate::case_view::{
    SourceViewOutput,
    viewer_session::{DisplayRequest, ViewerSession},
};
use martensite_tkservice::graphic3d::{Graphic3dAspectsMembers, Graphic3dNameOfMaterial};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
#[path = "../support/recording_device.rs"]
mod recording_device;

fn material_recorded_session() -> (ViewerSession, recording_device::RecordingDevice) {
    let backend = recording_device::RecordingDevice::default();
    let driver =
        martensite_vulkan::Graphic3dRenderDriver::with_backend(backend.clone(), 2048).unwrap();
    (
        ViewerSession::with_view_size(&driver, 64, 64).unwrap(),
        backend,
    )
}

#[test]
fn material_noredraw_changes_the_real_aspect_without_submitting_an_extra_frame() {
    let (mut session, backend) = material_recorded_session();
    let shape = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    session
        .display("s", &shape, &DisplayRequest::default().with_display_mode(1))
        .unwrap();
    let before = backend.frames.read().len();
    set_dump_material(&mut session, Graphic3dNameOfMaterial::Gold).unwrap();
    assert_eq!(backend.frames.read().len(), before);
    assert_gold_material(&session);
    session.view().redraw();
    assert!(backend.frames.read().len() > before);
    assert_eq!(backend.last_frame().extent(), [64, 64]);
    session.close();
}

#[test]
fn material_noredraw_failure_restores_the_existing_immediate_update_policy() {
    let (mut session, backend) = material_recorded_session();
    let previous = session.view().set_immediate_update(true);
    let before = backend.frames.read().len();
    assert!(set_dump_material(&mut session, Graphic3dNameOfMaterial::Gold).is_err());
    assert_eq!(backend.frames.read().len(), before);
    assert!(session.view().set_immediate_update(previous));
    session.close();
}

fn assert_gold_material(session: &ViewerSession) {
    let material = session
        .object("s")
        .unwrap()
        .attributes()
        .shading_aspect()
        .aspect()
        .front_material();
    assert_eq!(material.name(), Graphic3dNameOfMaterial::Gold);
}
