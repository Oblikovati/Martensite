use super::TopOpeBRepPoint2d;
use crate::top_ope_brep::hctxff2d::context_slot;
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::standard::Failure;

impl TopOpeBRepPoint2d {
    /// Sets an edge's vertex flag independently of its stored vertex; `point.set_is_vertex(1, false)?`.
    pub fn set_is_vertex(&mut self, index: i32, is_vertex: bool) -> Result<(), Failure> {
        self.edges[context_slot(index, "TopOpeBRepPoint2d::SetIsVertex")?].is_vertex = is_vertex;
        Ok(())
    }
    /// Reads an edge's vertex flag; `point.is_vertex(2)?`.
    pub fn is_vertex(&self, index: i32) -> Result<bool, Failure> {
        Ok(self.edges[context_slot(index, "TopOpeBRepPoint2d::IsVertex")?].is_vertex)
    }
    /// Copies a vertex and sets the edge's vertex flag true; `point.set_vertex(1, &vertex)?`.
    pub fn set_vertex(&mut self, index: i32, vertex: &Vertex) -> Result<(), Failure> {
        let edge = &mut self.edges[context_slot(index, "TopOpeBRepPoint2d::SetVertex")?];
        edge.vertex = vertex.clone();
        edge.is_vertex = true;
        Ok(())
    }
    /// Reads a vertex only when its flag is true; `point.vertex(1)?`.
    pub fn vertex(&self, index: i32) -> Result<&Vertex, Failure> {
        if !self.is_vertex(index)? {
            return Err(Failure::new("TopOpeBRepPoint2d::Vertex"));
        }
        Ok(&self.edges[context_slot(index, "TopOpeBRepPoint2d::Vertex")?].vertex)
    }
    /// Copies an edge transition without changing its vertex flag; `point.set_transition(2, &transition)?`.
    pub fn set_transition(
        &mut self,
        index: i32,
        transition: &TopOpeBRepDSTransition,
    ) -> Result<(), Failure> {
        self.edges[context_slot(index, "TopOpeBRepPoint2d::SetTransition")?].transition =
            *transition;
        Ok(())
    }
    /// Reads an edge transition; `point.transition(1)?`.
    pub fn transition(&self, index: i32) -> Result<&TopOpeBRepDSTransition, Failure> {
        Ok(&self.edges[context_slot(index, "TopOpeBRepPoint2d::Transition")?].transition)
    }
    /// Mutably borrows the stored transition; `*point.change_transition(1)? = transition`.
    pub fn change_transition(
        &mut self,
        index: i32,
    ) -> Result<&mut TopOpeBRepDSTransition, Failure> {
        Ok(&mut self.edges[context_slot(index, "TopOpeBRepPoint2d::ChangeTransition")?].transition)
    }
    /// Stores an edge parameter without clamping; `point.set_parameter(1, -3.5)?`.
    pub fn set_parameter(&mut self, index: i32, parameter: f64) -> Result<(), Failure> {
        self.edges[context_slot(index, "TopOpeBRepPoint2d::SetParameter")?].parameter = parameter;
        Ok(())
    }
    /// Reads an edge parameter; `point.parameter(2)?`.
    pub fn parameter(&self, index: i32) -> Result<f64, Failure> {
        Ok(self.edges[context_slot(index, "TopOpeBRepPoint2d::Parameter")?].parameter)
    }
}
