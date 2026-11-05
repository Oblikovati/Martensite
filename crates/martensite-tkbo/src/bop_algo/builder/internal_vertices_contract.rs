//! Closed-form planar membership pins vertex identity, orientation and source isolation.
use super::*;
use crate::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Face, ShapeIterator, Vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[test]
fn internal_vertex_insertion_preserves_identity_and_classifies_finite_trim() {
    for (x, expected) in [(0.0, 1), (4.0, 0)] {
        let (mut builder, structure, source, split, vertex) = planar_vertex_case(x);
        builder.fill_internal_vertices(&structure).unwrap();
        let found: Vec<_> = ShapeIterator::new(&split, true, true)
            .filter(|shape| shape.shape_type().unwrap() == ShapeEnum::Vertex)
            .collect();
        assert_eq!(found.len(), expected);
        for child in found {
            assert!(child.is_same(&vertex));
            assert_eq!(
                child.orientation(),
                martensite_tkg3d::top_abs::Orientation::Internal
            );
        }
        assert!(
            ShapeIterator::new(&source, true, true)
                .all(|shape| shape.shape_type().unwrap() != ShapeEnum::Vertex)
        );
    }
}

#[test]
fn missing_context_is_refused_before_empty_collection() {
    let mut builder = BOPAlgoBuilder::new();
    assert!(
        builder
            .fill_internal_vertices(&DataStructure::default())
            .is_err()
    );
    assert!(builder.images().is_empty());
}

fn planar_vertex_case(x: f64) -> (BOPAlgoBuilder, DataStructure, Face, Face, Vertex) {
    let source = bounded_plane();
    let split = bounded_plane();
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(x, 0., 0.), 1e-7)
        .unwrap();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[source.clone().into_shape(), vertex.clone().into_shape()]);
    structure.init(1e-7).unwrap();
    let face_index = structure.index(&source);
    let vertex_index = structure.index(&vertex);
    structure
        .change_face_info(face_index)
        .unwrap()
        .change_vertices_in()
        .add(vertex_index);
    let mut builder = BOPAlgoBuilder::new();
    builder.context = Handle::new(Context::default());
    let mut images = List::default();
    images.append(split.clone().into_shape());
    builder.images.bound(source.clone().into_shape(), images);
    (builder, structure, source, split, vertex)
}

fn bounded_plane() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn internal_vertex_pulse_cancellation_preserves_worker_and_parent_boundaries() {
    use crate::bop_algo::{
        ProgressCancellation, ProgressCompletion, progress_test_support::pulse_indicator,
    };
    for parallel in [false, true] {
        for target in 0..=4 {
            let (mut builder, structure, source, split, _) = planar_vertex_case(0.0);
            builder.options_mut().set_run_parallel(parallel);
            let (indicator, trace) = pulse_indicator(target);
            let observed = ProgressCancellation::default();
            let result = builder
                .fill_internal_vertices_progress_raw(&structure, &indicator.start(), &observed)
                .unwrap();
            assert_eq!(
                matches!(result, ProgressCompletion::Cancelled { .. }),
                (1..=3).contains(&target)
            );
            assert_eq!(builder.options().has_errors(), target == 1 || target == 3);
            assert_eq!(
                trace.queries.load(std::sync::atomic::Ordering::SeqCst),
                if target == 1 { 1 } else { 3 }
            );
            assert_eq!(
                ShapeIterator::new(&split, true, true)
                    .filter(|s| s.shape_type().unwrap() == ShapeEnum::Vertex)
                    .count(),
                usize::from(target == 0 || target == 4)
            );
            assert!(
                ShapeIterator::new(&source, true, true)
                    .all(|s| s.shape_type().unwrap() != ShapeEnum::Vertex)
            );
        }
    }
}

#[test]
fn internal_vertex_parallel_batches_publish_ordered_isolated_splits() {
    for parallel in [false, true] {
        for _ in 0..16 {
            let (mut builder, structure, source, _, vertex) = planar_vertex_case(0.0);
            let splits: Vec<_> = (0..8).map(|_| bounded_plane()).collect();
            let mut images = List::default();
            for split in &splits {
                images.append(split.clone().into_shape());
            }
            builder.images.bound(source.clone().into_shape(), images);
            builder.options_mut().set_run_parallel(parallel);
            builder.fill_internal_vertices(&structure).unwrap();
            for split in splits {
                let vertices: Vec<_> = ShapeIterator::new(&split, true, true)
                    .filter(|s| s.shape_type().unwrap() == ShapeEnum::Vertex)
                    .collect();
                assert_eq!(vertices.len(), 1);
                assert!(vertices[0].is_same(&vertex));
                assert_eq!(
                    vertices[0].orientation(),
                    martensite_tkg3d::top_abs::Orientation::Internal
                );
            }
        }
    }
}

#[path = "internal_vertices_census.rs"]
mod census;
