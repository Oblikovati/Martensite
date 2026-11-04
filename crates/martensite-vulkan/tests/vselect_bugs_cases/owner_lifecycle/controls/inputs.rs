use super::*;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, ShapeIterator},
};
use martensite_tkg3d::top_abs::ShapeEnum;

fn vertex_coordinates(shape: &Shape) -> [f64; 3] {
    let vertex = topods::vertex(shape).unwrap();
    let point = BRepTool::pnt(&vertex).unwrap();
    [point.x(), point.y(), point.z()]
}

#[test]
fn authentic_edge_endpoints_and_replacement_vertex_are_unchanged() {
    let edge = obsolete_shape::case_edge().unwrap();
    let endpoints: Vec<[f64; 3]> = ShapeIterator::new(&edge, true, true)
        .map(|vertex| vertex_coordinates(&vertex))
        .collect();
    assert_eq!(endpoints.len(), 2);
    assert!(endpoints.contains(&[100., 100., 100.]));
    assert!(endpoints.contains(&[300., 400., 600.]));
    let vertex = obsolete_shape::replacement_vertex().unwrap();
    assert_eq!(vertex_coordinates(&vertex), [75., 0., 0.]);
}

#[test]
fn dormant_global_owner_recomputes_actual_vertex_and_keeps_vertex_mode() {
    let session = cpu_session();
    let context = session.context();
    let edge = obsolete_shape::case_edge().unwrap();
    let object = obsolete_shape::display_edge_for_vertex_mode(context, &edge).unwrap();
    let vertex = obsolete_shape::replace_with_vertex(&object).unwrap();
    context
        .add_or_remove_selected_object(&object.cast(), true)
        .unwrap();
    assert!(selected_brep_shape(context).is_same(&vertex));
    assert!(!selected_brep_shape(context).is_same(&edge));
    let mut modes = List::default();
    context.activated_modes(&object.cast(), &mut modes);
    assert_eq!(modes.iter().copied().collect::<Vec<i32>>(), vec![1]);
    assert!(obsolete_shape::last_selected_kind_matches(context, ShapeEnum::Vertex).unwrap());
    assert!(!obsolete_shape::last_selected_kind_matches(context, ShapeEnum::Edge).unwrap());
}
