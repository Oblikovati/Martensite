//! Raw image-stage controls preserve exact native poll placement and partial diagnostic state.
use super::BOPAlgoBuilder;
use crate::bop_algo::{
    AlertUserBreak, ProgressCancellation,
    progress_test_support::{Trace, indicator, pulse_indicator},
};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::{message::ProgressRange, standard::RuntimeTyped};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex,
};
use std::sync::atomic::Ordering;

fn source_vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0.0, 0.0))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}

fn vertex_structure() -> DataStructure {
    let mut structure = DataStructure::default();
    let source = structure.append_shape(&source_vertex(0.0));
    let target = structure.append_shape(&source_vertex(1.0));
    structure.add_shape_sd(source, target);
    structure
}

/// Reuses the actual source edge/pave-block fixture across raw stage controls.
/// Example: `let structure = edge_structure();` keeps three original source records.
pub(super) fn edge_structure() -> DataStructure {
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0))
            .unwrap()
            .shape()
            .unwrap()
            .clone();
    let mut structure = DataStructure::default();
    structure.set_arguments(std::slice::from_ref(&edge));
    structure.init(1.0e-7).unwrap();
    let index = structure.index(&edge);
    for block in structure.change_pave_blocks(index).unwrap() {
        block.lock().unwrap().set_edge(index);
    }
    structure
}

/// Reuses an untouched compound/box source closure without introducing images.
/// Example: `let structure = compound_structure();` retains all source kinds.
pub(super) fn compound_structure() -> DataStructure {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
    let mut compound = Compound::default();
    let topology = BRepBuilder::default();
    topology.make_compound(&mut compound);
    let mut shape = compound.into_shape();
    topology.add(&mut shape, maker.shape().unwrap()).unwrap();
    let mut structure = DataStructure::default();
    structure.set_arguments(&[shape]);
    structure.init(1.0e-7).unwrap();
    structure
}

fn assert_scope(trace: &Trace, name: &str) {
    let events = trace.events.lock().unwrap();
    assert!(
        events
            .iter()
            .any(|(_, names)| names.iter().any(|actual| actual == name))
    );
    assert!(trace.queries.load(Ordering::SeqCst) > 0);
}

fn assert_cancelled(builder: &BOPAlgoBuilder, observed: &ProgressCancellation, trace: &Trace) {
    assert!(observed.is_cancelled());
    assert!(builder.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(trace.queries.load(Ordering::SeqCst), 1);
}

#[test]
fn vertex_scope_poll_precedes_any_image_binding() {
    let structure = vertex_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .fill_vertices_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_cancelled(&builder, &observed, &trace);
    assert!(builder.images().is_empty());
    record_stage("vertices", "query_1", &builder, &trace);
}

#[test]
fn vertex_scope_preserves_unbound_image_relations() {
    let structure = vertex_structure();
    let mut expected = BOPAlgoBuilder::new();
    expected.fill_images_vertices(&structure).unwrap();
    let mut actual = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    actual
        .fill_vertices_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(actual.images().len(), expected.images().len());
    assert!(!actual.options().has_errors());
    assert_scope(&trace, "Filling splits of vertices");
    record_stage("vertices", "", &actual, &trace);
}

#[test]
fn edge_scope_poll_follows_actual_image_binding() {
    let structure = edge_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .fill_edges_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_cancelled(&builder, &observed, &trace);
    assert_eq!(builder.images().len(), 1);
    record_stage("edges", "query_1", &builder, &trace);
}

#[test]
fn edge_scope_preserves_successful_unbound_images() {
    let structure = edge_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    builder
        .fill_edges_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(builder.images().len(), 1);
    assert!(!observed.is_cancelled());
    assert_scope(&trace, "Filling splits of edges");
    record_stage("edges", "", &builder, &trace);
}

#[test]
fn container_poll_runs_for_nonmatching_source_kinds() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = pulse_indicator(1);
    builder
        .fill_containers_progress_raw(ShapeEnum::Wire, &structure, &progress.start(), &observed)
        .unwrap();
    assert_cancelled(&builder, &observed, &trace);
    assert!(builder.images().is_empty());
    record_stage("containers", "query_1", &builder, &trace);
}

#[test]
fn compounds_poll_and_advance_without_inventing_images() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    let observed = ProgressCancellation::default();
    let (progress, trace) = indicator(0);
    builder
        .fill_compounds_progress_raw(&structure, &progress.start(), &observed)
        .unwrap();
    assert_eq!(
        trace.queries.load(Ordering::SeqCst),
        structure.nb_source_shapes() as usize
    );
    assert_scope(&trace, "Building splits of compounds");
    assert!(builder.images().is_empty());
    record_stage("compounds", "", &builder, &trace);
}

#[test]
fn unbound_container_wrapper_uses_same_raw_pipeline() {
    let structure = compound_structure();
    let mut builder = BOPAlgoBuilder::new();
    builder
        .fill_images_containers(ShapeEnum::Wire, &structure)
        .unwrap();
    builder.fill_images_compounds(&structure).unwrap();
    let observed = ProgressCancellation::default();
    builder
        .fill_containers_progress_raw(
            ShapeEnum::Shell,
            &structure,
            &ProgressRange::default(),
            &observed,
        )
        .unwrap();
    assert!(!builder.options().has_errors());
}

fn record_stage(kind: &str, cancel: &str, builder: &BOPAlgoBuilder, trace: &Trace) {
    let record = crate::bop_algo::progress_test_support::observation(
        kind,
        cancel,
        builder.options(),
        builder.images().len(),
        trace,
    );
    println!("{record}");
}
