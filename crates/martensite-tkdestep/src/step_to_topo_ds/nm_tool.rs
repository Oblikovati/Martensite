//! `StepToTopoDSNMTool`: the non-manifold topology shared between the shells of a representation.
use crate::step_repr::StepReprRepresentationItem;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The edges of a shape, in the order of the explorer.
fn edges_of(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut edges = Vec::new();
    let mut explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while explorer.more() {
        edges.push(explorer.current().clone());
        explorer.next()?;
    }
    Ok(edges)
}

/// `StepToTopoDSNMTool`: inactive until `set_active`; for I-DEAS-like files (ssv; 15.11.2010) the
/// vertices are also found by name.
#[derive(Default)]
pub struct StepToTopoDSNMTool {
    my_ri_map: DataMap<Handle<StepReprRepresentationItem>, Shape>,
    my_ri_names_map: DataMap<AsciiString, Shape>,
    my_nm_edges: Vec<Shape>,
    my_ideas_case: bool,
    my_active_flag: bool,
}

impl StepToTopoDSNMTool {
    /// `StepToTopoDSNMTool()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSNMTool(MapOfRI, MapOfRINames)`.
    pub fn with_maps(
        map_of_ri: &DataMap<Handle<StepReprRepresentationItem>, Shape>,
        map_of_ri_names: &DataMap<AsciiString, Shape>,
    ) -> Self {
        let mut tool = Self::new();
        tool.init(map_of_ri, map_of_ri_names);
        tool
    }
    /// `init(MapOfRI, MapOfRINames)`.
    pub fn init(
        &mut self,
        map_of_ri: &DataMap<Handle<StepReprRepresentationItem>, Shape>,
        map_of_ri_names: &DataMap<AsciiString, Shape>,
    ) {
        self.my_ri_map = map_of_ri.clone();
        self.my_ri_names_map = map_of_ri_names.clone();
    }
    /// `set_active`.
    pub fn set_active(&mut self, is_active: bool) {
        self.my_active_flag = is_active;
    }
    /// `is_active`.
    pub fn is_active(&self) -> bool {
        self.my_active_flag
    }
    /// `clean_up`: clears the two maps; the registered edges stay.
    pub fn clean_up(&mut self) {
        self.my_ri_map.clear(false);
        self.my_ri_names_map.clear(false);
    }
    /// `is_bound(RI)`.
    pub fn is_bound(&self, item: &Handle<StepReprRepresentationItem>) -> bool {
        self.my_ri_map.is_bound(item)
    }
    /// `is_bound(RIName)`.
    pub fn is_bound_name(&self, name: &AsciiString) -> bool {
        self.my_ri_names_map.is_bound(name)
    }
    /// `bind(RI, S)`.
    pub fn bind(&mut self, item: &Handle<StepReprRepresentationItem>, shape: &Shape) {
        self.my_ri_map.bind(item.clone(), shape.clone());
    }
    /// `bind(RIName, S)`.
    pub fn bind_name(&mut self, name: &AsciiString, shape: &Shape) {
        self.my_ri_names_map.bind(name.clone(), shape.clone());
    }
    /// `find(RI)`.
    ///
    /// # Errors
    /// a no such object failure when `item` is not bound.
    pub fn find(&self, item: &Handle<StepReprRepresentationItem>) -> Result<&Shape, Failure> {
        self.my_ri_map.find(item)
    }
    /// `find(RIName)`.
    ///
    /// # Errors
    /// a no such object failure when `name` is not bound.
    pub fn find_name(&self, name: &AsciiString) -> Result<&Shape, Failure> {
        self.my_ri_names_map.find(name)
    }
    /// `register_nm_edge`: once for each edge.
    pub fn register_nm_edge(&mut self, edge: &Shape) {
        if !self.is_edge_registered_as_nm(edge) {
            self.my_nm_edges.push(edge.clone());
        }
    }
    /// `is_suspected_as_closing`: `suspected` is pure non-manifold and adjacent to `base`.
    pub fn is_suspected_as_closing(
        &self,
        base: &Shape,
        suspected: &Shape,
    ) -> Result<bool, Failure> {
        Ok(self.is_pure_nm_shell(suspected)? && Self::is_adjacent_shell(base, suspected)?)
    }
    /// `set_ideas_case`.
    pub fn set_ideas_case(&mut self, ideas_case: bool) {
        self.my_ideas_case = ideas_case;
    }
    /// `is_ideas_case`.
    pub fn is_ideas_case(&self) -> bool {
        self.my_ideas_case
    }
    /// `is_pure_nm_shell`: every edge of the shell is registered.
    pub fn is_pure_nm_shell(&self, shell: &Shape) -> Result<bool, Failure> {
        Ok(edges_of(shell)?
            .iter()
            .all(|edge| self.is_edge_registered_as_nm(edge)))
    }
    fn is_edge_registered_as_nm(&self, edge: &Shape) -> bool {
        self.my_nm_edges.iter().any(|known| known.is_same(edge))
    }
    fn is_adjacent_shell(shell_a: &Shape, shell_b: &Shape) -> Result<bool, Failure> {
        if shell_a.is_same(shell_b) {
            return Ok(false);
        }
        let edges_b = edges_of(shell_b)?;
        Ok(edges_of(shell_a)?
            .iter()
            .any(|edge| edges_b.iter().any(|other| edge.is_same(other))))
    }
}
