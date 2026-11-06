//! CPU-only actual-role controls omit an entire boundary array, never a ranked segment.
use super::super::*;
use crate::session_run::presentation_role_scope as boundary_scope;
mod capture;
#[allow(dead_code)] // Role binding does not use the shared probe's color-based diagnostics.
#[path = "../../../support/selection_frame_probe.rs"]
mod frame_probe;
pub(crate) use capture::capture_selected_boundary;
mod role;
mod snapshot;
use boundary_scope::BoundaryPatternScope;
use frame_probe::SelectionFrameProbe;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
use martensite_vulkan::{PrimitiveDraw, RenderFrame};
use role::SelectedBoundaryRole;
use snapshot::SceneSnapshot;

fn actual_persistent_probe() -> SelectionFrameProbe {
    let mut probe = SelectionFrameProbe::new();
    persistent_point::frustum_persistent_scene(&mut probe.session).unwrap();
    persistent_point::frustum_ranked_marker(&mut probe.session, (20, 200), "p1", &["b", "bz"])
        .unwrap();
    persistent_point::frustum_ranked_marker(&mut probe.session, (48, 258), "p", &["b", "bz", "p1"])
        .unwrap();
    persistent_point::frustum_persistent_camera(&probe.session).unwrap();
    probe
}

#[test]
fn authored_selected_boundary_joins_one_actual_whole_array() {
    let probe = actual_persistent_probe();
    let role = SelectedBoundaryRole::resolve(&probe.session, "b").unwrap();
    let (before, observed) = probe.snapshot();
    let draws = role.draws(&probe.session).unwrap();
    assert_eq!(draws.len(), 1);
    assert_eq!(
        draws[0].topology(),
        martensite_vulkan::PrimitiveTopology::Lines
    );
    assert_eq!(draws[0].indices().len(), 24);
    assert_eq!(role.ordered_join(before.draws(), &draws).unwrap().len(), 1);
    assert_eq!(role.ordered_join(&observed, &draws).unwrap().len(), 1);
}

#[test]
fn role_scope_preserves_other_draws_and_restores_all_actual_identities() {
    let probe = actual_persistent_probe();
    let role = SelectedBoundaryRole::resolve(&probe.session, "b").unwrap();
    let state = SceneSnapshot::capture(&probe.session);
    let bindings = boundary_scope::aspect_bindings(&role.object);
    let (before, _) = probe.snapshot();
    let draws = role.draws(&probe.session).unwrap();
    let expected = role.without_role(before.draws(), &draws).unwrap();
    let without = assert_uncovered_scope(&probe, &role, &state, &before, &expected);
    assert_eq!(state, SceneSnapshot::capture(&probe.session));
    assert_eq!(bindings, boundary_scope::aspect_bindings(&role.object));
    let restored = probe.snapshot().0;
    assert_eq!(before.draws(), restored.draws());
    report_role_control(&role, &before, &without, &restored, &expected);
}

fn assert_uncovered_scope(
    probe: &SelectionFrameProbe,
    role: &SelectedBoundaryRole,
    state: &SceneSnapshot,
    before: &RenderFrame,
    expected: &[PrimitiveDraw],
) -> RenderFrame {
    let visible_role = role.draws(&probe.session).unwrap();
    let _restore = BoundaryPatternScope::new(&role.object);
    let (without, observed) = probe.snapshot();
    assert!(role.draws(&probe.session).unwrap().is_empty());
    assert!(observed.iter().all(|draw| !visible_role.contains(draw)));
    assert_eq!(expected, without.draws());
    assert_eq!(format!("{expected:?}"), format!("{:?}", without.draws()));
    state.assert_except_role_aspects(&SceneSnapshot::capture(&probe.session), role);
    assert_eq!(before.extent(), without.extent());
    assert_eq!(before.clear_color(), without.clear_color());
    assert_eq!(frame_policies(before), frame_policies(&without));
    without
}

fn frame_policies(
    frame: &RenderFrame,
) -> Vec<((bool, bool, bool), martensite_vulkan::RenderPolygonOffset)> {
    frame
        .layers()
        .iter()
        .map(|layer| (layer.depth_policy(), layer.polygon_offset()))
        .collect()
}

#[test]
fn selected_role_refuses_an_unselected_authored_object() {
    let probe = actual_persistent_probe();
    assert!(SelectedBoundaryRole::resolve(&probe.session, "bz").is_err());
    assert!(SelectedBoundaryRole::resolve(&probe.session, "missing").is_err());
}

#[test]
fn early_error_inside_role_scope_restores_actual_frame_and_handles() {
    let probe = actual_persistent_probe();
    let role = SelectedBoundaryRole::resolve(&probe.session, "b").unwrap();
    let before = SceneSnapshot::capture(&probe.session);
    let (frame, _) = probe.snapshot();
    let result: Result<(), Failure> = {
        let _restore = BoundaryPatternScope::new(&role.object);
        Err(program_error(
            "injected control failure; expected restoration",
        ))
    };
    assert!(result.is_err());
    assert_eq!(before, SceneSnapshot::capture(&probe.session));
    assert_eq!(frame.draws(), probe.snapshot().0.draws());
}

#[test]
fn full_draw_join_refuses_missing_or_duplicate_role_draws() {
    let probe = actual_persistent_probe();
    let role = SelectedBoundaryRole::resolve(&probe.session, "b").unwrap();
    let frame = probe.snapshot().0;
    let draws = role.draws(&probe.session).unwrap();
    let omitted = role.without_role(frame.draws(), &draws).unwrap();
    assert!(role.ordered_join(&omitted, &draws).is_err());
    let mut duplicate = frame.draws().to_vec();
    duplicate.extend(draws.clone());
    assert!(role.ordered_join(&duplicate, &draws).is_err());
}

#[test]
fn equal_looking_unbound_aspect_is_not_a_semantic_role_match() {
    use martensite_tkservice::graphic3d::Graphic3dAspectsEqual as _;
    use martensite_tkv3d::prs3d::Prs3dLineAspect;
    let probe = actual_persistent_probe();
    let object = frustum_operand(&probe.session, "b").unwrap();
    let original = object.attributes().un_free_boundary_aspect().aspect();
    let copy = unbound_line_copy(&original);
    assert!(copy.is_equal(&*original));
    assert_ne!(copy.object_address(), original.object_address());
    object
        .attributes()
        .set_un_free_boundary_aspect(Handle::new(Prs3dLineAspect::from_aspect(copy)));
    assert!(SelectedBoundaryRole::resolve(&probe.session, "b").is_err());
}

fn unbound_line_copy(
    original: &Handle<martensite_tkservice::graphic3d::Graphic3dAspectLine3d>,
) -> Handle<martensite_tkservice::graphic3d::Graphic3dAspectLine3d> {
    use martensite_tkservice::graphic3d::{Graphic3dAspectLine3d, Graphic3dAspectsSource as _};
    let copy = Handle::new(
        Graphic3dAspectLine3d::new(
            Color::named(NamedColor::YELLOW),
            martensite_tkservice::aspect::AspectTypeOfLine::Solid,
            1.,
        )
        .unwrap(),
    );
    *copy.aspects_base().fields.write() = original.aspects_base().fields.read().clone();
    copy
}

fn report_role_control(
    role: &SelectedBoundaryRole,
    before: &RenderFrame,
    without: &RenderFrame,
    restored: &RenderFrame,
    expected: &[PrimitiveDraw],
) {
    let (groups, aspects) = role_group_handles(role);
    let counts = [
        before.draws().len(),
        without.draws().len(),
        restored.draws().len(),
    ];
    let unchanged = expected == without.draws();
    let restored_exact = before.draws() == restored.draws();
    println!(
        "{{\"kind\":\"CPU actual semantic role control, not GPU pixels\",\"selected_name\":\"b\",\"presentation_mode\":0,\"role\":\"unfree-boundary\",\"group_ids\":{groups:?},\"aspect_ids\":{aspects:?},\"frame_draw_counts\":{counts:?},\"all_other_final_draws_exact\":{unchanged},\"restored_final_draws_exact\":{restored_exact}}}"
    );
}

fn role_group_handles(role: &SelectedBoundaryRole) -> (Vec<usize>, Vec<usize>) {
    let groups = role
        .groups
        .iter()
        .map(|group| group.object_address() as usize)
        .collect();
    let aspects = role
        .groups
        .iter()
        .map(|group| group.aspects().object_address() as usize)
        .collect();
    (groups, aspects)
}
