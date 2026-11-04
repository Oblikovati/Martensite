//! Pure Null controls bind the actual named shapes; no image or GPU claim is made.
use super::{cases, commands};
use crate::case_view::{
    SourceViewOutput,
    viewer_session::{DisplayRequest, ViewerSession},
};
use martensite_tkv3d::{
    ais::{AisShapeParameters, AisTextLabel},
    prs_mgr::PrsMgrPresentableObjectMembers,
    prs3d::Prs3dTypeOfHlr,
};

fn scene() -> ViewerSession {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let [cone, box_shape] = cases::shapes().unwrap();
    session
        .display_many(
            &[("a", &cone), ("b", &box_shape)],
            &DisplayRequest::default(),
        )
        .unwrap();
    session
}

#[test]
fn unchanged_polygonal_assignment_establishes_own_type_on_only_the_box() {
    let session = scene();
    let box_shape = commands::shape(&session, "b").unwrap();
    let cone = commands::shape(&session, "a").unwrap();
    assert_eq!(box_shape.type_of_hlr(), Prs3dTypeOfHlr::PolyAlgo);
    assert!(!box_shape.attributes().has_own_type_of_hlr());
    commands::set_object_algorithm(&session, "b", Prs3dTypeOfHlr::PolyAlgo).unwrap();
    assert!(box_shape.attributes().has_own_type_of_hlr());
    assert!(!cone.attributes().has_own_type_of_hlr());
    session.close();
}

#[test]
fn exact_cone_assignment_in_computed_mode_keeps_box_and_global_polygonal() {
    let session = scene();
    cases::computed_view(&session).unwrap();
    assert!(session.view().view_base().computed_mode());
    commands::set_object_algorithm(&session, "a", Prs3dTypeOfHlr::Algo).unwrap();
    assert_eq!(
        commands::shape(&session, "a").unwrap().type_of_hlr(),
        Prs3dTypeOfHlr::Algo
    );
    assert_eq!(
        commands::shape(&session, "b").unwrap().type_of_hlr(),
        Prs3dTypeOfHlr::PolyAlgo
    );
    assert_eq!(
        session.context().default_drawer().type_of_hlr(),
        Prs3dTypeOfHlr::PolyAlgo
    );
    session.close();
}

#[test]
fn noncomputed_assignment_does_not_enable_computed_mode() {
    let session = scene();
    assert!(!session.view().view_base().computed_mode());
    commands::set_object_algorithm(&session, "a", Prs3dTypeOfHlr::Algo).unwrap();
    assert!(!session.view().view_base().computed_mode());
    assert_eq!(
        commands::shape(&session, "a").unwrap().type_of_hlr(),
        Prs3dTypeOfHlr::Algo
    );
    session.close();
}

#[test]
fn width_command_changes_both_actual_objects_without_overwriting_hlr_binding() {
    let session = scene();
    commands::set_object_algorithm(&session, "a", Prs3dTypeOfHlr::Algo).unwrap();
    commands::set_all_line_width(&session, &["a", "b"], 5.).unwrap();
    for name in ["a", "b"] {
        let object = session.object(name).unwrap();
        assert_eq!(session.context().width(object).unwrap(), 5.);
    }
    assert_eq!(
        commands::shape(&session, "a").unwrap().type_of_hlr(),
        Prs3dTypeOfHlr::Algo
    );
    assert_eq!(
        commands::shape(&session, "b").unwrap().type_of_hlr(),
        Prs3dTypeOfHlr::PolyAlgo
    );
    session.close();
}

#[test]
fn missing_or_nonshape_bindings_refuse_with_the_offending_name() {
    let mut session = scene();
    session
        .bind_object("label", AisTextLabel::new().cast())
        .unwrap();
    for name in ["missing", "label"] {
        let error =
            commands::set_object_algorithm(&session, name, Prs3dTypeOfHlr::Algo).unwrap_err();
        assert!(error.to_string().contains(name));
        assert!(error.to_string().contains("AisShape"));
    }
    session.close();
}

#[test]
fn unspecified_algorithm_refuses_without_mutating_named_shape() {
    let session = scene();
    let shape = commands::shape(&session, "a").unwrap();
    assert!(!shape.attributes().has_own_type_of_hlr());
    let error = commands::set_object_algorithm(&session, "a", Prs3dTypeOfHlr::NotSet).unwrap_err();
    assert!(error.to_string().contains("NotSet"));
    assert!(!shape.attributes().has_own_type_of_hlr());
    session.close();
}
