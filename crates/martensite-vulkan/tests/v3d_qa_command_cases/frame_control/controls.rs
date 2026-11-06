//! Exact operation-handle controls run against the existing CPU RecordingDevice.
use super::*;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
#[allow(dead_code)]
#[path = "../../support/selection_frame_probe.rs"]
mod frame_probe;
use martensite_vulkan::PrimitiveTopology;
fn actual_scale_probe() -> (frame_probe::SelectionFrameProbe, Handle<AisColorScale>) {
    let probe = frame_probe::SelectionFrameProbe::new();
    let scale = super::super::color_scale::prepare_scale(&probe.session).unwrap();
    probe
        .session
        .context()
        .display(&scale.cast(), true)
        .unwrap();
    (probe, scale)
}
#[test]
fn frame_only_control_preserves_all_remaining_full_draws_and_restores_exact_handles() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let before = probe.snapshot().0;
    let expected = non_frame_draws(&before);
    with_frame_control(&probe.session, &scale, || {
        let without = probe.snapshot().0;
        assert_eq!(without.draws(), expected);
        assert_eq!(without.extent(), before.extent());
        assert_eq!(without.clear_color(), before.clear_color());
        Ok(())
    })
    .unwrap();
    assert_eq!(role.group.aspects(), original);
    assert_eq!(probe.snapshot().0.draws(), before.draws());
}
fn non_frame_draws(
    before: &martensite_vulkan::RenderFrame,
) -> Vec<martensite_vulkan::PrimitiveDraw> {
    assert_eq!(
        before
            .draws()
            .iter()
            .filter(|d| d.topology() == PrimitiveTopology::Lines)
            .count(),
        1
    );
    before
        .draws()
        .iter()
        .filter(|d| d.topology() != PrimitiveTopology::Lines)
        .cloned()
        .collect()
}

struct NamedRefusingCapture;
impl NamedRefusingCapture {
    fn capture(&self) -> super::super::Outcome {
        Err(refused("named capture failure").into())
    }
}
#[test]
fn capture_error_restores_same_aspect_and_all_actual_draw_values() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let before = probe.snapshot().0;
    let original = role.aspect.clone();
    let error =
        with_frame_control(&probe.session, &scale, || NamedRefusingCapture.capture()).unwrap_err();
    assert!(error.to_string().contains("named capture failure"));
    assert_eq!(role.group.aspects(), original);
    assert_eq!(probe.snapshot().0.draws(), before.draws());
}
#[test]
fn role_rejects_wrong_context_absent_and_ambiguous_bindings() {
    let (probe, scale) = actual_scale_probe();
    let other = frame_probe::SelectionFrameProbe::new();
    assert!(role::FrameRole::resolve(&other.session, &scale).is_err());
    assert!(role::FrameRole::resolve(&probe.session, &Handle::default()).is_err());
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    assert!(role::unique_frame_group(Vec::new()).is_err());
    assert!(role::unique_frame_group(vec![role.group.clone(), role.group.clone()]).is_err());
    role.presentation.set_update_status(true);
    assert!(role::FrameRole::resolve(&probe.session, &scale).is_err());
    role.presentation.set_update_status(false);
}
#[test]
fn strict_request_and_combined_error_contracts() {
    assert!(!requested(None).unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("1"))).unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("0"))).is_err());
    let error = combine(
        Err(refused("capture").into()),
        Err(refused("restore").into()),
    )
    .unwrap_err();
    assert!(error.to_string().contains("capture") && error.to_string().contains("restore"));
}

#[test]
fn replacement_value_interference_refuses_after_restoring_case_handle() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        role.group.aspects().set_line_width(9.).unwrap();
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("replacement fields changed"));
    assert_eq!(role.group.aspects(), original);
}

#[test]
fn owned_title_mutation_refuses_positive_trace() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        scale.set_title(&martensite_tkernel::tcollection::ExtendedString::from_utf8(
            "changed",
        ));
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
    assert_eq!(role.group.aspects(), original);
}
#[test]
fn dirty_presentation_cannot_pass_frame_control_trace() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        role.presentation.set_update_status(true);
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
    assert_eq!(role.group.aspects(), original);
}

#[test]
fn automatic_palette_endpoints_cannot_change_during_control() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        scale.set_hue_range(100., 200.);
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
    assert_eq!(role.group.aspects(), original);
}

#[test]
fn same_persistence_handle_payload_mutation_refuses_positive_trace() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        scale
            .transform_persistence()
            .set_offset_2d(martensite_tkernel::ncollection::Vec2::new(7, 8))?;
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
    assert_eq!(role.group.aspects(), original);
}
#[test]
fn local_and_parent_transform_values_refuse_positive_trace() {
    let (probe, scale) = actual_scale_probe();
    let role = role::FrameRole::resolve(&probe.session, &scale).unwrap();
    let original = role.aspect.clone();
    let error = with_frame_control(&probe.session, &scale, || {
        scale.set_local_transformation(&martensite_tkmath::gp::Transform3::translation([
            1., 2., 3.,
        ]))?;
        role.presentation.set_update_status(false);
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
    assert_eq!(role.group.aspects(), original);
}

#[test]
fn equal_matrix_camera_replacement_still_refuses_changed_owner() {
    let (probe, scale) = actual_scale_probe();
    let camera = probe.session.view().camera();
    let replacement = Handle::new(Graphic3dCamera::from_camera(&camera));
    let error = with_frame_control(&probe.session, &scale, || {
        probe.session.view().view_base().set_camera(&replacement)?;
        Ok(())
    })
    .unwrap_err();
    assert!(error.to_string().contains("scene state changed"));
}
