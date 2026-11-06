use super::{TopOpeBRepVPointInter, slot};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;

impl TopOpeBRepVPointInter<'_> {
    /// Stores the two supporting shape identifiers; `view.set_shapes(4, 7)`.
    pub fn set_shapes(&mut self, first: i32, second: i32) {
        self.shapes = (first, second);
    }
    /// Reads the supporting shape identifiers; `view.shapes()`.
    pub fn shapes(&self) -> (i32, i32) {
        self.shapes
    }
    /// Reads a domain state, rejecting indices outside 1 or 2; `view.state(1)?`.
    pub fn state(&self, index: i32) -> Result<State, Failure> {
        Ok(self.states[slot(index, "State")?])
    }
    /// Stores a domain state and recomputes keep; `view.set_state(State::In, 1)?`.
    pub fn set_state(&mut self, state: State, index: i32) -> Result<(), Failure> {
        self.states[slot(index, "State")?] = state;
        self.update_keep();
        Ok(())
    }
    /// Stores an ON edge and parameter; invalid indices are ignored, as natively; `view.set_edge_on(&edge, 0.5, 1)`.
    pub fn set_edge_on(&mut self, edge: &Shape, parameter: f64, index: i32) {
        if let Ok(index) = slot(index, "EdgeON") {
            self.edges_on[index] = edge.clone();
            self.edge_parameters[index] = parameter;
        }
    }
    /// Reads an ON edge, rejecting invalid indices; `view.edge_on(1)?`.
    pub fn edge_on(&self, index: i32) -> Result<&Shape, Failure> {
        Ok(&self.edges_on[slot(index, "EdgeON")?])
    }
    /// Reads its independently stored parameter; `view.edge_on_parameter(1)?`.
    pub fn edge_on_parameter(&self, index: i32) -> Result<f64, Failure> {
        Ok(self.edge_parameters[slot(index, "EdgeONParameter")?])
    }
    /// Reads the domain mask without recomputing it; `view.shape_index()`.
    pub fn shape_index(&self) -> i32 {
        self.shape_index
    }
    /// Stores the mask without recomputing keep; `view.set_shape_index(3)`.
    pub fn set_shape_index(&mut self, index: i32) {
        self.shape_index = index;
    }
    /// Recomputes keep from the mask and states; `view.update_keep()`.
    pub fn update_keep(&mut self) {
        let inside = |state| matches!(state, State::In | State::On);
        self.keep = match self.shape_index {
            1 => inside(self.states[1]),
            2 => inside(self.states[0]),
            0 | 3 => inside(self.states[0]) && inside(self.states[1]),
            _ => false,
        };
    }
    /// Reads the stored retention flag; `view.keep()`.
    pub fn keep(&self) -> bool {
        self.keep
    }
    /// Overrides retention without changing either state; `view.change_keep(true)`.
    pub fn change_keep(&mut self, keep: bool) {
        self.keep = keep;
    }
    /// Reads the point identifier; `view.index()`.
    pub fn index(&self) -> i32 {
        self.index
    }
    /// Stores the point identifier without validation; `view.set_index(-3)`.
    pub fn set_index(&mut self, index: i32) {
        self.index = index;
    }
    /// This record never marks a point internal; `view.is_internal() == false`.
    pub fn is_internal(&self) -> bool {
        false
    }
}
