use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::prs3d::{Prs3dDimensionAspect, Prs3dLineAspect};

fn glyph_scene() -> (ViewerSession, Handle<dyn AisInteractiveObject>) {
    crate::case_view::viewer_session::register_font_file(
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "SansFont",
    )
    .unwrap();
    let driver = NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let dim = PrsDimAngleDimension::from_points(
        &Point3::new(0., 0., 0.),
        &Point3::new(50., 50., 50.),
        &Point3::new(50., 50., 100.),
    )
    .unwrap();
    let aspect = Handle::new(Prs3dDimensionAspect::new().unwrap());
    aspect.make_text_3d(true);
    aspect.text_aspect().set_font("SansFont");
    dim.set_dimension_aspect(&aspect);
    session.display_object("dim2", dim.clone().cast()).unwrap();
    (session, dim.cast())
}
fn glyph_role(session: &ViewerSession, object: &Handle<dyn AisInteractiveObject>) -> ResolvedRole {
    resolve_owner_for(
        RoleKind::TextWire,
        object.clone(),
        object.global_sel_owner(),
        session.context().main_prs_mgr().get().unwrap(),
    )
    .unwrap()
}
#[test]
fn text_wire_uses_actual_owned_glyph_aspect_and_existing_presentation() {
    let (session, object) = glyph_scene();
    let role = glyph_role(&session, &object);
    assert_eq!(
        role.aspect,
        object
            .attributes()
            .free_boundary_aspect()
            .aspect()
            .cast::<dyn Graphic3dAspectsSource>()
    );
    assert!(!role.groups.is_empty());
    assert_ne!(
        role.aspect,
        object
            .cast::<PrsDimAngleDimension>()
            .dimension_aspect()
            .line_aspect()
            .aspect()
            .cast::<dyn Graphic3dAspectsSource>()
    );
}
#[test]
fn text_wire_scope_restores_all_bindings_after_success_and_unwind() {
    let (session, object) = glyph_scene();
    let role = glyph_role(&session, &object);
    let before = identity(&session, &role);
    {
        let mut scope =
            role_scope::PresentationLineRoleScope::new(role.groups.clone(), role.aspect.clone())
                .unwrap();
        assert!(
            role.groups
                .iter()
                .all(|group| group.aspects() != role.aspect)
        );
        scope.restore().unwrap();
    }
    assert_eq!(identity(&session, &role), before);
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _scope = role_scope::PresentationLineRoleScope::new(
                role.groups.clone(),
                role.aspect.clone(),
            )
            .unwrap();
            panic!("named glyph capture failure");
        }))
        .is_err()
    );
    assert_eq!(identity(&session, &role), before);
}
#[test]
fn text_wire_refuses_2d_shaded_missing_owned_and_aliased_roles() {
    let (session, object) = glyph_scene();
    let dim = object.cast::<PrsDimAngleDimension>();
    dim.dimension_aspect().make_text_3d(false);
    let error = text_wire::aspect(&dim).unwrap_err();
    assert!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<text_wire::TextWireRefusal>()
            .is_some()
    );
    dim.dimension_aspect().make_text_3d(true);
    dim.dimension_aspect().make_text_shaded(true);
    assert!(text_wire::aspect(&dim).is_err());
    dim.dimension_aspect().make_text_shaded(false);
    let saved = dim.attributes().free_boundary_aspect();
    dim.attributes()
        .set_free_boundary_aspect(dim.dimension_aspect().line_aspect());
    assert!(text_wire::aspect(&dim).is_err());
    dim.attributes().set_free_boundary_aspect(saved);
    assert!(!glyph_role(&session, &object).groups.is_empty());
    let missing = PrsDimAngleDimension::from_points(
        &Point3::new(0., 0., 0.),
        &Point3::new(1., 0., 0.),
        &Point3::new(1., 1., 0.),
    )
    .unwrap();
    let missing_aspect = Handle::new(Prs3dDimensionAspect::new().unwrap());
    missing_aspect.make_text_3d(true);
    missing.set_dimension_aspect(&missing_aspect);
    assert!(text_wire::aspect(&missing).is_err());
}
#[test]
fn text_wire_rejects_wrong_owner_and_missing_computed_binding() {
    let (session, object) = glyph_scene();
    let (_, other) = glyph_scene();
    assert!(
        resolve_owner_for(
            RoleKind::TextWire,
            object.clone(),
            other.global_sel_owner(),
            session.context().main_prs_mgr().get().unwrap()
        )
        .is_err()
    );
    let replacement = Handle::new(
        Prs3dLineAspect::new(
            martensite_tkernel::quantity::Color::named(
                martensite_tkernel::quantity::NamedColor::RED,
            ),
            martensite_tkservice::aspect::AspectTypeOfLine::Solid,
            1.,
        )
        .unwrap(),
    );
    object.attributes().set_free_boundary_aspect(replacement);
    assert!(
        resolve_owner_for(
            RoleKind::TextWire,
            object.clone(),
            object.global_sel_owner(),
            session.context().main_prs_mgr().get().unwrap()
        )
        .is_err()
    );
}
#[test]
fn text_wire_flags_are_exclusive_and_line_metadata_is_unchanged() {
    assert_eq!(
        text_wire::requested_kind(false, true).unwrap(),
        Some(RoleKind::TextWire)
    );
    assert!(text_wire::requested_kind(true, true).is_err());
    assert_eq!(
        text_wire::requested_kind(true, false).unwrap(),
        Some(RoleKind::DimensionLine)
    );
    assert_eq!(RoleKind::DimensionLine.metadata_name(), "angle-role.txt");
    assert_eq!(
        RoleKind::DimensionLine.empty_suffix(),
        "dimension-lines-empty"
    );
    assert_eq!(
        RoleKind::TextWire.metadata_name(),
        "angle-text-wire-role.txt"
    );
}

#[test]
fn text_wire_missing_dimension_line_aspect_returns_typed_refusal_without_mutation() {
    let (session, object) = glyph_scene();
    let role = glyph_role(&session, &object);
    let before = identity(&session, &role);
    let dimension = object.cast::<PrsDimAngleDimension>();
    let style = dimension.dimension_aspect();
    let original = style.line_aspect();
    for missing in [
        Handle::default(),
        Handle::new(Prs3dLineAspect::from_aspect(Handle::default())),
    ] {
        style.set_line_aspect(missing);
        let error = text_wire::aspect(&dimension).unwrap_err();
        assert_eq!(
            error
                .get_ref()
                .unwrap()
                .downcast_ref::<text_wire::TextWireRefusal>(),
            Some(&text_wire::TextWireRefusal::MissingAspect)
        );
        assert_eq!(identity(&session, &role), before);
    }
    style.set_line_aspect(original);
    assert_eq!(glyph_role(&session, &object).aspect, role.aspect);
}
