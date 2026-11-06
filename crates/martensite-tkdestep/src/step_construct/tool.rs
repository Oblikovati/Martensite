//! `StepConstructTool`: what the tools encoding and decoding STEP constructs read from a session.
use crate::step_entity::step_live;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{InterfaceHGraph, InterfaceInterfaceModel};
use martensite_tkxsbase::transfer::{TransferFinderProcess, TransferTransientProcess};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// `StepConstructTool`: gives access to a work session, its model and its transfer processes.
///
/// Reading the graph with `recompute` can take a time, so avoid creating this tool, and those built
/// on it, several times.
///
/// ```
/// use martensite_tkdestep::step_construct::StepConstructTool;
/// let tool = StepConstructTool::new();
/// assert!(tool.ws().is_null() && tool.finder_process().is_null());
/// ```
#[derive(Clone, Default)]
pub struct StepConstructTool {
    my_ws: Handle<XSControlWorkSession>,
    my_finder_process: Handle<TransferFinderProcess>,
    my_transient_process: Handle<TransferTransientProcess>,
    my_h_graph: Handle<InterfaceHGraph>,
}

impl StepConstructTool {
    /// `StepConstructTool()`: an empty tool.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructTool(WS)`: a tool loaded with a session.
    pub fn with_ws(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        let mut tool = Self::default();
        tool.set_ws(ws)?;
        Ok(tool)
    }
    /// `WS`.
    pub fn ws(&self) -> &Handle<XSControlWorkSession> {
        &self.my_ws
    }
    /// `model`: the model of the session; a null dereference when not loaded.
    pub fn model(&self) -> Result<Handle<dyn InterfaceInterfaceModel>, Failure> {
        Ok(step_live(&self.my_ws, "StepConstructTool::Model")?.model())
    }
    /// `Graph(recompute)`: the graph, read through its handle.
    pub fn graph(&self, recompute: bool) -> Result<Handle<InterfaceHGraph>, Failure> {
        if recompute {
            return step_live(&self.my_ws, "StepConstructTool::Graph")?.graph();
        }
        step_live(&self.my_h_graph, "StepConstructTool::Graph")?;
        Ok(self.my_h_graph.clone())
    }
    /// `transient_process`: of reading; null when not loaded.
    pub fn transient_process(&self) -> &Handle<TransferTransientProcess> {
        &self.my_transient_process
    }
    /// `finder_process`: of writing; null when not loaded.
    pub fn finder_process(&self) -> &Handle<TransferFinderProcess> {
        &self.my_finder_process
    }
    /// `set_ws`: loads a session; false when a process cannot be obtained or is null.
    pub fn set_ws(&mut self, ws: &Handle<XSControlWorkSession>) -> Result<bool, Failure> {
        self.my_ws = Handle::default();
        self.my_transient_process = Handle::default();
        self.my_finder_process = Handle::default();
        let Some(live) = ws.get() else {
            return Ok(false);
        };
        self.my_ws = ws.clone();
        self.my_h_graph = live.h_graph()?;
        if let Some(reader) = live.transfer_reader().get() {
            self.my_transient_process = reader.transient_process();
        }
        if let Some(writer) = live.transfer_writer().get() {
            self.my_finder_process = writer.finder_process();
        }
        Ok(self.my_transient_process.is_some() && self.my_finder_process.is_some())
    }
}
