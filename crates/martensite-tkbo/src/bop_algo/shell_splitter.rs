//! `BOPAlgoShellSplitter`: splits a set of faces into closed shells.
use super::Options;
use super::options::normalized_collection_allocator;
use crate::bop_tools::{ConnexityBlock, make_connexity_blocks_from_shapes, orient_faces_on_shell};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Shape, Shell};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

#[path = "shell_splitter/progress.rs"]
mod progress;
pub(crate) use progress::perform_shell_with_progress;
#[path = "shell_splitter/refine.rs"]
mod refine;
#[path = "shell_splitter/split_block.rs"]
mod split_block;

/// Builds closed shells from faces: a regular connexity block is one shell, the others are
/// walked face by face choosing the neighbour with the smallest angle.
///
/// ```
/// use martensite_tkbo::bop_algo::ShellSplitter;
/// let mut splitter = ShellSplitter::new();
/// splitter.perform().unwrap();
/// assert!(splitter.shells().is_empty());
/// ```
pub struct ShellSplitter {
    options: Options,
    start_shapes: List<Shape>,
    shells: List<Shape>,
    blocks: Vec<ConnexityBlock>,
}

impl Default for ShellSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellSplitter {
    /// An empty splitter (`BOPAlgoShellSplitter()`); `ShellSplitter::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains nullable options identity and normalizes owned collection policy.
    /// `ShellSplitter::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        let resource = normalized_collection_allocator(allocator.as_ref());
        Self {
            options: Options::with_allocator(allocator),
            start_shapes: List::with_allocator(resource.clone()),
            shells: List::with_allocator(resource),
            blocks: Vec::new(),
        }
    }

    /// Appends one face to split (`AddStartElement`); `splitter.add_start_element(&face)`.
    pub fn add_start_element(&mut self, face: &Shape) {
        self.start_shapes.append(face.clone());
    }

    /// The faces to split (`StartElements`); `splitter.start_elements().len()`.
    pub fn start_elements(&self) -> &List<Shape> {
        &self.start_shapes
    }

    /// The built closed shells (`Shells`); `splitter.shells().len()`.
    pub fn shells(&self) -> &List<Shape> {
        &self.shells
    }

    /// The report and run options; `splitter.options().has_errors()`.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Mutable report and run options; `splitter.options_mut().set_run_parallel(true)`.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    /// Splits the faces into shells (`Perform`); `splitter.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        super::builder_area::progress::legacy_unbound_area_result(perform_shell_with_progress(
            self, &range,
        ))
    }

    /// Execute this concrete splitter with borrowed progress and typed raw completion.
    ///
    /// `splitter.perform_with_progress(&indicator.start())` retains cancellation separately
    /// from the native report. Completed execution does not certify geometry or publication;
    /// cancelled, failed and incomplete receiver state remains diagnostic only.
    /// Inputs retain shared topology ownership: raw building may change input flags.
    /// This entry is not an isolated or non-destructive publication operation.
    pub fn perform_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> super::ProgressCompletion {
        // Concrete dispatch retains the splitter's own preparation and native report policy.
        perform_shell_with_progress(self, range)
    }

    fn prepare_blocks(&mut self) -> Result<(), Failure> {
        let start: Vec<Shape> = self.start_shapes.iter().cloned().collect();
        self.blocks.clear();
        make_connexity_blocks_from_shapes(
            &start,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            &mut self.blocks,
        )?;
        Ok(())
    }

    fn regular_shells(&mut self, scope: &ProgressScope<'_>) -> Result<Vec<usize>, Failure> {
        let mut irregular = Vec::new();
        for (position, block) in self.blocks.iter().enumerate() {
            if self.options.user_break(scope) {
                return Ok(irregular);
            }
            if !block.is_regular() {
                irregular.push(position);
                continue;
            }
            let shell = make_shell(block.shapes())?;
            shell.set_closed(true)?;
            self.shells.append(shell);
        }
        Ok(irregular)
    }

    fn append_irregular(&mut self, irregular: &[usize]) -> Result<(), Failure> {
        for position in irregular {
            for shell in self.blocks[*position].loops().iter() {
                shell.set_closed(true)?;
                self.shells.append(shell.clone());
            }
        }
        Ok(())
    }
}

/// A shell of the faces with consistently oriented faces (`MakeShell`).
fn make_shell(faces: &List<Shape>) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut shell = Shell::default();
    builder.make_shell(&mut shell);
    let mut shell = shell.into_shape();
    for face in faces.iter() {
        builder.add(&mut shell, face)?;
    }
    orient_faces_on_shell(&mut shell)?;
    Ok(shell)
}

#[cfg(test)]
#[path = "shell_splitter/progress_tests.rs"]
mod progress_tests;
