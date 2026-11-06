use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::prs_dim::PrsDimDimensionSelectionMode;

fn scene() -> (ViewerSession, Handle<dyn AisInteractiveObject>) {
    let driver = NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let object = PrsDimAngleDimension::from_points(
        &Point3::new(0., 0., 0.),
        &Point3::new(50., 50., 50.),
        &Point3::new(50., 50., 100.),
    )
    .unwrap();
    session
        .display_object("dim2", object.clone().cast())
        .unwrap();
    (session, object.cast())
}
fn role(session: &ViewerSession, object: &Handle<dyn AisInteractiveObject>) -> ResolvedRole {
    let owner = PrsDimDimensionOwner::new(&object.cast(), PrsDimDimensionSelectionMode::All, 6);
    resolve_owner(
        object.clone(),
        owner.cast(),
        session.context().main_prs_mgr().get().unwrap(),
    )
    .unwrap()
}
#[test]
fn exact_angle_owner_role_uses_existing_mode_and_actual_aspect_identity() {
    let (session, object) = scene();
    let resolved = role(&session, &object);
    assert_eq!(resolved.mode, 0);
    assert!(!resolved.groups.is_empty());
    assert!(
        resolved
            .groups
            .iter()
            .all(|g| g.aspects() == resolved.aspect)
    );
}
#[test]
fn role_refuses_wrong_owner_identity_without_presentation_creation() {
    let (session, object) = scene();
    let (_, other) = scene();
    let owner = PrsDimDimensionOwner::new(&other.cast(), PrsDimDimensionSelectionMode::All, 6);
    assert!(
        resolve_owner(
            object,
            owner.cast(),
            session.context().main_prs_mgr().get().unwrap()
        )
        .is_err()
    );
}
#[test]
fn actual_role_scope_restores_exact_handles_after_success_and_unwind() {
    let (session, object) = scene();
    let resolved = role(&session, &object);
    let before: Vec<_> = resolved.groups.iter().map(|g| g.aspects()).collect();
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _scope = role_scope::PresentationLineRoleScope::new(
            resolved.groups.clone(),
            resolved.aspect.clone(),
        )
        .unwrap();
        panic!("owned capture failure");
    }));
    assert!(failed.is_err());
    assert_eq!(
        before,
        resolved
            .groups
            .iter()
            .map(|g| g.aspects())
            .collect::<Vec<_>>()
    );
    let mut scope = role_scope::PresentationLineRoleScope::new(
        resolved.groups.clone(),
        resolved.aspect.clone(),
    )
    .unwrap();
    scope.restore().unwrap();
    assert_eq!(
        before,
        resolved
            .groups
            .iter()
            .map(|g| g.aspects())
            .collect::<Vec<_>>()
    );
}
#[test]
fn role_scope_refuses_empty_and_foreign_aspect_bindings() {
    let (session, object) = scene();
    let resolved = role(&session, &object);
    assert!(
        role_scope::PresentationLineRoleScope::new(Vec::new(), resolved.aspect.clone()).is_err()
    );
    let unrelated = Handle::new(Graphic3dAspects::default()).cast();
    assert!(role_scope::PresentationLineRoleScope::new(resolved.groups, unrelated).is_err());
}
#[test]
fn missing_named_dimension_refuses_without_mutating_null_scene() {
    let driver = NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    assert!(resolve(&session).is_err());
}

#[test]
fn actual_neutral_dimension_owner_resolves_mode_zero_without_typed_local_substitution() {
    let (session, object) = scene();
    let owner = object.global_sel_owner();
    assert!(!owner.is_null());
    assert!(owner.cast::<PrsDimDimensionOwner>().is_null());
    let resolved = resolve_owner(
        object,
        owner,
        session.context().main_prs_mgr().get().unwrap(),
    )
    .unwrap();
    assert_eq!(resolved.mode, 0);
}

#[test]
fn flag_absence_is_noop_and_another_case_is_refused_before_mutation() {
    assert!(!requested(None, "any-original.png").unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("1")), "bug24133_2.png").unwrap());
    assert!(requested(Some(std::ffi::OsStr::new("1")), "other.png").is_err());
}

#[test]
fn uncomputed_text_mode_refuses_without_computing_a_substitute_presentation() {
    let (session, object) = scene();
    let owner = PrsDimDimensionOwner::new(&object.cast(), PrsDimDimensionSelectionMode::Text, 6);
    assert!(
        resolve_owner(
            object.clone(),
            owner.cast(),
            session.context().main_prs_mgr().get().unwrap()
        )
        .is_err()
    );
    assert!(object.presentations().iter().all(|p| p.mode() != 2));
}
#[test]
fn unrelated_generic_owner_of_the_same_dimension_cannot_impersonate_neutral_selection() {
    let (session, object) = scene();
    let owner =
        martensite_tkv3d::select_mgr::SelectMgrEntityOwner::with_selectable(&object.cast(), 5);
    assert!(
        resolve_owner(
            object,
            owner.cast(),
            session.context().main_prs_mgr().get().unwrap()
        )
        .is_err()
    );
}

struct NamedStaleRoleWriter;
impl DumpWitnessWriter for NamedStaleRoleWriter {
    fn write_piece(&mut self, name: &str, _: &[u8]) -> io::Result<()> {
        assert_eq!(name, "angle-role.txt");
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "named stale role fixture",
        ))
    }
}
#[test]
fn stale_role_output_is_refused_and_invalid_flags_cannot_enable_controls() {
    let (session, object) = scene();
    let role = role(&session, &object);
    let error =
        write_role_to(&mut NamedStaleRoleWriter, &role, &identity(&session, &role)).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    for flag in ["0", "garbage", ""] {
        assert!(requested(Some(std::ffi::OsStr::new(flag)), "bug24133_2.png").is_err());
    }
}

#[test]
fn foreign_default_interference_refuses_and_restores_owned_case_handles() {
    let (session, object) = scene();
    let resolved = role(&session, &object);
    let original = resolved.aspect.clone();
    let mut scope =
        role_scope::PresentationLineRoleScope::new(resolved.groups.clone(), original.clone())
            .unwrap();
    let foreign = Handle::new(Graphic3dAspects::default()).cast::<dyn Graphic3dAspectsSource>();
    resolved.groups[0].set_group_primitives_aspect(&foreign);
    assert!(scope.restore().is_err());
    assert!(
        resolved
            .groups
            .iter()
            .all(|group| group.aspects() == original)
    );
}
