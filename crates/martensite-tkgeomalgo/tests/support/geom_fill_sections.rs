//! Section curves shared by the GeomFill skinning targets.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomEllipse};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

/// The circle and ellipse of the regression case `bug13904`, in that order;
/// `let [circle, ellipse] = circle_and_ellipse()?;`.
pub fn circle_and_ellipse() -> Result<[Handle<dyn GeomCurve>; 2], Failure> {
    let normal = Direction3::new(0.0, 1.0, 1.0)?;
    let circle = GeomCircle::new(Axis2::new(Point3::new(0.0, 100.0, 100.0), normal)?, 50.0)?;
    let ellipse = GeomEllipse::new(Axis2::new(Point3::new(0.0, 0.0, 0.0), normal)?, 100.0, 50.0)?;
    Ok([Handle::new(circle).cast(), Handle::new(ellipse).cast()])
}
