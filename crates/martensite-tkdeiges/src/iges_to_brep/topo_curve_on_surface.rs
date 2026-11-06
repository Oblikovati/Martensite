//! Curves on surfaces (type 142) and boundaries (type 141): wires with pcurves on a face.
use super::{IgesToBRep, IgesToBRepTopoCurve, IgesToBRepTopoSurface};
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataToolLocation, iges_array1_set, iges_live, iges_new_array1,
};
use crate::iges_geom::{IgesGeomBoundary, IgesGeomCurveOnSurface};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Face, Shape, ShapeIterator, Wire};
use martensite_tkernel::message::Msg;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Transform2, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_lib::BRepLibMakeFace;

const WHERE: &str = "IgesToBRepTopoCurve";

/// The single face of a shell or a face, `None` for a shell of another count.
fn single_face(shape: &Shape) -> Result<Option<Face>, Failure> {
    let mut iterator = ShapeIterator::new(shape, true, true);
    let (mut count, mut last) = (0, Face::default());
    while iterator.more() {
        count += 1;
        last = topods::face(iterator.value()?)?;
        iterator.next();
    }
    Ok((count == 1).then_some(last))
}

/// A face as the surface entity gives it; `Err` when the shape is neither one face nor a shell of one.
enum ParametricFace {
    Found(Face),
    Unusable,
}

fn parametric_face(shape: &Shape) -> Result<ParametricFace, Failure> {
    if shape.is_null() {
        return Ok(ParametricFace::Found(Face::default()));
    }
    Ok(match shape.shape_type()? {
        ShapeEnum::Face => ParametricFace::Found(topods::face(shape)?),
        ShapeEnum::Shell => match single_face(shape)? {
            Some(face) => ParametricFace::Found(face),
            None => ParametricFace::Unusable,
        },
        _ => ParametricFace::Unusable,
    })
}

impl IgesToBRepTopoCurve {
    /// The 3D curve of a curve on surface whose surface cannot serve.
    fn three_d_fallback(
        &mut self,
        start: &Handle<IgesGeomCurveOnSurface>,
        live: &IgesGeomCurveOnSurface,
    ) -> Result<Shape, Failure> {
        let curve = live.curve3d();
        if curve.is_null() {
            let mut message = Msg::from_key(&"IGES_1061".into());
            message.arg_text(b"CurveOnSurface")?;
            self.send_fail(start, &message);
            return Ok(Shape::default());
        }
        if !IgesToBRep::is_topo_curve(&curve) {
            return Ok(Shape::default());
        }
        let shape = self.transfer_topo_curve(&curve)?;
        if !shape.is_null() {
            self.warn_key(start, "IGES_1062");
        }
        Ok(shape)
    }
    /// The wire of a curve on surface, on an empty copy of the face of its surface; with an unusable
    /// surface, its 3D curve; `tool.transfer_curve_on_surface(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_curve_on_surface(
        &mut self,
        start: &Handle<IgesGeomCurveOnSurface>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        let surface = live.surface();
        if surface.is_null() || !IgesToBRep::is_topo_surface(&surface) {
            self.fail_key(start, "XSTEP_131");
            return Ok(Shape::default());
        }
        let mut tool = IgesToBRepTopoSurface::with_base(self);
        let (shape, trans, u_fact) = tool.param_surface(&surface)?;
        let mut face = match parametric_face(&shape)? {
            ParametricFace::Found(face) => face,
            ParametricFace::Unusable => return self.three_d_fallback(start, live),
        };
        face.empty_copy()?;
        self.transfer_curve_on_face(&mut face, start, trans, u_fact, true)
    }
    /// A location that moves the wire, without a unit: the curves are already in its scale.
    fn moved_wire(
        &mut self,
        entity: &IgesDataIgesEntity,
        handle: &Handle<IgesDataIgesEntity>,
        wire: &mut Wire,
    ) -> Result<(), Failure> {
        if !entity.has_transf() {
            return Ok(());
        }
        let mut similarity = Transform3::default();
        self.set_epsilon(1.0e-4);
        let location = entity.compound_location()?;
        if IgesDataToolLocation::convert_location(
            self.get_epsilon(),
            &location,
            &mut similarity,
            1.0,
        )? {
            wire.move_by(&Location::new(similarity), false)?;
        } else {
            self.warn_key(handle, "IGES_1035");
        }
        Ok(())
    }
    /// Adds the wire of a curve on surface to the face; `is_curve_on_surf` is false when the curve is
    /// a boundary of a trimmed surface; `tool.transfer_curve_on_face(&mut face, &curve, trans, 1.0, true)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_curve_on_face(
        &mut self,
        face: &mut Face,
        start: &Handle<IgesGeomCurveOnSurface>,
        trans: Transform2,
        u_fact: f64,
        is_curve_on_surf: bool,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        let preference = match live.preference_mode() {
            1 => 2,
            2 => 3,
            _ => 0,
        };
        let curves_2d: Handle<HArray1<Handle<IgesDataIgesEntity>>> = iges_new_array1(1, 1)?;
        iges_array1_set(&curves_2d, 1, live.curve_uv(), WHERE)?;
        let boundary = IgesToBRep::algo_container()?
            .tool_container()?
            .iges_boundary();
        let tool = iges_live(&boundary, WHERE)?;
        tool.init(self, &start.cast(), face, trans, u_fact, preference);
        let mut flags = super::BoundaryFlags::default();
        let result = tool.transfer_entity(&mut flags, &live.curve3d(), false, &curves_2d, 1)?;
        tool.check(
            result,
            !is_curve_on_surf,
            flags.ok_curve_3d,
            flags.ok_curve_2d,
        )?;
        let edges = tool.base().wire_data();
        let list = iges_live(&edges, WHERE)?;
        if list.nb_edges() == 0 {
            self.fail_key(start, "IGES_1095");
            return Ok(Shape::default());
        }
        let mut wire = list.wire()?;
        self.moved_wire(live, &start.cast(), &mut wire)?;
        let mut maker = BRepLibMakeFace::from_face(face)?;
        maker.add(&wire)?;
        *face = maker.face()?;
        let result = wire.into_shape();
        self.set_shape_result(&start.cast(), &result)?;
        Ok(result)
    }
    /// A boundary in the parameters of a face: on an empty copy of the face of its surface;
    /// `tool.transfer_boundary(&boundary)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_boundary(
        &mut self,
        start: &Handle<IgesGeomBoundary>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        if live.boundary_type() == 0 {
            self.fail_key(start, "IGES_1125");
            return Ok(Shape::default());
        }
        let surface = live.surface();
        if surface.is_null() || !IgesToBRep::is_topo_surface(&surface) {
            self.fail_key(start, "XSTEP_124");
            return Ok(Shape::default());
        }
        let mut tool = IgesToBRepTopoSurface::with_base(self);
        let (shape, trans, u_fact) = tool.param_surface(&surface)?;
        let mut face = match parametric_face(&shape)? {
            ParametricFace::Found(face) => face,
            ParametricFace::Unusable => {
                return self.boundary_surface_failure(start, &surface, &shape);
            }
        };
        face.empty_copy()?;
        self.transfer_boundary_on_face(&mut face, start, trans, u_fact)
    }
    fn boundary_surface_failure(
        &self,
        start: &Handle<IgesGeomBoundary>,
        surface: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
    ) -> Result<Shape, Failure> {
        if shape.shape_type()? == ShapeEnum::Shell {
            self.fail_key(start, "IGES_1061");
            return Ok(Shape::default());
        }
        self.send_fail(start, &self.label_message(surface, b"basic surface")?);
        Ok(Shape::default())
    }
    /// Adds the wire of a boundary to the face;
    /// `tool.transfer_boundary_on_face(&mut face, &boundary, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_boundary_on_face(
        &mut self,
        face: &mut Face,
        start: &Handle<IgesGeomBoundary>,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Shape::default());
        };
        let preference = match live.preference_type() {
            2 => 2,
            1 => 3,
            _ => 0,
        };
        let boundary = IgesToBRep::algo_container()?
            .tool_container()?
            .iges_boundary();
        let tool = iges_live(&boundary, WHERE)?;
        tool.init(self, &start.cast(), face, trans, u_fact, preference);
        let mut flags = super::BoundaryFlags::default();
        let mut result = true;
        for rank in 1..=live.nb_model_space_curves()? {
            let mut curves_2d = Handle::default();
            if live.nb_parameter_curves(rank)? == 0 && live.boundary_type() == 1 {
                let mut message = Msg::from_key(&"IGES_1135".into());
                message.arg_integer(2)?;
                message.arg_integer(3)?;
                self.send_warning(start, &message);
            } else {
                curves_2d = live.parameter_curves(rank)?;
            }
            let reversed = live.sense(rank)? == 2;
            let curve = live.model_space_curve(rank)?;
            result &= tool.transfer_entity(&mut flags, &curve, reversed, &curves_2d, rank)?;
        }
        tool.check(result, true, flags.ok_curve_3d, flags.ok_curve_2d)?;
        let edges = tool.base().wire_data();
        let list = iges_live(&edges, WHERE)?;
        if list.nb_edges() == 0 {
            self.fail_key(start, "IGES_1095");
            return Ok(Shape::default());
        }
        // The first wire is outer, the others inner.
        let mut wire = list.wire()?;
        self.moved_wire(live, &start.cast(), &mut wire)?;
        BRepBuilder::default().add(face, &wire)?;
        let result = wire.into_shape();
        self.set_shape_result(&start.cast(), &result)?;
        Ok(result)
    }
}
