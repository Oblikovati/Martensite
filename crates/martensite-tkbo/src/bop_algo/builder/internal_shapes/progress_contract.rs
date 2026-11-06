//! Closed-form membership verifies copied original solids and exact cancellation boundaries.
use super::*;
use crate::{
    bop_algo::progress_test_support::{indicator, pulse_indicator},
    int_tools::Context,
};
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
fn internal_case(x: f64) -> (BOPAlgoBuilder, DataStructure, Shape, Shape) {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let solid = maker.shape().unwrap().clone();
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(x, 1., 1.), 1e-7)
        .unwrap();
    let vertex = vertex.into_shape();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[solid.clone(), vertex.clone()]);
    structure.init(1e-7).unwrap();
    let mut builder = BOPAlgoBuilder::new();
    builder.context = Handle::new(Context::default());
    (builder, structure, solid, vertex)
}
#[test]
fn internal_vertex_membership_copies_only_the_containing_original() {
    for (x, inside) in [(1., true), (4., false)] {
        let (mut builder, structure, solid, vertex) = internal_case(x);
        let (progress, trace) = indicator(0);
        let observed = ProgressCancellation::default();
        let completion = builder
            .fill_internal_shapes_progress_raw(&structure, &progress.start(), &observed)
            .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert!(!builder.options().has_errors());
        assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 2);
        assert_eq!(builder.images().is_bound(&solid), inside);
        assert!(
            ShapeIterator::new(&solid, true, true)
                .all(|s| s.shape_type().unwrap() != ShapeEnum::Vertex)
        );
        if inside {
            assert_inserted_vertex(&builder, &solid, &vertex);
        }
    }
}
fn assert_inserted_vertex(builder: &BOPAlgoBuilder, solid: &Shape, vertex: &Shape) {
    let images = builder.images().seek(solid).unwrap();
    assert_eq!(images.len(), 1);
    let copy = images.iter().next().unwrap();
    assert!(!copy.is_same(solid));
    let vertices: Vec<_> = ShapeIterator::new(copy, true, true)
        .filter(|s| s.shape_type().unwrap() == ShapeEnum::Vertex)
        .collect();
    assert_eq!(vertices.len(), 1);
    assert!(vertices[0].is_same(vertex));
    assert_eq!(vertices[0].orientation(), Orientation::Internal);
    let origins = builder.origins().seek(copy).unwrap();
    assert_eq!(origins.len(), 1);
    assert!(origins.iter().next().unwrap().is_same(solid));
}
#[test]
fn internal_shape_collection_stops_before_copy_or_origin_binding() {
    for target in [1, 2] {
        let (mut builder, structure, solid, _) = internal_case(1.);
        let (progress, trace) = pulse_indicator(target);
        let observed = ProgressCancellation::default();
        let completion = builder
            .fill_internal_shapes_progress_raw(&structure, &progress.start(), &observed)
            .unwrap();
        assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
        assert!(builder.options().has_errors());
        assert!(builder.images().is_empty());
        assert!(builder.origins().is_empty());
        assert_eq!(
            trace.queries.load(std::sync::atomic::Ordering::SeqCst),
            target
        );
        assert!(
            ShapeIterator::new(&solid, true, true)
                .all(|s| s.shape_type().unwrap() != ShapeEnum::Vertex)
        );
    }
}
#[test]
fn unobserved_tail_request_does_not_become_cancelled() {
    let (mut builder, structure, solid, vertex) = internal_case(1.);
    let (progress, trace) = pulse_indicator(3);
    let observed = ProgressCancellation::default();
    assert_eq!(
        builder
            .fill_internal_shapes_progress_raw(&structure, &progress.start(), &observed)
            .unwrap(),
        ProgressCompletion::Completed
    );
    assert!(!observed.is_cancelled());
    assert!(!builder.options().has_errors());
    assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert_inserted_vertex(&builder, &solid, &vertex);
    assert!(trace.events.lock().unwrap().iter().any(|(_, names)| {
        names
            .iter()
            .any(|name| name == "Looking for internal shapes")
    }));
}
