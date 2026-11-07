//! `RwMeshEdgeIterator`: the edges of a shape or a document label with a 3D polygon.
use super::{RwMeshShapeIterator, RwMeshShapeIteratorBase};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::poly::Polygon3D;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// Iterates the free edges with a polygon; `RwMeshEdgeIterator::with_shape(&shape, &style)?`.
pub struct RwMeshEdgeIterator {
    base: RwMeshShapeIteratorBase,
    edge: Edge,
    shape: Shape,
    polygon: Handle<Polygon3D>,
}

impl RwMeshEdgeIterator {
    fn started(base: RwMeshShapeIteratorBase) -> Result<Self, Failure> {
        let mut edges = Self {
            base,
            edge: Edge::default(),
            shape: Shape::default(),
            polygon: Handle::default(),
        };
        RwMeshShapeIterator::next(&mut edges)?;
        Ok(edges)
    }
    /// Over the edges of a label outside faces;
    /// `RwMeshEdgeIterator::with_label(&label, &location, true, &style)?`.
    pub fn with_label(
        label: &TdfLabel,
        location: &Location,
        to_map_colors: bool,
        style: &XCAFPrsStyle,
    ) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_label(
            label,
            location,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            to_map_colors,
            style,
        )?)
    }
    /// Over the edges of a shape outside faces; `RwMeshEdgeIterator::with_shape(&shape, &style)?`.
    pub fn with_shape(shape: &Shape, style: &XCAFPrsStyle) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_shape(
            shape,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            style,
        )?)
    }
    /// The current edge; `edges.edge()`.
    pub fn edge(&self) -> &Edge {
        &self.edge
    }
    /// The current polygon; `edges.polygon3d()`.
    pub fn polygon3d(&self) -> &Handle<Polygon3D> {
        &self.polygon
    }
    fn reset_edge(&mut self) {
        self.polygon = Handle::default();
        self.edge = Edge::default();
        self.shape = Shape::default();
        self.base.reset_shape();
    }
}

impl RwMeshShapeIterator for RwMeshEdgeIterator {
    fn iterator_base(&self) -> &RwMeshShapeIteratorBase {
        &self.base
    }
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn more(&self) -> bool {
        self.polygon.is_some()
    }
    fn next(&mut self) -> Result<(), Failure> {
        while self.base.iter.more() {
            self.shape = self.base.iter.current().clone();
            self.edge = topods::edge(&self.shape)?;
            let (polygon, location) = BRepTool::polygon3d(&self.edge)?;
            self.polygon = polygon;
            self.base.location = location;
            self.base.trsf = self.base.location.transformation();
            if self.nb_nodes() == 0 {
                self.reset_edge();
                self.base.iter.next()?;
                continue;
            }
            let shape = self.shape.clone();
            self.base.init_shape(&shape);
            self.base.iter.next()?;
            return Ok(());
        }
        self.reset_edge();
        Ok(())
    }
    fn is_empty(&self) -> bool {
        self.nb_nodes() < 1
    }
    fn elem_lower(&self) -> i32 {
        1
    }
    fn elem_upper(&self) -> i32 {
        self.nb_nodes()
    }
    fn nb_nodes(&self) -> i32 {
        self.polygon.get().map_or(0, Polygon3D::nb_nodes)
    }
    fn node_lower(&self) -> i32 {
        1
    }
    fn node_upper(&self) -> i32 {
        self.nb_nodes()
    }
    fn node(&self, node: i32) -> Result<Point3, Failure> {
        let polygon = self.polygon.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "RwMeshEdgeIterator: no current polygon; expected more() to be true",
                "",
            )
        })?;
        polygon
            .nodes()
            .value(node)
            .copied()
            .map_err(|error| error.into_failure("RwMeshEdgeIterator::node"))
    }
}
