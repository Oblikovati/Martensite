//! Closed-form disjoint/interior fixtures drive the actual draft-classification caller.
use super::*;
use crate::{
    bop_algo::progress_test_support::{indicator, pulse_indicator},
    int_tools::Context,
};
use martensite_tkernel::standard::Handle;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn classification_case(height: Option<f64>) -> (BOPAlgoBuilder, DataStructure) {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let shape = maker.shape().unwrap().clone();
    let mut structure = DataStructure::default();
    let mut arguments = vec![shape];
    if let Some(height) = height {
        let plane = martensite_tkmath::gp::Plane3::new(
            martensite_tkmath::gp::Point3::new(1., 1., height),
            martensite_tkmath::gp::Direction3::Z,
        )
        .unwrap();
        let face =
            martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace::from_plane_bounds(
                plane,
                (-0.25, 0.25, -0.25, 0.25),
            )
            .unwrap()
            .face()
            .unwrap();
        arguments.push(face.into_shape());
    }
    structure.set_arguments(&arguments);
    structure.init(1e-7).unwrap();
    let mut builder = BOPAlgoBuilder::new();
    builder.context = Handle::new(Context::default());
    (builder, structure)
}
#[test]
fn actual_box_classification_keeps_unsplit_solids_unbound() {
    for parallel in [false, true] {
        let (mut builder, mut structure) = classification_case(None);
        builder.options_mut().set_run_parallel(parallel);
        let (progress, trace) = indicator(0);
        let observed = ProgressCancellation::default();
        let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
        let completion = builder
            .fill_in_3d_parts_progress_raw(
                &mut structure,
                &mut drafts,
                &progress.start(),
                &observed,
            )
            .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert!(drafts.is_empty());
        assert!(builder.in_parts.is_empty());
        assert!(!builder.options().has_errors());
        assert!(trace.queries.load(std::sync::atomic::Ordering::SeqCst) > 7);
    }
}
#[test]
fn actual_face_and_solid_collection_cancellation_prevents_classification() {
    for target in 1..=7 {
        let (mut builder, mut structure) = classification_case(None);
        let (progress, trace) = pulse_indicator(target);
        let observed = ProgressCancellation::default();
        let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
        let completion = builder
            .fill_in_3d_parts_progress_raw(
                &mut structure,
                &mut drafts,
                &progress.start(),
                &observed,
            )
            .unwrap();
        assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
        assert!(builder.options().has_errors());
        assert!(drafts.is_empty());
        assert!(builder.in_parts.is_empty());
        assert_eq!(
            trace.queries.load(std::sync::atomic::Ordering::SeqCst),
            target
        );
    }
}

#[test]
fn finite_face_membership_drives_actual_draft_and_internal_maps() {
    for parallel in [false, true] {
        for (height, expected) in [(2., 1), (6., 0)] {
            let (mut builder, mut structure) = classification_case(Some(height));
            builder.options_mut().set_run_parallel(parallel);
            let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
            builder
                .fill_in_3d_parts(&mut structure, &mut drafts)
                .unwrap();
            assert!(!builder.options().has_errors());
            assert_eq!(drafts.len(), expected);
            assert_eq!(builder.in_parts.len(), expected);
            for (_, faces) in builder.in_parts.items() {
                assert_eq!(faces.len(), 1);
                assert!(
                    faces
                        .iter()
                        .next()
                        .unwrap()
                        .is_same(&structure.arguments()[1])
                );
            }
        }
    }
}

#[test]
fn final_binding_poll_cancellation_prevents_draft_publication() {
    let (mut builder, mut structure) = classification_case(Some(2.));
    let (progress, trace) = indicator(0);
    let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
    builder
        .fill_in_3d_parts_progress_raw(
            &mut structure,
            &mut drafts,
            &progress.start(),
            &ProgressCancellation::default(),
        )
        .unwrap();
    let count = trace.queries.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(drafts.len(), 1);
    let (mut builder, mut structure) = classification_case(Some(2.));
    let (progress, trace) = pulse_indicator(count);
    let observed = ProgressCancellation::default();
    let mut drafts = DataMap::with_hasher(ShapeMapHasher, 1);
    let completion = builder
        .fill_in_3d_parts_progress_raw(&mut structure, &mut drafts, &progress.start(), &observed)
        .unwrap();
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(builder.options().has_errors());
    assert!(drafts.is_empty());
    assert!(builder.in_parts.is_empty());
    assert_eq!(
        trace.queries.load(std::sync::atomic::Ordering::SeqCst),
        count
    );
}
