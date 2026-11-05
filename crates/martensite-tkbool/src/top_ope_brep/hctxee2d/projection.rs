use super::TopOpeBRepHctxee2d;
use crate::top_ope_brep_tool::{TopOpeBRepToolCurveTool, fun_tool_onapex, fun_tool_proj_ponf};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Face},
    toptools::IndexedDataMapOfShapeListOfShape,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::{geom::GeomCurve, top_abs::ShapeEnum};
use martensite_tkgeombase::extrema::{ExtAlgo, ExtFlag};
use martensite_tkmath::gp::{Point2, Point3};

type ProjectedRange = (Handle<dyn Geom2dCurve>, f64, f64);
impl TopOpeBRepHctxee2d {
    pub(super) fn load_projected(&mut self, face: &Face, range: (f64, f64)) -> Result<(), Failure> {
        let Some((curve, first, last)) = self.project_second(face, range)? else {
            return Ok(());
        };
        if curve.is_null() {
            return Ok(());
        }
        self.curves[1].load(&curve)?;
        let tolerance = BRepTool::edge_tolerance(&self.edges[1])?;
        self.domains[1].set_values(
            curve.value(first)?,
            first,
            tolerance,
            curve.value(last)?,
            last,
            tolerance,
        );
        Ok(())
    }
    fn project_second(
        &self,
        face: &Face,
        range: (f64, f64),
    ) -> Result<Option<ProjectedRange>, Failure> {
        if BRepTool::degenerated(&self.edges[1])? {
            return self.project_degenerated(face, range);
        }
        let source = BRepTool::curve_with_location(&self.edges[1])?;
        let curve = source.curve.get().ok_or_else(null_curve)?;
        let transformed: Handle<dyn GeomCurve> =
            curve.transformed(source.location.transformation())?.cast();
        let (projected, _) =
            TopOpeBRepToolCurveTool::make_pcurve_on_face(face, &transformed, 0., 0.)?;
        Ok(Some((projected, source.first, source.last)))
    }
    fn project_degenerated(
        &self,
        face: &Face,
        range: (f64, f64),
    ) -> Result<Option<ProjectedRange>, Failure> {
        let explorer = Explorer::new(&self.edges[1], ShapeEnum::Vertex, ShapeEnum::Shape)?;
        let vertex = topods::vertex(explorer.current())?;
        let point = BRepTool::pnt(&vertex)?;
        if !projects_on_apex(point, face)? {
            return Ok(None);
        }
        let shared_vertex = self.apex_vertex(point)?;
        self.apex_curve(face, &shared_vertex, range).map(Some)
    }
    fn apex_vertex(&self, point: Point3) -> Result<topods::Vertex, Failure> {
        let (first, last) = topexp::vertices(&self.edges[0], false);
        let on_first =
            point.distance(&BRepTool::pnt(&first)?) < BRepTool::vertex_tolerance(&first)?;
        Ok(if on_first { first } else { last })
    }
    fn apex_curve(
        &self,
        face: &Face,
        vertex: &topods::Vertex,
        range: (f64, f64),
    ) -> Result<ProjectedRange, Failure> {
        let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
        topexp::map_shapes_and_ancestors(face, ShapeEnum::Vertex, ShapeEnum::Edge, &mut ancestors)?;
        let mut found = (Handle::default(), range.0, range.1);
        for ancestor in ancestors.find_from_key(vertex)?.iter() {
            let edge = topods::edge(ancestor)?;
            if !BRepTool::degenerated(&edge)? {
                continue;
            }
            let curve = BRepTool::curve_on_face(&edge, face)?;
            found = (curve.pcurve, curve.first, curve.last);
        }
        Ok(found)
    }
}
fn null_curve() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "TopOpeBRepHctxee2d: curve=None; expected a 3D curve for a nondegenerate projected edge",
    )
}

fn projects_on_apex(point: Point3, face: &Face) -> Result<bool, Failure> {
    let (mut uv, mut distance) = (Point2::default(), 0.);
    if !fun_tool_proj_ponf(
        point,
        face,
        &mut uv,
        &mut distance,
        ExtFlag::MinMax,
        ExtAlgo::Grad,
    )? {
        return Ok(false);
    }
    fun_tool_onapex(uv, &BRepTool::surface(face)?)
}
