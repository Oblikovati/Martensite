//! `BOPAlgoBOP`: the Boolean operations (common, fuse, cut, cut21) of arguments and tools.
use super::builder::perform::{
    adopt_filler, fill_images_with_results, no_structure, perform_on, perform_with,
};
use super::{BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoToolsProvider, BuilderHooks, Operation};
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::{Deref, DerefMut};

#[path = "bop/build_rc.rs"]
mod build_rc;
#[path = "bop/build_shape.rs"]
mod build_shape;
#[path = "bop/build_solid.rs"]
mod build_solid;
#[path = "bop/check.rs"]
mod check;
#[path = "bop/progress.rs"]
pub(crate) mod progress;
#[path = "bop/shape_progress.rs"]
mod shape_progress;

/// The Boolean operation of the arguments (objects) and the tools, built on the General
/// Fuse of both groups.
///
/// ```
/// use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
/// let mut bop = BOPAlgoBOP::new();
/// bop.set_operation(Operation::Cut);
/// assert_eq!(bop.operation(), Operation::Cut);
/// ```
pub struct BOPAlgoBOP {
    provider: BOPAlgoToolsProvider,
    operation: Operation,
    dims: [i32; 2],
    /// `myRC`: the splits making the result before the containers are rebuilt.
    rc: Shape,
}

impl Default for BOPAlgoBOP {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BOPAlgoBOP {
    type Target = BOPAlgoToolsProvider;
    fn deref(&self) -> &BOPAlgoToolsProvider {
        &self.provider
    }
}

impl DerefMut for BOPAlgoBOP {
    fn deref_mut(&mut self) -> &mut BOPAlgoToolsProvider {
        &mut self.provider
    }
}

impl BOPAlgoBOP {
    /// An operation of unknown type with no shapes (`BOPAlgoBOP()`); `BOPAlgoBOP::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// `BOPAlgoBOP::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            provider: BOPAlgoToolsProvider::with_allocator(allocator),
            operation: Operation::Unknown,
            dims: [-1, -1],
            rc: Shape::default(),
        }
    }

    /// Forgets the operation, the dimensions and the shapes (`Clear`); `bop.clear()`.
    pub fn clear(&mut self) {
        self.operation = Operation::Unknown;
        self.dims = [-1, -1];
        self.provider.clear();
    }

    /// Sets the operation (`SetOperation`); `bop.set_operation(Operation::Fuse)`.
    pub fn set_operation(&mut self, operation: Operation) {
        self.operation = operation;
    }

    /// The operation (`Operation`); `bop.operation()`.
    pub fn operation(&self) -> Operation {
        self.operation
    }

    /// Intersects the arguments and the tools with an own pave filler, then builds the
    /// result (`Perform`); `bop.perform();`.
    pub fn perform(&mut self) {
        Self::perform_hooks(self);
    }

    /// Builds the result from the results of `filler` (`PerformWithFiller`).
    /// `bop.perform_with_filler(&mut filler);`.
    pub fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        perform_with(self, filler);
    }

    /// Intersects the arguments and the tools of the algorithm `hooks` (a Boolean
    /// operation or an algorithm derived from it) with an own pave filler, then runs its
    /// pipeline (`Perform` with the virtual members of `hooks`);
    /// `BOPAlgoBOP::perform_hooks(&mut algo)`.
    pub fn perform_hooks<H: BopHooks>(hooks: &mut H) {
        let shapes = hooks.boolean().intersection_shapes();
        perform_on(hooks, shapes);
    }

    /// Run the whole ranged Boolean program with the actual derived receiver.
    /// Example: `BOPAlgoBOP::perform_hooks_with_progress(&mut feature, &range)?` retains its overrides.
    pub fn perform_hooks_with_progress<H: BopHooks>(
        hooks: &mut H,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        let shapes = hooks.boolean().intersection_shapes();
        let profile = super::builder::ranged_driver::BuilderProgressProfile::Boolean {
            operation: hooks.boolean().operation,
        };
        super::builder::entry_progress::perform_profile_on_progress(hooks, shapes, range, profile)
    }

    /// Build from a prepared filler with the actual derived Boolean receiver.
    /// Example: `BOPAlgoBOP::build_hooks_with_progress(&mut feature, &mut filler, &range)?` requires publication validation.
    pub fn build_hooks_with_progress<H: BopHooks>(
        hooks: &mut H,
        filler: &mut BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        progress::run_boolean_progress(hooks, filler, range)
    }

    fn intersection_shapes(&self) -> List<Shape> {
        let mut shapes = self.provider.builder.arguments.clone();
        for tool in self.provider.tools().iter() {
            shapes.append(tool.clone());
        }
        shapes
    }

    /// The operation dimensions of the arguments and the tools (`myDims`), set by the
    /// check of the operation data; `bop.dims()`.
    pub fn dims(&self) -> [i32; 2] {
        self.dims
    }

    fn builder(&self) -> &BOPAlgoBuilder {
        &self.provider.builder
    }
}

/// The virtual members of `BOPAlgoBOP` that an algorithm derived from it
/// (BRepFeatBuilder) overrides, on top of those of the General Fuse.
pub trait BopHooks: BuilderHooks {
    /// The Boolean operation state; `hooks.boolean().operation()`.
    fn boolean(&mut self) -> &mut BOPAlgoBOP;

    /// Whether an argument solid is open, so that the result is built from the face
    /// splits (`CheckArgsForOpenSolid`).
    fn check_args_for_open_solid(&mut self, structure: &DataStructure) -> Result<bool, Failure> {
        self.boolean().check_open_solid_arguments(structure)
    }
}

/// The pipeline of the Boolean operation (`PerformInternal1`) of the algorithm `hooks`:
/// the check, its `Prepare`, the General Fuse stages, the result shape, the history and
/// the post treatment; `perform_bop_internal1(&mut algo, &mut filler)?`.
pub fn perform_bop_internal1<H: BopHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
) -> Result<(), Failure> {
    adopt_filler(&mut hooks.boolean().provider.builder, filler);
    hooks.boolean().check_operation_data(filler)?;
    if hooks.boolean().builder().options().has_errors() {
        return Ok(());
    }
    hooks.prepare()?;
    if hooks.boolean().builder().options().has_errors() {
        return Ok(());
    }
    let Some(structure) = filler.data_structure_mut() else {
        return Err(no_structure());
    };
    let bop = hooks.boolean();
    if bop.has_empty_shape_warning() && bop.treat_empty_shape()? {
        return bop.provider.builder.prepare_history(structure);
    }
    if !fill_images_with_results(hooks, structure)? {
        return Ok(());
    }
    build_bop_shape(hooks, structure)?;
    let builder = &mut hooks.boolean().provider.builder;
    if builder.options().has_errors() {
        return Ok(());
    }
    builder.prepare_history(structure)?;
    if builder.options().has_errors() {
        return Ok(());
    }
    builder.post_treat(structure)
}

/// Builds the result of the algorithm `hooks` (`BuildShape`), its
/// `CheckArgsForOpenSolid` deciding for solids; `build_bop_shape(&mut algo, &ds)?`.
pub fn build_bop_shape<H: BopHooks>(
    hooks: &mut H,
    structure: &DataStructure,
) -> Result<(), Failure> {
    let open_solids =
        hooks.boolean().dims == [3, 3] && hooks.check_args_for_open_solid(structure)?;
    hooks.boolean().build_shape(structure, open_solids)
}

impl BopHooks for BOPAlgoBOP {
    fn boolean(&mut self) -> &mut BOPAlgoBOP {
        self
    }
}

impl BuilderHooks for BOPAlgoBOP {
    fn perform_with_progress(
        &mut self,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        Self::perform_hooks_with_progress(self, range)
    }
    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        Self::build_hooks_with_progress(self, filler, range)
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

    fn build_result(&mut self, kind: ShapeEnum, structure: &DataStructure) -> Result<(), Failure> {
        self.build_result_of_arguments(kind, structure)
    }

    fn perform_internal1(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        perform_bop_internal1(self, filler)
    }
}
