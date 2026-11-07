//! Basic curve entities as edges: a wire with a warning when the curve has to be split.
use super::{IgesToBRepBasicCurve, IgesToBRepTopoCurve};
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{IgesGeomCircularArc, IgesGeomSplineCurve};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, ShapeIterator};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomPlane, GeomTrimmedCurve};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Point2, Transform2, TransformForm};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};

const WHERE: &str = "IgesToBRepTopoCurve";

/// The wire of the edges, connected at the tolerance of its vertices.
pub(super) fn connected_wire(edges: &Handle<ShapeExtendWireData>) -> Result<(), Failure> {
    let fix = ShapeFixWire::new();
    fix.load_wire_data(edges)?;
    // The minimum tolerance.
    fix.fix_connected_all(-1.0)?;
    Ok(())
}

impl IgesToBRepTopoCurve {
    fn label_failure(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        what: &[u8],
    ) -> Result<(), Failure> {
        self.send_fail(start, &self.label_message(start, what)?);
        Ok(())
    }
    /// The curves to make edges of: the pieces of a polyline when asked, else the curve.
    fn split_for_approximation(&mut self, curve: Handle<dyn GeomCurve>) -> Result<(), Failure> {
        self.curves.clear();
        let bspline = curve.cast::<GeomBSplineCurve>();
        if self.get_mode_approx() && bspline.get().is_some_and(|live| live.degree() == 1) {
            return self.approx_bspline_curve(&bspline);
        }
        self.curves.push(curve);
        Ok(())
    }
    /// The edges of a curve; `false` after a fail. A C0 B-spline is split, or topology cannot use it.
    fn add_curve_edges(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        curve: &Handle<dyn GeomCurve>,
        edges: &ShapeExtendWireData,
    ) -> Result<bool, Failure> {
        let live = iges_live(curve, WHERE)?;
        let bspline = curve.cast::<GeomBSplineCurve>();
        if bspline.is_some() && live.continuity() < Continuity::C1 && self.get_continuity() >= 1 {
            let container = ShapeAlgo::algo_container();
            let pieces = iges_live(&container, WHERE)?
                .c0_bspline_to_sequence_of_c1_bspline_curve(&bspline)?;
            for piece in pieces
                .iter()
                .flat_map(|list| list.sequence(|items| items.iter().cloned().collect::<Vec<_>>()))
            {
                let maker = BRepBuilderAPIMakeEdge::from_curve(&piece.cast())?;
                if !maker.is_done() {
                    self.null_entity(start);
                    return Ok(false);
                }
                edges.add_edge(&maker.edge()?, 0)?;
            }
            return Ok(true);
        }
        let (first, last) = (live.first_parameter(), live.last_parameter());
        let trimmed = curve.cast::<GeomTrimmedCurve>();
        let basis = trimmed
            .get()
            .map_or_else(|| curve.clone(), GeomTrimmedCurve::basis_curve);
        let maker = BRepBuilderAPIMakeEdge::from_curve_params(&basis, first, last)?;
        if !maker.is_done() || precision::is_infinite(first) || precision::is_infinite(last) {
            self.null_entity(start);
            return Ok(false);
        }
        edges.add_edge(&maker.edge()?, 0)?;
        Ok(true)
    }
    /// The shape of a basic curve entity: an edge, or a wire with a warning. A closed arc marks the
    /// bad case: its ends can be an error of the file or a period, which 2D tells;
    /// `tool.transfer_topo_basic_curve(&entity)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_topo_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Shape::default());
        }
        let mut basic = IgesToBRepBasicCurve::with_base(self);
        // The position of the curve comes on the edge, not in the geometry.
        basic.set_mode_transfer(false);
        let curve = basic.transfer_basic_curve(start)?;
        if curve.is_null() {
            return Ok(Shape::default());
        }
        self.split_for_approximation(curve)?;
        if self.curves.is_empty() {
            self.label_failure(start, b"3D curve")?;
            return Ok(Shape::default());
        }
        let edges = Handle::new(ShapeExtendWireData::new());
        let list = iges_live(&edges, WHERE)?;
        for piece in self.curves.clone() {
            if !self.add_curve_edges(start, &piece, list)? {
                return Ok(Shape::default());
            }
        }
        connected_wire(&edges)?;
        let mut shape = list.wire()?.into_shape();
        // Only a spline curve keeps its position out of the geometry.
        let entity = iges_live(start, WHERE)?;
        if entity.has_transf() && start.cast::<IgesGeomSplineCurve>().is_some() {
            self.moved_to_location(entity, start, &mut shape, true)?;
        }
        if list.nb_edges() != 1 {
            self.warn_key(start, "IGES_1120");
            return Ok(shape);
        }
        let single: Edge = topods::edge(ShapeIterator::new(&shape, true, true).value()?)?;
        self.mark_bad_case(start);
        self.set_shape_result(start, &shape)?;
        Ok(single.into_shape())
    }
    /// A curve that gave nothing is marked.
    fn mark_bad_case(&mut self, start: &Handle<IgesDataIgesEntity>) {
        let arc = start.cast::<IgesGeomCircularArc>();
        if arc.get().is_some_and(IgesGeomCircularArc::is_closed) {
            self.bad_case = true;
        }
    }
    fn split_2d_for_approximation(
        &mut self,
        curve: Handle<dyn Geom2dCurve>,
    ) -> Result<(), Failure> {
        self.curves_2d.clear();
        let bspline = curve.cast::<Geom2dBSplineCurve>();
        if self.get_mode_approx() && bspline.get().is_some_and(|live| live.degree() == 1) {
            return self.approx_2d_bspline_curve(&bspline);
        }
        self.curves_2d.push(curve);
        Ok(())
    }
    /// The curve moved into the parameters of the face, and its range.
    fn placed_2d_curve(
        &self,
        curve: &Handle<dyn Geom2dCurve>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<(Handle<dyn Geom2dCurve>, f64, f64), Failure> {
        let live = iges_live(curve, WHERE)?;
        if trans.form() != TransformForm::Identity {
            live.transform(trans)?;
        }
        let mut scale = Transform2::default();
        let surface = BRepTool::surface(face)?;
        if surface.cast::<GeomPlane>().is_some() {
            scale.set_scale(Point2::new(0.0, 0.0), self.get_unit_factor());
        }
        let (first, last) = (live.first_parameter(), live.last_parameter());
        ShapeBuildEdge.transform_pcurve(curve, scale, u_fact, first, last)
    }
    fn add_curve_2d_edges(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        placed: (Handle<dyn Geom2dCurve>, f64, f64),
        face: &Face,
        edges: &ShapeExtendWireData,
    ) -> Result<bool, Failure> {
        let (curve, first, last) = placed;
        let live = iges_live(&curve, WHERE)?;
        let bspline = curve.cast::<Geom2dBSplineCurve>();
        let mut edge = Edge::default();
        if bspline.is_some() && live.continuity() < Continuity::C1 && self.get_continuity() >= 1 {
            let container = ShapeAlgo::algo_container();
            let pieces = iges_live(&container, WHERE)?
                .c0_bspline2d_to_sequence_of_c1_bspline_curve(&bspline)?;
            for piece in pieces
                .iter()
                .flat_map(|list| list.sequence(|items| items.iter().cloned().collect::<Vec<_>>()))
            {
                ShapeBuildEdge.make_edge_on_face(&mut edge, &piece.cast(), face)?;
                if edge.is_null() {
                    self.null_entity(start);
                    return Ok(false);
                }
                edges.add_edge(&edge, 0)?;
            }
            return Ok(true);
        }
        let trimmed = curve.cast::<Geom2dTrimmedCurve>();
        let basis = trimmed
            .get()
            .map_or_else(|| curve.clone(), Geom2dTrimmedCurve::basis_curve);
        ShapeBuildEdge.make_edge_on_face_range(&mut edge, &basis, face, first, last)?;
        if edge.is_null() {
            self.null_entity(start);
            return Ok(false);
        }
        edges.add_edge(&edge, 0)?;
        Ok(true)
    }
    /// The shape of a basic curve entity in the parameters of a face, its transformation applied
    /// to the curve; `tool.transfer_2d_topo_basic_curve(&entity, &face, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity, the face or the construction of the shape.
    pub fn transfer_2d_topo_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Shape::default());
        }
        let mut basic = IgesToBRepBasicCurve::with_base(self);
        basic.set_mode_transfer(false);
        let curve = basic.transfer_2d_basic_curve(start);
        if curve.is_null() {
            return Ok(Shape::default());
        }
        self.split_2d_for_approximation(curve)?;
        if self.curves_2d.is_empty() {
            self.label_failure(start, b"2D curve")?;
            return Ok(Shape::default());
        }
        let edges = Handle::new(ShapeExtendWireData::new());
        let list = iges_live(&edges, WHERE)?;
        for piece in self.curves_2d.clone() {
            let placed = self.placed_2d_curve(&piece, face, trans, u_fact)?;
            if !self.add_curve_2d_edges(start, placed, face, list)? {
                return Ok(Shape::default());
            }
        }
        connected_wire(&edges)?;
        let shape = list.wire()?.into_shape();
        if list.nb_edges() != 1 {
            self.warn_key(start, "IGES_1120");
            return Ok(shape);
        }
        self.mark_bad_case(start);
        Ok(list.edge(1)?.into_shape())
    }
}
