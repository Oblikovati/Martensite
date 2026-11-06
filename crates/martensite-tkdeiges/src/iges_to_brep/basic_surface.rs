//! `IgesToBRepBasicSurface`: the surface entities with a direct geometric form. This file holds the
//! choice among the kinds and the elementary surfaces.
use super::curve_and_surface::curve_and_surface_tool;
use crate::iges_conv_geom;
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_geom::{
    IgesGeomBSplineSurface, IgesGeomDirection, IgesGeomPoint, IgesGeomSplineSurface,
};
use crate::iges_solid::{
    IgesSolidConicalSurface, IgesSolidCylindricalSurface, IgesSolidPlaneSurface,
    IgesSolidSphericalSurface, IgesSolidToroidalSurface,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
    GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3};
use std::f64::consts::PI;

curve_and_surface_tool!(
    /// Translates elementary and spline surfaces.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_to_brep::IgesToBRepBasicSurface;
    /// assert!(!IgesToBRepBasicSurface::new().get_mode_transfer());
    /// ```
    IgesToBRepBasicSurface,
    |base| base.set_mode_transfer(false)
);

/// What an elementary surface entity gives to place it.
struct ElementaryPlacement {
    parametrised: bool,
    location: Handle<IgesGeomPoint>,
    axis: Handle<IgesGeomDirection>,
    reference: Handle<IgesGeomDirection>,
}

impl ElementaryPlacement {
    /// The frame at the location; with a reference direction parallel to the axis there is none.
    fn frame(&self) -> Result<Option<Axis3>, Failure> {
        let point = location_of(&self.location)?;
        let axis = direction_of(&self.axis)?;
        if !self.parametrised {
            return Axis3::from_normal(point, axis).map(Some);
        }
        let reference = direction_of(&self.reference)?;
        if reference.crossed(axis)?.xyz().modulus() < precision::confusion() {
            return Ok(None);
        }
        Axis3::new(point, axis, reference).map(Some)
    }
}

fn location_of(point: &Handle<IgesGeomPoint>) -> Result<Point3, Failure> {
    Ok(crate::iges_data::iges_live(point, "IgesToBRepBasicSurface")?.value())
}

fn direction_of(direction: &Handle<IgesGeomDirection>) -> Result<Direction3, Failure> {
    Direction3::from_vector(
        crate::iges_data::iges_live(direction, "IgesToBRepBasicSurface")?.value(),
    )
}

impl IgesToBRepBasicSurface {
    /// The fails of a location or an axis that are not there.
    fn placement_is_complete<T: ?Sized + RuntimeTyped>(
        &self,
        start: &Handle<T>,
        placement: &ElementaryPlacement,
    ) -> bool {
        if placement.location.is_null() {
            self.fail_key(start, "XSTEP_174");
            return false;
        }
        if placement.axis.is_null() {
            self.fail_key(start, "IGES_1280");
            return false;
        }
        true
    }
    /// The surface of any basic surface entity, scaled to the file unit; null on a failure, which has
    /// its message or is the exception a transfer takes as no result;
    /// `tool.transfer_basic_surface(&entity)?`.
    ///
    /// # Errors
    /// A failure of the scaling of the surface.
    pub fn transfer_basic_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        }
        // A B-spline surface raises on weights that are not positive.
        let result = self.dispatch_basic_surface(start).unwrap_or_default();
        if let Some(live) = result.get() {
            live.scale(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor())?;
        }
        Ok(result)
    }
    fn dispatch_basic_surface(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        let bspline = start.cast::<IgesGeomBSplineSurface>();
        if bspline.is_some() {
            return Ok(self.transfer_bspline_surface(&bspline)?.cast());
        }
        let spline = start.cast::<IgesGeomSplineSurface>();
        if spline.is_some() {
            return Ok(self.transfer_spline_surface(&spline)?.cast());
        }
        let plane = start.cast::<IgesSolidPlaneSurface>();
        if plane.is_some() {
            return Ok(self.transfer_plane_surface(&plane)?.cast());
        }
        let cylinder = start.cast::<IgesSolidCylindricalSurface>();
        if cylinder.is_some() {
            return Ok(self.transfer_rigth_cylindrical_surface(&cylinder)?.cast());
        }
        let cone = start.cast::<IgesSolidConicalSurface>();
        if cone.is_some() {
            return Ok(self.transfer_rigth_conical_surface(&cone)?.cast());
        }
        let sphere = start.cast::<IgesSolidSphericalSurface>();
        if sphere.is_some() {
            return Ok(self.transfer_spherical_surface(&sphere)?.cast());
        }
        let torus = start.cast::<IgesSolidToroidalSurface>();
        if torus.is_some() {
            return Ok(self.transfer_toroidal_surface(&torus)?.cast());
        }
        Ok(Handle::default())
    }
    /// The plane of the entity; `tool.transfer_plane_surface(&plane)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the frame.
    pub fn transfer_plane_surface(
        &self,
        start: &Handle<IgesSolidPlaneSurface>,
    ) -> Result<Handle<GeomPlane>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let placement = ElementaryPlacement {
            parametrised: live.is_parametrised(),
            location: live.location_point(),
            axis: live.normal(),
            reference: live.reference_dir(),
        };
        if !self.placement_is_complete(start, &placement) {
            return Ok(Handle::default());
        }
        let point = location_of(&placement.location)?;
        let normal = direction_of(&placement.axis)?;
        let frame = if placement.parametrised {
            Axis3::new(point, normal, direction_of(&placement.reference)?)?
        } else {
            Axis3::from_normal(point, normal)?
        };
        Ok(Handle::new(GeomPlane::from_pln(Plane3::from_position(
            frame,
        ))))
    }
    /// The cylinder of the entity, null for a null radius; `tool.transfer_rigth_cylindrical_surface(&cylinder)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the frame.
    pub fn transfer_rigth_cylindrical_surface(
        &self,
        start: &Handle<IgesSolidCylindricalSurface>,
    ) -> Result<Handle<GeomCylindricalSurface>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let placement = ElementaryPlacement {
            parametrised: live.is_parametrised(),
            location: live.location_point(),
            axis: live.axis(),
            reference: live.reference_dir(),
        };
        let radius = live.radius();
        if !self.placement_is_complete(start, &placement) || radius < precision::confusion() {
            return Ok(Handle::default());
        }
        let Some(frame) = placement.frame()? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomCylindricalSurface::new(frame, radius)?))
    }
    /// The cone of the entity, null for an angle out of 0 to 90 degrees or a negative radius;
    /// `tool.transfer_rigth_conical_surface(&cone)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the frame.
    pub fn transfer_rigth_conical_surface(
        &self,
        start: &Handle<IgesSolidConicalSurface>,
    ) -> Result<Handle<GeomConicalSurface>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let placement = ElementaryPlacement {
            parametrised: live.is_parametrised(),
            location: live.location_point(),
            axis: live.axis(),
            reference: live.reference_dir(),
        };
        if !self.placement_is_complete(start, &placement) {
            return Ok(Handle::default());
        }
        let angle = live.semi_angle() / 180.0 * PI;
        let radius = live.radius();
        if angle < precision::confusion() || angle > PI / 2.0 || radius < 0.0 {
            return Ok(Handle::default());
        }
        let radius = if radius < precision::confusion() {
            0.0
        } else {
            radius
        };
        let Some(frame) = placement.frame()? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomConicalSurface::new(frame, angle, radius)?))
    }
    /// The sphere of the entity, null for a null radius; `tool.transfer_spherical_surface(&sphere)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the frame.
    pub fn transfer_spherical_surface(
        &self,
        start: &Handle<IgesSolidSphericalSurface>,
    ) -> Result<Handle<GeomSphericalSurface>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let placement = ElementaryPlacement {
            parametrised: live.is_parametrised(),
            location: live.center(),
            axis: live.axis(),
            reference: live.reference_dir(),
        };
        let radius = live.radius();
        if !self.placement_is_complete(start, &placement) || radius < precision::confusion() {
            return Ok(Handle::default());
        }
        let Some(frame) = placement.frame()? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomSphericalSurface::new(frame, radius)?))
    }
    /// The torus of the entity, null for a null radius; `tool.transfer_toroidal_surface(&torus)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the frame.
    pub fn transfer_toroidal_surface(
        &self,
        start: &Handle<IgesSolidToroidalSurface>,
    ) -> Result<Handle<GeomToroidalSurface>, Failure> {
        let Some(live) = start.get() else {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        };
        let placement = ElementaryPlacement {
            parametrised: live.is_parametrised(),
            location: live.center(),
            axis: live.axis(),
            reference: live.reference_dir(),
        };
        let (major, minor) = (live.major_radius(), live.minor_radius());
        let tolerance = precision::confusion();
        if !self.placement_is_complete(start, &placement) || major < tolerance || minor < tolerance
        {
            return Ok(Handle::default());
        }
        let Some(frame) = placement.frame()? else {
            return Ok(Handle::default());
        };
        Ok(Handle::new(GeomToroidalSurface::new(frame, major, minor)?))
    }
    /// The B-spline of a spline surface entity with its continuity raised; a fail without a surface;
    /// `tool.transfer_spline_surface(&surface)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the surface.
    pub fn transfer_spline_surface(
        &self,
        start: &Handle<IgesGeomSplineSurface>,
    ) -> Result<Handle<GeomBSplineSurface>, Failure> {
        if start.is_null() {
            self.fail_key(start, "IGES_1005");
            return Ok(Handle::default());
        }
        let eps_geom = self.get_eps_geom();
        let (status, surface) =
            iges_conv_geom::spline_surface_from_iges(start, self.get_eps_coeff(), eps_geom)?;
        if status == 5 {
            self.fail_key(start, "IGES_1305");
            return Ok(surface);
        }
        if status == 4 {
            self.fail_key(start, "IGES_1190");
            return Ok(surface);
        }
        if let Some(live) = surface.get() {
            iges_conv_geom::increase_surface_continuity(live, eps_geom, self.get_continuity())?;
        }
        Ok(surface)
    }
}
