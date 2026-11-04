//! Focused NullDriver controls do not construct a Vulkan device.
use super::{
    axial::axial_scale,
    highlight::{explicit_highlight, red_dynamic_style},
    modes::{last_mode, transition},
    rectangle::rectangle_pick,
};
use crate::{
    case_view::{
        SourceViewOutput,
        viewer_session::{DisplayRequest, ViewerSession},
    },
    primitive_shapes::{box_at, box_sized},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{
    ais::AisStatusOfPick,
    prs3d::{Prs3dDrawer, Prs3dTypeOfHighlight},
};

fn null_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap()
}

fn fixture_display(session: &mut ViewerSession, name: &str, shape: Shape, mode: i32) {
    let request = DisplayRequest::default().with_display_mode(mode);
    session.display(name, &shape, &request).unwrap();
}

#[test]
fn all_eight_modes_report_the_actual_last_activation() {
    let mut session = null_session();
    fixture_display(&mut session, "b", box_sized([10.; 3]).unwrap(), 0);
    for mode in [0, 1, 2, 3, 4, 5, 6, -1] {
        transition(&session, mode).unwrap();
        assert_eq!(last_mode(&session), mode);
    }
}

#[test]
fn rectangle_reports_actual_empty_one_and_several_status_without_fit() {
    let mut session = null_session();
    assert_eq!(session.source_window_size(), (409, 409));
    assert_eq!(
        rectangle_pick(&session).unwrap(),
        AisStatusOfPick::NothingSelected
    );
    fixture_display(&mut session, "b", box_sized([10.; 3]).unwrap(), 0);
    assert_eq!(
        rectangle_pick(&session).unwrap(),
        AisStatusOfPick::OneSelected
    );
    fixture_display(&mut session, "w", box_at([20.; 3], [20.; 3]).unwrap(), 0);
    assert_eq!(
        rectangle_pick(&session).unwrap(),
        AisStatusOfPick::SeveralSelected
    );
}

#[test]
fn axial_command_updates_the_existing_camera_without_replacement() {
    let mut probe = frame_probe::SelectionFrameProbe::new();
    fixture_display(&mut probe.session, "b", box_sized([10.; 3]).unwrap(), 0);
    probe.session.fit().unwrap();
    let camera = probe.session.view().view_base().camera();
    axial_scale(&probe.session, [1.; 3]).unwrap();
    let before = probe.snapshot().0;
    axial_scale(&probe.session, [0.5, 1.5, 0.7]).unwrap();
    let after = probe.snapshot().0;
    assert_axial_camera(&probe.session, &camera);
    assert_submitted_axial_change(&before, &after);
}

#[test]
fn explicit_highlight_retains_the_passed_dynamic_style_handle() {
    let mut session = null_session();
    fixture_display(&mut session, "b", box_sized([1., 2., 3.]).unwrap(), 1);
    red_dynamic_style(&session).unwrap();
    let expected = (*session
        .context()
        .highlight_style(Prs3dTypeOfHighlight::Dynamic)
        .unwrap())
    .clone();
    explicit_highlight(&session).unwrap();
    assert_style_handle(&session, expected.object_address());
    assert!(session.context().detected_interactive().is_null());
}

fn assert_style_handle(session: &ViewerSession, expected: *const ()) {
    let mut actual = Handle::<Prs3dDrawer>::default();
    assert!(
        session
            .context()
            .object_highlight_style(session.object("b").unwrap(), &mut actual)
    );
    assert_eq!(actual.object_address(), expected);
}

#[allow(dead_code)] // This host uses the shared injected backend, not its optional geometry export.
#[path = "../support/selection_frame_probe.rs"]
mod frame_probe;

fn assert_axial_camera(
    session: &ViewerSession,
    camera: &Handle<martensite_tkservice::graphic3d::Graphic3dCamera>,
) {
    assert_eq!(
        camera.object_address(),
        session.view().view_base().camera().object_address()
    );
    assert_eq!(
        camera.axial_scale(),
        martensite_tkmath::gp::Xyz::new(0.5, 1.5, 0.7)
    );
}

fn assert_submitted_axial_change(
    before: &martensite_vulkan::RenderFrame,
    after: &martensite_vulkan::RenderFrame,
) {
    assert!(!before.draws().is_empty());
    assert_ne!(
        before.draws()[0].clip_transform(),
        after.draws()[0].clip_transform()
    );
    assert_eq!(before.draws()[0].vertices(), after.draws()[0].vertices());
}
