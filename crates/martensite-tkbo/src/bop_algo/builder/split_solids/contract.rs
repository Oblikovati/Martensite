//! Draft-only solids pin ordered identity and parent cancellation without invented worker geometry.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
fn solid_case() -> (BOPAlgoBuilder, DataStructure, SameDomainShapes, Shape) {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let solid = maker.shape().unwrap().clone();
    let mut structure = DataStructure::default();
    structure.set_arguments(std::slice::from_ref(&solid));
    structure.init(1e-7).unwrap();
    let mut drafts = SameDomainShapes::with_hasher(ShapeMapHasher, 1);
    drafts.bind(solid.clone(), solid.clone());
    (BOPAlgoBuilder::new(), structure, drafts, solid)
}
#[test]
fn draft_only_solid_keeps_identity_and_zero_worker_scope() {
    for parallel in [false, true] {
        let (mut builder, structure, drafts, solid) = solid_case();
        builder.options_mut().set_run_parallel(parallel);
        let (progress, trace) = indicator(0);
        let observed = ProgressCancellation::default();
        assert_eq!(
            builder
                .build_split_solids_progress_raw(&structure, &drafts, &progress.start(), &observed)
                .unwrap(),
            ProgressCompletion::Completed
        );
        let images = builder.images().seek(&solid).unwrap();
        assert_eq!(images.len(), 1);
        assert!(images.iter().next().unwrap().is_same(&solid));
        assert_eq!(trace.queries.load(std::sync::atomic::Ordering::SeqCst), 2);
    }
}
#[test]
fn split_collection_and_postjoin_cancellation_leave_images_unbound() {
    for target in [1, 2] {
        let (mut builder, structure, drafts, _) = solid_case();
        let (progress, trace) = pulse_indicator(target);
        let observed = ProgressCancellation::default();
        let completion = builder
            .build_split_solids_progress_raw(&structure, &drafts, &progress.start(), &observed)
            .unwrap();
        assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
        assert!(builder.images().is_empty());
        assert!(builder.options().has_errors());
        assert_eq!(
            trace.queries.load(std::sync::atomic::Ordering::SeqCst),
            target
        );
    }
}
