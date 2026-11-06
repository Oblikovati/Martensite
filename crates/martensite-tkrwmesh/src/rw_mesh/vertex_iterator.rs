//! `RwMeshVertexIterator`: the free vertices of a shape or a document label.
use super::{RwMeshShapeIterator, RwMeshShapeIteratorBase};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Shape, Vertex};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// Iterates the vertices outside edges; `RwMeshVertexIterator::with_shape(&shape, &style)?`.
pub struct RwMeshVertexIterator {
    base: RwMeshShapeIteratorBase,
    vertex: Vertex,
    shape: Shape,
    point: Point3,
}

impl RwMeshVertexIterator {
    fn started(base: RwMeshShapeIteratorBase) -> Result<Self, Failure> {
        let mut vertices = Self {
            base,
            vertex: Vertex::default(),
            shape: Shape::default(),
            point: Point3::default(),
        };
        RwMeshShapeIterator::next(&mut vertices)?;
        Ok(vertices)
    }
    /// Over the vertices of a label outside edges;
    /// `RwMeshVertexIterator::with_label(&label, &location, true, &style)?`.
    pub fn with_label(
        label: &TdfLabel,
        location: &Location,
        to_map_colors: bool,
        style: &XCAFPrsStyle,
    ) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_label(
            label,
            location,
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            to_map_colors,
            style,
        )?)
    }
    /// Over the vertices of a shape outside edges;
    /// `RwMeshVertexIterator::with_shape(&shape, &style)?`.
    pub fn with_shape(shape: &Shape, style: &XCAFPrsStyle) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_shape(
            shape,
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            style,
        )?)
    }
    /// The current vertex; `vertices.vertex()`.
    pub fn vertex(&self) -> &Vertex {
        &self.vertex
    }
    /// The geometry of the current vertex; `vertices.point()`.
    pub fn point(&self) -> Point3 {
        self.point
    }
    fn reset_vertex(&mut self) {
        self.vertex = Vertex::default();
        self.shape = Shape::default();
        self.base.reset_shape();
    }
}

impl RwMeshShapeIterator for RwMeshVertexIterator {
    fn iterator_base(&self) -> &RwMeshShapeIteratorBase {
        &self.base
    }
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn more(&self) -> bool {
        !self.shape.is_null()
    }
    fn next(&mut self) -> Result<(), Failure> {
        if !self.base.iter.more() {
            self.reset_vertex();
            return Ok(());
        }
        self.shape = self.base.iter.current().clone();
        self.vertex = topods::vertex(&self.shape)?;
        self.point = BRepTool::pnt(&self.vertex)?;
        // A vertex point is already located, so its transformation stays the identity.
        self.base.trsf = Transform3::default();
        self.base.location = Location::default();
        let shape = self.shape.clone();
        self.base.init_shape(&shape);
        self.base.iter.next()
    }
    fn is_empty(&self) -> bool {
        self.shape.is_null()
    }
    fn elem_lower(&self) -> i32 {
        1
    }
    fn elem_upper(&self) -> i32 {
        1
    }
    fn nb_nodes(&self) -> i32 {
        1
    }
    fn node_lower(&self) -> i32 {
        1
    }
    fn node_upper(&self) -> i32 {
        1
    }
    fn node(&self, _node: i32) -> Result<Point3, Failure> {
        Ok(self.point)
    }
}
