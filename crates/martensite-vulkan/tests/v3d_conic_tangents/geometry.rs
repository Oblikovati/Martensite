use super::*;
use martensite_tkg3d::geom::{GeomCircle, GeomEllipse, GeomPlane};
use martensite_tkmath::gp::{Axis2, Axis3, Direction3, Point3};

pub(super) struct SpatialConics {
    pub circle: Handle<dyn GeomCurve>,
    pub ellipse: Handle<dyn GeomCurve>,
    pub plane: Handle<GeomPlane>,
}

fn source_conic_axis(origin: Point3) -> Result<Axis2, Failure> {
    Axis2::with_x_direction(
        origin,
        Direction3::new(0.173648, 0.984808, 0.)?,
        Direction3::new(-0.932169, 0.164367, -0.322560)?,
    )
}

pub(super) fn spatial_conics() -> Result<SpatialConics, Failure> {
    let first = source_conic_axis(Point3::new(823.687192, 502.366825, 478.960440))?;
    let circle = Handle::new(GeomCircle::new(first, 50.)?).cast();
    let second = source_conic_axis(Point3::new(1262.224429, 425.040878, 363.609716))?;
    let ellipse = Handle::new(GeomEllipse::new(second, 150., 100.)?).cast();
    let plane = Handle::new(GeomPlane::new(Axis3::from_axis2(second)));
    Ok(SpatialConics {
        circle,
        ellipse,
        plane,
    })
}

pub(super) fn qualified_outside(
    curve: &Handle<dyn Geom2dCurve>,
) -> Result<QualifiedCurve, Failure> {
    let adaptor = Geom2dAdaptorCurve::with_curve(curve)?;
    QualifiedCurve::new(&adaptor, Position::Outside)
}

pub(super) fn case_tangent_solver(
    ellipse: &Handle<dyn Geom2dCurve>,
    circle: &Handle<dyn Geom2dCurve>,
) -> Result<Lin2d2Tan, Failure> {
    Lin2d2Tan::with_parameters(
        &qualified_outside(ellipse)?,
        &qualified_outside(circle)?,
        0.1,
        0.,
        0.,
    )
}
