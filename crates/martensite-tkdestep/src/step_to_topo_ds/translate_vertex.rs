//! `StepToTopoDSTranslateVertex`: a vertex_point to a vertex, shared through the tools.
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateVertexError,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_shape::{StepShapeVertex, StepShapeVertexPoint};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Shape, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::geom::GeomPoint;

/// Translate vertex.
#[derive(Default)]
pub struct StepToTopoDSTranslateVertex {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslateVertexError,
}

impl StepToTopoDSTranslateVertex {
    /// `StepToTopoDSTranslateVertex()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateVertex(V, T, NMTool, factors)`.
    pub fn with_vertex(
        vertex: &Handle<StepShapeVertex>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(vertex, tool, nm_tool, factors)?;
        Ok(made)
    }
    fn finish(&mut self, shape: Shape) {
        self.my_result = shape;
        self.my_error = StepToTopoDSTranslateVertexError::Done;
        self.root.done = true;
    }
    fn fail(&mut self) {
        self.my_error = StepToTopoDSTranslateVertexError::Other;
        self.root.done = false;
    }
    /// The name of a vertex that an I-DEAS-like file shares vertices by.
    fn shared_name(vertex: &StepShapeVertex) -> Option<AsciiString> {
        let name = vertex.name();
        let live = name.get()?;
        (!live.is_empty()).then(|| live.string().clone())
    }
    /// The vertex a non-manifold tool already knows (ssv; 14.11.2010 and 15.11.2010).
    fn shared_vertex(
        vertex: &Handle<StepShapeVertex>,
        nm_tool: &StepToTopoDSNMTool,
        live: &StepShapeVertex,
    ) -> Result<Option<Shape>, Failure> {
        if !nm_tool.is_active() {
            return Ok(None);
        }
        if nm_tool.is_bound(&vertex.cast()) {
            return Ok(Some(nm_tool.find(&vertex.cast())?.clone()));
        }
        if !nm_tool.is_ideas_case() {
            return Ok(None);
        }
        match Self::shared_name(live) {
            Some(name) if nm_tool.is_bound_name(&name) => {
                Ok(Some(nm_tool.find_name(&name)?.clone()))
            }
            _ => Ok(None),
        }
    }
    /// `init(vertex, tool, NMTool, factors)`.
    pub fn init(
        &mut self,
        vertex: &Handle<StepShapeVertex>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let Some(live) = vertex.get() else {
            self.fail();
            return Ok(());
        };
        if tool.is_bound(&vertex.cast()) {
            let found = topods::vertex(tool.find(&vertex.cast())?)?;
            self.finish(found.into_shape());
            return Ok(());
        }
        if let Some(shared) = Self::shared_vertex(vertex, nm_tool, live)? {
            self.finish(shared);
            return Ok(());
        }
        let vertex_point = vertex.cast::<StepShapeVertexPoint>();
        let geometry = step_live(&vertex_point, "StepToTopoDSTranslateVertex")?.vertex_geometry();
        if geometry.is_null() {
            self.fail();
            return Ok(());
        }
        let point: Handle<StepGeomCartesianPoint> = geometry.cast();
        let made = StepToGeom::make_cartesian_point(&point, factors)?;
        let position = step_live(&made, "StepToTopoDSTranslateVertex")?.pnt();
        let mut result = Vertex::default();
        BRepBuilder::default().make_vertex_at(&mut result, position, precision::confusion())?;
        let shape = result.into_shape();
        tool.bind(&vertex.cast(), &shape)?;
        if nm_tool.is_active() {
            nm_tool.bind(&vertex.cast(), &shape);
            if let Some(name) = Self::shared_name(live).filter(|_| nm_tool.is_ideas_case()) {
                nm_tool.bind_name(&name, &shape);
            }
        }
        self.finish(shape);
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure without a result.
    pub fn value(&self) -> Result<&Shape, Failure> {
        self.root
            .check_done("StepToTopoDSTranslateVertex::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateVertexError {
        self.my_error
    }
}
