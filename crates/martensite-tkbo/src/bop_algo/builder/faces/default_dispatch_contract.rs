//! Public unbound entry retains the actual ranged dispatch and legacy error boundary.
use super::BOPAlgoBuilder;
use crate::bop_algo::builder::image_progress_contract::compound_structure;
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use crate::bop_ds::DataStructure;
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;

#[test]
fn public_face_dispatch_matches_unbound_raw_empty_task_completion() {
    for parallel in [false, true] {
        let structure = compound_structure();
        let mut public = BOPAlgoBuilder::new();
        public.options_mut().set_run_parallel(parallel);
        public.build_split_faces(&structure).unwrap();
        let mut raw = BOPAlgoBuilder::new();
        raw.options_mut().set_run_parallel(parallel);
        let observed = ProgressCancellation::default();
        let completion = raw
            .build_split_faces_progress_raw(&structure, &ProgressRange::default(), &observed)
            .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert_eq!(public.images().len(), raw.images().len());
        assert!(!public.options().has_errors());
        assert_eq!(public.options().run_parallel(), parallel);
        assert!(!observed.is_cancelled());
    }
}

#[test]
fn public_face_dispatch_preserves_original_collection_failure() {
    let structure = invalid_face_pool_reference();
    let mut builder = BOPAlgoBuilder::new();
    let expected = builder
        .build_split_faces_progress_raw(
            &structure,
            &ProgressRange::default(),
            &ProgressCancellation::default(),
        )
        .unwrap_err();
    let actual = builder.build_split_faces(&structure).unwrap_err();
    assert_eq!(actual, expected);
    assert_eq!(actual.exception_type(), "Standard_OutOfRange");
    assert!(builder.images().is_empty());
}

fn invalid_face_pool_reference() -> DataStructure {
    let mut structure = compound_structure();
    let face = (0..structure.nb_source_shapes())
        .find(|index| structure.shape_info(*index).unwrap().shape_type() == ShapeEnum::Face)
        .unwrap();
    structure
        .change_shape_info(face)
        .unwrap()
        .set_reference(i32::MAX);
    structure
}
