//! Argument/tool lists shared by BOPAlgo tools-based algorithms.
use super::BOPAlgoBuilder;
use super::options::normalized_collection_allocator;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::{MapOfShape, ShapeMapHasher};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::List;
use std::ops::{Deref, DerefMut};

/// The source-pinned tool storage state of `BOPAlgoToolsProvider`.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoToolsProvider;
/// assert!(BOPAlgoToolsProvider::new().tools().is_empty());
/// ```
pub struct BOPAlgoToolsProvider {
    pub(super) builder: BOPAlgoBuilder,
    tools: List<Shape>,
    tool_fence: martensite_tkbrep::toptools::MapOfShape,
}

impl Default for BOPAlgoToolsProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl BOPAlgoToolsProvider {
    /// Creates a tool provider with empty input state; `BOPAlgoToolsProvider::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains nullable options identity and normalizes owned collection policy.
    /// `BOPAlgoToolsProvider::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        let resource = normalized_collection_allocator(allocator.as_ref());
        Self {
            builder: BOPAlgoBuilder::with_allocator(allocator),
            tools: List::with_allocator(resource.clone()),
            tool_fence: MapOfShape::with_allocator(ShapeMapHasher, 1, resource),
        }
    }

    /// Adds a unique tool shape identity; `provider.add_tool(shape)`.
    pub fn add_tool(&mut self, shape: &Shape) {
        if self.tool_fence.add(shape.clone()) {
            self.tools.append(shape.clone());
        }
    }

    /// Replaces listed tools while retaining the native fence until `clear`; `provider.set_tools(&tools)`.
    pub fn set_tools(&mut self, shapes: &List<Shape>) {
        self.tools.clear();
        for shape in shapes.iter() {
            self.add_tool(shape);
        }
    }

    /// Borrows the unique tool list; `provider.tools().len()`.
    pub fn tools(&self) -> &List<Shape> {
        &self.tools
    }

    /// Clears builder and tool maps; `provider.clear()`.
    pub fn clear(&mut self) {
        self.builder.clear();
        self.tools.clear();
        self.tool_fence.clear(false);
    }
}

impl Deref for BOPAlgoToolsProvider {
    type Target = BOPAlgoBuilder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for BOPAlgoToolsProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}
