//! Derived constructor state retains the supplied nullable allocation-policy identity.
use martensite_tkbo::bop_algo::{Algo, Options, WireEdgeSet};
use martensite_tkbo::bop_algo::{
    BOPAlgoBOP, BOPAlgoCellsBuilder, BOPAlgoMakerVolume, BOPAlgoSection, BOPAlgoSplitter,
    BOPAlgoToolsProvider, BuilderFace, BuilderSolid, Operation, ShellSplitter, WireSplitter,
};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::{message::ProgressRange, standard::Failure};
use martensite_tkg3d::top_abs::Orientation;

fn allocator_cases() -> [Option<BaseAllocator>; 3] {
    [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ]
}

#[test]
fn bop_algo_tools_provider_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoToolsProvider::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.tools().is_empty() && algorithm.arguments().is_empty());
        assert!(algorithm.shape().is_null() && algorithm.context().is_null());
    }
}

#[test]
fn bop_algobop_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoBOP::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert_eq!(algorithm.operation(), Operation::Unknown);
        assert_eq!(algorithm.dims(), [-1, -1]);
        assert!(algorithm.tools().is_empty() && algorithm.arguments().is_empty());
    }
}

#[test]
fn builder_face_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BuilderFace::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.face().is_null() && algorithm.shapes().is_empty());
        assert_eq!(algorithm.orientation(), Orientation::External);
        assert!(!algorithm.is_avoid_internal_shapes());
    }
}

#[test]
fn builder_solid_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BuilderSolid::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.boxes_map().is_empty() && algorithm.shapes().is_empty());
        assert!(algorithm.context().is_null() && algorithm.areas().is_empty());
    }
}

#[test]
fn bop_algo_cells_builder_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoCellsBuilder::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.all_parts().is_null() && algorithm.shape().is_null());
        assert!(algorithm.arguments().is_empty());
    }
}

#[test]
fn bop_algo_maker_volume_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoMakerVolume::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.is_intersect() && !algorithm.is_avoid_internal_shapes());
        assert!(algorithm.bounding_box().is_null() && algorithm.faces().is_empty());
    }
}

#[test]
fn bop_algo_section_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoSection::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.arguments().is_empty() && algorithm.shape().is_null());
        assert!(algorithm.context().is_null() && algorithm.has_history());
    }
}

#[test]
fn shell_splitter_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = ShellSplitter::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.start_elements().is_empty() && algorithm.shells().is_empty());
    }
}

#[test]
fn bop_algo_splitter_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = BOPAlgoSplitter::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.arguments().is_empty() && algorithm.tools().is_empty());
        assert!(algorithm.shape().is_null() && algorithm.context().is_null());
    }
}

#[test]
fn wire_splitter_retains_nullable_identity_and_initial_state() {
    for allocator in allocator_cases() {
        let algorithm = WireSplitter::with_allocator(allocator.clone());
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
        assert!(algorithm.wes().is_none() && algorithm.context().is_null());
    }
}

#[test]
fn derived_defaults_keep_common_identity() {
    assert_common_options(BOPAlgoToolsProvider::new().options());
    assert_common_options(BOPAlgoBOP::new().options());
    assert_common_options(BuilderFace::new().options());
    assert_common_options(BuilderSolid::new().options());
    assert_common_options(BOPAlgoCellsBuilder::new().options());
    assert_common_options(BOPAlgoMakerVolume::new().options());
    assert_common_options(BOPAlgoSection::new().options());
    assert_common_options(ShellSplitter::new().options());
    assert_common_options(BOPAlgoSplitter::new().options());
    assert_common_options(WireSplitter::new().options());
}

fn assert_common_options(options: &Options) {
    let common = BaseAllocator::common();
    assert_eq!(options.allocator(), Some(&common));
}

struct AlgorithmOptionsFixture {
    options: Options,
}
impl Algo for AlgorithmOptionsFixture {
    fn options(&self) -> &Options {
        &self.options
    }
    fn perform(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        Ok(())
    }
}

#[test]
fn abstract_algorithm_owns_nullable_options_without_a_constructor_on_the_trait() {
    for allocator in allocator_cases() {
        let algorithm = AlgorithmOptionsFixture {
            options: Options::with_allocator(allocator.clone()),
        };
        assert_eq!(algorithm.options().allocator(), allocator.as_ref());
    }
}

#[test]
fn edge_set_null_policy_is_represented_by_normalized_common_list_identity() {
    for policy in allocator_cases() {
        let normalized = policy.unwrap_or_else(BaseAllocator::common);
        let set = WireEdgeSet::with_allocator(normalized.clone());
        assert_eq!(set.start_elements().allocator(), &normalized);
        assert_eq!(set.shapes().allocator(), &normalized);
        assert!(set.face().is_null() && set.shapes().is_empty());
        assert!(set.start_elements().is_empty());
    }
}
