//! Repeated point visits consume each incident edge once per line and operation.
use super::*;

impl DegenerateContext {
    pub(super) fn reduce_treated(&mut self, line: &TopOpeBRepLineInter) {
        let identity = line.record_identity();
        if self
            .line_identity
            .as_ref()
            .is_some_and(|old| old.is_same_record(identity))
        {
            return;
        }
        self.line_identity = Some(identity.clone());
        self.treated.clear(false);
    }

    pub(super) fn compute_untreated_edge(
        &mut self,
        vertex: &Vertex,
        ancestors: &IndexedDataMapOfShapeListOfShape,
        maker: &mut TopOpeBRepToolMkTondgE,
        transitions: &mut (i32, f64, f64),
    ) -> Result<bool, Failure> {
        for shape in ancestors.find_from_key(vertex.as_ref())? {
            let edge = topods::edge(shape)?;
            let parameter = BRepTool::parameter(vertex, &edge)?;
            if BRepTool::degenerated(&edge)? || self.is_treated(shape, vertex.as_ref())? {
                continue;
            }
            maker.set_rest(parameter, &edge)?;
            let ok = maker.mk_ton_e_on(&edge, transitions)?;
            self.treated
                .try_bound(vertex.as_ref().clone(), List::default())
                .append(shape.clone());
            return Ok(ok);
        }
        maker.mk_ton_e(transitions)
    }

    fn is_treated(&self, edge: &Shape, vertex: &Shape) -> Result<bool, Failure> {
        if !self.treated.is_bound(vertex) {
            return Ok(false);
        }
        Ok(self
            .treated
            .find(vertex)?
            .iter()
            .any(|shape| shape.is_same(edge)))
    }
}
