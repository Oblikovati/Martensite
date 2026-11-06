//! `BRepToIgesBRWire`: vertices, edges and wires as IGES curves and points.
use super::{BRepToIgesBREntity, BRepToIgesOriginMap, entity_array};
use crate::geom_to_iges::{GeomToIgesGeomCurve, GeomToIgesGeomPoint};
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::IgesGeomCompositeCurve;
use crate::iges_to_brep::wire_fix_on_face;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools::WireExplorer;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Vertex, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomCurve};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Point2, TransformForm};
use std::ops::{Deref, DerefMut};

const WHERE: &str = "BRepToIgesBRWire";

/// Translates vertices, edges and wires.
///
/// ```
/// use martensite_tkdeiges::brep_to_iges::BRepToIgesBRWire;
/// use martensite_tkbrep::topods::Vertex;
/// let tool = BRepToIgesBRWire::new();
/// assert!(tool.transfer_vertex(&Vertex::default()).unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct BRepToIgesBRWire {
    base: BRepToIgesBREntity,
}

impl Deref for BRepToIgesBRWire {
    type Target = BRepToIgesBREntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BRepToIgesBRWire {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// The original shape of a copy, the shape itself without a map.
pub(super) fn origin_of(shape: &Shape, origins: &BRepToIgesOriginMap) -> Result<Shape, Failure> {
    if origins.is_empty() {
        return Ok(shape.clone());
    }
    origins.find(shape).cloned()
}

impl BRepToIgesBRWire {
    /// A tool at the initial state; `BRepToIgesBRWire::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the process and the model of `base`; `BRepToIgesBRWire::with_entity(&base)`.
    pub fn with_entity(base: &BRepToIgesBREntity) -> Self {
        Self { base: base.clone() }
    }
    /// The entity of a vertex, an edge or a wire; `tool.transfer_wire_shape(&shape)?`.
    ///
    /// # Errors
    /// A failure of the geometry of the shape.
    pub fn transfer_wire_shape(
        &self,
        start: &Shape,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if start.is_null() {
            return Ok(Handle::default());
        }
        match start.shape_type()? {
            ShapeEnum::Vertex => self.transfer_vertex(&topods::vertex(start)?),
            ShapeEnum::Edge => self.transfer_edge(
                &topods::edge(start)?,
                &BRepToIgesOriginMap::default(),
                false,
            ),
            ShapeEnum::Wire => self.transfer_wire(&topods::wire(start)?),
            _ => Ok(Handle::default()),
        }
    }
    /// The point of a vertex; `tool.transfer_vertex(&vertex)?`.
    ///
    /// # Errors
    /// A failure of the point of the vertex.
    pub fn transfer_vertex(&self, vertex: &Vertex) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if vertex.is_null() {
            return Ok(Handle::default());
        }
        let point = Handle::new(GeomCartesianPoint::from_pnt(BRepTool::pnt(vertex)?));
        let mut tool = GeomToIgesGeomPoint::new();
        tool.set_model(&self.get_model())?;
        Ok(tool.transfer_cartesian_point(&point)?.cast())
    }
    /// The point of a vertex and its parameter on an edge;
    /// `let (point, parameter) = tool.transfer_vertex_on_edge(&vertex, &edge)?;`.
    ///
    /// # Errors
    /// A failure of the point or of the parameter of the vertex.
    pub fn transfer_vertex_on_edge(
        &self,
        vertex: &Vertex,
        edge: &Edge,
    ) -> Result<(Handle<IgesDataIgesEntity>, f64), Failure> {
        if vertex.is_null() {
            return Ok((Handle::default(), 0.0));
        }
        Ok((
            self.transfer_vertex(vertex)?,
            BRepTool::parameter(vertex, edge)?,
        ))
    }
    /// The point of a vertex and its parameter on an edge of a face;
    /// `let (point, parameter) = tool.transfer_vertex_on_edge_of_face(&vertex, &edge, &face)?;`.
    ///
    /// # Errors
    /// A failure of the point or of the parameter of the vertex.
    pub fn transfer_vertex_on_edge_of_face(
        &self,
        vertex: &Vertex,
        edge: &Edge,
        face: &Face,
    ) -> Result<(Handle<IgesDataIgesEntity>, f64), Failure> {
        if vertex.is_null() {
            return Ok((Handle::default(), 0.0));
        }
        let parameter = BRepTool::parameter_on_face(vertex, edge, face)?;
        Ok((self.transfer_vertex(vertex)?, parameter))
    }
    /// The point of a vertex and its parameters on a face;
    /// `let (point, uv) = tool.transfer_vertex_on_face(&vertex, &face)?;`.
    ///
    /// # Errors
    /// A failure of the point or of the parameters of the vertex.
    pub fn transfer_vertex_on_face(
        &self,
        vertex: &Vertex,
        face: &Face,
    ) -> Result<(Handle<IgesDataIgesEntity>, Point2), Failure> {
        if vertex.is_null() {
            return Ok((Handle::default(), Point2::new(0.0, 0.0)));
        }
        Ok((
            self.transfer_vertex(vertex)?,
            BRepTool::parameters(vertex, face)?,
        ))
    }
    /// The curve of an edge in space; out of BRep mode the sense of the edge goes in the curve;
    /// `tool.transfer_edge(&edge, &origins, false)?`.
    ///
    /// # Errors
    /// A failure of the curve of the edge or of the binding of the result.
    pub fn transfer_edge(
        &self,
        edge: &Edge,
        origins: &BRepToIgesOriginMap,
        b_rep_mode: bool,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if edge.is_null() {
            return Ok(Handle::default());
        }
        let mut result = Handle::default();
        let carried = BRepTool::curve_with_location(edge)?;
        if let Some(curve) = carried.curve.get() {
            let form = carried.location.transformation();
            let placed: Handle<dyn GeomCurve> = if form.form() != TransformForm::Identity {
                curve.transformed(form)?.cast()
            } else {
                curve.copy()?.cast()
            };
            let live = iges_live(&placed, WHERE)?;
            let (first, last) = if edge.orientation() == Orientation::Reversed && !b_rep_mode {
                let range = (
                    live.reversed_parameter(carried.last)?,
                    live.reversed_parameter(carried.first)?,
                );
                live.reverse()?;
                range
            } else {
                (carried.first, carried.last)
            };
            let mut tool = GeomToIgesGeomCurve::new();
            tool.set_model(&self.get_model())?;
            result = tool.transfer_curve(&placed, first, last)?;
        }
        self.set_shape_result(&origin_of(edge, origins)?, &result.cast())?;
        Ok(result)
    }
    /// The composite of a list of curves, the curve alone when it is the last and only one.
    fn single_or_composite(
        curves: &[Handle<IgesDataIgesEntity>],
        last: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        match curves.len() {
            0 => Ok(Handle::default()),
            1 => Ok(last.clone()),
            _ => {
                let composite = IgesGeomCompositeCurve::new();
                composite.init(&entity_array(curves)?)?;
                Ok(Handle::new(composite).cast())
            }
        }
    }
    /// The curve of a wire in space: one edge, or a composite; `tool.transfer_wire(&wire)?`.
    ///
    /// # Errors
    /// A failure of a curve of the wire or of the binding of the result.
    pub fn transfer_wire(&self, wire: &Wire) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if wire.is_null() {
            return Ok(Handle::default());
        }
        let mut curves = Vec::new();
        if Explorer::new(wire, ShapeEnum::Vertex, ShapeEnum::Shape)?.more() {
            let mut explorer = WireExplorer::with_wire(wire)?;
            while explorer.more() {
                let edge = explorer.current().clone();
                if edge.is_null() {
                    self.add_warning(wire, "an Edge is a null entity")?;
                } else {
                    let curve =
                        self.transfer_edge(&edge, &BRepToIgesOriginMap::default(), false)?;
                    if curve.is_some() {
                        curves.push(curve);
                    }
                }
                explorer.next()?;
            }
        } else {
            self.add_warning(wire, " no Vertex associated to the Wire")?;
        }
        let last = curves.last().cloned().unwrap_or_default();
        let result = Self::single_or_composite(&curves, &last)?;
        self.set_shape_result(wire, &result.cast())?;
        Ok(result)
    }
    /// The curves of a wire on a face: gives the 3D one and sets the 2D one. A wire without a 2D
    /// curve leaves `curve_2d` as it was;
    /// `let curve = tool.transfer_wire_on_face(&wire, &face, &origins, &mut curve_2d, 1.0)?;`.
    ///
    /// # Errors
    /// A failure of a curve of the wire or of the binding of the result.
    pub fn transfer_wire_on_face(
        &self,
        wire: &Wire,
        face: &Face,
        origins: &BRepToIgesOriginMap,
        curve_2d: &mut Handle<IgesDataIgesEntity>,
        length: f64,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        if wire.is_null() {
            return Ok(Handle::default());
        }
        let (mut curves_3d, mut curves_2d) = (Vec::new(), Vec::new());
        let (mut last_3d, mut last_2d) = (Handle::default(), Handle::default());
        if Explorer::new(wire, ShapeEnum::Vertex, ShapeEnum::Shape)?.more() {
            let fix = wire_fix_on_face(wire, face, precision::confusion())?;
            let _ = fix.fix_reorder(false)?;
            let edges = fix.wire_data()?;
            let list = iges_live(&edges, WHERE)?;
            for rank in 1..=list.nb_edges() {
                let edge = list.edge(rank)?;
                if edge.is_null() {
                    self.add_warning(wire, "an Edge is a null entity")?;
                    continue;
                }
                last_3d = self.transfer_edge(&edge, origins, false)?;
                if last_3d.is_some() {
                    curves_3d.push(last_3d.clone());
                }
                last_2d = self.transfer_edge_on_face(&edge, face, origins, length, false)?;
                if last_2d.is_some() {
                    curves_2d.push(last_2d.clone());
                }
            }
        } else {
            self.add_warning(wire, " no Vertex associated to the Wire")?;
        }
        let result = Self::single_or_composite(&curves_3d, &last_3d)?;
        if !curves_2d.is_empty() {
            *curve_2d = Self::single_or_composite(&curves_2d, &last_2d)?;
        }
        self.set_shape_result(&origin_of(wire, origins)?, &result.cast())?;
        Ok(result)
    }
}
