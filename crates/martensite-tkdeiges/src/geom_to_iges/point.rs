//! `GeomToIgesGeomPoint`: points into IGES points.
use super::GeomToIgesGeomEntity;
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_geom::IgesGeomPoint;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomPoint};
use martensite_tkmath::gp::Point3;
use std::ops::{Deref, DerefMut};

/// Translates points.
///
/// ```
/// use martensite_tkdeiges::geom_to_iges::GeomToIgesGeomPoint;
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkg3d::geom::GeomPoint;
/// let tool = GeomToIgesGeomPoint::new();
/// let none: Handle<dyn GeomPoint> = Handle::default();
/// assert!(tool.transfer_point(&none).is_ok());
/// ```
#[derive(Clone, Default)]
pub struct GeomToIgesGeomPoint {
    base: GeomToIgesGeomEntity,
}

impl Deref for GeomToIgesGeomPoint {
    type Target = GeomToIgesGeomEntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for GeomToIgesGeomPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GeomToIgesGeomPoint {
    /// A tool at the initial state; `GeomToIgesGeomPoint::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the model and the unit of `base`; `GeomToIgesGeomPoint::with_entity(&base)`.
    pub fn with_entity(base: &GeomToIgesGeomEntity) -> Self {
        Self { base: base.clone() }
    }
    /// The IGES point of a point; a null point gives an empty one; `tool.transfer_point(&point)?`.
    ///
    /// # Errors
    /// A failure of the coordinates of the point.
    pub fn transfer_point(
        &self,
        point: &Handle<dyn GeomPoint>,
    ) -> Result<Handle<IgesGeomPoint>, Failure> {
        match point.get() {
            None => Ok(Handle::new(IgesGeomPoint::new())),
            Some(live) => self.point_entity(live.pnt()),
        }
    }
    /// The same for a cartesian point; `tool.transfer_cartesian_point(&point)?`.
    ///
    /// # Errors
    /// A failure of the coordinates of the point.
    pub fn transfer_cartesian_point(
        &self,
        point: &Handle<GeomCartesianPoint>,
    ) -> Result<Handle<IgesGeomPoint>, Failure> {
        self.transfer_point(&point.cast())
    }
    fn point_entity(&self, point: Point3) -> Result<Handle<IgesGeomPoint>, Failure> {
        let entity = IgesGeomPoint::new();
        entity.init(
            self.in_unit(point.xyz()),
            &Handle::<IgesBasicSubfigureDef>::default(),
        );
        Ok(Handle::new(entity))
    }
}
