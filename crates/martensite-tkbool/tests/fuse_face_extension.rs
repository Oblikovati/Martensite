//! An external composed fusion operation edits the actual retained classification lists.
use martensite_tkbool::top_ope_brep_build::{FuseFaceExtensionState, TopOpeBRepBuildFuseFace};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::ncollection::List;

struct ComposedFusion {
    base: TopOpeBRepBuildFuseFace,
}

fn unique_shape() -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    compound.into_shape()
}

fn populate_lists(record: &mut TopOpeBRepBuildFuseFace, shapes: &[Shape; 6]) {
    let mut state = record.extension_state();
    state.internal_edges_mut().append(shapes[0].clone());
    state.external_edges_mut().append(shapes[1].clone());
    state.modified_edges_mut().append(shapes[2].clone());
    state.internal_vertices_mut().append(shapes[3].clone());
    state.external_vertices_mut().append(shapes[4].clone());
    state.modified_vertices_mut().append(shapes[5].clone());
}

fn retained_lists(record: &TopOpeBRepBuildFuseFace) -> [&List<Shape>; 6] {
    [
        record.l_intern_edge(),
        record.l_extern_edge(),
        record.l_modif_edge(),
        record.l_intern_vertex(),
        record.l_extern_vertex(),
        record.l_modif_vertex(),
    ]
}

#[test]
fn external_composition_reads_each_mutation_through_existing_queries() {
    let shapes = std::array::from_fn(|_| unique_shape());
    let mut operation = ComposedFusion {
        base: TopOpeBRepBuildFuseFace::new(),
    };
    populate_lists(&mut operation.base, &shapes);
    for (list, expected) in retained_lists(&operation.base).into_iter().zip(&shapes) {
        assert_eq!(list.len(), 1);
        assert!(list.first().unwrap().is_same(expected));
    }
    assert!(!operation.base.is_done());
    assert!(!operation.base.is_modified());
}

#[test]
fn operation_steps_retain_or_reset_the_live_extension_lists() {
    let shapes = std::array::from_fn(|_| unique_shape());
    let mut operation = TopOpeBRepBuildFuseFace::new();
    populate_lists(&mut operation, &shapes);
    operation.clear_vertex();
    assert!(
        retained_lists(&operation)
            .iter()
            .all(|list| list.len() == 1)
    );
    operation.init(&List::default(), &List::default(), 2);
    assert!(
        retained_lists(&operation)
            .iter()
            .all(|list| list.is_empty())
    );
}

#[test]
fn view_mutates_case_nodes_without_modifying_another_record() {
    let shapes = std::array::from_fn(|_| unique_shape());
    let mut first = TopOpeBRepBuildFuseFace::new();
    populate_lists(&mut first, &shapes);
    let copied = first.clone();
    let mut state = first.extension_state();
    state.internal_edges_mut().clear();
    state.external_vertices_mut().append(shapes[0].clone());
    assert!(first.l_intern_edge().is_empty());
    assert_eq!(first.l_extern_vertex().len(), 2);
    assert_eq!(copied.l_intern_edge().len(), 1);
    assert_eq!(copied.l_extern_vertex().len(), 1);
}

#[test]
fn operation_and_extension_view_preserve_thread_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FuseFaceExtensionState<'static>>();
    assert_send_sync::<TopOpeBRepBuildFuseFace>();
    assert_send_sync::<ComposedFusion>();
}
