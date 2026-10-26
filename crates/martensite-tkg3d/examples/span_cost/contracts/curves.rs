//! Small exact rational segments and a periodic diamond exercise both coordinate domains.
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::{geom2d::Geom2dBSplineCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::{geom::GeomBSplineCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkmath::gp::{Point2, Point3};

pub(super) const DIAMOND: [[f64; 2]; 4] = [[1., 0.], [0., 1.], [-1., 0.], [0., -1.]];

fn array<T: Default + Clone>(values: &[T]) -> Array1<'static, T> {
    let mut output = Array1::with_bounds(1, values.len() as i32).unwrap();
    output.as_mut_slice().clone_from_slice(values);
    output
}

/// Two rational segments share a tangent at their middle knot; `rational_spatial()`.
pub(super) fn rational_spatial() -> GeomAdaptorCurve {
    let poles = array(&[
        Point3::new(0., 0., 0.),
        Point3::new(1., 2., -1.),
        Point3::new(2., 4., -2.),
    ]);
    let curve = GeomBSplineCurve::with_weights(
        &poles,
        &array(&[1., 2., 1.]),
        &array(&[0., 0.5, 1.]),
        &array(&[2, 1, 2]),
        1,
        false,
    )
    .unwrap();
    GeomAdaptorCurve::with_curve(&Handle::new(curve).cast()).unwrap()
}

/// Keep the same rational equations in the planar adaptor; `rational_planar()`.
pub(super) fn rational_planar() -> Geom2dAdaptorCurve {
    let poles = array(&[
        Point2::new(0., 0.),
        Point2::new(1., 2.),
        Point2::new(2., 4.),
    ]);
    let curve = Geom2dBSplineCurve::with_weights(
        &poles,
        &array(&[1., 2., 1.]),
        &array(&[0., 0.5, 1.]),
        &array(&[2, 1, 2]),
        1,
        false,
    )
    .unwrap();
    Geom2dAdaptorCurve::with_curve(&Handle::new(curve).cast()).unwrap()
}

/// A degree-one periodic polygon has an independent linear segment equation; `periodic_spatial()`.
pub(super) fn periodic_spatial() -> GeomAdaptorCurve {
    let poles = array(&DIAMOND.map(|[x, y]| Point3::new(x, y, 0.)));
    let curve = GeomBSplineCurve::new(
        &poles,
        &array(&[0., 1., 2., 3., 4.]),
        &array(&[1, 1, 1, 1, 1]),
        1,
        true,
    )
    .unwrap();
    GeomAdaptorCurve::with_curve(&Handle::new(curve).cast()).unwrap()
}

/// The planar polygon uses the identical period and control sequence; `periodic_planar()`.
pub(super) fn periodic_planar() -> Geom2dAdaptorCurve {
    let poles = array(&DIAMOND.map(|[x, y]| Point2::new(x, y)));
    let curve = Geom2dBSplineCurve::new(
        &poles,
        &array(&[0., 1., 2., 3., 4.]),
        &array(&[1, 1, 1, 1, 1]),
        1,
        true,
    )
    .unwrap();
    Geom2dAdaptorCurve::with_curve(&Handle::new(curve).cast()).unwrap()
}
