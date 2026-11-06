use super::super::grid::{display_frustum_grid, frustum_prefix_matches, frustum_selected_names};
use super::*;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
mod camera_contract;

#[test]
fn selection_prefix_is_cut_before_lexical_sort_and_ignores_untested_tail() {
    let actual = ["3", "10", "2"].map(str::to_owned);
    assert!(frustum_prefix_matches(&actual, &["10", "3"]));
    assert!(!frustum_prefix_matches(&actual, &["10", "2"]));
    assert!(!frustum_prefix_matches(&actual[..1], &["3", "10"]));
    assert!(frustum_prefix_matches(
        &["24".into(), "other".into()],
        &["24"]
    ));
}

#[test]
fn late_region_messages_preserve_all_three_independent_failures() {
    let errors = perspective::frustum_region_errors([false, false, false]);
    assert_eq!(
        errors,
        [
            "Error : Perspective selection by rectangle is broken.",
            "Error : Perspective selection by point is broken.",
            "Error : Perspective selection by polygon is broken.",
        ]
    );
    assert!(perspective::frustum_region_errors([true; 3]).is_empty());
}

#[test]
fn real_grid_keeps_ninety_eight_named_point_owners_in_source_order() {
    let mut session = frustum_cpu_session();
    let grid = display_frustum_grid(&mut session).unwrap();
    assert_eq!(grid.len(), 98);
    let mut identities = std::collections::HashSet::new();
    for (index, (name, object)) in grid.iter().enumerate() {
        assert_eq!(name, &index.to_string());
        assert!(identities.insert(object.object_address()));
        let expected = [
            (index % 49 / 7) as f64,
            (index % 7) as f64,
            if index < 49 { 10. } else { 0. },
        ];
        assert_eq!(frustum_point_position(object), expected);
        assert_eq!(frustum_operand(&session, name).unwrap(), *object);
    }
}

#[test]
fn selected_names_follow_real_global_owner_order_not_grid_binding_order() {
    let mut session = frustum_cpu_session();
    let grid = display_frustum_grid(&mut session).unwrap();
    session.toggle_selected("7").unwrap();
    session.toggle_selected("2").unwrap();
    assert_eq!(frustum_selected_names(&session, &grid).unwrap(), ["7", "2"]);
    assert_eq!(frustum_bound_name(&grid[7].1, &grid).unwrap(), "7");
}

#[test]
fn unknown_bound_objects_and_missing_operands_refuse_without_expected_names() {
    let session = frustum_cpu_session();
    let missing = frustum_operand(&session, "absent-point").unwrap_err();
    assert!(missing.to_string().contains("absent-point"));
    let actual = point_object([0.; 3]).unwrap();
    let error = frustum_bound_name(&actual, &Vec::new()).unwrap_err();
    assert!(error.to_string().contains("actual named operands"));
}

#[test]
fn real_region_queries_keep_source_rectangle_point_and_outside_polygon() {
    let mut session = frustum_cpu_session();
    session.set_display_mode(1).unwrap();
    let bindings = display_frustum_grid(&mut session).unwrap();
    grid::frustum_region_camera(&mut session).unwrap();
    let actual = perspective::frustum_region_checks(&mut session, &bindings).unwrap();
    assert_eq!(actual, [true; 3]);
    assert_eq!(
        session.view().camera().projection_type(),
        Graphic3dCameraProjection::Perspective
    );
}

#[test]
fn front_point_camera_and_post_selection_camera_remain_perspective() {
    let mut session = frustum_cpu_session();
    session
        .set_projection(Graphic3dCameraProjection::Perspective)
        .unwrap();
    display_frustum_grid(&mut session).unwrap();
    grid::frustum_front_camera(&mut session).unwrap();
    let before = camera_contract::frustum_mapping_before(&session);
    perspective::frustum_final_camera(&session).unwrap();
    camera_contract::frustum_final_pose(&session);
    camera_contract::frustum_final_mapping(&session, before);
}
