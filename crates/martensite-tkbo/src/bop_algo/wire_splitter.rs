//! `BOPAlgoWireSplitter`: splits the edges of a [`WireEdgeSet`] into closed wires.
use super::alerts::alert;
use super::{AlertNullInputShapes, Options, WireEdgeSet};
use crate::bop_tools::ConnexityBlock;
use crate::int_tools::Context;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{Shape, Wire};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::{Failure, Handle};

#[path = "wire_splitter/angles.rs"]
mod angles;
#[path = "wire_splitter/edge_info.rs"]
mod edge_info;
#[path = "wire_splitter/path.rs"]
mod path;
#[path = "wire_splitter/progress.rs"]
mod progress;
#[path = "wire_splitter/split_block.rs"]
mod split_block;
#[path = "wire_splitter/tasks.rs"]
mod tasks;
pub use edge_info::EdgeInfo;
pub use progress::{WireOperationContext, WirePreparedExtension, WireSplitterHooks};

/// Builds the wires of a face from its split edges: regular connexity blocks become one
/// wire each, the others are split by the smallest clockwise angle at their vertices.
///
/// ```
/// use martensite_tkbo::bop_algo::{WireEdgeSet, WireSplitter};
/// let mut splitter = WireSplitter::new();
/// splitter.set_wes(WireEdgeSet::new());
/// splitter.perform().unwrap();
/// assert!(splitter.wes().unwrap().shapes().is_empty());
/// ```
pub struct WireSplitter {
    options: Options,
    wes: Option<WireEdgeSet>,
    blocks: Vec<ConnexityBlock>,
    context: Handle<Context>,
}

impl Default for WireSplitter {
    fn default() -> Self {
        Self::new()
    }
}

impl WireSplitter {
    /// A splitter with no edge set (`BOPAlgoWireSplitter()`); `WireSplitter::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// Collection storage is unchanged; `WireSplitter::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            options: Options::with_allocator(allocator),
            wes: None,
            blocks: Vec::new(),
            context: Handle::default(),
        }
    }

    /// Hands the edge set to split; the built wires are added to it (`SetWES`).
    /// `splitter.set_wes(set)`.
    pub fn set_wes(&mut self, wes: WireEdgeSet) {
        self.wes = Some(wes);
    }

    /// The edge set, with the built wires after `perform` (`WES`); `splitter.wes()`.
    pub fn wes(&self) -> Option<&WireEdgeSet> {
        self.wes.as_ref()
    }

    /// Takes the edge set back out of the splitter; `splitter.take_wes()`.
    pub fn take_wes(&mut self) -> Option<WireEdgeSet> {
        self.wes.take()
    }

    /// Shares a geometric context (`SetContext`); `splitter.set_context(context)`.
    pub fn set_context(&mut self, context: Handle<Context>) {
        self.context = context;
    }

    /// The geometric context, created by `perform` when none was set (`Context`).
    /// `splitter.context()`.
    pub fn context(&self) -> &Handle<Context> {
        &self.context
    }

    /// The report and run options; `splitter.options().has_errors()`.
    pub fn options(&self) -> &Options {
        &self.options
    }

    /// Mutable report and run options; `splitter.options_mut().set_run_parallel(true)`.
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.options
    }

    /// Splits the start edges of the set into wires (`Perform`); `splitter.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        super::builder_area::progress::legacy_unbound_area_result(
            self.perform_with_progress(&range),
        )
    }

    /// Reports `AlertNullInputShapes` when no edge set was given (`CheckData`).
    fn check_data(&mut self) {
        if self.wes.is_none() {
            self.options.add_error(&alert(AlertNullInputShapes));
        }
    }

    /// A wire made of `edges`, flagged closed when it has no free vertex (`MakeWire`).
    /// `WireSplitter::make_wire(&edges)?`.
    pub fn make_wire(edges: &List<Shape>) -> Result<Wire, Failure> {
        let builder = BRepBuilder::default();
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        let mut shape = wire.into_shape();
        for edge in edges.iter() {
            builder.add(&mut shape, edge)?;
        }
        shape.set_closed(BRepTool::is_closed(&shape)?)?;
        Ok(Wire::unchecked(shape))
    }
}
