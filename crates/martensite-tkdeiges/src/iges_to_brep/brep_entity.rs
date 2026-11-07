//! `IgesToBRepBRepEntity`: the BRep entities: vertex lists, edge lists, loops, faces, shells, solids.
use super::curve_and_surface::curve_and_surface_tool;
use super::{BoundaryFlags, IgesToBRep, IgesToBRepTopoCurve, IgesToBRepTopoSurface};
use crate::iges_basic::IgesBasicSingleParent;
use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_live, iges_new_array1};
use crate::iges_geom::{
    IgesGeomBoundary, IgesGeomBoundedSurface, IgesGeomCurveOnSurface, IgesGeomPlane,
    IgesGeomTrimmedSurface,
};
use crate::iges_solid::{
    IgesSolidEdgeList, IgesSolidFace, IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidShell,
    IgesSolidVertexList,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Vertex};
use martensite_tkernel::message::{Msg, ProgressRange};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::geom::GeomTrimmedCurve;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Point3, Transform2};
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;

const WHERE: &str = "IgesToBRepBRepEntity";

curve_and_surface_tool!(
    /// Translates the BRep entities.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_to_brep::IgesToBRepBRepEntity;
    /// let tool = IgesToBRepBRepEntity::new();
    /// assert_eq!(tool.get_continuity(), 0);
    /// ```
    IgesToBRepBRepEntity,
    |base| base.set_mode_transfer(false),
    |base| base.set_continuity(0)
);

impl IgesToBRepBRepEntity {
    fn warn_label<T: ?Sized + RuntimeTyped>(
        &self,
        entity: &Handle<T>,
        what: &[u8],
    ) -> Result<(), Failure> {
        self.send_warning(entity, &self.label_message(entity, what)?);
        Ok(())
    }
    pub(super) fn fail_label<T: ?Sized + RuntimeTyped>(
        &self,
        entity: &Handle<T>,
        what: &[u8],
    ) -> Result<(), Failure> {
        self.send_fail(entity, &self.label_message(entity, what)?);
        Ok(())
    }
    /// The shape of a BRep entity; `tool.transfer_b_rep_entity(&entity, &range)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_b_rep_entity(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        range: &ProgressRange,
    ) -> Result<Shape, Failure> {
        let face = start.cast::<IgesSolidFace>();
        if face.is_some() {
            return self.transfer_face(&face);
        }
        let shell = start.cast::<IgesSolidShell>();
        if shell.is_some() {
            return self.transfer_shell(&shell, range);
        }
        let solid = start.cast::<IgesSolidManifoldSolid>();
        if solid.is_some() {
            return self.transfer_manifold_solid(&solid, range);
        }
        self.fail_key(start, "IGES_1005");
        Ok(Shape::default())
    }
    /// A vertex of a list; the first call makes all the vertices of the list;
    /// `tool.transfer_vertex(&list, 1)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of a vertex.
    pub fn transfer_vertex(
        &self,
        start: &Handle<IgesSolidVertexList>,
        index: i32,
    ) -> Result<Vertex, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if self.nb_shape_result(&entity)? == 0 {
            let live = iges_live(start, WHERE)?;
            let builder = BRepBuilder::default();
            for rank in 1..=live.nb_vertices() {
                let point = live
                    .vertex(rank)?
                    .scaled(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor());
                let mut vertex = Vertex::default();
                builder.make_vertex_at(&mut vertex, point, precision::confusion())?;
                self.add_shape_result(&entity, &vertex)?;
            }
        }
        let shape = self.get_shape_result_at(&entity, index)?;
        if shape.is_null() {
            self.warn_label(start, b"vertex")?;
            return Ok(Vertex::default());
        }
        topods::vertex(&shape)
    }
    /// The vertices joined by the curve of an edge, in the sense that fits their distances best.
    fn bounded_edge(&self, edge: &Edge, v1: Vertex, v2: Vertex) -> Result<Shape, Failure> {
        let builder = BRepBuilder::default();
        let (vf, vl) = topexp::vertices(edge, false);
        let mut made = Edge::default();
        builder.make_edge(&mut made)?;
        let carried = BRepTool::curve_with_location(edge)?;
        let trimmed = carried.curve.cast::<GeomTrimmedCurve>();
        let basis = trimmed
            .get()
            .map_or_else(|| carried.curve.clone(), GeomTrimmedCurve::basis_curve);
        builder.update_edge_located_curve(&made, &basis, &carried.location, 0.0)?;
        let (p1, p2) = (BRepTool::pnt(&v1)?, BRepTool::pnt(&v2)?);
        let (pf, pl) = (BRepTool::pnt(&vf)?, BRepTool::pnt(&vl)?);
        let forward = v1.is_same(&v2)
            || p1.distance(&pf) + p2.distance(&pl)
                <= p1.distance(&pl) + p2.distance(&pf) + precision::confusion();
        if !forward {
            made.reverse();
        }
        let mut start = v1.clone();
        start.set_orientation(Orientation::Forward);
        let mut end = v2.clone();
        end.set_orientation(Orientation::Reversed);
        builder.add(&mut made, &start)?;
        builder.add(&mut made, &end)?;
        // A null tolerance.
        let (at_start, at_end) = if forward {
            (carried.first, carried.last)
        } else {
            (carried.last, carried.first)
        };
        builder.update_vertex_on_edge(&start, at_start, &made, 0.0)?;
        builder.update_vertex_on_edge(&end, at_end, &made, 0.0)?;
        builder.range(&made, carried.first, carried.last, false)?;
        Ok(made.into_shape())
    }
    /// One edge of a list: its result is added, a null shape when it has none.
    fn add_edge_result(
        &self,
        start: &Handle<IgesSolidEdgeList>,
        tool: &mut IgesToBRepTopoCurve,
        rank: i32,
    ) -> Result<(), Failure> {
        let live = iges_live(start, WHERE)?;
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        let v1 = self.transfer_vertex(
            &live.start_vertex_list(rank)?,
            live.start_vertex_index(rank)?,
        )?;
        let v2 =
            self.transfer_vertex(&live.end_vertex_list(rank)?, live.end_vertex_index(rank)?)?;
        let curve = live.curve(rank)?;
        let usable = curve.is_some()
            && IgesToBRep::is_topo_curve(&curve)
            && curve.cast::<IgesGeomCurveOnSurface>().is_null()
            && curve.cast::<IgesGeomBoundary>().is_null();
        if !usable {
            let mut message = Msg::from_key(&"IGES_1306".into());
            message.arg_integer(rank)?;
            self.send_warning(start, &message);
            return self.add_shape_result(&entity, &Shape::default());
        }
        let shape = tool.transfer_topo_curve(&curve)?;
        if shape.is_null() {
            self.warn_label(&curve, b"underlying curve")?;
            return self.add_shape_result(&entity, &shape);
        }
        match shape.shape_type()? {
            ShapeEnum::Edge => {
                let made = self.bounded_edge(&topods::edge(&shape)?, v1, v2)?;
                self.add_shape_result(&entity, &made)
            }
            ShapeEnum::Wire => {
                let mut message = Msg::from_key(&"IGES_1325".into());
                message.arg_integer(rank)?;
                self.send_warning(start, &message);
                self.add_shape_result(&entity, &shape)
            }
            _ => Ok(()),
        }
    }
    /// An edge of a list; the first call makes all the edges of the list;
    /// `tool.transfer_edge(&list, 1)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of an edge.
    pub fn transfer_edge(
        &self,
        start: &Handle<IgesSolidEdgeList>,
        index: i32,
    ) -> Result<Shape, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if self.nb_shape_result(&entity)? == 0 {
            let mut tool = IgesToBRepTopoCurve::with_base(self);
            for rank in 1..=iges_live(start, WHERE)?.nb_edges() {
                self.add_edge_result(start, &mut tool, rank)?;
            }
        }
        let shape = self.get_shape_result_at(&entity, index)?;
        if shape.is_null() {
            self.warn_label(start, b"edge")?;
        }
        Ok(shape)
    }
    /// The 3D part of an edge of a loop; `None` to skip the edge.
    fn loop_edge_3d(
        &self,
        start: &Handle<IgesSolidLoop>,
        rank: i32,
    ) -> Result<Option<Handle<ShapeExtendWireData>>, Failure> {
        let live = iges_live(start, WHERE)?;
        let edge = live.edge(rank)?;
        let index = live.list_index(rank)?;
        let edge_type = live.edge_type(rank)?;
        let list = Handle::new(ShapeExtendWireData::new());
        let vertices = edge.cast::<IgesSolidVertexList>();
        let edges = edge.cast::<IgesSolidEdgeList>();
        if edge_type == 1 && vertices.is_some() {
            let vertex = self.transfer_vertex(&vertices, index)?;
            let builder = BRepBuilder::default();
            let mut degenerated = Edge::default();
            builder.make_edge(&mut degenerated)?;
            let (mut forward, mut reversed) = (vertex.clone(), vertex);
            forward.set_orientation(Orientation::Forward);
            reversed.set_orientation(Orientation::Reversed);
            builder.add(&mut degenerated, &forward)?;
            builder.add(&mut degenerated, &reversed)?;
            builder.degenerated(&degenerated, true)?;
            iges_live(&list, WHERE)?.add_edge(&degenerated, 0)?;
            return Ok(Some(list));
        }
        if edge_type == 0 && edges.is_some() {
            let shape = self.transfer_edge(&edges, index)?;
            if shape.is_null() {
                return Ok(None);
            }
            iges_live(&list, WHERE)?.add_shape(&shape, 0)?;
            return Ok(Some(list));
        }
        let mut message = Msg::from_key(&"IGES_1365".into());
        message.arg_integer(rank)?;
        self.send_warning(start, &message);
        Ok(None)
    }
    /// The parameter curves of an edge of a loop when they answer one 3D edge with one curve each.
    fn loop_curves_2d(
        live: &IgesSolidLoop,
        rank: i32,
        nb_3d: i32,
    ) -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure> {
        let nb_parameter = live.nb_parameter_curves(rank)?;
        if nb_3d != 1 || nb_parameter != 1 {
            return Ok(Handle::default());
        }
        let mut sequence = Vec::new();
        let curve = live.parametric_curve(rank, 1)?;
        if IgesToBRep::iges_curve_to_sequence_of_iges_curve(&curve, &mut sequence) != 1 {
            return Ok(Handle::default());
        }
        let curves = iges_new_array1(1, nb_parameter)?;
        iges_array1_set(&curves, 1, curve, WHERE)?;
        Ok(curves)
    }
    /// The wire of a loop on a face, once; `tool.transfer_loop(&entity, &face, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the wire.
    pub fn transfer_loop(
        &self,
        start: &Handle<IgesSolidLoop>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if !self.has_shape_result(&entity)? {
            let wire = self.built_loop(start, face, trans, u_fact)?;
            self.set_shape_result(&entity, &wire)?;
        }
        let shape = self.get_shape_result(&entity)?;
        if shape.is_null() {
            self.warn_label(start, b"loop")?;
        }
        Ok(shape)
    }
    fn built_loop(
        &self,
        start: &Handle<IgesSolidLoop>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        let live = iges_live(start, WHERE)?;
        let boundary = IgesToBRep::algo_container()?
            .tool_container()?
            .iges_boundary();
        let tool = iges_live(&boundary, WHERE)?;
        // The file preference of a loop is its 3D representation.
        tool.init(self, &start.cast(), face, trans, u_fact, 3);
        let mut flags = BoundaryFlags::default();
        let mut wires: Handle<ShapeExtendWireData> = Handle::default();
        for rank in 1..=live.nb_edges() {
            if live.edge(rank)?.is_null() {
                let mut message = Msg::from_key(&"IGES_1365".into());
                message.arg_integer(rank)?;
                self.send_warning(start, &message);
                continue;
            }
            let Some(curve_3d) = self.loop_edge_3d(start, rank)? else {
                continue;
            };
            let edges_3d = iges_live(&curve_3d, WHERE)?;
            let orientation = live.orientation(rank)?;
            if !orientation {
                edges_3d.reverse()?;
            }
            let curves_2d = Self::loop_curves_2d(live, rank, edges_3d.nb_edges())?;
            let (_, local) =
                tool.transfer_wire_data(&mut flags, &curve_3d, &curves_2d, !orientation, rank)?;
            if rank == 1 {
                wires = tool.base().wire_data();
            }
            let local_list = iges_live(&local, WHERE)?;
            if edges_3d.nb_edges() == 1 && local_list.nb_edges() == 1 {
                let (from, to) = (local_list.edge(1)?, edges_3d.edge(1)?);
                if !from.is_same(&to) {
                    ShapeBuildEdge.remove_curve_3d(&to)?;
                    IgesToBRep::transfer_pcurve(&from, &to, face)?;
                    let list = iges_live(&wires, WHERE)?;
                    list.set(&to, list.index(&from)?)?;
                }
            }
        }
        match wires.get() {
            Some(list) => Ok(list.wire()?.into_shape()),
            None => Ok(Shape::default()),
        }
    }
    /// The surface of a face, null with a warning for a kind that is not allowed there.
    fn face_surface(
        &self,
        start: &Handle<IgesSolidFace>,
        surface: &Handle<IgesDataIgesEntity>,
    ) -> bool {
        let allowed = surface.is_some()
            && IgesToBRep::is_topo_surface(surface)
            && surface.cast::<IgesGeomPlane>().is_null()
            && surface.cast::<IgesGeomBoundedSurface>().is_null()
            && surface.cast::<IgesGeomTrimmedSurface>().is_null()
            && surface.cast::<IgesBasicSingleParent>().is_null();
        if !allowed {
            self.warn_key(start, "XSTEP_196");
        }
        allowed
    }
    fn built_face(
        &self,
        start: &Handle<IgesSolidFace>,
        param: (Shape, Transform2, f64),
    ) -> Result<(), Failure> {
        let (shape, trans, u_fact) = param;
        if shape.shape_type()? != ShapeEnum::Face {
            return Ok(());
        }
        let live = iges_live(start, WHERE)?;
        let builder = BRepBuilder::default();
        let mut face = topods::face(&shape)?;
        face.empty_copy()?;
        let nb_loops = live.nb_loops()?;
        if nb_loops == 0 {
            builder.natural_restriction(&face, true)?;
        }
        for rank in 1..=nb_loops {
            let wire = self.transfer_loop(&live.loop_(rank)?, &face, trans, u_fact)?;
            if !wire.is_null() {
                builder.add(&mut face, &wire)?;
            }
        }
        brep_tools::update_face(&face)?;
        face.set_orientable(true)?;
        self.set_shape_result(&start.cast(), &face)
    }
    /// The face of a face entity, once; `tool.transfer_face(&entity)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_face(&self, start: &Handle<IgesSolidFace>) -> Result<Shape, Failure> {
        let entity: Handle<IgesDataIgesEntity> = start.cast();
        if !self.has_shape_result(&entity)? {
            let live = iges_live(start, WHERE)?;
            let surface = live.surface();
            if !self.face_surface(start, &surface) {
                self.set_shape_result(&entity, &Shape::default())?;
            } else {
                let mut tool = IgesToBRepTopoSurface::with_base(self);
                let param = tool.param_surface(&surface)?;
                if param.0.is_null() {
                    let message = self.label_message(&surface, b"surface")?;
                    self.send_fail(start, &message);
                    self.set_shape_result(&entity, &Shape::default())?;
                } else {
                    self.built_face(start, param)?;
                }
            }
        }
        let shape = self.get_shape_result(&entity)?;
        if shape.is_null() {
            self.fail_label(start, b"face")?;
        }
        Ok(shape)
    }
}
