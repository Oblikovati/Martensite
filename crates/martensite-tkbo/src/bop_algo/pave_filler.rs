//! Native `BOPAlgoPaveFiller::Init` state and broad-phase preparation.
#[path = "pave_filler/degenerated.rs"]
mod degenerated;
#[path = "pave_filler/edge_edge.rs"]
mod edge_edge;
#[path = "pave_filler/edge_face.rs"]
mod edge_face;
#[path = "pave_filler/face_face_perform.rs"]
mod face_face_perform;
#[cfg(test)]
#[path = "pave_filler/face_face_perform_tests.rs"]
mod face_face_perform_tests;
#[path = "pave_filler/face_face_prepare.rs"]
mod face_face_prepare;
#[path = "pave_filler/face_face_store.rs"]
mod face_face_store;
#[path = "pave_filler/face_face_task.rs"]
mod face_face_task;
#[cfg(test)]
#[path = "pave_filler/face_face_task_tests.rs"]
mod face_face_task_tests;
#[path = "pave_filler/face_face_tasks.rs"]
mod face_face_tasks;
#[path = "pave_filler/initialization.rs"]
mod initialization;
#[path = "pave_filler/make_blocks.rs"]
mod make_blocks;
#[cfg(test)]
#[path = "pave_filler/owned_progress_fixture.rs"]
pub(in crate::bop_algo) mod owned_progress_fixture;
#[path = "pave_filler/pcurves.rs"]
mod pcurves;
#[path = "pave_filler/perform.rs"]
mod perform;
#[path = "pave_filler/preparation.rs"]
mod preparation;
#[path = "pave_filler/progress_observation.rs"]
mod progress_observation;
#[path = "pave_filler/progress_pairs.rs"]
mod progress_pairs;
#[path = "pave_filler/repeat_intersection.rs"]
mod repeat_intersection;
#[path = "pave_filler/sd_updates.rs"]
mod sd_updates;
#[cfg(test)]
#[path = "pave_filler/section_attribute_tests.rs"]
mod section_attribute_tests;
#[path = "pave_filler/self_interference.rs"]
mod self_interference;
#[path = "pave_filler/shrunk_data.rs"]
mod shrunk_data;
#[path = "pave_filler/split_edges.rs"]
mod split_edges;
#[path = "pave_filler/vertex_edge.rs"]
mod vertex_edge;
#[path = "pave_filler/vertex_face.rs"]
mod vertex_face;
#[path = "pave_filler/vertex_vertex.rs"]
mod vertex_vertex;
#[path = "pave_filler/weights.rs"]
mod weights;
use super::options::normalized_collection_allocator;
use super::{Glue, Options, SectionAttribute};
use crate::{
    bop_ds::{BOPDSIterator, DataStructure, Pair, PaveBlockHandle},
    int_tools::Context,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::{
    ncollection::{DataMap, DefaultHashPolicy, List, Map as IndexMap},
    standard::Handle,
};
use std::collections::HashMap;

/// `BOPAlgoPaveFiller::EdgeRangeDistance`: the distance of an edge range to a face that it
/// does not intersect; `EdgeRangeDistance { first: 0., last: 1., distance: 0.5 }`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeRangeDistance {
    /// First parameter of the range.
    pub first: f64,
    /// Last parameter of the range.
    pub last: f64,
    /// Minimal distance between the range and the face.
    pub distance: f64,
}

/// `BOPAlgoPaveFiller` through data-structure and iterator preparation.
/// Boolean intersection stages are not attached yet; `filler.initialize()?` prepares candidates.
pub struct BOPAlgoPaveFiller {
    options: Options,
    arguments: List<Shape>,
    data_structure: Option<DataStructure>,
    context: Handle<Context>,
    iterator: Option<BOPDSIterator>,
    non_destructive: bool,
    glue: Glue,
    avoid_build_pcurve: bool,
    is_primary: bool,
    vertices_to_avoid_extension: Vec<i32>,
    increased_ss: IndexMap<i32>,
    fpb_done: HashMap<i32, Vec<PaveBlockHandle>>,
    distances: DataMap<Pair, Vec<EdgeRangeDistance>>,
    section_attribute: SectionAttribute,
}

impl Default for BOPAlgoPaveFiller {
    fn default() -> Self {
        Self::new()
    }
}

impl BOPAlgoPaveFiller {
    /// Uses native defaults for primary intersection preparation; `BOPAlgoPaveFiller::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied allocation-policy identity, including null.
    /// `BOPAlgoPaveFiller::with_allocator(None)` uses no explicit allocation policy.
    /// Collection policy identity does not redirect Rust collection storage.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        let resource = normalized_collection_allocator(allocator.as_ref());
        Self {
            options: Options::with_allocator(allocator),
            arguments: List::default(),
            data_structure: None,
            context: Handle::default(),
            iterator: None,
            non_destructive: false,
            glue: Glue::Off,
            avoid_build_pcurve: false,
            is_primary: true,
            vertices_to_avoid_extension: Vec::new(),
            increased_ss: IndexMap::with_allocator(DefaultHashPolicy, 1, resource.clone()),
            fpb_done: HashMap::new(),
            distances: DataMap::with_allocator(DefaultHashPolicy, 1, resource),
            section_attribute: SectionAttribute::new(),
        }
    }

    /// Replaces the argument list by copy; `filler.set_arguments(&arguments)`.
    pub fn set_arguments(&mut self, arguments: &List<Shape>) {
        // Native assignment retains the destination allocator; clone is construction.
        self.arguments.assign(arguments);
    }

    /// Adopts the source allocator and nodes, leaving its header empty and reusable.
    /// `filler.set_arguments_moved(&mut arguments)` preserves argument order.
    pub fn set_arguments_moved(&mut self, arguments: &mut List<Shape>) {
        // The rvalue overload adopts the source allocator rather than assigning values.
        self.arguments.move_assign(arguments);
    }

    /// Appends one argument without deduplication; `filler.add_argument(&shape)`.
    pub fn add_argument(&mut self, shape: &Shape) {
        self.arguments.append(shape.clone());
    }

    /// Borrows configured arguments; `filler.arguments().len()`.
    pub fn arguments(&self) -> &List<Shape> {
        &self.arguments
    }

    /// Copies intersection flags, retained by Clear; `filler.set_section_attribute(&flags)`.
    pub fn set_section_attribute(&mut self, flags: &SectionAttribute) {
        self.section_attribute = *flags;
    }

    /// Clears report and prepared DS/iterator while retaining configured arguments and options.
    /// `filler.clear()`.
    pub fn clear(&mut self) {
        self.options.clear();
        self.data_structure = None;
        self.iterator = None;
        self.increased_ss.clear(false);
    }

    /// Borrows the prepared data structure when initialization reached that stage.
    /// `filler.data_structure().is_some()`.
    pub fn data_structure(&self) -> Option<&DataStructure> {
        self.data_structure.as_ref()
    }

    /// Mutably borrows the initialized DS as native `PDS()` does; `filler.data_structure_mut().unwrap()`.
    pub fn data_structure_mut(&mut self) -> Option<&mut DataStructure> {
        self.data_structure.as_mut()
    }

    /// Borrows the geometric context created for broad-phase OBB checks.
    /// `filler.context().is_some()`.
    pub fn context(&self) -> Option<&Context> {
        self.context.get()
    }

    /// The shared geometric context (`Context`), null before `initialize`;
    /// `filler.context_handle().is_null()`.
    pub fn context_handle(&self) -> Handle<Context> {
        self.context.clone()
    }

    /// Borrows prepared AABB/OBB candidates; `filler.iterator().is_some()`.
    pub fn iterator(&self) -> Option<&BOPDSIterator> {
        self.iterator.as_ref()
    }

    /// Mutably borrows prepared pair traversal; `filler.iterator_mut().unwrap().initialize(...)`.
    pub fn iterator_mut(&mut self) -> Option<&mut BOPDSIterator> {
        self.iterator.as_mut()
    }

    /// Configures non-destructive operation; `filler.set_non_destructive(true)`.
    pub fn set_non_destructive(&mut self, enabled: bool) {
        self.non_destructive = enabled;
    }

    /// Reads configured non-destructive operation; `filler.non_destructive()`.
    pub fn non_destructive(&self) -> bool {
        self.non_destructive
    }

    /// Configures coincident-shape gluing; `filler.set_glue(Glue::Full)`.
    pub fn set_glue(&mut self, glue: Glue) {
        self.glue = glue;
    }

    /// Reads coincident-shape gluing mode; `filler.glue() == Glue::Off`.
    pub fn glue(&self) -> Glue {
        self.glue
    }

    /// Configures p-curve construction avoidance; `filler.set_avoid_build_pcurve(true)`.
    pub fn set_avoid_build_pcurve(&mut self, avoid: bool) {
        self.avoid_build_pcurve = avoid;
    }

    /// Reads p-curve construction avoidance; `filler.is_avoid_build_pcurve()`.
    pub fn is_avoid_build_pcurve(&self) -> bool {
        self.avoid_build_pcurve
    }

    /// Sets whether this filler is the primary operation's intersection engine.
    /// `filler.set_is_primary(false)` prevents automatic safety-mode rewriting.
    pub fn set_is_primary(&mut self, primary: bool) {
        self.is_primary = primary;
    }

    /// Reads primary-operation status; `filler.is_primary()`.
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// Reads vertices protected from later curve-end extension; `filler.vertices_to_avoid_extension()`.
    pub fn vertices_to_avoid_extension(&self) -> &[i32] {
        &self.vertices_to_avoid_extension
    }

    /// Reads sub-shapes whose tolerance grew during the operation; `filler.increased_ss()`.
    pub fn increased_ss(&self) -> &IndexMap<i32> {
        &self.increased_ss
    }

    /// Borrows inherited algorithm options; `filler.options().fuzzy_value()`.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Mutably borrows inherited algorithm options; `filler.options_mut().set_use_obb(true)`.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    /// A new data structure over the arguments, initialized with the fuzzy value (step 1 of
    /// `Init`, shared by `BOPAlgoCheckerSI::Init`).
    pub(super) fn initialize_data_structure(
        &mut self,
    ) -> Result<(), martensite_tkernel::standard::Failure> {
        let arguments: Vec<Shape> = self.arguments.iter().cloned().collect();
        self.data_structure = Some(DataStructure::with_allocator(
            self.options.allocator().cloned(),
        ));
        let structure = self
            .data_structure
            .as_mut()
            .expect("created data structure");
        structure.set_arguments(&arguments);
        structure.init(self.options.fuzzy_value())?;
        Ok(())
    }

    fn initialize_context_and_iterator(
        &mut self,
    ) -> Result<(), martensite_tkernel::standard::Failure> {
        self.context = Handle::new(Context::default());
        self.iterator = Some(BOPDSIterator::with_allocator(
            self.options.allocator().cloned(),
        ));
        let context = self.context.get().expect("created context");
        let structure = self
            .data_structure
            .as_ref()
            .expect("initialized data structure");
        let iterator = self.iterator.as_mut().expect("created iterator");
        iterator.set_run_parallel(self.options.run_parallel());
        iterator.prepare_with_context(
            structure,
            Some(context),
            self.options.use_obb(),
            self.options.fuzzy_value(),
        )?;
        Ok(())
    }

    /// Installs the context and the prepared broad-phase iterator (steps 2 and 3 of `Init`)
    /// of a derived filler; `filler.install_context_and_iterator(context, iterator)`.
    pub(super) fn install_context_and_iterator(
        &mut self,
        context: Handle<Context>,
        iterator: BOPDSIterator,
    ) {
        self.context = context;
        self.iterator = Some(iterator);
    }

    fn set_non_destructive_from_arguments(
        &mut self,
    ) -> Result<(), martensite_tkernel::standard::Failure> {
        if !self.is_primary || self.non_destructive {
            return Ok(());
        }
        for shape in self.arguments.iter() {
            if shape.locked()? {
                self.non_destructive = true;
                break;
            }
        }
        Ok(())
    }
}

/// The failure of a stage run before `initialize` built the data structure.
fn missing_structure() -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_type(
        "Standard_NullObject",
        "BOPAlgoPaveFiller: data structure is None; expected initialize() to have run",
        "",
    )
}
