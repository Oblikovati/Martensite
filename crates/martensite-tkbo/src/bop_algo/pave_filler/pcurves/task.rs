//! Existing projection and periodic adjustment calculations use the actual launcher's context.
use super::*;
use crate::int_tools::Context;
use martensite_tkbrep::brep::EdgePCurve;
impl PCurveTask {
    /// Worker-local computation keeps attachment to the source edge in the coordinator.
    /// Example: `task.perform_on(context)?` before inspecting `task.failed`.
    pub(super) fn perform_on(&mut self, context: &Handle<Context>) -> Result<(), Failure> {
        let context = context.get().ok_or_else(missing_structure)?;
        let mut edge = self.edge.clone();
        let found = BRepTool::curve_on_face(&edge, &self.face)?;
        if found.pcurve.is_null() {
            self.project_missing(&mut edge, context)?;
            if self.failed {
                return Ok(());
            }
        } else {
            self.adjust_existing(&edge, &found, context)?;
        }
        if self.update_vertices {
            update_vertices(&edge, &self.face)?;
        }
        Ok(())
    }
    fn project_missing(&mut self, edge: &mut Edge, context: &Context) -> Result<(), Failure> {
        // Work on a copy to avoid data races, as natively.
        *edge = copy_edge(edge)?;
        if self.attach_source(edge, context)? != 0 {
            build_pcurve_for_edge_on_face(edge, &self.face, Some(context))?;
        }
        self.pcurve = BRepTool::curve_on_face(edge, &self.face)?.pcurve;
        if self.pcurve.is_null() {
            self.failed = true;
            return Ok(());
        }
        self.tolerance = BRepTool::edge_tolerance(edge)?;
        Ok(())
    }
    fn attach_source(&self, edge: &Edge, context: &Context) -> Result<i32, Failure> {
        let Some((source, v1, t1, v2, t2)) = &self.source else {
            return Ok(1);
        };
        let mut split = Edge::default();
        make_split_edge(source, v1, *t1, v2, *t2, &mut split)?;
        attach_existing_pcurve(&split, edge, &self.face, context)
    }
    fn adjust_existing(
        &mut self,
        edge: &Edge,
        found: &EdgePCurve,
        context: &Context,
    ) -> Result<(), Failure> {
        let surface = context.surface_adaptor(&self.face)?;
        let surface = surface.read();
        if !surface.is_u_periodic()? && !surface.is_v_periodic()? {
            return Ok(());
        }
        let adjusted = adjust_pcurve_on_surface(&surface, found.first, found.last, &found.pcurve)?;
        if adjusted != found.pcurve {
            self.pcurve = adjusted;
            self.tolerance = BRepTool::edge_tolerance(edge)?;
        }
        Ok(())
    }
}
