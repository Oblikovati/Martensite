//! The face of a surface entity with the transformation of its parameter curves.
use super::IgesToBRepTopoSurface;
use super::topo_surface::face_of_shape;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{
    IgesGeomCircularArc, IgesGeomOffsetSurface, IgesGeomRuledSurface, IgesGeomSurfaceOfRevolution,
    IgesGeomTabulatedCylinder,
};
use crate::iges_solid::{
    IgesSolidConicalSurface, IgesSolidCylindricalSurface, IgesSolidSphericalSurface,
    IgesSolidToroidalSurface,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::geom::{
    GeomConicalSurface, GeomCylindricalSurface, GeomRectangularTrimmedSurface,
    GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::elclib;
use martensite_tkmath::elslib;
use martensite_tkmath::gp::{
    Axis2d, Axis22d, Circle2, Direction2, Point2, Point3, StandardDirection2, Transform2, Vector2,
};
use martensite_tkshhealing::shape_analysis;
use std::f64::consts::PI;

const WHERE: &str = "IgesToBRepTopoSurface";

/// The parameter of the start point of an arc on its circle.
fn arc_start_parameter(arc: &IgesGeomCircularArc) -> Result<f64, Failure> {
    let frame = Axis22d::from_axis(Axis2d::new(arc.center(), Direction2::new(1.0, 0.0)?), true)?;
    let circle = Circle2::new(frame, arc.radius())?;
    elclib::parameter(&circle, arc.start_point())
}

/// The seam edge of a face: its pcurve is a line along V.
fn seam_edge(face: &Face) -> Result<Edge, Failure> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while explorer.more() {
        let edge = topods::edge(explorer.current())?;
        explorer.next()?;
        let carried = BRepTool::curve_on_face(&edge, face)?;
        let line = carried.pcurve.cast::<Geom2dLine>();
        let along_v = line.get().is_some_and(|live| {
            live.direction().is_parallel(
                Direction2::from(StandardDirection2::Y),
                precision::angular(),
            )
        });
        if along_v {
            return Ok(edge);
        }
    }
    Ok(Edge::default())
}

/// The shift of the parameters of an elementary surface, from the first point of its seam.
fn seam_parameters(
    start: &Handle<IgesDataIgesEntity>,
    surface: &Handle<dyn GeomSurface>,
    face: &Face,
) -> Result<(f64, f64), Failure> {
    let (curve, first, _) = BRepTool::curve(&seam_edge(face)?)?;
    let first = if precision::is_negative_infinite(first) {
        0.0
    } else {
        first
    };
    let point: Point3 = iges_live(&curve, WHERE)?.value(first)?;
    let cylinder = surface.cast::<GeomCylindricalSurface>();
    if let Some(live) = cylinder.get() {
        let shape = live.cylinder()?;
        return Ok(elslib::cylinder_parameters(
            shape.position(),
            shape.radius(),
            point,
        ));
    }
    let cone = surface.cast::<GeomConicalSurface>();
    if let Some(live) = cone.get() {
        let shape = live.cone()?;
        return Ok(elslib::cone_parameters(
            shape.position(),
            shape.ref_radius(),
            shape.semi_angle(),
            point,
        ));
    }
    let torus = surface.cast::<GeomToroidalSurface>();
    if let Some(live) = torus.get() {
        let shape = live.torus()?;
        return Ok(elslib::torus_parameters(
            shape.position(),
            shape.major_radius(),
            shape.minor_radius(),
            point,
        ));
    }
    let sphere = surface.cast::<GeomSphericalSurface>();
    if let Some(live) = sphere.get() {
        // On a spherical surface entity the parameters are its own.
        if start.cast::<IgesSolidSphericalSurface>().is_null() {
            let shape = live.sphere()?;
            return Ok(elslib::sphere_parameters(
                shape.position(),
                shape.radius(),
                point,
            ));
        }
    }
    Ok((0.0, 0.0))
}

/// The shift corrected by where the generatrix or the directrix starts.
fn phased_parameters(
    start: &Handle<IgesDataIgesEntity>,
    surface: &Handle<dyn GeomSurface>,
    (mut param_u, mut param_v): (f64, f64),
) -> Result<(f64, f64), Failure> {
    let revolution = start.cast::<IgesGeomSurfaceOfRevolution>();
    if let Some(live) = revolution.get() {
        let arc = live.generatrix().cast::<IgesGeomCircularArc>();
        if let Some(circle) = arc.get() {
            param_v -= arc_start_parameter(circle)?;
            if surface.cast::<GeomSphericalSurface>().is_some() {
                param_v += shape_analysis::adjust_to_period(param_v, -PI, PI);
            } else if surface.cast::<GeomToroidalSurface>().is_some() {
                param_v += shape_analysis::adjust_to_period(param_v, 0.0, PI * 2.0);
            }
        }
        return Ok((param_u, param_v));
    }
    let tabulated = start.cast::<IgesGeomTabulatedCylinder>();
    if let Some(live) = tabulated.get() {
        let arc = live.directrix().cast::<IgesGeomCircularArc>();
        if let Some(circle) = arc.get() {
            param_u -= arc_start_parameter(circle)?;
            param_u += shape_analysis::adjust_to_period(param_u, 0.0, PI * 2.0);
        }
    }
    Ok((param_u, param_v))
}

/// The swap and the mirror that a surface of revolution brings between IGES and the geometry.
fn swap_parameters(trans: &mut Transform2, translation_v: f64) -> Result<(), Failure> {
    let mut step = Transform2::default();
    step.set_translation(Vector2::new(0.0, translation_v));
    trans.pre_multiply(step);
    step.set_mirror_axis(Axis2d::new(
        Point2::new(0.0, 0.0),
        Direction2::new(1.0, 0.0)?,
    ));
    trans.pre_multiply(step);
    step.set_mirror_axis(Axis2d::new(
        Point2::new(0.0, 0.0),
        Direction2::new(1.0, 1.0)?,
    ));
    trans.pre_multiply(step);
    Ok(())
}

/// The scales and shift a kind of entity brings to the parameters.
struct ParameterScales {
    u_scale: f64,
    c_scale: f64,
    param_u: f64,
    param_v: f64,
}

fn scales_of_entity(
    entity: &Handle<IgesDataIgesEntity>,
    surface: &Handle<dyn GeomSurface>,
    trans: &mut Transform2,
    scales: &mut ParameterScales,
) -> Result<(), Failure> {
    let revolution = entity.cast::<IgesGeomSurfaceOfRevolution>();
    if let Some(live) = revolution.get() {
        swap_parameters(trans, -2.0 * PI)?;
        scales.u_scale = 1.0 / scales.c_scale;
        scales.param_u = -(2.0 * PI - live.end_angle());
    } else {
        scales.param_u = 0.0;
    }
    if entity.cast::<IgesGeomRuledSurface>().is_some() {
        scales.u_scale = 1.0 / scales.c_scale;
    }
    if entity.cast::<IgesGeomTabulatedCylinder>().is_some() {
        let bounds = iges_live(surface, WHERE)?.bounds();
        let length = (bounds.1 - bounds.0).abs();
        scales.u_scale = length / scales.c_scale;
        scales.param_u = bounds.0 / length;
    }
    if entity.cast::<IgesSolidCylindricalSurface>().is_some()
        || entity.cast::<IgesSolidConicalSurface>().is_some()
    {
        scales.u_scale = PI / 180.0;
    }
    if entity.cast::<IgesSolidSphericalSurface>().is_some() {
        scales.c_scale = PI / 180.0;
        scales.u_scale = 1.0;
    }
    if entity.cast::<IgesSolidToroidalSurface>().is_some() {
        swap_parameters(trans, -360.0)?;
        if scales.param_v > 0.0 {
            scales.param_v = scales.param_v * 180.0 / PI;
        }
        scales.c_scale = PI / 180.0;
        scales.u_scale = 1.0;
    }
    Ok(())
}

impl IgesToBRepTopoSurface {
    /// The face of a surface entity, with the transformation and the U factor that take its IGES
    /// parameter curves to the parameters of the face; with a shell of several faces, the shell;
    /// `let (shape, trans, u_fact) = tool.param_surface(&entity)?;`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn param_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<(Shape, Transform2, f64), Failure> {
        let mut trans = Transform2::default();
        let basis = self.transfer_topo_surface(start)?;
        let c_scale = self.u_length;
        if basis.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok((Shape::default(), trans, 0.0));
        }
        let kind = basis.shape_type()?;
        if kind != ShapeEnum::Face && kind != ShapeEnum::Shell {
            return Ok((Shape::default(), trans, 0.0));
        }
        let Some(face) = face_of_shape(&basis)? else {
            let message = self.label_message(start, b"basis surface")?;
            self.send_warning(start, &message);
            return Ok((basis, trans, 0.0));
        };
        let mut surface = BRepTool::surface(&face)?;
        let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
        if let Some(live) = trimmed.get() {
            surface = live.basis_surface();
        }
        let is_elementary = surface.cast::<GeomCylindricalSurface>().is_some()
            || surface.cast::<GeomConicalSurface>().is_some()
            || surface.cast::<GeomToroidalSurface>().is_some()
            || surface.cast::<GeomSphericalSurface>().is_some();
        let (mut param_u, mut param_v) = (0.0, 0.0);
        if is_elementary {
            let seam = seam_parameters(start, &surface, &face)?;
            (param_u, param_v) = phased_parameters(start, &surface, seam)?;
        }
        if param_u.abs() <= precision::confusion() {
            param_u = 0.0;
        }
        if param_v.abs() <= precision::confusion() {
            param_v = 0.0;
        }
        let offset = start.cast::<IgesGeomOffsetSurface>();
        let entity = offset
            .get()
            .map_or_else(|| start.clone(), IgesGeomOffsetSurface::surface);
        let mut scales = ParameterScales {
            u_scale: 1.0,
            c_scale,
            param_u,
            param_v,
        };
        scales_of_entity(&entity, &surface, &mut trans, &mut scales)?;
        let mut step = Transform2::default();
        step.set_translation_between(
            Point2::new(0.0, 0.0),
            Point2::new(scales.param_u, scales.param_v),
        );
        trans.pre_multiply(step);
        step.set_scale(Point2::new(0.0, 0.0), scales.c_scale);
        trans.pre_multiply(step);
        Ok((face.into_shape(), trans, scales.u_scale))
    }
}
