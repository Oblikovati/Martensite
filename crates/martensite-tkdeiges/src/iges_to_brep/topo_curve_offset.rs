//! Offset curves (type 130).
use super::{IgesToBRep, IgesToBRepTopoCurve};
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::IgesGeomOffsetCurve;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, ShapeIterator, Wire};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dOffsetCurve;
use martensite_tkg3d::geom::GeomOffsetCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction3, Transform2};
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};

const WHERE: &str = "IgesToBRepTopoCurve";

/// The fixed wire of the edges, with its connections made at the tolerance of its vertices.
fn fixed_wire(edges: &Handle<ShapeExtendWireData>) -> Result<Shape, Failure> {
    let fix = ShapeFixWire::new();
    fix.load_wire_data(edges)?;
    // The minimum tolerance.
    fix.fix_connected_all(-1.0)?;
    Ok(fix.wire()?.into_shape())
}

impl IgesToBRepTopoCurve {
    fn offset_base_failure(
        &self,
        start: &Handle<IgesGeomOffsetCurve>,
        base: &Handle<IgesDataIgesEntity>,
        what: &[u8],
    ) -> Result<(), Failure> {
        self.send_fail(start, &self.label_message(base, what)?);
        Ok(())
    }
    /// The curve under nested offsets and the sum of the distances; `None` after a fail.
    fn base_and_distance(
        &self,
        start: &Handle<IgesGeomOffsetCurve>,
        live: &IgesGeomOffsetCurve,
    ) -> Option<(Handle<IgesDataIgesEntity>, f64)> {
        let mut distance = live.first_offset_distance();
        let mut base = live.base_curve();
        loop {
            let nested = base.cast::<IgesGeomOffsetCurve>();
            let Some(inner) = nested.get() else {
                return Some((base, distance));
            };
            if inner.offset_type() != 1 {
                self.fail_key(start, "IGES_1100");
                return None;
            }
            distance += inner.first_offset_distance();
            base = inner.base_curve();
        }
    }
    /// The edge offset from an edge; a null edge after a fail.
    fn offset_edge(
        &self,
        start: &Handle<IgesGeomOffsetCurve>,
        edge: &Edge,
        distance: f64,
        normal: Direction3,
        (first, last): (f64, f64),
    ) -> Result<Edge, Failure> {
        let carried = BRepTool::curve_with_location(edge)?;
        let offset = GeomOffsetCurve::new(&carried.curve, distance, normal, false)?;
        let maker =
            BRepBuilderAPIMakeEdge::from_curve_params(&Handle::new(offset).cast(), first, last)?;
        if !maker.is_done() {
            self.null_entity(start);
            return Ok(Edge::default());
        }
        let mut made = maker.edge()?;
        made.move_by(&carried.location, false)?;
        Ok(made)
    }
    fn offset_of_wire(
        &self,
        start: &Handle<IgesGeomOffsetCurve>,
        live: &IgesGeomOffsetCurve,
        wire: &Wire,
        distance: f64,
        normal: Direction3,
    ) -> Result<Option<Shape>, Failure> {
        let edges = Handle::new(ShapeExtendWireData::new());
        let list = iges_live(&edges, WHERE)?;
        let (mut begin, mut length) = (true, 0.0);
        let (start_parameter, end_parameter) = (live.start_parameter(), live.end_parameter());
        let mut iterator = ShapeIterator::new(wire, true, true);
        while iterator.more() {
            let edge = topods::edge(iterator.value()?)?;
            iterator.next();
            let carried = BRepTool::curve_with_location(&edge)?;
            let span = carried.last - carried.first;
            if length + span <= start_parameter {
                continue;
            }
            if length >= end_parameter {
                if begin {
                    self.fail_key(start, "IGES_1105");
                    return Ok(None);
                }
                break;
            }
            let range = (start_parameter - length, end_parameter - length);
            let made = self.offset_edge(start, &edge, distance, normal, range)?;
            if made.is_null() {
                return Ok(None);
            }
            begin = false;
            length += span;
            list.add_edge(&made, 0)?;
        }
        fixed_wire(&edges).map(Some)
    }
    /// The edge or the wire of an offset curve; only a constant distance is treated;
    /// `tool.transfer_offset_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_offset_curve(
        &mut self,
        start: &Handle<IgesGeomOffsetCurve>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        if live.offset_type() != 1 {
            self.fail_key(start, "IGES_1100");
            return Ok(Shape::default());
        }
        let normal = Direction3::from_vector(if live.has_transf() {
            live.transformed_normal_vector()?
        } else {
            live.normal_vector()
        })?;
        // The offset direction goes with the reversed normal.
        let normal = normal.reversed();
        let Some((base, distance)) = self.base_and_distance(start, live) else {
            return Ok(Shape::default());
        };
        if !IgesToBRep::is_topo_curve(&base) {
            self.fail_key(start, "XSTEP_110");
            return Ok(Shape::default());
        }
        let mut tool = self.clone();
        let shape = tool.transfer_topo_curve(&base)?;
        let kind = if shape.is_null() {
            None
        } else {
            Some(shape.shape_type()?)
        };
        let mut result = match kind {
            Some(ShapeEnum::Edge) => {
                let range = (live.start_parameter(), live.end_parameter());
                let made =
                    self.offset_edge(start, &topods::edge(&shape)?, distance, normal, range)?;
                if made.is_null() {
                    return Ok(Shape::default());
                }
                made.into_shape()
            }
            Some(ShapeEnum::Wire) => {
                let wire = topods::wire(&shape)?;
                match self.offset_of_wire(start, live, &wire, distance, normal)? {
                    Some(made) => made,
                    None => return Ok(Shape::default()),
                }
            }
            _ => {
                self.offset_base_failure(start, &base, b"basis curve")?;
                return Ok(Shape::default());
            }
        };
        if live.has_transf() {
            self.moved_to_location(live, &start.cast(), &mut result, true)?;
        }
        self.set_shape_result(&start.cast(), &result)?;
        Ok(result)
    }
    /// The edge offset from a pcurve on the face; a null edge after a fail.
    fn offset_2d_edge(
        &self,
        start: &Handle<IgesGeomOffsetCurve>,
        live: &IgesGeomOffsetCurve,
        edge: &Edge,
        (face, distance): (&Face, f64),
    ) -> Result<Edge, Failure> {
        let carried = BRepTool::curve_on_surface_by_index(edge, 1)?;
        let offset = Geom2dOffsetCurve::new(&carried.pcurve, distance, false)?;
        let mut made = Edge::default();
        ShapeBuildEdge.make_edge_on_face_range(
            &mut made,
            &Handle::new(offset).cast(),
            face,
            live.start_parameter(),
            live.end_parameter(),
        )?;
        if made.is_null() {
            self.null_entity(start);
        }
        Ok(made)
    }
    /// The edge or the wire of an offset curve on a face; a distance that is not constant has
    /// a fail and goes on; `tool.transfer_2d_offset_curve(&curve, &face, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_2d_offset_curve(
        &mut self,
        start: &Handle<IgesGeomOffsetCurve>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        if live.offset_type() != 1 {
            self.fail_key(start, "IGES_1100");
        }
        let distance = live.first_offset_distance() * u_fact;
        let base = live.base_curve();
        let mut result = Shape::default();
        if IgesToBRep::is_topo_curve(&base) {
            let mut tool = self.clone();
            tool.set_mode_transfer(false);
            let shape = tool.transfer_2d_topo_curve(&base, face, trans, u_fact)?;
            let kind = if shape.is_null() {
                None
            } else {
                Some(shape.shape_type()?)
            };
            result = match kind {
                Some(ShapeEnum::Edge) => {
                    let made =
                        self.offset_2d_edge(start, live, &topods::edge(&shape)?, (face, distance))?;
                    if made.is_null() {
                        return Ok(Shape::default());
                    }
                    made.into_shape()
                }
                Some(ShapeEnum::Wire) => {
                    let edges = Handle::new(ShapeExtendWireData::new());
                    let list = iges_live(&edges, WHERE)?;
                    let mut iterator = ShapeIterator::new(&shape, true, true);
                    while iterator.more() {
                        let edge = topods::edge(iterator.value()?)?;
                        iterator.next();
                        let made = self.offset_2d_edge(start, live, &edge, (face, distance))?;
                        if made.is_null() {
                            return Ok(Shape::default());
                        }
                        list.add_edge(&made, 0)?;
                    }
                    fixed_wire(&edges)?
                }
                _ => {
                    self.offset_base_failure(start, &base, b"2D basis curve")?;
                    return Ok(Shape::default());
                }
            };
        }
        self.set_shape_result(&start.cast(), &result)?;
        Ok(result)
    }
}
