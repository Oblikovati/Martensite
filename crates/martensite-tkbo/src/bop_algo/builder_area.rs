//! `BOPAlgoBuilderArea`: the state shared by the face and solid builders.
use super::Options;
use super::options::normalized_collection_allocator;
use crate::int_tools::Context;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::{BaseAllocator, DefaultHashPolicy, IndexedMap, List};
use martensite_tkernel::standard::{Failure, Handle};

pub(crate) mod progress;

/// The input shapes, the loops and areas built from them, the shapes to avoid and the
/// internal-shape option of an area builder.
///
/// ```
/// use martensite_tkbo::bop_algo::BuilderArea;
/// let mut area = BuilderArea::new();
/// area.set_avoid_internal_shapes(true);
/// assert!(area.is_avoid_internal_shapes() && area.areas().is_empty());
/// ```
pub struct BuilderArea {
    pub(super) options: Options,
    pub(super) context: Handle<Context>,
    pub(super) shapes: List<Shape>,
    pub(super) loops: List<Shape>,
    pub(super) loops_internal: List<Shape>,
    pub(super) areas: List<Shape>,
    /// `myShapesToAvoid`: keyed by `TopoDSShape::IsEqual` (orientation counts).
    pub(super) shapes_to_avoid: IndexedMap<Shape>,
    pub(super) avoid_internal_shapes: bool,
}

impl Default for BuilderArea {
    fn default() -> Self {
        Self::new()
    }
}

impl BuilderArea {
    /// Empty state with internal shapes kept (`BOPAlgoBuilderArea()`); `BuilderArea::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains nullable options identity and normalizes owned collection policy.
    /// `BuilderArea::with_allocator(None)` uses no explicit allocation policy.
    /// Collection policy identity does not redirect Rust collection storage.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        let resource = normalized_collection_allocator(allocator.as_ref());
        Self {
            options: Options::with_allocator(allocator),
            context: Handle::default(),
            shapes: List::with_allocator(resource.clone()),
            loops: List::with_allocator(resource.clone()),
            loops_internal: List::with_allocator(resource.clone()),
            areas: List::with_allocator(resource.clone()),
            shapes_to_avoid: IndexedMap::with_allocator(DefaultHashPolicy, 100, resource),
            avoid_internal_shapes: false,
        }
    }

    /// Shares a geometric context (`SetContext`); `area.set_context(context)`.
    pub fn set_context(&mut self, context: Handle<Context>) {
        self.context = context;
    }

    /// The geometric context; `area.context()`.
    pub fn context(&self) -> &Handle<Context> {
        &self.context
    }

    /// The input shapes (`Shapes`); `area.shapes().len()`.
    pub fn shapes(&self) -> &List<Shape> {
        &self.shapes
    }

    /// Sets the input shapes (`SetShapes`); `area.set_shapes(&edges)`.
    pub fn set_shapes(&mut self, shapes: &List<Shape>) {
        // Native assignment retains the destination allocator; clone is construction.
        self.shapes.assign(shapes);
    }

    /// The loops built from the shapes (`Loops`); `area.loops().len()`.
    pub fn loops(&self) -> &List<Shape> {
        &self.loops
    }

    /// The built areas (`Areas`); `area.areas().len()`.
    pub fn areas(&self) -> &List<Shape> {
        &self.areas
    }

    /// Whether internal shapes are left out of the areas (`SetAvoidInternalShapes`).
    /// `area.set_avoid_internal_shapes(true)`.
    pub fn set_avoid_internal_shapes(&mut self, avoid: bool) {
        self.avoid_internal_shapes = avoid;
    }

    /// Whether internal shapes are left out (`IsAvoidInternalShapes`).
    /// `area.is_avoid_internal_shapes()`.
    pub fn is_avoid_internal_shapes(&self) -> bool {
        self.avoid_internal_shapes
    }

    /// The report and run options; `area.options().has_errors()`.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Mutable report and run options; `area.options_mut().set_run_parallel(true)`.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }
}

/// The context an area builder created before building (`myContext`); `algorithm` names
/// the builder in the failure.
pub(super) fn required_context<'a>(
    handle: &'a Handle<Context>,
    algorithm: &str,
) -> Result<&'a Context, Failure> {
    handle.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            &format!("{algorithm}: context is null; expected the builder to create it"),
            "",
        )
    })
}
