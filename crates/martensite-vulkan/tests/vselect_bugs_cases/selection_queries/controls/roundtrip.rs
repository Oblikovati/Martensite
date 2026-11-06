use super::super::topology::{TopologyOperands, compare_text, displayed_shape, remove_named};
use super::*;

#[test]
fn real_box_global_and_solid_pick_roundtrip_has_new_ais_but_same_topology() {
    let mut session = session();
    let shape = crate::primitive_shapes::box_sized([5.; 3]).unwrap();
    session
        .display("aBox", &shape, &DisplayRequest::default())
        .unwrap();
    session.fit().unwrap();
    let mut operands = TopologyOperands::default();
    operands.bind("aBox", shape);
    check_roundtrip(&mut session, &mut operands);
    session.select_point(0, 0).unwrap();
    remove_named(&mut session, "aSelected").unwrap();
    assert!(
        operands.shape("aSelected").is_ok(),
        "topology-command alias outlives AIS removal"
    );
    session.set_object_selection_mode("aBox", 6, true).unwrap();
    check_roundtrip(&mut session, &mut operands);
}

fn check_roundtrip(session: &mut ViewerSession, operands: &mut TopologyOperands) {
    session.select_point(100, 100).unwrap();
    assert!(
        session.selected_count() > 0,
        "real source pick must select an owner"
    );
    assert!(!predicates::has_token(
        &selected::has_selected_text(session.context()),
        "0"
    ));
    let original = query_named_object(session, "aBox").unwrap();
    super::super::wire::roundtrip_checks(session, operands, false).unwrap();
    let extracted = query_named_object(session, "aSelected").unwrap();
    assert_ne!(original.object_address(), extracted.object_address());
    assert_alias_topology(session, operands);
    observations::record_scene("roundtrip", session, &["aBox", "aSelected"]);
}

fn assert_alias_topology(session: &ViewerSession, operands: &TopologyOperands) {
    assert!(
        operands
            .shape("aSelected")
            .unwrap()
            .is_same(operands.shape("aBox").unwrap())
    );
    assert!(
        displayed_shape(session, "aSelected")
            .unwrap()
            .is_same(operands.shape("aSelected").unwrap())
    );
}

#[test]
fn real_multiple_owners_keep_pick_names_aliases_and_remove_repick_lifetimes_distinct() {
    let mut session = boxes();
    pick_world(&mut session, [1., 1., 1.5], AisSelectionScheme::Replace);
    pick_world(&mut session, [6., 5.5, 5.5], AisSelectionScheme::Xor);
    let owners = selected::selected_operands(session.context()).unwrap();
    assert_eq!(owners.len(), 2);
    let mut operands = TopologyOperands::default();
    let names = operands.pick(&mut session, "aSelected").unwrap();
    assert_eq!(names, ["aSelected", "aSelected1"]);
    verify_aliases(&session, &operands, &owners, &names);
    record_multiple(&session);
    let old = query_named_object(&session, "aSelected").unwrap();
    retire_picked(&mut session, &operands, &names);
    assert_repick(&mut session, &mut operands, &old);
}

fn assert_repick(
    session: &mut ViewerSession,
    operands: &mut TopologyOperands,
    old: &Handle<dyn AisInteractiveObject>,
) {
    pick_world(session, [1., 1., 1.5], AisSelectionScheme::Replace);
    operands.pick(session, "aSelected").unwrap();
    assert_ne!(
        old.object_address(),
        query_named_object(session, "aSelected")
            .unwrap()
            .object_address()
    );
}

fn record_multiple(session: &ViewerSession) {
    observations::record_scene(
        "multiple-pick",
        session,
        &["b1", "b2", "aSelected", "aSelected1"],
    );
}

fn verify_aliases(
    session: &ViewerSession,
    operands: &TopologyOperands,
    owners: &[selected::SelectedOperand],
    names: &[String],
) {
    for (owner, name) in owners.iter().zip(names) {
        let shape = owner
            .shape
            .as_ref()
            .expect("real box owner must carry topology");
        assert!(operands.shape(name).unwrap().is_same(shape));
        assert_new_alias(session, owner, name);
    }
    assert_eq!(
        owners.len(),
        names.len(),
        "consume every selected owner in original cursor order"
    );
}

fn assert_new_alias(session: &ViewerSession, owner: &selected::SelectedOperand, name: &str) {
    let original = selected::selected_object(owner).unwrap();
    assert_ne!(
        original.object_address(),
        query_named_object(session, name).unwrap().object_address()
    );
}

fn retire_picked(session: &mut ViewerSession, operands: &TopologyOperands, names: &[String]) {
    session.select_point(0, 0).unwrap();
    for name in names {
        remove_named(session, name).unwrap();
        assert!(session.object(name).is_none());
        assert!(operands.shape(name).is_ok());
    }
    assert!(remove_named(session, "missing").is_err());
}

#[test]
fn real_compare_is_same_ignores_orientation_but_not_location_or_coincident_rebuild() {
    let shape = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let reversed = shape.reversed();
    let moved = crate::primitive_shapes::translated(&shape, [1., 0., 0.]).unwrap();
    let coincident = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    assert_eq!(compare_text(&shape, &shape), "same shapes\nequal shapes\n");
    assert!(shape.is_same(&reversed) && !shape.is_equal(&reversed));
    assert_eq!(compare_text(&shape, &reversed), "same shapes\n");
    for other in [&moved, &coincident] {
        assert!(predicates::has_token(compare_text(&shape, other), "not"));
    }
    assert!(!predicates::has_token(
        compare_text(&shape, &reversed),
        "not"
    ));
}
