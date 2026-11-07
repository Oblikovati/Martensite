//! Planes (type 108) with their bounding curves, and perforated planes.
use super::{IgesToBRep, IgesToBRepTopoSurface};
use crate::iges_basic::IgesBasicSingleParent;
use crate::iges_data::{IgesDataIgesEntity, IgesDataToolLocation, iges_live};
use crate::iges_geom::{IgesGeomCurveOnSurface, IgesGeomPlane};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape, Wire};
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomGeometry, GeomPlane};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Plane3, Point3, Transform2, Transform3, TransformForm};
use martensite_tkmath::toploc::Location;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tktopalgo::brep_lib::BRepLibMakeFace;

const WHERE: &str = "IgesToBRepTopoSurface";

/// What the parts of a plane entity give.
pub struct PlaneParts {
    /// The face, for the first plane, or the wire, for a hole.
    pub shape: Shape,
    /// The plane of the equation.
    pub plane: Plane3,
    /// The location of the entity.
    pub location: Transform3,
}

/// The wire of a bounding curve; `None` for a shape that is neither an edge nor a wire.
fn bounding_wire(shape: &Shape) -> Result<Option<Wire>, Failure> {
    match shape.shape_type()? {
        ShapeEnum::Edge => {
            let list = ShapeExtendWireData::new();
            list.add_edge(&topods::edge(shape)?, 0)?;
            list.wire().map(Some)
        }
        ShapeEnum::Wire => topods::wire(shape).map(Some),
        _ => Ok(None),
    }
}

impl IgesToBRepTopoSurface {
    /// The face of a plane at its location; `tool.transfer_plane(&plane)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_plane(&mut self, start: &Handle<IgesGeomPlane>) -> Result<Shape, Failure> {
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        }
        let parts = self.transfer_plane_parts(start, true)?;
        let mut result = parts.shape;
        if parts.location.form() != TransformForm::Identity {
            result.set_location(&Location::new(parts.location), false)?;
        }
        Ok(result)
    }
    /// A hole of a perforated plane, on the face.
    fn add_perforation(
        &mut self,
        start: &Handle<IgesBasicSingleParent>,
        child: &Handle<IgesDataIgesEntity>,
        rank: i32,
        (result, plane): (&mut Shape, Plane3),
    ) -> Result<(), Failure> {
        let hole = child.cast::<IgesGeomPlane>();
        if hole.is_null() {
            let mut message = Msg::from_key(&"IGES_1285".into());
            message.arg_integer(rank)?;
            self.send_warning(start, &message);
            return Ok(());
        }
        let parts = self.transfer_plane_parts(&hole, false)?;
        let mut wire = parts.shape;
        if wire.is_null() || wire.shape_type()? != ShapeEnum::Wire {
            let message = self.label_message(&hole.cast(), b"hole")?;
            self.send_warning(start, &message);
            return Ok(());
        }
        let coplanar = plane.position().is_coplanar(
            parts.plane.position(),
            self.get_eps_geom(),
            self.get_epsilon(),
        );
        if !coplanar {
            let mut message = Msg::from_key(&"IGES_1295".into());
            message.arg_integer(rank)?;
            self.send_warning(start, &message);
        }
        if parts.location.form() != TransformForm::Identity {
            wire.set_location(&Location::new(parts.location), false)?;
        }
        BRepBuilder::default().add(result, &wire)
    }
    /// The face of a plane with its holes; `tool.transfer_perforate(&parent)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the face.
    pub fn transfer_perforate(
        &mut self,
        start: &Handle<IgesBasicSingleParent>,
    ) -> Result<Shape, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Shape::default());
        };
        let parent = live.single_parent().cast::<IgesGeomPlane>();
        if parent.is_null() {
            self.fail_key(start, "XSTEP_206");
            return Ok(Shape::default());
        }
        let parts = self.transfer_plane_parts(&parent, true)?;
        let mut result = parts.shape;
        for rank in 1..=live.nb_children() {
            let child = live.child(rank)?;
            self.add_perforation(start, &child, rank, (&mut result, parts.plane))?;
        }
        if parts.location.form() != TransformForm::Identity {
            result.set_location(&Location::new(parts.location), false)?;
        }
        Ok(result)
    }
    /// An empty infinite face on the plane in the file unit.
    fn infinite_plane_face(&self, plane: Plane3) -> Result<Face, Failure> {
        let builder = BRepBuilder::default();
        let mut face = Face::default();
        builder.make_face(&mut face);
        let surface = GeomPlane::from_pln(plane);
        surface.scale(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor())?;
        builder.update_face(
            &face,
            &Handle::new(surface).cast(),
            &Location::default(),
            precision::confusion(),
        )?;
        face.set_infinite(true)?;
        Ok(face)
    }
    /// A wire of the bounding curve, in the sense its hole flag asks for; `Err` gives the shape to return.
    fn oriented_bounding_wire(
        &self,
        live: &IgesGeomPlane,
        plane: Plane3,
        mut wire: Wire,
    ) -> Result<Option<Wire>, Failure> {
        let fix = ShapeFixWire::new();
        fix.load(&wire)?;
        fix.fix_connected_all(self.get_max_tol())?;
        wire = fix.wire()?;
        let maker = BRepLibMakeFace::from_plane_wire(plane, &wire, false)?;
        if !maker.is_done() {
            return Ok(None);
        }
        let mut properties = GProps::new();
        BRepGProp::surface_properties(&maker.face()?.into_shape(), &mut properties, false, false)?;
        // A negative area is a wire that runs as a hole.
        if (properties.mass() < 0.0) != live.has_bounding_curve_hole() {
            wire.reverse();
        }
        Ok(Some(wire))
    }
    /// The bounding curve of a plane, on its face for the first plane. Gives its wire; `Err` holds
    /// what the caller returns at once.
    fn plane_bounding_curve(
        &mut self,
        start: &Handle<IgesGeomPlane>,
        live: &IgesGeomPlane,
        plane: Plane3,
        face: &mut Face,
        first: bool,
    ) -> Result<Result<Wire, Shape>, Failure> {
        let curve = live.bounding_curve();
        if curve.is_null() {
            self.warn_key(start, "IGES_1300");
            return Ok(Ok(Wire::default()));
        }
        if !IgesToBRep::is_topo_curve(&curve) {
            let message = self.label_message(&curve, b"Bounding curve")?;
            self.send_warning(start, &message);
            return Ok(Ok(Wire::default()));
        }
        let mut tool = self.curve_tool();
        let on_surface = curve.cast::<IgesGeomCurveOnSurface>();
        if on_surface.is_some() {
            let length = self.u_length;
            let _ = tool.transfer_curve_on_face(
                face,
                &on_surface,
                Transform2::default(),
                length,
                false,
            )?;
            if first && Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape)?.more() {
                face.set_infinite(false)?;
            }
            return Ok(Ok(Wire::default()));
        }
        let shape = tool.transfer_topo_curve(&curve)?;
        let Some(wire) = bounding_wire(&shape)? else {
            let message = self.label_message(&curve, b"Bounding curve")?;
            self.send_warning(start, &message);
            let early = if first {
                face.clone().into_shape()
            } else {
                Shape::default()
            };
            return Ok(Err(early));
        };
        let Some(wire) = self.oriented_bounding_wire(live, plane, wire)? else {
            return Ok(Err(Shape::default()));
        };
        if first {
            BRepBuilder::default().add(face, &wire)?;
            face.set_infinite(false)?;
        }
        Ok(Ok(wire))
    }
    /// The plane of the entity with its face for the first plane of a surface, or the wire of its
    /// bounding curve for a hole; `tool.transfer_plane_parts(&plane, true)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_plane_parts(
        &mut self,
        start: &Handle<IgesGeomPlane>,
        first: bool,
    ) -> Result<PlaneParts, Failure> {
        let live = iges_live(start, WHERE)?;
        let (mut a, mut b, mut c, mut d) = (0.0, 0.0, 0.0, 0.0);
        live.equation(&mut a, &mut b, &mut c, &mut d);
        let plane = Plane3::from_equation(a, b, c, -d)?;
        let mut face = if first {
            self.infinite_plane_face(plane)?
        } else {
            Face::default()
        };
        let mut location = Transform3::default();
        if live.has_transf() {
            self.set_epsilon(1.0e-4);
            let compound = live.compound_location()?;
            let unit = self.get_unit_factor();
            if !IgesDataToolLocation::convert_location(
                self.get_epsilon(),
                &compound,
                &mut location,
                unit,
            )? {
                self.warn_key(start, "IGES_1035");
            }
        }
        let mut wire = Wire::default();
        if live.has_bounding_curve() {
            match self.plane_bounding_curve(start, live, plane, &mut face, first)? {
                Ok(made) => wire = made,
                Err(shape) => {
                    return Ok(PlaneParts {
                        shape,
                        plane,
                        location,
                    });
                }
            }
        }
        let shape = if first {
            face.into_shape()
        } else {
            wire.into_shape()
        };
        Ok(PlaneParts {
            shape,
            plane,
            location,
        })
    }
}
