use super::*;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods::vertex};
use martensite_tkernel::standard::Transient as _;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::{ais::AisPoint, prs_mgr::PrsMgrPresentableObjectMembers as _};

#[test]
fn real_finite_line_point_and_box_have_source_inputs_and_nonvacuous_discrimination() {
    let mut session = session();
    let line = geometry::display_line_scene(&mut session).unwrap();
    assert_finite_line(&line);
    let point = query_named_object(&session, "p")
        .unwrap()
        .cast::<AisPoint>();
    assert_eq!(point.component().coord(), (20., 20., 0.));
    assert_eq!(point.get().unwrap().dynamic_type().name(), "AisPoint");
    verify_box_coordinates(&session);
    session
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)
        .unwrap();
    session.fit().unwrap();
    pick_world(&mut session, [2., 8., 0.1], AisSelectionScheme::Replace);
    assert_box_only_query(&session);
    observations::record_scene("box-away-from-line", &session, &["b", "l", "p"]);
    verify_line_only_pick();
}

fn assert_finite_line(line: &Handle<martensite_tkv3d::ais::AisLine>) {
    assert_eq!(line.start_point().coord(), (0., 0., 0.));
    assert_eq!(line.end_point().coord(), (10., 10., 0.));
    assert!(!line.is_infinite());
    assert_eq!(line.get().unwrap().dynamic_type().name(), "AisLine");
}

fn verify_box_coordinates(session: &ViewerSession) {
    let shape = topology::displayed_shape(session, "b").unwrap();
    let mut lower = [f64::INFINITY; 3];
    let mut upper = [f64::NEG_INFINITY; 3];
    for item in Explorer::new(&shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap() {
        let point = BRepTool::pnt(&vertex(&item).unwrap()).unwrap();
        for (axis, value) in [point.x(), point.y(), point.z()].into_iter().enumerate() {
            lower[axis] = lower[axis].min(value);
            upper[axis] = upper[axis].max(value);
        }
    }
    assert_eq!((lower, upper), ([0.; 3], [10., 10., 0.1]));
}

fn assert_box_only_query(session: &ViewerSession) {
    assert!(session.selected_count() > 0);
    let text = objects::state_text(session, &["b", "l", "p"], false).unwrap();
    assert!(text.contains("AisShape"));
    assert!(!predicates::line_was_reported(&text), "{text:?}");
}

fn verify_line_only_pick() {
    let mut session = session();
    session
        .display_object("l", geometry::finite_line().cast())
        .unwrap();
    session
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)
        .unwrap();
    session.fit().unwrap();
    pick_world(&mut session, [5., 5., 0.], AisSelectionScheme::Replace);
    assert!(session.selected_count() > 0);
    let text = objects::state_text(&session, &["l"], false).unwrap();
    assert!(predicates::line_was_reported(&text), "{text:?}");
    observations::record_scene("line-only", &session, &["l"]);
}

#[test]
fn empty_real_selection_neutral_line_and_weak_literal_tokens_remain_fail_capable() {
    let mut session = session();
    geometry::display_line_scene(&mut session).unwrap();
    assert_eq!(session.selected_count(), 0);
    assert_empty_has_shape(&session);
    let text = objects::state_text(&session, &["b", "l", "p"], false).unwrap();
    assert!(text.starts_with("Neutral-point state:\n"));
    assert!(predicates::line_was_reported(&text));
    let missing = objects::named_text(&session, "aSelected").unwrap();
    assert_eq!(missing, "aSelected doesn't exist!\n");
    assert!(!predicates::missing_selected_literal(&missing));
    assert_weak_literals(&missing);
    assert_unsupported_query(&session);
}

fn assert_weak_literals(missing: &str) {
    assert!(predicates::missing_selected_literal(missing.trim_end()));
    assert!(predicates::has_token(
        "Check if context has selected shape: 0\n",
        "0"
    ));
    assert!(!predicates::has_token("object0 has digit0", "0"));
    assert!(!predicates::line_was_reported("ais_line"));
    assert!(
        topology::TopologyOperands::default()
            .shape("aSelected")
            .is_err()
    );
}

fn assert_unsupported_query(session: &ViewerSession) {
    use martensite_tkv3d::{ais::AisConnectedInteractive, prs_mgr::PrsMgrTypeOfPresentation3d};
    let unknown = AisConnectedInteractive::new(PrsMgrTypeOfPresentation3d::AllView).cast();
    assert!(objects::object_info(session, &unknown, &[]).is_err());
    assert!(objects::object_info(session, &Handle::default(), &[]).is_err());
}

fn assert_empty_has_shape(session: &ViewerSession) {
    let actual = selected::has_selected_text(session.context());
    assert_eq!(actual, "Check if context has selected shape: 0\n");
    assert!(predicates::has_token(&actual, "0"));
}
