//! `BOPAlgoSection`: the section of the arguments, made of the section edges and vertices
//! and the common edges of the arguments.
use super::alerts::alert;
use super::builder::perform::{no_structure, perform_on, perform_with};
use super::{AlertTooFewArguments, BOPAlgoBuilder, BOPAlgoPaveFiller, BuilderHooks};
use crate::bop_ds::DataStructure;
use crate::bop_tools::make_container;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{
    IndexedDataMapOfShapeListOfShape, IndexedMapOfShape, MapOfShape, ShapeMapHasher,
};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::{Deref, DerefMut};
#[path = "section/boundaries.rs"]
mod boundaries;
#[path = "section/build.rs"]
mod build;
#[path = "section/face_sections.rs"]
mod face_sections;
#[path = "section/progress.rs"]
mod progress;

/// The section algorithm: the General Fuse of vertices and edges, then the section.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoSection;
/// let section = BOPAlgoSection::new();
/// assert!(section.arguments().is_empty());
/// ```
pub struct BOPAlgoSection {
    builder: BOPAlgoBuilder,
}

impl Deref for BOPAlgoSection {
    type Target = BOPAlgoBuilder;
    fn deref(&self) -> &BOPAlgoBuilder {
        &self.builder
    }
}

impl DerefMut for BOPAlgoSection {
    fn deref_mut(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
}

impl Default for BOPAlgoSection {
    fn default() -> Self {
        Self::new()
    }
}

impl BOPAlgoSection {
    /// An empty section (`BOPAlgoSection()`); `BOPAlgoSection::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// `BOPAlgoSection::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            builder: BOPAlgoBuilder::with_allocator(allocator),
        }
    }

    /// Intersects the arguments with an own pave filler, then builds the section
    /// (`Perform`); `section.perform();`.
    pub fn perform(&mut self) {
        let shapes = self.builder.arguments.clone();
        perform_on(self, shapes);
    }

    /// Builds the section from the results of `filler` (`PerformWithFiller`).
    /// `section.perform_with_filler(&mut filler);`.
    pub fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        perform_with(self, filler);
    }

    /// The pipeline of the section (`PerformInternal1`).
    fn run(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        self.builder.context = filler.context_handle();
        self.check_data(filler);
        if self.builder.options().has_errors() {
            return Ok(());
        }
        self.builder.prepare();
        let Some(structure) = filler.data_structure_mut() else {
            return Err(no_structure());
        };
        self.builder.fill_images_vertices(structure)?;
        self.builder.build_result_default(ShapeEnum::Vertex)?;
        self.builder.fill_images_edges(structure)?;
        self.builder.build_result_default(ShapeEnum::Edge)?;
        self.build_section(structure)?;
        if self.builder.options().has_errors() {
            return Ok(());
        }
        self.builder.prepare_history(structure)?;
        if self.builder.options().has_errors() {
            return Ok(());
        }
        self.builder.post_treat(structure)
    }

    /// The section shape (`BuildSection`): the section vertices and edges of the faces,
    /// the edges of edge/face common blocks, and the vertices and edges shared by the
    /// boundaries of several arguments, as edges at their vertices or alone vertices.
    pub fn build_section(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        self.build_section_unbound(structure)
    }
}

impl BuilderHooks for BOPAlgoSection {
    fn perform_with_progress(
        &mut self,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        let shapes = self.builder.arguments.clone();
        super::builder::entry_progress::perform_profile_on_progress(
            self,
            shapes,
            range,
            super::builder::ranged_driver::BuilderProgressProfile::Section,
        )
    }
    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        progress::run_section_progress(self, filler, range)
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
        &mut self.builder
    }

    /// At least one argument, then the filler report merged (`CheckData`).
    fn check_data(&mut self, filler: &BOPAlgoPaveFiller) {
        if self.builder.arguments.is_empty() {
            self.builder
                .options()
                .add_error(&alert(AlertTooFewArguments));
            return;
        }
        self.builder.check_filler(filler);
    }

    fn perform_internal1(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        self.run(filler)
    }
}
