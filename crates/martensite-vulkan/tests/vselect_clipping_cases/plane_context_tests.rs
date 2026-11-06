//! Plane lifecycle is observed on actual view/object sequences and submitted renderer frames.
use super::{cpu_scene::*, prelude::*};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;

#[test]
fn clipping_same_handle_mutation_attachment_and_delete_reach_real_view_and_object() {
    assert_small_declared_device_refused();
    let (mut session, device) = session();
    session
        .display("b", &box_sized([1.; 3]).unwrap(), &shaded_highlighted())
        .unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    let live = planes.plane("p").unwrap().clone();
    planes.on_view("p", session.view()).unwrap();
    object_plane(&mut planes, &session, "p", "b", false).unwrap();
    planes.equation("p", [0., 1., 0., -0.5]).unwrap();
    assert_live_membership(&session, &device, &planes, &live);
    assert!(planes.delete("p").unwrap());
    assert_deleted_membership(&session, &device);
}

#[test]
fn clipping_ordinary_attach_resets_real_sequence_override_without_duplicate_membership() {
    let (mut session, _) = session();
    session
        .display("b", &box_sized([1.; 3]).unwrap(), &shaded_highlighted())
        .unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.ensure("p").unwrap();
    object_plane(&mut planes, &session, "p", "b", true).unwrap();
    let sequence = session.object("b").unwrap().clip_planes();
    assert!(sequence.to_override_global());
    object_plane(&mut planes, &session, "p", "b", false).unwrap();
    assert!(!sequence.to_override_global());
    assert_eq!(sequence.items().len(), 1);
    planes.on_view("p", session.view()).unwrap();
    planes.on_view("p", session.view()).unwrap();
    assert_eq!(session.view().clip_planes().items().len(), 1);
}

#[test]
fn clipping_create_replaces_attached_identity_ensure_keeps_it_and_missing_targets_refuse() {
    let (session, _) = session();
    let mut planes = ClippingPlanes::new(session.view());
    planes.ensure("p").unwrap();
    let old = planes.plane("p").unwrap().clone();
    planes.on_view("p", session.view()).unwrap();
    planes.ensure("p").unwrap();
    assert_eq!(old, *planes.plane("p").unwrap());
    planes.create("p").unwrap();
    assert_ne!(old, *planes.plane("p").unwrap());
    assert!(session.view().clip_planes().items().is_empty());
    assert_unknown_targets(&session, &mut planes);
}

#[test]
fn clipping_equation_mutation_preserves_prior_render_snapshot() {
    let (mut session, device) = session();
    session
        .display("b", &box_sized([1.; 3]).unwrap(), &shaded_highlighted())
        .unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    planes.on_view("p", session.view()).unwrap();
    planes.equation("p", [0., 1., 0., -0.5]).unwrap();
    let previous = checked_frame(&session, &device);
    planes.equation("p", [0., 1., 0., -0.75]).unwrap();
    let first = |frame: &martensite_vulkan::RenderFrame| {
        frame.draws()[0].clipping().unwrap().chains()[0][0]
    };
    assert_eq!(first(&previous), [0., 1., 0., -0.5]);
    assert_eq!(
        first(&checked_frame(&session, &device)),
        [0., 1., 0., -0.75]
    );
}

fn assert_live_membership(
    session: &ViewerSession,
    device: &RecordingDevice,
    planes: &ClippingPlanes,
    live: &Handle<martensite_tkservice::graphic3d::Graphic3dClipPlane>,
) {
    assert_eq!(*live, *planes.plane("p").unwrap());
    assert_eq!(session.view().clip_planes().items().len(), 1);
    assert_eq!(session.object("b").unwrap().clip_planes().items()[0], *live);
    assert!(
        checked_frame(session, device)
            .draws()
            .iter()
            .any(|draw| draw.clipping().is_some())
    );
}

fn assert_deleted_membership(session: &ViewerSession, device: &RecordingDevice) {
    assert!(session.view().clip_planes().items().is_empty());
    assert!(
        session
            .object("b")
            .unwrap()
            .clip_planes()
            .items()
            .is_empty()
    );
    assert!(
        checked_frame(session, device)
            .draws()
            .iter()
            .all(|draw| draw.clipping().is_none())
    );
}

fn assert_unknown_targets(session: &ViewerSession, planes: &mut ClippingPlanes) {
    assert!(!planes.delete("absent").unwrap());
    assert!(
        planes
            .equation("absent", [1., 0., 0., 0.])
            .unwrap_err()
            .message()
            .contains("absent")
    );
    assert!(
        object_plane(planes, session, "p", "missing", false)
            .unwrap_err()
            .message()
            .contains("missing")
    );
    assert!(planes.on_view("missing", session.view()).is_err());
}

#[test]
fn clipping_explicit_view_target_resolves_live_view_and_refuses_unknown_name() {
    let (session, _) = session();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    planes.on_named_view("p", "Driver1/Viewer1/View1").unwrap();
    assert_eq!(
        session.view().clip_planes().items()[0],
        *planes.plane("p").unwrap()
    );
    let failure = planes
        .on_named_view("p", "Driver1/Viewer1/Other")
        .unwrap_err();
    assert!(failure.message().contains("Driver1/Viewer1/Other"));
    assert_eq!(session.view().clip_planes().items().len(), 1);
}

fn assert_small_declared_device_refused() {
    let (session, device) = session_with_limit(128);
    let view = session
        .view()
        .get()
        .unwrap()
        .view()
        .cast::<martensite_vulkan::Graphic3dRenderView>();
    let error = view.try_redraw().unwrap_err();
    assert!(error.to_string().contains("positive extent at most 128"));
    assert!(device.frames.read().is_empty());
}
