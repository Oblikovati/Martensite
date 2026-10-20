//! Shared analytic and generic surface fixtures with lazy parameter bounds.
use martensite_tkernel::{
    ncollection::Array2,
    standard::{Failure, Handle},
};
use martensite_tkg3d::{
    adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle,
    geom::{
        GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
        GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
    },
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::gp::{Axis3, Point3};

fn warped_pole(i: i32, j: i32, height: f64) -> Point3 {
    Point3::new(
        f64::from(i - 1),
        f64::from(j - 1),
        f64::from((i - 1) * (j - 1)) * height,
    )
}

fn warped_surface(height: f64) -> Result<Handle<dyn GeomSurface>, Failure> {
    let mut poles = Array2::with_bounds(1, 3, 1, 3).unwrap();
    for i in 1..=3 {
        for j in 1..=3 {
            poles.set_value(i, j, warped_pole(i, j, height)).unwrap();
        }
    }
    Ok(Handle::new(GeomBezierSurface::new(&poles)?).cast())
}

fn make_surface(
    kind: i32,
    frame: Axis3,
    radius: f64,
    semi: f64,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    Ok(match kind {
        0 => Handle::new(GeomPlane::new(frame)).cast(),
        1 => Handle::new(GeomSphericalSurface::new(frame, radius)?).cast(),
        2 => Handle::new(GeomCylindricalSurface::new(frame, radius)?).cast(),
        3 => Handle::new(GeomConicalSurface::new(frame, semi, radius)?).cast(),
        4 => Handle::new(GeomToroidalSurface::new(frame, radius, 0.5)?).cast(),
        _ => return warped_surface(radius),
    })
}

/// Construct geometry before consuming bounds; `from_fields(0, frame, 0., 0., || (0., 1., 0., 1.))`.
pub fn from_fields(
    kind: i32,
    frame: Axis3,
    radius: f64,
    semi: f64,
    bounds: impl FnOnce() -> (f64, f64, f64, f64),
) -> Result<Adaptor3dSurfaceHandle, Failure> {
    let surface = make_surface(kind, frame, radius, semi)?;
    Ok(Handle::new(GeomAdaptorSurface::with_bounds(&surface, bounds())?).cast())
}

#[cfg(test)]
mod tests {
    use super::from_fields;
    use martensite_tkg3d::adaptor3d::h_surface_tool as surface_tool;
    use martensite_tkmath::gp::{Axis3, Point3};

    #[test]
    fn rejected_geometry_does_not_consume_parameter_bounds() {
        let mut reads = 0;
        let result = from_fields(1, Axis3::default(), -1., 0., || {
            reads += 1;
            (0., 1., 0., 1.)
        });
        assert!(result.is_err());
        assert_eq!(reads, 0);
    }

    #[test]
    fn valid_geometry_consumes_bounds_once_and_retains_each_limit() {
        let mut reads = 0;
        let surface = from_fields(0, Axis3::default(), 0., 0., || {
            reads += 1;
            (-1., 2., -3., 4.)
        })
        .unwrap();
        assert_eq!(reads, 1);
        assert_eq!(surface_tool::first_u_parameter(&surface).unwrap(), -1.);
        assert_eq!(surface_tool::last_u_parameter(&surface).unwrap(), 2.);
        assert_eq!(surface_tool::first_v_parameter(&surface).unwrap(), -3.);
        assert_eq!(surface_tool::last_v_parameter(&surface).unwrap(), 4.);
    }

    #[test]
    fn tensor_patch_matches_its_bilinear_closed_form() {
        let surface = from_fields(5, Axis3::default(), 2., 0., || (0., 1., 0., 1.)).unwrap();
        assert_eq!(
            surface_tool::value(&surface, 0.25, 0.75).unwrap(),
            Point3::new(0.5, 1.5, 1.5)
        );
    }
}
