//! Composite curves as wires.
use super::{IgesToBRep, IgesToBRepTopoCurve};
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{IgesGeomBoundary, IgesGeomCompositeCurve, IgesGeomCurveOnSurface};
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::message::Msg;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Transform2;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisWire;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;

const WHERE: &str = "IgesToBRepTopoCurve";

/// Where the curves of a composite go: in space, or in the parameters of a face.
#[derive(Clone, Copy)]
struct CompositeTarget<'a> {
    on_face: Option<(&'a Face, Transform2, f64)>,
}

impl CompositeTarget<'_> {
    fn dimension(self) -> i32 {
        if self.on_face.is_some() { 2 } else { 3 }
    }
}

impl IgesToBRepTopoCurve {
    fn composite_message(
        &self,
        key: &str,
        target: CompositeTarget<'_>,
        rank: i32,
    ) -> Result<Msg, Failure> {
        let mut message = Msg::from_key(&key.into());
        message.arg_integer(target.dimension())?;
        message.arg_integer(rank)?;
        Ok(message)
    }
    /// Appends one curve; `false` after a fail, which ends the composite.
    fn connect_composite_curve(
        &mut self,
        start: &Handle<IgesGeomCompositeCurve>,
        analysis: &Handle<ShapeAnalysisWire>,
        shape: &Shape,
        target: CompositeTarget<'_>,
        rank: i32,
    ) -> Result<bool, Failure> {
        let next = Handle::new(ShapeExtendWireData::new());
        iges_live(&next, WHERE)?.add_shape(shape, 0)?;
        let container = ShapeAlgo::algo_container();
        let connection =
            iges_live(&container, WHERE)?.connect_next_wire(analysis, &next, self.get_max_tol())?;
        if !connection.connected {
            self.send_fail(start, &self.composite_message("IGES_1060", target, rank)?);
            return Ok(false);
        }
        if connection.reversed_wire {
            self.send_warning(start, &self.composite_message("IGES_1051", target, rank)?);
        }
        if connection.reversed_next {
            self.send_warning(start, &self.composite_message("IGES_1050", target, rank)?);
        }
        if connection.distance > self.get_eps_geom() * self.get_unit_factor() {
            self.send_warning(start, &self.composite_message("IGES_1055", target, rank)?);
        }
        Ok(true)
    }
    fn composite_part(
        &mut self,
        part: &Handle<IgesDataIgesEntity>,
        target: CompositeTarget<'_>,
    ) -> Result<Shape, Failure> {
        match target.on_face {
            Some((face, trans, u_fact)) => self.transfer_2d_topo_curve(part, face, trans, u_fact),
            None => self.transfer_topo_curve(part),
        }
    }
    fn composite_rank_fail(
        &self,
        start: &Handle<IgesGeomCompositeCurve>,
        rank: i32,
    ) -> Result<(), Failure> {
        let mut message = Msg::from_key(&"IGES_1040".into());
        message.arg_integer(rank)?;
        self.send_fail(start, &message);
        Ok(())
    }
    /// Goes through the curves; `false` after a fail.
    fn collect_composite(
        &mut self,
        start: &Handle<IgesGeomCompositeCurve>,
        analysis: &Handle<ShapeAnalysisWire>,
        target: CompositeTarget<'_>,
    ) -> Result<bool, Failure> {
        let live = iges_live(start, WHERE)?;
        let mut previous: Handle<IgesDataIgesEntity> = Handle::default();
        for rank in 1..=live.nb_curves() {
            let part = live.curve(rank)?;
            // A part given twice in a row counts once.
            if rank > 1 && part.object_address() == previous.object_address() {
                let mut message = Msg::from_key(&"IGES_1045".into());
                message.arg_integer(rank)?;
                self.send_warning(start, &message);
                continue;
            }
            previous = part.clone();
            let flat_only = part.cast::<IgesGeomBoundary>().is_some()
                || part.cast::<IgesGeomCurveOnSurface>().is_some();
            if part.is_null() || (target.on_face.is_some() && flat_only) {
                self.composite_rank_fail(start, rank)?;
                return Ok(false);
            }
            if !IgesToBRep::is_topo_curve(&part) {
                continue;
            }
            let shape = self.composite_part(&part, target)?;
            if shape.is_null() || shape.shape_type()? == ShapeEnum::Vertex {
                continue;
            }
            if !self.connect_composite_curve(start, analysis, &shape, target, rank)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn transfer_composite_general(
        &mut self,
        start: &Handle<IgesGeomCompositeCurve>,
        target: CompositeTarget<'_>,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Shape::default());
        }
        let precision = self.get_eps_geom() * self.get_unit_factor();
        let edges = Handle::new(ShapeExtendWireData::new());
        let analysis = Handle::new(ShapeAnalysisWire::new());
        let analyser = iges_live(&analysis, WHERE)?;
        analyser.load_wire_data(&edges);
        analyser.set_precision(precision);
        if !self.collect_composite(start, &analysis, target)? {
            return Ok(Shape::default());
        }
        let fix = ShapeFixWire::new();
        fix.load_wire_data(&edges)?;
        // The closing of face boundaries comes later.
        fix.set_closed_mode(false);
        fix.fix_connected_all(self.get_max_tol())?;
        // A composite curve on a plane closes at its first joint.
        fix.fix_connected_at(1, precision, true)?;
        let result = iges_live(&edges, WHERE)?.wire()?.into_shape();
        self.set_shape_result(&start.cast(), &result)?;
        Ok(result)
    }
    /// The wire of a composite curve, at its location;
    /// `tool.transfer_composite_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the wire.
    pub fn transfer_composite_curve(
        &mut self,
        start: &Handle<IgesGeomCompositeCurve>,
    ) -> Result<Shape, Failure> {
        let target = CompositeTarget { on_face: None };
        let mut result = self.transfer_composite_general(start, target)?;
        if let Some(live) = start.get()
            && !result.is_null()
            && live.has_transf()
        {
            self.moved_to_location(live, &start.cast(), &mut result, true)?;
        }
        Ok(result)
    }
    /// The wire of a composite curve in the parameters of a face;
    /// `tool.transfer_2d_composite_curve(&curve, &face, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the wire.
    pub fn transfer_2d_composite_curve(
        &mut self,
        start: &Handle<IgesGeomCompositeCurve>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        let target = CompositeTarget {
            on_face: Some((face, trans, u_fact)),
        };
        self.transfer_composite_general(start, target)
    }
}
