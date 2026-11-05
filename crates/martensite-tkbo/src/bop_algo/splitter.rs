//! `BOPAlgoSplitter`: splits the arguments by the tools; the splits of the tools stay out
//! of the result.
use super::alerts::alert;
use super::builder::perform::{perform_on, perform_with};
use super::{
    AlertTooFewArguments, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoToolsProvider, BuilderHooks,
    GeneralFuseAlgorithm,
};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::{Deref, DerefMut};

/// The splitter: the General Fuse of the arguments and the tools keeping only the splits of
/// the arguments.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoSplitter;
/// let splitter = BOPAlgoSplitter::new();
/// assert!(splitter.tools().is_empty());
/// ```
pub struct BOPAlgoSplitter {
    provider: BOPAlgoToolsProvider,
}

impl Deref for BOPAlgoSplitter {
    type Target = BOPAlgoToolsProvider;
    fn deref(&self) -> &BOPAlgoToolsProvider {
        &self.provider
    }
}

impl DerefMut for BOPAlgoSplitter {
    fn deref_mut(&mut self) -> &mut BOPAlgoToolsProvider {
        &mut self.provider
    }
}

impl Default for BOPAlgoSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl BOPAlgoSplitter {
    /// An empty splitter (`BOPAlgoSplitter()`); `BOPAlgoSplitter::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// `BOPAlgoSplitter::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            provider: BOPAlgoToolsProvider::with_allocator(allocator),
        }
    }

    /// Intersects the arguments and the tools with an own pave filler, then splits the
    /// arguments (`Perform`); `splitter.perform();`.
    pub fn perform(&mut self) {
        let shapes = self.intersection_shapes();
        perform_on(self, shapes);
    }

    fn intersection_shapes(&self) -> List<Shape> {
        let mut shapes = self.provider.builder.arguments.clone();
        for tool in self.provider.tools().iter() {
            shapes.append(tool.clone());
        }
        shapes
    }

    /// Splits the arguments with the results of `filler` (`PerformWithFiller`).
    /// `splitter.perform_with_filler(&mut filler);`.
    pub fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        perform_with(self, filler);
    }
}

impl BuilderHooks for BOPAlgoSplitter {
    fn perform_with_progress(
        &mut self,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        let shapes = self.intersection_shapes();
        crate::bop_algo::builder::entry_progress::perform_splitter_on_progress(self, shapes, range)
    }

    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut crate::bop_algo::BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, martensite_tkernel::standard::Failure> {
        crate::bop_algo::builder::operation_progress::run_general_fuse(
            self,
            filler,
            range,
            crate::bop_algo::builder::operation_progress::BuildMode::Ranged,
        )
    }

    fn fill_in_3d_parts_with_progress(
        &mut self,
        structure: &mut crate::bop_ds::DataStructure,
        drafts: &mut crate::bop_algo::builder::SameDomainShapes,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, martensite_tkernel::standard::Failure> {
        self.general_fuse()
            .fill_in_3d_parts_with_progress(structure, drafts, range)
    }

    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.provider.builder
    }

    /// At least one argument and two shapes in all, then the filler report merged
    /// (`CheckData`).
    fn check_data(&mut self, filler: &BOPAlgoPaveFiller) {
        let tools = self.provider.tools().len();
        let builder = &mut self.provider.builder;
        if builder.arguments.is_empty() || builder.arguments.len() + tools < 2 {
            builder.options().add_error(&alert(AlertTooFewArguments));
            return;
        }
        builder.check_filler(filler);
    }

    /// The images of the arguments; after the compounds, a single argument kept as one
    /// shape is the result itself, not a compound of it (`BuildResult`).
    fn build_result(&mut self, kind: ShapeEnum, _structure: &DataStructure) -> Result<(), Failure> {
        let builder = &mut self.provider.builder;
        builder.build_result_default(kind)?;
        if kind != ShapeEnum::Compound || builder.arguments.len() != 1 {
            return Ok(());
        }
        let mut parts = ShapeIterator::new(builder.shape(), true, true);
        if let (Some(first), None) = (Iterator::next(&mut parts), Iterator::next(&mut parts)) {
            builder.shape = first;
        }
        Ok(())
    }
}

impl GeneralFuseAlgorithm for BOPAlgoSplitter {
    fn general_fuse(&self) -> &BOPAlgoBuilder {
        &self.provider.builder
    }
    fn general_fuse_mut(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.provider.builder
    }
    fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        BOPAlgoSplitter::perform_with_filler(self, filler);
    }
    fn perform_with_filler_on_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        crate::bop_algo::perform_with_filler_progress(self, filler, range)
    }
}
