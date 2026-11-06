//! Differential behavior tests for `BRepAlgoImage` root and image links.
use martensite_tkbool::brep_algo::BRepAlgoImage;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, Vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

#[test]
fn direct_and_terminal_images_preserve_root_chain_order() {
    let source = vertex(0.0);
    let middle = vertex(1.0);
    let image = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.set_root(&source);
    links.bind(&source, &middle).unwrap();
    links.set_root(&middle);
    links.bind(&middle, &image).unwrap();

    assert_eq!(links.image(&source), vec![middle.clone()]);
    assert_eq!(links.last_image(&source), vec![image.clone()]);
    assert!(links.root(&image).unwrap().is_same(&source));
    assert!(links.image_from(&image).unwrap().is_same(&middle));
    assert_eq!(links.roots(), &[source, middle]);
}

#[test]
fn bind_add_remove_and_clear_match_image_map_state() {
    let source = vertex(0.0);
    let first = vertex(1.0);
    let second = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.bind(&source, &first).unwrap();
    links.add(&source, &second).unwrap();
    assert_eq!(links.image(&source), vec![first.clone(), second.clone()]);

    links.remove(&first).unwrap();
    assert_eq!(links.image(&source), vec![second.clone()]);
    links.remove(&second).unwrap();
    assert!(!links.has_image(&source));
    assert_eq!(links.image(&source), vec![source.clone()]);

    links.set_root(&source);
    links.clear();
    assert!(links.roots().is_empty());
    assert!(!links.is_image(&source));
}

#[test]
fn bind_many_and_add_many_keep_image_insertion_order() {
    let source = vertex(0.0);
    let first = vertex(1.0);
    let second = vertex(2.0);
    let third = vertex(3.0);
    let mut links = BRepAlgoImage::new();
    links
        .bind_many(&source, &[first.clone(), second.clone()])
        .unwrap();
    links
        .add_many(&source, std::slice::from_ref(&third))
        .unwrap();

    assert_eq!(links.image(&source), vec![first, second, third]);
}

#[test]
fn remove_root_and_replace_root_update_roots_and_links() {
    let old = vertex(0.0);
    let new = vertex(1.0);
    let image = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.set_root(&old);
    links.bind(&old, &image).unwrap();
    links.replace_root(&old, &new).unwrap();
    assert_eq!(links.roots(), std::slice::from_ref(&new));
    assert!(links.image_from(&image).unwrap().is_same(&new));

    links.remove_root(&new);
    assert!(links.roots().is_empty());
    assert!(!links.has_image(&new));
    assert!(!links.is_image(&image));
}

#[test]
fn duplicate_bindings_report_construction_errors_without_mutation() {
    let source = vertex(0.0);
    let first = vertex(1.0);
    let second = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.bind(&source, &first).unwrap();

    assert_eq!(
        links.bind(&source, &second).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
    assert_eq!(
        links
            .add(&vertex(3.0), &second)
            .unwrap_err()
            .exception_type(),
        "Standard_ConstructionError"
    );
    assert_eq!(links.image(&source), vec![first]);
}

#[test]
fn shared_image_rebinds_its_direct_generator_but_keeps_all_source_links() {
    let first_source = vertex(0.0);
    let second_source = vertex(1.0);
    let image = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.bind(&first_source, &image).unwrap();

    links.bind(&second_source, &image).unwrap();
    assert_eq!(links.image(&second_source), vec![image.clone()]);
    assert!(links.image(&first_source).contains(&image));
    assert!(links.image_from(&image).unwrap().is_same(&second_source));
    assert!(links.root(&image).unwrap().is_same(&second_source));
}

#[test]
fn compact_replaces_intermediate_links_with_terminal_images() {
    let source = vertex(0.0);
    let middle = vertex(1.0);
    let image = vertex(2.0);
    let mut links = BRepAlgoImage::new();
    links.set_root(&source);
    links.bind(&source, &middle).unwrap();
    links.bind(&middle, &image).unwrap();

    links.compact().unwrap();

    assert_eq!(links.image(&source), vec![image.clone()]);
    assert_eq!(links.image_from(&image).unwrap(), source);
}

#[test]
fn filter_removes_images_missing_from_the_selected_shape() {
    let source_a = vertex(0.0);
    let source_b = vertex(1.0);
    let image_a = vertex(2.0);
    let image_b = vertex(3.0);
    let mut links = BRepAlgoImage::new();
    links.bind(&source_a, &image_a).unwrap();
    links.bind(&source_b, &image_b).unwrap();
    let mut selected = Compound::default();
    BRepBuilder::default().make_compound(&mut selected);
    BRepBuilder::default().add(&mut selected, &image_a).unwrap();

    links
        .filter(&selected.clone().into_shape(), ShapeEnum::Vertex)
        .unwrap();

    assert!(links.has_image(&source_a));
    assert!(!links.has_image(&source_b));
}

#[test]
fn last_image_of_an_unbound_shape_is_the_shape_not_the_fallback_list() {
    let shape = vertex(2.0);
    let other = vertex(3.0);
    let mut graph = BRepAlgoImage::new();
    graph.bind(&other, &shape).unwrap();
    let last = graph.last_image(&shape);
    assert_eq!(last.len(), 1);
    assert!(last[0].is_same(&shape));
    // A bound shape whose image is itself gives the shape (its own orientation).
    graph.bind(&shape, &shape.reversed()).unwrap();
    let last = graph.last_image(&shape);
    assert_eq!(last.len(), 1);
    assert_eq!(last[0].orientation(), shape.orientation());
}

fn vertex(x: f64) -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(x, 0.0, 0.0), 1.0e-7)
        .unwrap();
    vertex.into_shape()
}

#[test]
fn empty_graph_and_missing_links_preserve_native_construction_errors() {
    let source = vertex(0.0);
    let graph = BRepAlgoImage::default();

    assert!(graph.roots().is_empty());
    assert!(!graph.has_image(&source));
    assert!(!graph.is_image(&source));
    assert_eq!(
        graph.image_from(&source).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
    assert_eq!(
        graph.root(&source).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
}

#[test]
fn removal_of_an_unbound_image_preserves_native_construction_error() {
    let source = vertex(0.0);
    let mut graph = BRepAlgoImage::new();

    assert_eq!(
        graph.remove(&source).unwrap_err().exception_type(),
        "Standard_ConstructionError"
    );
    assert!(!graph.has_image(&source));
}

#[test]
fn repeated_roots_and_unlinked_root_replacement_follow_native_list_rules() {
    let old = vertex(0.0);
    let unrelated = vertex(1.0);
    let replacement = vertex(2.0);
    let mut graph = BRepAlgoImage::new();

    graph.set_root(&old);
    graph.set_root(&old);
    graph.set_root(&unrelated);
    graph.replace_root(&old, &replacement).unwrap();

    assert_eq!(graph.roots(), &[old.clone(), old, unrelated]);
    assert!(!graph.has_image(&replacement));
}

#[test]
fn compact_preserves_roots_without_images_and_discards_unreachable_links() {
    let root = vertex(0.0);
    let no_image_root = vertex(1.0);
    let middle = vertex(2.0);
    let leaf = vertex(3.0);
    let unrelated = vertex(4.0);
    let unrelated_image = vertex(5.0);
    let mut graph = BRepAlgoImage::new();

    graph.set_root(&root);
    graph.set_root(&no_image_root);
    graph.bind(&root, &middle).unwrap();
    graph.bind(&middle, &leaf).unwrap();
    graph.bind(&unrelated, &unrelated_image).unwrap();
    graph.compact().unwrap();

    assert_eq!(graph.roots(), &[root.clone(), no_image_root.clone()]);
    assert_eq!(graph.image(&root), vec![leaf.clone()]);
    assert!(!graph.is_image(&middle));
    assert!(!graph.has_image(&unrelated));
    assert!(!graph.is_image(&unrelated_image));
    assert_eq!(graph.last_image(&no_image_root), vec![no_image_root]);
}

#[test]
fn filter_only_removes_images_of_the_requested_shape_kind() {
    let vertex_source = vertex(0.0);
    let retained_vertex = vertex(1.0);
    let edge_source = vertex(2.0);
    let edge_image = edge();
    let selected = vertex(3.0);
    let mut graph = BRepAlgoImage::new();
    graph.bind(&vertex_source, &retained_vertex).unwrap();
    graph.bind(&edge_source, &edge_image).unwrap();

    graph.filter(&selected, ShapeEnum::Vertex).unwrap();

    assert!(!graph.has_image(&vertex_source));
    assert!(graph.has_image(&edge_source));
    assert!(graph.is_image(&edge_image));
}

fn edge() -> Shape {
    let mut edge = martensite_tkbrep::topods::Edge::default();
    BRepBuilder::default().make_edge(&mut edge).unwrap();
    edge.into_shape()
}
