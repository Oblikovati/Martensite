//! Planes and analytic surfaces into the solid surface entities of the BRep mode (types 190-198).
use super::curve::EntityResult;
use super::{GeomToIgesGeomPoint, GeomToIgesGeomSurface};
use crate::iges_geom::{IgesGeomDirection, IgesGeomPoint};
use crate::iges_solid::{
    IgesSolidConicalSurface, IgesSolidCylindricalSurface, IgesSolidPlaneSurface,
    IgesSolidSphericalSurface, IgesSolidToroidalSurface,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomCartesianPoint, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
    GeomSphericalSurface, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Axis1, Point3, Xyz};
use std::f64::consts::PI;

fn direction_entity(direction: Xyz) -> Handle<IgesGeomDirection> {
    let entity = IgesGeomDirection::new();
    entity.init(direction);
    Handle::new(entity)
}

fn axis_direction(axis: Axis1) -> Handle<IgesGeomDirection> {
    direction_entity(axis.direction().xyz())
}

impl GeomToIgesGeomSurface {
    fn location_entity(&self, location: Point3) -> Result<Handle<IgesGeomPoint>, Failure> {
        let point = Handle::new(GeomCartesianPoint::from_pnt(location));
        GeomToIgesGeomPoint::with_entity(&self.base).transfer_cartesian_point(&point)
    }
    /// The plane surface (type 190); `tool.transfer_plane_surface(&plane)?`.
    ///
    /// # Errors
    /// A failure of the location point.
    pub fn transfer_plane_surface(&mut self, start: &Handle<GeomPlane>) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let plane = live.pln();
        let entity = IgesSolidPlaneSurface::new();
        entity.init(
            &self.location_entity(plane.location())?,
            &axis_direction(plane.axis()),
            &axis_direction(plane.x_axis()),
        );
        Ok(Handle::new(entity).cast())
    }
    /// The cylindrical surface (type 192); `tool.transfer_cylindrical_surface(&cylinder)?`.
    ///
    /// # Errors
    /// A failure of the location point or of the surface.
    pub fn transfer_cylindrical_surface(
        &mut self,
        start: &Handle<GeomCylindricalSurface>,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let cylinder = live.cylinder()?;
        let entity = IgesSolidCylindricalSurface::new();
        entity.init(
            &self.location_entity(cylinder.location())?,
            &axis_direction(cylinder.axis()),
            cylinder.radius() / self.get_unit(),
            &axis_direction(cylinder.x_axis()),
        );
        Ok(Handle::new(entity).cast())
    }
    /// The conical surface (type 194); a negative angle moves the location across the apex,
    /// `tool.transfer_conical_surface(&cone)?`.
    ///
    /// # Errors
    /// A failure of the location point or of the surface.
    pub fn transfer_conical_surface(&mut self, start: &Handle<GeomConicalSurface>) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let cone = live.cone()?;
        let mut angle = cone.semi_angle();
        let mut location = cone.location();
        let mut x_direction = cone.x_axis().direction();
        if angle < 0.0 {
            location = Point3::from(cone.apex().xyz() * 2.0 - location.xyz());
            angle = -angle;
            x_direction = x_direction.reversed();
        }
        let entity = IgesSolidConicalSurface::new();
        entity.init(
            &self.location_entity(location)?,
            &axis_direction(cone.axis()),
            cone.ref_radius() / self.get_unit(),
            angle * 180.0 / PI,
            &direction_entity(x_direction.xyz()),
        );
        Ok(Handle::new(entity).cast())
    }
    /// The spherical surface (type 196); `tool.transfer_spherical_surface(&sphere)?`.
    ///
    /// # Errors
    /// A failure of the location point or of the surface.
    pub fn transfer_spherical_surface(
        &mut self,
        start: &Handle<GeomSphericalSurface>,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let sphere = live.sphere()?;
        let entity = IgesSolidSphericalSurface::new();
        entity.init(
            &self.location_entity(sphere.location())?,
            sphere.radius() / self.get_unit(),
            &direction_entity(sphere.position().direction().xyz()),
            &axis_direction(sphere.x_axis()),
        );
        Ok(Handle::new(entity).cast())
    }
    /// The toroidal surface (type 198); `tool.transfer_toroidal_surface(&torus)?`.
    ///
    /// # Errors
    /// A failure of the location point or of the surface.
    pub fn transfer_toroidal_surface(
        &mut self,
        start: &Handle<GeomToroidalSurface>,
    ) -> EntityResult {
        self.length = 1.0;
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let torus = live.torus()?;
        let unit = self.get_unit();
        let entity = IgesSolidToroidalSurface::new();
        entity.init(
            &self.location_entity(torus.location())?,
            &axis_direction(torus.axis()),
            torus.major_radius() / unit,
            torus.minor_radius() / unit,
            &axis_direction(torus.x_axis()),
        );
        Ok(Handle::new(entity).cast())
    }
}
