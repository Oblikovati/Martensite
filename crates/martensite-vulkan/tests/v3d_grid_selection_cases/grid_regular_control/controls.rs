//! Actual default-grid identities and failure cleanup run on the named CPU recorder.
use super::super::cpu::final_line_probe as probe;
use super::*;
pub(super) fn fixture() -> probe::SelectionFrameProbe {
    let probe = probe::SelectionFrameProbe::new();
    super::super::grid::prepare_shifted(&probe.session).unwrap();
    super::super::grid::plane(&probe.session, [20., 20., 30.]).unwrap();
    probe
        .session
        .set_projection(Graphic3dCameraProjection::Perspective)
        .unwrap();
    super::super::grid::plane(&probe.session, [10., 5., 10.]).unwrap();
    let _ = probe.snapshot();
    probe
}
#[test]
fn actual_public_dump_matches_owned_group_handle_without_pointer_parsing() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    let source = role::grid_dump(&role.grid).unwrap();
    let group = dump::unique(&source, b"Group").unwrap();
    assert_eq!(
        dump::unique(&group, b"this").unwrap(),
        dump::pointer(role.group.object_address() as usize).unwrap()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}
#[test]
fn regular_only_scope_restores_exact_handles_and_all_actual_draws() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    let before = probe.snapshot().0;
    with_regular(&probe.session, || {
        let removed = probe.snapshot().0;
        assert!(removed.draws().len() < before.draws().len());
        assert!(removed.draws().iter().all(|d| before.draws().contains(d)));
        assert_eq!(removed.extent(), before.extent());
        assert_eq!(removed.clear_color(), before.clear_color());
        Ok(())
    })
    .unwrap();
    assert_eq!(role.group.aspects(), role.aspect);
    assert_eq!(probe.snapshot().0.draws(), before.draws());
}
#[test]
fn capture_failure_and_role_interference_restore_the_case_handle() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    assert!(
        with_regular(&probe.session, || Err(
            refused("named capture failure").into()
        ))
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
    assert!(
        with_regular(&probe.session, || {
            role.group
                .set_group_primitives_aspect(&Handle::new(Graphic3dAspects::default()).cast());
            Ok(())
        })
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}
#[test]
fn camera_mutation_is_refused_without_hiding_scope_restoration() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    assert!(
        with_regular(&probe.session, || {
            probe
                .session
                .set_projection(Graphic3dCameraProjection::Orthographic)?;
            Ok(())
        })
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}
#[test]
fn flag_and_duplicate_public_identity_fields_fail_closed() {
    assert!(!requested(None).unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("1"))).unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("true"))).is_err());
    assert!(dump::unique(b"\"Group\": 1, \"Group\": 2", b"Group").is_err());
    assert!(dump::unique(b"\"other\": 1", b"Group").is_err());
}

#[path = "output_controls.rs"]
mod outputs;

#[test]
fn grid_mutation_refuses_even_with_exact_regular_handle_restore() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    assert!(
        with_regular(&probe.session, || {
            role.grid.set_rotation_angle(1e-12);
            Ok(())
        })
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}

#[test]
fn default_drawer_mutation_is_a_refusal_not_hidden_by_role_inverse() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    assert!(
        with_regular(&probe.session, || {
            probe
                .session
                .context()
                .default_drawer()
                .line_aspect()
                .set_width(3.)?;
            Ok(())
        })
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}
#[test]
fn null_and_duplicate_owned_group_bindings_refuse_without_rank_fallback() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    let group = dump::pointer(role.group.object_address() as usize).unwrap();
    let structure = dump::pointer(role.structure.object_address() as usize).unwrap();
    assert!(role::unique_binding(Vec::new(), &group, &structure).is_err());
    let pairs = vec![(role.structure.clone(), role.group.clone()); 2];
    assert!(role::unique_binding(pairs, &group, &structure).is_err());
}
#[test]
fn replacement_field_mutation_is_refused_after_known_handle_restore() {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    assert!(
        with_regular(&probe.session, || {
            role.group.aspects().set_line_width(3.)?;
            Ok(())
        })
        .is_err()
    );
    assert_eq!(role.group.aspects(), role.aspect);
}
