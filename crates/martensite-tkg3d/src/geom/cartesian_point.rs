//! `GeomCartesianPoint`: a point defined by its coordinates.
use super::geometry::GeomGeometry;
use super::point::GeomPoint;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::SharedState;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Point3, Transform3};

/// A Cartesian point; `Handle::new(GeomCartesianPoint::new(1.0, 2.0, 3.0))`.
#[derive(Default)]
pub struct GeomCartesianPoint {
    parent: StandardTransient,
    state: SharedState<Point3>,
}
impl_transient!(
    GeomCartesianPoint,
    "Geom_CartesianPoint",
    dyn GeomPoint,
    parent,
    interfaces: [dyn GeomGeometry, dyn GeomPoint]
);
impl Clone for GeomCartesianPoint {
    fn clone(&self) -> Self {
        Self::from_pnt(self.pnt())
    }
}
impl GeomCartesianPoint {
    /// Point from a gp point (`GeomCartesianPoint(Point3)`);
    /// `GeomCartesianPoint::from_pnt(Point3::new(1.0, 2.0, 3.0))`.
    pub fn from_pnt(point: Point3) -> Self {
        Self {
            parent: StandardTransient::default(),
            state: SharedState::new(point),
        }
    }
    /// Point from coordinates (`GeomCartesianPoint(X, Y, Z)`);
    /// `GeomCartesianPoint::new(1.0, 2.0, 3.0)`.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self::from_pnt(Point3::new(x, y, z))
    }
    /// Set the coordinates (`SetCoord`); `point.set_coord(1.0, 2.0, 3.0)`.
    pub fn set_coord(&self, x: f64, y: f64, z: f64) {
        self.state.write().set_coordinates(x, y, z);
    }
    /// Replace the point (`SetPnt`); `point.set_pnt(Point3::new(1.0, 2.0, 3.0))`.
    pub fn set_pnt(&self, point: Point3) {
        *self.state.write() = point;
    }
    /// Set the X coordinate (`SetX`); `point.set_x(1.0)`.
    pub fn set_x(&self, x: f64) {
        self.state.write().set_x(x);
    }
    /// Set the Y coordinate (`SetY`); `point.set_y(2.0)`.
    pub fn set_y(&self, y: f64) {
        self.state.write().set_y(y);
    }
    /// Set the Z coordinate (`SetZ`); `point.set_z(3.0)`.
    pub fn set_z(&self, z: f64) {
        self.state.write().set_z(z);
    }
}
impl GeomGeometry for GeomCartesianPoint {
    fn transform(&self, transform: Transform3) -> Result<(), Failure> {
        self.state.write().transform(transform);
        Ok(())
    }
    fn copy(&self) -> Result<Handle<dyn GeomGeometry>, Failure> {
        Ok(Handle::new(self.clone()).cast())
    }
}
impl GeomPoint for GeomCartesianPoint {
    fn coord(&self) -> (f64, f64, f64) {
        let point: Point3 = *self.state.read();
        (point.x(), point.y(), point.z())
    }
    fn pnt(&self) -> Point3 {
        *self.state.read()
    }
    fn x(&self) -> f64 {
        self.state.read().x()
    }
    fn y(&self) -> f64 {
        self.state.read().y()
    }
    fn z(&self) -> f64 {
        self.state.read().z()
    }
}
