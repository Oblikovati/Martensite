//! `TopoDSToStepTool`: the entities already made from the shapes of one translation, and the
//! shape being translated at each level.
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use crate::step_shape::StepShapeTopologicalRepresentationItem;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{Edge, Face, Shape, Shell, Vertex, Wire};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};

/// `DataMap<Shape, Handle<`StandardTransient`>, the shape hasher>`.
pub type TopoDSToStepShapeMap = DataMap<Shape, Handle<StandardTransient>, ShapeMapHasher>;

/// Tool.
pub struct TopoDSToStepTool {
    my_data_map: TopoDSToStepShapeMap,
    my_faceted_context: bool,
    my_lowest_tol: f64,
    my_current_shell: Shell,
    my_current_face: Face,
    my_current_wire: Wire,
    my_current_edge: Edge,
    my_current_vertex: Vertex,
    my_reversed_surface: bool,
    my_pcurve_mode: bool,
}

impl TopoDSToStepTool {
    /// `TopoDSToStepTool(theModel)`: an empty map, the pcurve mode of the model's parameters.
    pub fn with_model(model: &Handle<StepDataStepModel>) -> Result<Self, Failure> {
        let live = step_live(model, "TopoDSToStepTool")?;
        let mode = live.internal_parameters().write_surface_cur_mode;
        Ok(Self::with_map(
            &TopoDSToStepShapeMap::default(),
            false,
            mode,
        ))
    }
    /// `TopoDSToStepTool(M, FacetedContext, theSurfCurveMode)`.
    pub fn with_map(
        map: &TopoDSToStepShapeMap,
        faceted_context: bool,
        surf_curve_mode: bool,
    ) -> Self {
        Self {
            my_data_map: map.clone(),
            my_faceted_context: faceted_context,
            my_lowest_tol: 0.0,
            my_current_shell: Shell::default(),
            my_current_face: Face::default(),
            my_current_wire: Wire::default(),
            my_current_edge: Edge::default(),
            my_current_vertex: Vertex::default(),
            my_reversed_surface: false,
            my_pcurve_mode: surf_curve_mode,
        }
    }
    /// `init(M, FacetedContext, surf_curve_mode)`: the current shapes and the lowest tolerance
    /// are kept.
    pub fn init(
        &mut self,
        map: &TopoDSToStepShapeMap,
        faceted_context: bool,
        surf_curve_mode: bool,
    ) {
        self.my_data_map = map.clone();
        self.my_faceted_context = faceted_context;
        self.my_pcurve_mode = surf_curve_mode;
    }
    /// `is_bound(S)`.
    pub fn is_bound(&self, shape: &Shape) -> bool {
        self.my_data_map.is_bound(shape)
    }
    /// `bind(S, T)`.
    pub fn bind(&mut self, shape: &Shape, item: &Handle<StepShapeTopologicalRepresentationItem>) {
        self.my_data_map.bind(shape.clone(), item.cast());
    }
    /// `find(S)`.
    ///
    /// # Errors
    /// a no such object failure when `shape` is not bound.
    pub fn find(
        &self,
        shape: &Shape,
    ) -> Result<Handle<StepShapeTopologicalRepresentationItem>, Failure> {
        Ok(self.my_data_map.find(shape)?.cast())
    }
    /// `faceted`.
    pub fn faceted(&self) -> bool {
        self.my_faceted_context
    }
    /// `set_current_shell(S)`.
    pub fn set_current_shell(&mut self, shell: &Shell) {
        self.my_current_shell = shell.clone();
    }
    /// `current_shell`.
    pub fn current_shell(&self) -> &Shell {
        &self.my_current_shell
    }
    fn raise_lowest_tol(&mut self, tolerance: f64) {
        if tolerance > self.my_lowest_tol {
            self.my_lowest_tol = tolerance;
        }
    }
    /// `set_current_face(F)`.
    pub fn set_current_face(&mut self, face: &Face) -> Result<(), Failure> {
        self.raise_lowest_tol(BRepTool::face_tolerance(face)?);
        self.my_current_face = face.clone();
        Ok(())
    }
    /// `current_face`.
    pub fn current_face(&self) -> &Face {
        &self.my_current_face
    }
    /// `set_current_wire(W)`.
    pub fn set_current_wire(&mut self, wire: &Wire) {
        self.my_current_wire = wire.clone();
    }
    /// `current_wire`.
    pub fn current_wire(&self) -> &Wire {
        &self.my_current_wire
    }
    /// `set_current_edge(E)`.
    pub fn set_current_edge(&mut self, edge: &Edge) -> Result<(), Failure> {
        self.raise_lowest_tol(BRepTool::edge_tolerance(edge)?);
        self.my_current_edge = edge.clone();
        Ok(())
    }
    /// `current_edge`.
    pub fn current_edge(&self) -> &Edge {
        &self.my_current_edge
    }
    /// `set_current_vertex(V)`.
    pub fn set_current_vertex(&mut self, vertex: &Vertex) -> Result<(), Failure> {
        self.raise_lowest_tol(BRepTool::vertex_tolerance(vertex)?);
        self.my_current_vertex = vertex.clone();
        Ok(())
    }
    /// `current_vertex`.
    pub fn current_vertex(&self) -> &Vertex {
        &self.my_current_vertex
    }
    /// `Lowest3DTolerance`: the greatest tolerance of the shapes met.
    pub fn lowest_3d_tolerance(&self) -> f64 {
        self.my_lowest_tol
    }
    /// `set_surface_reversed(B)`.
    pub fn set_surface_reversed(&mut self, reversed: bool) {
        self.my_reversed_surface = reversed;
    }
    /// `surface_reversed`.
    pub fn surface_reversed(&self) -> bool {
        self.my_reversed_surface
    }
    /// `Map`.
    pub fn map(&self) -> &TopoDSToStepShapeMap {
        &self.my_data_map
    }
    /// `PCurveMode`: whether pcurves are written (`write.surfacecurve.mode`).
    pub fn pcurve_mode(&self) -> bool {
        self.my_pcurve_mode
    }
}
