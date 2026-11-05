//! The `BOPAlgoBuilder*` the BRepAlgoAPI classes drive: any General Fuse algorithm run on
//! the results of a pave filler.
use super::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoSection, ProgressCompletion,
    ProgressFailureClass, ProgressOperation, ProgressStage, perform_with_filler_progress,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};

/// A General Fuse algorithm (the builder, the Boolean operation, the section, the splitter)
/// seen through its `BOPAlgoBuilder` base.
///
/// ```
/// use martensite_tkbo::bop_algo::{BOPAlgoBuilder, GeneralFuseAlgorithm};
/// let builder = BOPAlgoBuilder::new();
/// assert!(builder.general_fuse().arguments().is_empty());
/// ```
pub trait GeneralFuseAlgorithm {
    /// The `BOPAlgoBuilder` base; `algorithm.general_fuse().images()`.
    fn general_fuse(&self) -> &BOPAlgoBuilder;
    /// The mutable `BOPAlgoBuilder` base; `algorithm.general_fuse_mut().clear()`.
    fn general_fuse_mut(&mut self) -> &mut BOPAlgoBuilder;
    /// Builds the result from the results of `filler` (`PerformWithFiller`).
    fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller);
    /// Builds the result in the caller's range (`PerformWithFiller`); an algorithm without a ranged
    /// program stays incomplete rather than running a substitute.
    /// Example: `algorithm.perform_with_filler_on_progress(&mut filler, &indicator.start())?`.
    fn perform_with_filler_on_progress(
        &mut self,
        _filler: &mut BOPAlgoPaveFiller,
        _range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        Ok(ProgressCompletion::Incomplete {
            operation: ProgressOperation::Builder,
            stage: ProgressStage::Completion,
            class: ProgressFailureClass::UnqualifiedStage,
            cause: None,
            context: None,
        })
    }
}

impl GeneralFuseAlgorithm for BOPAlgoBuilder {
    fn general_fuse(&self) -> &BOPAlgoBuilder {
        self
    }
    fn general_fuse_mut(&mut self) -> &mut BOPAlgoBuilder {
        self
    }
    fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        BOPAlgoBuilder::perform_with_filler(self, filler);
    }
    fn perform_with_filler_on_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        perform_with_filler_progress(self, filler, range)
    }
}

impl GeneralFuseAlgorithm for BOPAlgoBOP {
    fn general_fuse(&self) -> &BOPAlgoBuilder {
        self.deref().deref()
    }
    fn general_fuse_mut(&mut self) -> &mut BOPAlgoBuilder {
        self.deref_mut().deref_mut()
    }
    fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        BOPAlgoBOP::perform_with_filler(self, filler);
    }
    fn perform_with_filler_on_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        perform_with_filler_progress(self, filler, range)
    }
}

impl GeneralFuseAlgorithm for BOPAlgoSection {
    fn general_fuse(&self) -> &BOPAlgoBuilder {
        self.deref()
    }
    fn general_fuse_mut(&mut self) -> &mut BOPAlgoBuilder {
        self.deref_mut()
    }
    fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        BOPAlgoSection::perform_with_filler(self, filler);
    }
    fn perform_with_filler_on_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        perform_with_filler_progress(self, filler, range)
    }
}
