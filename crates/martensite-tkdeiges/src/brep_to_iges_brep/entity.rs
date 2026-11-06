//! `BRepToIgesBRepEntity`: the lists of a translation to BRep entities, and any shape.
use crate::brep_to_iges::{BRepToIgesBREntity, BRepToIgesBRWire, BRepToIgesOriginMap};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_new_array1};
use crate::iges_solid::{IgesSolidEdgeList, IgesSolidVertexList};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Edge, Shape, Vertex};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Xyz;
use std::ops::{Deref, DerefMut};

const WHERE: &str = "BRepToIgesBRepEntity";

/// The vertices or edges of a list, with their ranks from 1 by identity.
#[derive(Default)]
pub(super) struct ShapeList {
    shapes: Vec<Shape>,
}

impl ShapeList {
    fn index(&self, shape: &Shape) -> i32 {
        self.shapes
            .iter()
            .position(|known| known.is_same(shape))
            .map_or(0, |index| index as i32 + 1)
    }
    fn add(&mut self, shape: Shape) -> i32 {
        let index = self.index(&shape);
        if index != 0 {
            return index;
        }
        self.shapes.push(shape);
        self.shapes.len() as i32
    }
}

/// Translates shapes to BRep entities.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkdeiges::brep_to_iges_brep::BRepToIgesBRepEntity;
/// use martensite_tkernel::message::ProgressRange;
/// let mut tool = BRepToIgesBRepEntity::new();
/// assert!(tool.transfer_shape(&Shape::default(), &ProgressRange::default()).unwrap().is_null());
/// ```
pub struct BRepToIgesBRepEntity {
    base: BRepToIgesBREntity,
    pub(super) vertex_list: Handle<IgesSolidVertexList>,
    pub(super) edge_list: Handle<IgesSolidEdgeList>,
    pub(super) vertices: ShapeList,
    pub(super) edges: ShapeList,
    pub(super) curves: Vec<Handle<IgesDataIgesEntity>>,
}

impl Deref for BRepToIgesBRepEntity {
    type Target = BRepToIgesBREntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BRepToIgesBRepEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Default for BRepToIgesBRepEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepToIgesBRepEntity {
    /// A tool with empty lists; `BRepToIgesBRepEntity::new()`.
    pub fn new() -> Self {
        Self {
            base: BRepToIgesBREntity::new(),
            vertex_list: Handle::new(IgesSolidVertexList::new()),
            edge_list: Handle::new(IgesSolidEdgeList::new()),
            vertices: ShapeList::default(),
            edges: ShapeList::default(),
            curves: Vec::new(),
        }
    }
    /// Forgets the vertices, the edges and the curves; `tool.clear()`.
    pub fn clear(&mut self) {
        self.vertices = ShapeList::default();
        self.edges = ShapeList::default();
        self.curves.clear();
    }
    /// The wire tool on this process and model.
    pub(super) fn wire_tool(&self) -> Result<BRepToIgesBRWire, Failure> {
        let mut tool = BRepToIgesBRWire::with_entity(&self.base);
        tool.set_model(&self.get_model())?;
        Ok(tool)
    }
    /// The rank of a vertex in the list, 0 when it is not there; `tool.index_vertex(&vertex)`.
    pub fn index_vertex(&self, vertex: &Vertex) -> i32 {
        self.vertices.index(vertex)
    }
    /// Adds a vertex, once; gives its rank, 0 for a null vertex; `tool.add_vertex(&vertex)`.
    pub fn add_vertex(&mut self, vertex: &Vertex) -> i32 {
        if vertex.is_null() {
            return 0;
        }
        self.vertices.add(vertex.clone().into_shape())
    }
    /// The rank of an edge in the list, 0 when it is not there; `tool.index_edge(&edge)`.
    pub fn index_edge(&self, edge: &Edge) -> i32 {
        self.edges.index(edge)
    }
    /// Adds an edge with its curve, once; gives its rank, 0 for a null edge;
    /// `tool.add_edge(&edge, &curve)`.
    pub fn add_edge(&mut self, edge: &Edge, curve: &Handle<IgesDataIgesEntity>) -> i32 {
        if edge.is_null() {
            return 0;
        }
        let known = self.edges.index(edge);
        if known != 0 {
            return known;
        }
        self.curves.push(curve.clone());
        self.edges.add(edge.clone().into_shape())
    }
    /// Fills the vertex list entity from the vertices; `tool.transfer_vertex_list()?`.
    ///
    /// # Errors
    /// A failure of a vertex.
    pub fn transfer_vertex_list(&self) -> Result<(), Failure> {
        let count = self.vertices.shapes.len() as i32;
        if count == 0 {
            return Ok(());
        }
        let unit = self.get_unit();
        let points: Handle<HArray1<Xyz>> = iges_new_array1(1, count)?;
        for (rank, shape) in (1..).zip(&self.vertices.shapes) {
            let point = BRepTool::pnt(&topods::vertex(shape)?)?;
            let scaled = Xyz::new(point.x() / unit, point.y() / unit, point.z() / unit);
            iges_array1_set(&points, rank, scaled, WHERE)?;
        }
        crate::iges_data::iges_live(&self.vertex_list, WHERE)?.init(&points)?;
        Ok(())
    }
    /// Fills the edge list entity from the edges; `tool.transfer_edge_list()?`.
    ///
    /// # Errors
    /// A failure of an edge.
    pub fn transfer_edge_list(&self) -> Result<(), Failure> {
        let count = self.edges.shapes.len() as i32;
        if count == 0 {
            return Ok(());
        }
        let curves: Handle<HArray1<Handle<IgesDataIgesEntity>>> = iges_new_array1(1, count)?;
        let start_lists: Handle<HArray1<Handle<IgesSolidVertexList>>> = iges_new_array1(1, count)?;
        let end_lists: Handle<HArray1<Handle<IgesSolidVertexList>>> = iges_new_array1(1, count)?;
        let start_indices: Handle<HArray1<i32>> = iges_new_array1(1, count)?;
        let end_indices: Handle<HArray1<i32>> = iges_new_array1(1, count)?;
        for (rank, shape) in (1..).zip(&self.edges.shapes) {
            let (first, last) = topexp::vertices(&topods::edge(shape)?, false);
            iges_array1_set(&curves, rank, self.curves[rank as usize - 1].clone(), WHERE)?;
            iges_array1_set(&start_indices, rank, self.index_vertex(&first), WHERE)?;
            iges_array1_set(&end_indices, rank, self.index_vertex(&last), WHERE)?;
            iges_array1_set(&start_lists, rank, self.vertex_list.clone(), WHERE)?;
            iges_array1_set(&end_lists, rank, self.vertex_list.clone(), WHERE)?;
        }
        crate::iges_data::iges_live(&self.edge_list, WHERE)?.init(
            &curves,
            &start_lists,
            &start_indices,
            &end_lists,
            &end_indices,
        )?;
        Ok(())
    }
    /// The entity of any shape, with the lists filled; a vertex, an edge or a wire alone has a
    /// warning and its wire form; `tool.transfer_shape(&shape, &range)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the shape.
    pub fn transfer_shape(
        &mut self,
        start: &Shape,
        range: &ProgressRange<'_>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let result: Handle<IgesDataIgesEntity> = match start.shape_type()? {
            ShapeEnum::Vertex => {
                self.add_warning(start, " A Vertex alone is not a IGESBRep Entity")?;
                return self.wire_tool()?.transfer_vertex(&topods::vertex(start)?);
            }
            ShapeEnum::Edge => {
                self.add_warning(start, " An Edge alone is not a IGESBRep Entity")?;
                return self.wire_tool()?.transfer_edge(
                    &topods::edge(start)?,
                    &BRepToIgesOriginMap::default(),
                    false,
                );
            }
            ShapeEnum::Wire => {
                self.add_warning(start, " An Wire alone is not a IGESBRep Entity")?;
                return self.wire_tool()?.transfer_wire(&topods::wire(start)?);
            }
            ShapeEnum::Face => self.transfer_face(&topods::face(start)?)?.cast(),
            ShapeEnum::Shell => self.transfer_shell(&topods::shell(start)?, range)?.cast(),
            ShapeEnum::Solid => self.transfer_solid(&topods::solid(start)?, range)?.cast(),
            ShapeEnum::CompSolid => self.transfer_comp_solid(&topods::comp_solid(start)?, range)?,
            ShapeEnum::Compound => self.transfer_compound(&topods::compound(start)?, range)?,
            ShapeEnum::Shape => Handle::default(),
        };
        if result.is_null() {
            return Ok(result);
        }
        self.transfer_vertex_list()?;
        self.transfer_edge_list()?;
        Ok(result)
    }
}
