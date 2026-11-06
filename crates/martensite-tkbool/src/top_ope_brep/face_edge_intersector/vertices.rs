use super::*;
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::{topexp::Explorer, topods::Vertex};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

impl TopOpeBRepFaceEdgeIntersector {
    /// Searches vertices while retaining the native last-visit flag; `inter.is_vertex_on(&shape, point, 1e-7, &mut vertex)?;`.
    pub fn is_vertex_on(
        &self,
        shape: &Shape,
        point: Point3,
        tolerance: f64,
        vertex: &mut Vertex,
    ) -> Result<bool, Failure> {
        *vertex = Vertex::default();
        let mut found = false;
        for current in Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)? {
            let candidate = topods::vertex(&current)?;
            found = point.square_distance(&BRepTool::pnt(&candidate)?) < tolerance * tolerance;
            if found {
                *vertex = candidate;
            }
        }
        Ok(found)
    }
    /// Searches the face or edge for the current point; `let found = inter.is_vertex(2, &mut vertex)?;`.
    pub fn is_vertex(&self, index: i32, vertex: &mut Vertex) -> Result<bool, Failure> {
        let point = self.value()?;
        if index != 1 && index != 2 {
            return Ok(false);
        }
        self.is_vertex_on(self.shape(index)?, point, self.tolerance, vertex)
    }
    pub(super) fn shape_tolerances(
        &mut self,
        first: &Shape,
        second: &Shape,
    ) -> Result<(), Failure> {
        self.tolerance = self
            .tolerance_max(first, ShapeEnum::Edge)?
            .max(self.tolerance_max(second, ShapeEnum::Edge)?);
        self.force_tolerance = false;
        Ok(())
    }
    pub(super) fn tolerance_max(&self, shape: &Shape, kind: ShapeEnum) -> Result<f64, Failure> {
        let mut values = Explorer::new(shape, kind, ShapeEnum::Shape)?;
        if !values.more() {
            return Ok(precision::intersection());
        }
        let mut tolerance = f64::MIN;
        while values.more() {
            tolerance = tolerance.max(TopOpeBRepToolShapeTool::tolerance(values.current())?);
            values.next()?;
        }
        Ok(tolerance)
    }
}
