//! `GeomPoint`: points of space as transient geometry.
use super::geometry::GeomGeometry;
use martensite_tkernel::impl_transient_interface;
use martensite_tkmath::gp::Point3;

/// A point with coordinates; `point.distance(&*other)`.
pub trait GeomPoint: GeomGeometry {
    /// Coordinates as `(x, y, z)` (`Coord`); `point.coord()`.
    fn coord(&self) -> (f64, f64, f64);
    /// A gp copy (`Pnt`); `point.pnt()`.
    fn pnt(&self) -> Point3;
    /// X coordinate (`X`); `point.x()`.
    fn x(&self) -> f64;
    /// Y coordinate (`Y`); `point.y()`.
    fn y(&self) -> f64;
    /// Z coordinate (`Z`); `point.z()`.
    fn z(&self) -> f64;
    /// Distance to `other` (`Distance`); `point.distance(&*other)`.
    fn distance(&self, other: &dyn GeomPoint) -> f64 {
        let first: Point3 = self.pnt();
        let second: Point3 = other.pnt();
        first.distance(&second)
    }
    /// Squared distance to `other` (`SquareDistance`); `point.square_distance(&*other)`.
    fn square_distance(&self, other: &dyn GeomPoint) -> f64 {
        let first: Point3 = self.pnt();
        let second: Point3 = other.pnt();
        first.square_distance(&second)
    }
}
impl_transient_interface!(dyn GeomPoint, "Geom_Point", dyn GeomGeometry);
