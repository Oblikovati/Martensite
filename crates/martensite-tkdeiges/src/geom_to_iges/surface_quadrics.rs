//! Planes and quadrics into a B-spline or a plane entity, and into surfaces of revolution.
use super::curve::EntityResult;
use super::surface::SurfaceRange;
use super::{GeomToIgesGeomCurve, GeomToIgesGeomSurface, bounded_range};
use crate::iges_conv_geom::IgesConvGeomGeomBuilder;
use crate::iges_data::{
    IgesDataIgesEntity, iges_array1_set, iges_array2_set, iges_live, iges_new_array1,
    iges_new_array2,
};
use crate::iges_geom::{
    IgesGeomBSplineSurface, IgesGeomCircularArc, IgesGeomLine, IgesGeomPlane,
    IgesGeomSurfaceOfRevolution,
};
use martensite_cmath::{cos, sin};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomCircle, GeomConicalSurface, GeomCurve, GeomCylindricalSurface, GeomElementarySurface,
    GeomLine, GeomPlane, GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3, StandardDirection3, Xyz};
use martensite_tkxsbase::interface::InterfaceStatic;
use std::f64::consts::PI;

const WHERE: &str = "GeomToIgesGeomSurface";

/// The axis of a surface of revolution: an IGES axis runs against the direction of the geometry.
fn reversed_z_axis() -> Handle<IgesGeomLine> {
    let axis = IgesGeomLine::new();
    axis.init(Xyz::new(0.0, 0.0, 1.0), Xyz::new(0.0, 0.0, 0.0));
    Handle::new(axis)
}

/// A generatrix frame of a meridian circle: normal along -Y, X along X.
fn meridian_frame(center: Point3) -> Result<Axis2, Failure> {
    let y_axis = Direction3::from(StandardDirection3::Y);
    Axis2::with_x_direction(
        center,
        y_axis.reversed(),
        Direction3::from(StandardDirection3::X),
    )
}

impl GeomToIgesGeomSurface {
    /// A B-spline for the plane, or the plane entity (type 108) in the plane mode 0;
    /// `tool.transfer_plane(&plane, (0.0, 1.0, 0.0, 1.0))?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the plane.
    pub fn transfer_plane(
        &mut self,
        start: &Handle<GeomPlane>,
        range: SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        if InterfaceStatic::i_val("write.iges.plane.mode") == 0 {
            return self.plane_entity(live);
        }
        self.plane_bspline(live, range)
    }
    fn plane_entity(&self, plane: &GeomPlane) -> EntityResult {
        let [a, b, c, d] = plane.coefficients()?;
        let attach = self.in_unit(plane.location().xyz());
        let entity = IgesGeomPlane::new();
        // A negated constant term, against the plane of the geometry.
        entity.init(
            a,
            b,
            c,
            -d / self.get_unit(),
            &Handle::default(),
            attach,
            0.0,
        );
        Ok(Handle::new(entity).cast())
    }
    fn plane_bspline(&self, plane: &GeomPlane, (u1, u2, v1, v2): SurfaceRange) -> EntityResult {
        let poles: Handle<HArray2<Xyz>> = iges_new_array2(0, 1, 0, 1)?;
        let corners = [
            (0, 0, u1, v1),
            (0, 1, u1, v2),
            (1, 0, u2, v1),
            (1, 1, u2, v2),
        ];
        for (row, column, u, v) in corners {
            let corner = self.in_unit(plane.d0(u, v)?.xyz());
            iges_array2_set(&poles, row, column, corner, WHERE)?;
        }
        let weights: Handle<HArray2<f64>> = iges_new_array2(0, 1, 0, 1)?;
        for (row, column, _, _) in corners {
            iges_array2_set(&weights, row, column, 1.0, WHERE)?;
        }
        let entity = IgesGeomBSplineSurface::new();
        entity.init(
            1,
            1,
            1,
            1,
            false,
            false,
            true,
            false,
            false,
            &linear_knots(u1, u2)?,
            &linear_knots(v1, v2)?,
            &weights,
            &poles,
            u1,
            u2,
            v1,
            v2,
        )?;
        Ok(Handle::new(entity).cast())
    }
    /// A surface of revolution with its own position, between reversed U parameters.
    fn revolution_entity(
        &self,
        generatrix: &Handle<IgesDataIgesEntity>,
        (first_u, last_u): (f64, f64),
        position: Axis3,
    ) -> EntityResult {
        let surface = IgesGeomSurfaceOfRevolution::new();
        surface.init(
            &reversed_z_axis(),
            generatrix,
            2.0 * PI - last_u,
            2.0 * PI - first_u,
        );
        let mut builder = IgesConvGeomGeomBuilder::new();
        builder.set_position_axis3(position);
        if !builder.is_identity() {
            let matrix = builder.make_transformation(self.get_unit())?;
            surface.init_transf(&matrix.cast());
        }
        Ok(Handle::new(surface).cast())
    }
    fn generatrix_tool(&self) -> GeomToIgesGeomCurve {
        GeomToIgesGeomCurve::with_entity(&self.base)
    }
    /// A ruling of a surface: its entity, and the length between both parameters.
    fn ruling(
        &mut self,
        line: GeomLine,
        (lower, upper): (f64, f64),
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let handle = Handle::new(line);
        let generatrix = self
            .generatrix_tool()
            .transfer_line(&handle, lower, upper)?;
        let live = iges_live(&handle, WHERE)?;
        self.length = live.value(lower)?.distance(&live.value(upper)?);
        Ok(generatrix)
    }
    /// A surface of revolution with the line in the XZ plane as generatrix;
    /// `tool.transfer_cylinder(&cylinder, range)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the generatrix.
    pub fn transfer_cylinder(
        &mut self,
        start: &Handle<GeomCylindricalSurface>,
        (u1, u2, v1, v2): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let cylinder = live.cylinder()?;
        let origin = Point3::new(cylinder.radius(), 0.0, 0.0);
        let line = GeomLine::from_point(origin, Direction3::from(StandardDirection3::Z));
        let generatrix = self.ruling(line, bounded_range(v1, v2))?;
        self.revolution_entity(&generatrix, (u1, u2), cylinder.position())
    }
    /// A surface of revolution with the slanted line as generatrix;
    /// `tool.transfer_cone(&cone, range)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the generatrix.
    pub fn transfer_cone(
        &mut self,
        start: &Handle<GeomConicalSurface>,
        (u1, u2, v1, v2): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let cone = live.cone()?;
        let angle = cone.semi_angle();
        let origin = Point3::new(cone.ref_radius(), 0.0, 0.0);
        let line = GeomLine::from_point(origin, Direction3::new(sin(angle), 0.0, cos(angle))?);
        let generatrix = self.ruling(line, bounded_range(v1, v2))?;
        self.revolution_entity(&generatrix, (u1, u2), cone.position())
    }
    /// A surface of revolution with a half circle as generatrix;
    /// `tool.transfer_sphere(&sphere, range)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the generatrix.
    pub fn transfer_sphere(
        &mut self,
        start: &Handle<GeomSphericalSurface>,
        (u1, u2, v1, v2): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let sphere = live.sphere()?;
        let frame = meridian_frame(Point3::new(0.0, 0.0, 0.0))?;
        let circle: Handle<dyn GeomCurve> =
            Handle::new(GeomCircle::new(frame, sphere.radius())?).cast();
        let generatrix = self.generatrix_tool().transfer_curve(&circle, v1, v2)?;
        if generatrix.cast::<IgesGeomCircularArc>().is_null() {
            return Ok(Handle::new(IgesGeomSurfaceOfRevolution::new()).cast());
        }
        self.revolution_entity(&generatrix, (u1, u2), sphere.position())
    }
    /// A surface of revolution with a circle as generatrix;
    /// `tool.transfer_torus(&torus, range)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the generatrix.
    pub fn transfer_torus(
        &mut self,
        start: &Handle<GeomToroidalSurface>,
        (u1, u2, v1, v2): SurfaceRange,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let torus = live.torus()?;
        let frame = meridian_frame(Point3::new(torus.major_radius(), 0.0, 0.0))?;
        let circle: Handle<dyn GeomCurve> =
            Handle::new(GeomCircle::new(frame, torus.minor_radius())?).cast();
        let generatrix = self.generatrix_tool().transfer_curve(&circle, v1, v2)?;
        self.revolution_entity(&generatrix, (u1, u2), torus.position())
    }
}

/// The knots of one linear span.
fn linear_knots(first: f64, last: f64) -> Result<Handle<HArray1<f64>>, Failure> {
    let knots = iges_new_array1(-1, 2)?;
    for (rank, knot) in [(-1, first), (0, first), (1, last), (2, last)] {
        iges_array1_set(&knots, rank, knot, WHERE)?;
    }
    Ok(knots)
}
