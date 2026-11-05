//! `BOPAlgoCellsBuilder`: the split parts (cells) of the arguments, from which any
//! combination is taken into the result, optionally with materials whose internal
//! boundaries are removed.
#[path = "cells_builder/boundaries.rs"]
mod boundaries;
#[path = "cells_builder/parts.rs"]
mod parts;
#[path = "cells_builder/progress.rs"]
mod progress;
#[path = "cells_builder/unify.rs"]
mod unify;
use super::builder::history::HistorySources;
use super::builder::perform::{no_structure, perform_internal1, perform_on, perform_with};
use super::options::normalized_collection_allocator;
use super::{BOPAlgoBuilder, BOPAlgoPaveFiller, BuilderHooks};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape, ShapeIterator};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::{DataMap, DefaultHashPolicy, IndexedDataMap, List, Map};
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};

/// The split parts of the shapes and the argument shapes each part comes from (`myIndex`).
type PartOrigins = IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>;

/// The cells builder: the General Fuse of the arguments, whose split parts are added to or
/// removed from the result by the arguments they belong to and do not belong to.
///
/// ```
/// use martensite_tkbo::bop_algo::BOPAlgoCellsBuilder;
/// let cells = BOPAlgoCellsBuilder::new();
/// assert!(cells.all_parts().is_null());
/// ```
pub struct BOPAlgoCellsBuilder {
    builder: BOPAlgoBuilder,
    all_parts: Shape,
    index: PartOrigins,
    materials: DataMap<i32, List<Shape>>,
    shape_material: DataMap<Shape, i32, ShapeMapHasher>,
    map_modified: DataMap<Shape, Shape, ShapeMapHasher>,
    /// The source shapes of the General Fuse for the history of every later result.
    history_sources: HistorySources,
}

impl Default for BOPAlgoCellsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BOPAlgoCellsBuilder {
    type Target = BOPAlgoBuilder;
    fn deref(&self) -> &BOPAlgoBuilder {
        &self.builder
    }
}

impl DerefMut for BOPAlgoCellsBuilder {
    fn deref_mut(&mut self) -> &mut BOPAlgoBuilder {
        &mut self.builder
    }
}

impl BOPAlgoCellsBuilder {
    /// A builder without arguments (`BOPAlgoCellsBuilder()`); `BOPAlgoCellsBuilder::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains nullable options identity and normalizes owned collection policy.
    /// `BOPAlgoCellsBuilder::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        let resource = normalized_collection_allocator(allocator.as_ref());
        Self {
            builder: BOPAlgoBuilder::with_allocator(allocator),
            all_parts: Shape::default(),
            index: IndexedDataMap::with_allocator(ShapeMapHasher, 100, resource.clone()),
            materials: DataMap::with_allocator(DefaultHashPolicy, 100, resource.clone()),
            shape_material: DataMap::with_allocator(ShapeMapHasher, 100, resource.clone()),
            map_modified: DataMap::with_allocator(ShapeMapHasher, 100, resource.clone()),
            history_sources: Vec::new(),
        }
    }

    /// Forgets the arguments, the parts and the materials (`Clear`); `cells.clear()`.
    pub fn clear(&mut self) {
        self.builder.clear();
        self.index.clear(false);
        self.materials.clear(false);
        self.shape_material.clear(false);
        self.map_modified.clear(false);
    }

    /// The compound of all split parts (`GetAllParts`); `cells.all_parts()`.
    pub fn all_parts(&self) -> &Shape {
        &self.all_parts
    }

    /// Splits the arguments with an own pave filler and indexes the parts; the result starts
    /// empty (`Perform`); `cells.perform()`.
    pub fn perform(&mut self) {
        let shapes = self.builder.arguments.clone();
        perform_on(self, shapes);
    }

    /// Splits the arguments with the results of `filler` (`PerformWithFiller`);
    /// `cells.perform_with_filler(&mut filler)`.
    pub fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        perform_with(self, filler);
    }

    /// Empties the result and forgets the materials (`RemoveAllFromResult`);
    /// `cells.remove_all_from_result()?`.
    pub fn remove_all_from_result(&mut self) -> Result<(), Failure> {
        let mut compound = Compound::default();
        BRepBuilder::default().make_compound(&mut compound);
        self.builder.shape = compound.into_shape();
        self.materials.clear(false);
        self.shape_material.clear(false);
        self.map_modified.clear(false);
        self.prepare_history()
    }

    /// The General Fuse with the history off, then the index of the parts and an empty
    /// result; the history setting is restored only after a successful split
    /// (`PerformInternal1`).
    fn run(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        let fill_history = self.builder.has_history();
        self.builder.set_to_fill_history(false);
        perform_internal1(&mut self.builder, filler)?;
        if self.builder.options().has_errors() {
            return Ok(());
        }
        self.finish_split_index(filler)?;
        self.builder.set_to_fill_history(fill_history);
        Ok(())
    }

    /// Retain the post-split index and empty-result order; `cells.finish_split_index(filler)?`.
    fn finish_split_index(&mut self, filler: &BOPAlgoPaveFiller) -> Result<(), Failure> {
        let structure = filler.data_structure().ok_or_else(no_structure)?;
        self.history_sources = self.builder.history_sources(structure)?;
        self.index_parts()?;
        self.remove_all_from_result()
    }

    /// The history of the current result, with the unified shapes (`PrepareHistory`).
    fn prepare_history(&mut self) -> Result<(), Failure> {
        let map_modified = &self.map_modified;
        let loc_modified =
            |builder: &BOPAlgoBuilder, shape: &Shape| loc_modified(builder, map_modified, shape);
        self.builder
            .prepare_history_from(&self.history_sources, &loc_modified)
    }
}

impl BuilderHooks for BOPAlgoCellsBuilder {
    fn perform_with_progress(
        &mut self,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        let sources = self.builder.arguments.clone();
        super::builder::entry_progress::perform_general_fuse_on_progress(self, sources, range)
    }

    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        progress::run_cells_progress(self, filler, range)
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

    fn perform_internal1(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure> {
        self.run(filler)
    }
}

/// The General Fuse splits of `shape` with each split replaced by the shape it was unified
/// into, each once; without splits, the unified shape only (`LocModified`).
fn loc_modified(
    builder: &BOPAlgoBuilder,
    map_modified: &DataMap<Shape, Shape, ShapeMapHasher>,
    shape: &Shape,
) -> Option<List<Shape>> {
    let splits = builder.loc_modified(shape);
    if map_modified.is_empty() {
        return splits.cloned();
    }
    let Some(splits) = splits else {
        let unified = map_modified.seek(shape)?;
        return Some([unified.clone()].into_iter().collect());
    };
    let mut fence: Map<Shape, ShapeMapHasher> = Map::with_hasher(ShapeMapHasher, 1);
    let mut shapes = List::default();
    for split in splits.iter() {
        let split = map_modified.seek(split).unwrap_or(split);
        if fence.add(split.clone()) {
            shapes.append(split.clone());
        }
    }
    Some(shapes)
}

/// The direct children of a shape.
fn children(shape: &Shape) -> Vec<Shape> {
    ShapeIterator::new(shape, true, true).collect()
}
