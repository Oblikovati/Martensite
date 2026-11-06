use super::*;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkservice::graphic3d::{
    Graphic3dTransModeFlags, Graphic3dTransformPersMembers as _,
};

#[test]
fn fifth_wire_is_the_same_real_unique_topology_on_repeated_exploration() {
    let shape = box_sized([10.; 3]).unwrap();
    let first = wire_depth::frustum_distinct_wires(&shape).unwrap();
    let second = wire_depth::frustum_distinct_wires(&shape).unwrap();
    assert_eq!(first.len(), 6);
    assert_eq!(first[4].shape_type().unwrap(), ShapeEnum::Wire);
    for (left, right) in first.iter().zip(&second) {
        assert!(left.is_same(right));
    }
    for (index, wire) in first.iter().enumerate() {
        assert!(!first[..index].iter().any(|earlier| earlier.is_same(wire)));
    }
}

#[test]
fn separate_edge_owns_the_authentic_two_vertex_positions() {
    let shape = wire_depth::frustum_separate_edge().unwrap();
    let mut explorer = Explorer::new(&shape, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    let mut positions = Vec::new();
    while explorer.more() {
        let vertex = martensite_tkbrep::topods::vertex(explorer.current()).unwrap();
        let point = BRepTool::pnt(&vertex).unwrap();
        positions.push([point.x(), point.y(), point.z()]);
        explorer.next().unwrap();
    }
    assert_eq!(positions, [[10., 0., 0.], [10., 10., 0.]]);
}

#[test]
fn persistent_box_keeps_real_anchor_color_and_transparency_without_refit() {
    let mut session = frustum_cpu_session();
    persistent_point::frustum_persistent_scene(&mut session).unwrap();
    let object = frustum_operand(&session, "bz").unwrap();
    assert_eq!(object.color(), Color::named(NamedColor::GREEN));
    assert!((object.transparency() - 0.5).abs() < 1e-12);
    let persistence = object.transform_persistence();
    assert_eq!(persistence.flags(), Graphic3dTransModeFlags::ZOOM_PERS);
    let anchor = persistence.anchor_point().unwrap();
    assert_eq!([anchor.x(), anchor.y(), anchor.z()], [0., -10., 50.]);
}

#[test]
fn actual_ranked_point_marker_is_created_from_the_consumed_text_coordinates() {
    let mut session = frustum_cpu_session();
    persistent_point::frustum_persistent_scene(&mut session).unwrap();
    persistent_point::frustum_ranked_marker(&mut session, (20, 200), "p1", &["b", "bz"]).unwrap();
    let bindings = state_rows::frustum_bindings(&session, &["b", "bz", "p1"]).unwrap();
    let row = state_rows::frustum_entity_row(&session, &bindings, 1).unwrap();
    let expected = state_rows::frustum_point_from_row(&row).unwrap();
    assert_eq!(
        frustum_point_position(&frustum_operand(&session, "p1").unwrap()),
        expected
    );
    persistent_point::frustum_persistent_camera(&session).unwrap();
    assert!((session.view().view_base().scale() - 22.3).abs() < 1e-12);
}

#[test]
fn owned_ranked_text_values_are_send_and_sync_without_view_borrows() {
    fn assert_send_sync<T: Send + Sync>() {
        let marker = std::marker::PhantomData::<T>;
        let _ = marker;
    }
    assert_send_sync::<state_rows::FrustumCriterionText>();
    assert_send_sync::<Vec<String>>();
}

#[test]
fn real_ranked_edge_wire_rows_keep_actual_shape_and_last_detected_child() {
    let mut session = frustum_cpu_session();
    wire_depth::frustum_wire_scene(&mut session).unwrap();
    session.fit().unwrap();
    session.move_cursor(240, 300).unwrap();
    let bindings = state_rows::frustum_bindings(&session, &["b_5", "e"]).unwrap();
    let rows = state_rows::frustum_wire_rows(&session, &bindings).unwrap();
    assert!(!rows.is_empty());
    assert!(
        wire_depth::frustum_wire_diagnostics(&rows).is_empty(),
        "{rows:?}"
    );
    assert!(
        rows.iter()
            .any(|row| row.trim() == "Detected Child: Select3DSensitiveSegment")
    );
    assert!(session.object("b").is_none());
}
