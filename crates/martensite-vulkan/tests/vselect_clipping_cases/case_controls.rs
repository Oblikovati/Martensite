//! Independent geometry and genuine picking controls; none qualify the original pixels/images.
use super::{bug26973, bug29791, bug30695, bug30777, bug30906, bug31650, cpu_scene::*, prelude::*};
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkservice::graphic3d::{Graphic3dSequenceOfHClipPlane, Graphic3dStructureMembers};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
use martensite_tkv3d::select_mgr::SelectMgrViewClipRange;

#[test]
fn clipping_four_actual_face_picks_keep_transformed_occurrence_and_cleanup_isolated() {
    let (mut session, _) = session();
    session.set_axonometric().unwrap();
    session.set_display_mode(1).unwrap();
    for (view, transformed) in [(true, false), (true, true), (false, false), (false, true)] {
        check_one_face_probe(&mut session, view, transformed);
    }
    assert!(session.object("b").is_none());
    assert!(session.view().clip_planes().items().is_empty());
}

fn check_one_face_probe(session: &mut ViewerSession, view: bool, transformed: bool) {
    bug26973::prepare_probe(session, transformed).unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    if view {
        planes.on_view("p", session.view()).unwrap();
    } else {
        object_plane(&mut planes, session, "p", "b", false).unwrap();
    }
    planes.equation("p", [0., 1., 0., -0.5]).unwrap();
    session.select_point(200, 140).unwrap();
    assert_eq!(session.selected_count(), 1);
    assert_eq!(
        super::selected_state::local_selected_faces(session.context()).unwrap(),
        ["Face #1"]
    );
    session.clear().unwrap();
    planes.delete("p").unwrap();
}

#[test]
fn clipping_transformed_probe_preserves_real_shape_location_not_rebuilt_origin() {
    let plain = bug26973::probe_shape(false).unwrap();
    let located = bug26973::probe_shape(true).unwrap();
    assert!(plain.location().is_identity());
    assert!(!located.location().is_identity());
    let shift = located.location().transformation().translation_part();
    assert_eq!([shift.x(), shift.y(), shift.z()], [100.; 3]);
}

#[test]
fn clipping_bottle_selected_bytes_accept_exact_input_and_refuse_same_length_shadow() {
    let path = crate::dataset_shape::locate_dataset_file("occ/bottle.brep").unwrap();
    let mut bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 401311);
    bug29791::verified_bottle_bytes(&bytes).unwrap();
    assert!(!bug29791::read_pinned_bottle().unwrap().is_null());
    bytes[200] ^= 1;
    let error = bug29791::verified_bottle_bytes(&bytes).unwrap_err();
    assert!(
        error
            .message()
            .contains("expected exact 401311-byte SHA256")
    );
    assert!(bug29791::verified_bottle_bytes(&[]).is_err());
}

#[test]
fn clipping_actual_bottle_camera_keeps_second_regime_scale_center_and_projection() {
    let (mut session, _) = session();
    let bottle = bug29791::read_pinned_bottle().unwrap();
    session
        .display("b", &bottle, &shaded_highlighted())
        .unwrap();
    session.fit().unwrap();
    session
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)
        .unwrap();
    session.fit().unwrap();
    let parameters = bug29791::negative_depth_camera();
    session.set_view_parameters(&parameters).unwrap();
    assert_bottle_camera(&session, &parameters);
}

#[test]
fn clipping_actual_ray_range_rejects_orthogonal_outside_and_refreshes_behind_origin() {
    let (session, _) = session();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    planes.equation("p", [1., 0., 0., -20.]).unwrap();
    let ray = Axis1::new(Point3::new(25., 0., 0.), Direction3::Z);
    assert!(!ray_range(&planes, ray).is_clipped(10.));
    planes.equation("p", [1., 0., 0., -30.]).unwrap();
    assert!(ray_range(&planes, ray).is_clipped(10.));
    planes.equation("p", [0., 0., 1., 10.]).unwrap();
    let behind = ray_range(&planes, ray);
    assert!(!behind.is_clipped(0.));
    assert!(behind.is_clipped(-11.));
    planes.equation("p", [0., 0., -1., -10.]).unwrap();
    assert!(ray_range(&planes, ray).is_clipped(0.));
}

fn ray_range(planes: &ClippingPlanes, ray: Axis1) -> SelectMgrViewClipRange {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    sequence.append(planes.plane("p").unwrap().clone());
    let mut range = SelectMgrViewClipRange::default();
    range.add_clipping_planes(&sequence, &ray).unwrap();
    range
}

#[test]
fn clipping_actual_rectangle_has_four_then_one_contained_then_two_overlapping_identities() {
    let (mut session, _) = session();
    bug30695::display_boxes(&mut session).unwrap();
    session.fit().unwrap();
    session.zoom(0.5).unwrap();
    bug30695::select_box(&mut session, false).unwrap();
    assert_eq!(
        selected_names(&session, &["b", "b1", "b2", "b3"]),
        ["b", "b1", "b2", "b3"]
    );
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    planes.on_view("p", session.view()).unwrap();
    planes.equation("p", [-1., 0., 0., 5.]).unwrap();
    check_rectangle_plane(&mut session, &planes);
}

fn check_rectangle_plane(session: &mut ViewerSession, planes: &ClippingPlanes) {
    bug30695::select_box(session, false).unwrap();
    assert_eq!(selected_names(session, &["b", "b1", "b2", "b3"]), ["b1"]);
    bug30695::select_box(session, true).unwrap();
    assert_eq!(
        selected_names(session, &["b", "b1", "b2", "b3"]),
        ["b", "b1"]
    );
    planes.plane("p").unwrap().set_on(false).unwrap();
    bug30695::select_box(session, true).unwrap();
    assert_eq!(session.selected_count(), 4);
}

#[test]
fn clipping_actual_sphere_hit_chooses_unclipped_analytic_intersection_not_front_root() {
    let (mut session, _) = session();
    bug30777::sphere_scene(&mut session).unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.create("p").unwrap();
    object_plane(&mut planes, &session, "p", "s", false).unwrap();
    planes.equation("p", [0., 1., 0., 0.]).unwrap();
    let point = assert_point_sphere_root(&mut session, true);
    planes.equation("p", [0., -1., 0., 0.]).unwrap();
    assert_point_sphere_root(&mut session, false);
    planes.equation("p", [0., 1., 0., 0.]).unwrap();
    assert_axis_sphere_interval(&session);
    bug30777::display_point(&mut session, "pp", point).unwrap();
    assert!(session.object("pp").is_some());
}

fn assert_point_sphere_root(session: &mut ViewerSession, positive_y: bool) -> [f64; 3] {
    session.move_cursor(200, 200).unwrap();
    let actual = session.detected_point().unwrap();
    let (origin, direction) = actual_point_ray(session);
    let eye = session.view().camera().eye();
    let expected = analytic_retained_sphere_root(origin, direction, eye, positive_y);
    assert_analytic_point(actual, expected);
    assert!(
        session
            .picked_entity_types()
            .unwrap()
            .iter()
            .any(|name| name == "Select3DSensitiveSphere")
    );
    actual
}

fn actual_point_ray(session: &ViewerSession) -> ([f64; 3], [f64; 3]) {
    use martensite_tkv3d::{
        select_basics::SelectBasicsSelectingVolumeManager,
        select_mgr::SelectMgrViewerSelectorMembers,
    };
    let selector = session.context().main_selector();
    let manager = selector.get_manager();
    let origin = manager.get_near_picked_pnt().unwrap();
    let direction = manager.get_view_ray_direction().unwrap();
    (
        [origin.x(), origin.y(), origin.z()],
        [direction.x(), direction.y(), direction.z()],
    )
}

fn analytic_retained_sphere_root(
    origin: [f64; 3],
    direction: [f64; 3],
    eye: Point3,
    positive_y: bool,
) -> [f64; 3] {
    let sign = if positive_y { 1. } else { -1. };
    let eye = [eye.x(), eye.y(), eye.z()];
    sphere_surface_roots(origin, direction)
        .into_iter()
        .filter(|point| point[1] * sign >= 0.)
        .find(|point| sphere_root_is_forward(*point, eye, direction))
        .expect("independent radius10 quadratic must have a retained visible surface root")
}

fn sphere_surface_roots(origin: [f64; 3], direction: [f64; 3]) -> [[f64; 3]; 2] {
    let dot: f64 = origin.into_iter().zip(direction).map(|(a, b)| a * b).sum();
    let norm: f64 = origin.into_iter().map(|value| value * value).sum();
    let discriminant = dot * dot - norm + 100.;
    assert!(discriminant >= 0.);
    let root = discriminant.sqrt();
    [-dot - root, -dot + root]
        .map(|time| std::array::from_fn(|axis| origin[axis] + time * direction[axis]))
}

fn sphere_root_is_forward(point: [f64; 3], eye: [f64; 3], direction: [f64; 3]) -> bool {
    let depth: f64 = point
        .into_iter()
        .zip(eye)
        .zip(direction)
        .map(|((position, camera), axis)| (position - camera) * axis)
        .sum();
    depth >= 0.
}

fn assert_analytic_point(actual: [f64; 3], expected: [f64; 3]) {
    for (value, reference) in actual.into_iter().zip(expected) {
        assert!(
            value.is_finite() && (value - reference).abs() < 1e-9,
            "point={actual:?} reference={expected:?}"
        );
    }
}

fn assert_axis_sphere_interval(session: &ViewerSession) {
    let hits = axis_hits(session, [0., -20., 0.], [0., 1., 0.]);
    assert_eq!(hits.len(), 1);
    // The solid interval[10,30] intersects y>=0 at depth20, unlike the surface-root query.
    assert_eq!(hits[0].1, [0.; 3]);
    assert_eq!(
        hits[0].0,
        session.object("s").unwrap().object_address() as usize
    );
}

#[test]
fn clipping_override_per_object_volume_and_renderer_planes_never_leak_to_next_pick() {
    let (mut session, device) = session();
    bug30906::display_pair(&mut session).unwrap();
    // Independent distinguishing point y=0.5: shift only this control's b1 into the slice.
    session.set_location("b1", [0., -1., 0.]).unwrap();
    let mut planes = bug30906::override_planes(&session).unwrap();
    check_repeated_override_picks(&session);
    assert_owned_render_planes(&session, &device);
    object_plane(&mut planes, &session, "c1", "b1", false).unwrap();
    assert_object_pick(&session, "b1", [1.5, 0.5, 10.], false);
}

fn assert_object_pick(session: &ViewerSession, name: &str, origin: [f64; 3], expected: bool) {
    let hits = axis_hits(session, origin, [0., 0., -1.]);
    let identity = session.object(name).unwrap().object_address() as usize;
    assert_eq!(hits.iter().any(|(object, _)| *object == identity), expected);
    assert!(hits.iter().all(|(object, _)| *object == identity));
}

fn assert_owned_render_planes(session: &ViewerSession, device: &RecordingDevice) {
    let before = pair_presentation_ids(session);
    let local = isolated_frame(session, device, "b1", "b2");
    assert_eq!(local.draws().len(), 1);
    assert!(local.draws()[0].clipping().is_none());
    let global = isolated_frame(session, device, "b2", "b1");
    assert_global_frame(&global);
    restore_pair(session);
    assert_eq!(before, pair_presentation_ids(session));
}

fn presentation_ids(session: &ViewerSession, name: &str) -> std::collections::BTreeSet<i32> {
    let object = named_object(session, name).unwrap();
    object
        .presentations()
        .iter()
        .map(|presentation| presentation.identification())
        .collect()
}

fn isolated_frame(
    session: &ViewerSession,
    device: &RecordingDevice,
    shown: &str,
    hidden: &str,
) -> martensite_vulkan::RenderFrame {
    session
        .context()
        .erase(&named_object(session, hidden).unwrap(), false)
        .unwrap();
    let object = named_object(session, shown).unwrap();
    session.context().display(&object, false).unwrap();
    session.context().update_current_viewer();
    let mut visible = Default::default();
    session.context().displayed_objects(&mut visible).unwrap();
    assert_eq!(visible.iter().cloned().collect::<Vec<_>>(), [object]);
    checked_frame(session, device)
}

fn assert_global_frame(frame: &martensite_vulkan::RenderFrame) {
    assert_eq!(frame.draws().len(), 1);
    let clipping = frame.draws()[0].clipping().unwrap();
    assert_eq!(clipping.chains(), &[vec![[0., 1., 0., -1.]]]);
}

#[test]
fn clipping_transformed_box_keeps_shared_global_and_local_plane_in_world_space() {
    let (mut session, device) = session();
    bug31650::located_box(&mut session).unwrap();
    let mut planes = ClippingPlanes::new(session.view());
    planes.ensure("p").unwrap();
    planes.equation("p", [0., 0., -1., -10.]).unwrap();
    planes.on_view("p", session.view()).unwrap();
    let before = axis_hits(&session, [-20., 0., -30.], [0., 0., 1.]);
    object_plane(&mut planes, &session, "p", "b", false).unwrap();
    let after = axis_hits(&session, [-20., 0., -30.], [0., 0., 1.]);
    assert_eq!(before, after);
    assert_eq!(after.len(), 1);
    assert!((after[0].1[2] + 15.).abs() < 1e-9);
    assert_shared_plane_transform(&session, &device, &planes);
}

fn assert_shared_plane_transform(
    session: &ViewerSession,
    device: &RecordingDevice,
    planes: &ClippingPlanes,
) {
    let live = planes.plane("p").unwrap();
    assert_eq!(session.view().clip_planes().items()[0], *live);
    assert_eq!(session.object("b").unwrap().clip_planes().items()[0], *live);
    let frame = checked_frame(session, device);
    let clipping = frame.draws()[0].clipping().unwrap();
    assert_eq!(&clipping.world_from_object()[12..16], &[0., 0., 20., 1.]);
    assert_eq!(
        clipping.chains(),
        &[vec![[0., 0., -1., -10.]], vec![[0., 0., -1., -10.]]]
    );
}

fn assert_bottle_camera(
    session: &ViewerSession,
    parameters: &crate::case_view::viewer_session::ViewParameters,
) {
    let camera = session.view().camera();
    let center = camera.center();
    assert_eq!(parameters.at, Some([11.3689, 1.32152, 24.9954]));
    assert_eq!(parameters.scale, Some(14.7435));
    assert_eq!(parameters.proj, Some([0.193921, -0.891229, 0.410007]));
    assert_eq!(parameters.up, Some([-0.0205984, 0.414149, 0.909976]));
    assert_eq!(session.view().scale(), 14.7435);
    let expected_raw_scale = session.view().default_camera().scale() / 14.7435;
    assert_eq!(camera.scale(), expected_raw_scale);
    assert_camera_roundoff([center.x(), center.y(), center.z()], &camera);
}

fn check_repeated_override_picks(session: &ViewerSession) {
    for direction in [1., -1., 1.] {
        let origin_x = if direction > 0. { 0. } else { 5. };
        let hits = axis_hits(session, [origin_x, 0.5, 1.5], [direction, 0., 0.]);
        let identity = session.object("b1").unwrap().object_address() as usize;
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].0, identity);
        assert_object_pick(session, "b2", [3.5, 0.5, 10.], false);
        assert_object_pick(session, "b2", [3.5, 1.5, 10.], true);
    }
}

fn assert_camera_roundoff(
    actual: [f64; 3],
    camera: &martensite_tkservice::graphic3d::Graphic3dCamera,
) {
    let eye = camera.eye();
    let eye = [eye.x(), eye.y(), eye.z()];
    // A conservative64-operation gamma bound covers the center/eye/direction setter round trip.
    let gamma = 64. * f64::EPSILON / (1. - 64. * f64::EPSILON);
    for axis in 0..3 {
        let requested: [f64; 3] = [11.3689, 1.32152, 24.9954];
        let scale = requested[axis].abs() + eye[axis].abs() + camera.distance() + 1.;
        assert!(actual[axis].is_finite());
        assert!((actual[axis] - requested[axis]).abs() <= gamma * scale);
    }
}

fn restore_pair(session: &ViewerSession) {
    for name in ["b1", "b2"] {
        session
            .context()
            .display(&named_object(session, name).unwrap(), false)
            .unwrap();
    }
    session.context().update_current_viewer();
}

fn pair_presentation_ids(session: &ViewerSession) -> [std::collections::BTreeSet<i32>; 2] {
    let first = presentation_ids(session, "b1");
    let second = presentation_ids(session, "b2");
    [first, second]
}
