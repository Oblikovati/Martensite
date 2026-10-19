//! Authored Bernstein/quotient calculus, independent of geometry implementation outputs.
use martensite_tkernel::{
    ncollection::{Array1, Array2},
    standard::Handle,
};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    geom::{GeomBezierCurve, GeomBezierSurface, GeomCurve, GeomPlane, GeomSurface, GeomSurfaceD3},
    geom_adaptor::GeomAdaptorTransformedSurface,
};
use martensite_tkmath::gp::{Plane3, Point3, Transform3, Vector3};

fn quadratic_fixture() -> GeomBezierCurve {
    let mut poles = Array1::with_bounds(1, 3).unwrap();
    for (index, coordinates) in [[0., 0., 0.], [1., 2., 0.], [2., 0., 2.]]
        .into_iter()
        .enumerate()
    {
        poles[index as i32 + 1] = Point3::new(coordinates[0], coordinates[1], coordinates[2]);
    }
    let mut weights = Array1::with_bounds(1, 3).unwrap();
    weights.as_mut_slice().copy_from_slice(&[1., 2., 1.]);
    GeomBezierCurve::with_weights(&poles, &weights).unwrap()
}

fn rational_quadratic_expected(t: f64) -> [[f64; 3]; 4] {
    let denominator = 1. + 2. * t - 2. * t * t;
    let denominator_derivative = 2. - 4. * t;
    let numerator = [4. * t - 2. * t * t, 8. * t - 8. * t * t, 2. * t * t];
    let first = [4. - 4. * t, 8. - 16. * t, 4. * t];
    let second = [-4., -16., 4.];
    let mut expected = [[0.; 3]; 4];
    for axis in 0..3 {
        expected[0][axis] = numerator[axis] / denominator;
        expected[1][axis] =
            (first[axis] - denominator_derivative * expected[0][axis]) / denominator;
        expected[2][axis] = (second[axis] - 2. * denominator_derivative * expected[1][axis]
            + 4. * expected[0][axis])
            / denominator;
        expected[3][axis] = (-3. * denominator_derivative * expected[2][axis]
            + 12. * expected[1][axis])
            / denominator;
    }
    expected
}

fn assert_rounding_budget(actual: [f64; 3], expected: [f64; 3]) {
    for axis in 0..3 {
        let limit = 64. * f64::EPSILON * expected[axis].abs().max(1.);
        assert!(
            (actual[axis] - expected[axis]).abs() <= limit,
            "axis={axis}, actual={}, expected={} within {limit}",
            actual[axis],
            expected[axis]
        );
    }
}

#[test]
fn rational_quadratic_matches_authored_value_and_three_derivatives() {
    let curve = quadratic_fixture();
    for index in 0..=16 {
        let t = f64::from(index) / 16.;
        let (point, first, second, third) = curve.d3(t).unwrap();
        let actual = [
            [point.x(), point.y(), point.z()],
            xyz(first),
            xyz(second),
            xyz(third),
        ];
        for (observed, expected) in actual.into_iter().zip(rational_quadratic_expected(t)) {
            assert_rounding_budget(observed, expected);
        }
    }
}

fn xyz(vector: Vector3) -> [f64; 3] {
    [vector.x(), vector.y(), vector.z()]
}

fn bilinear_fixture() -> Handle<GeomBezierSurface> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    poles.set_value(1, 1, Point3::new(0., 0., 0.)).unwrap();
    poles.set_value(2, 1, Point3::new(2., 0., 0.)).unwrap();
    poles.set_value(1, 2, Point3::new(0., 3., 0.)).unwrap();
    poles.set_value(2, 2, Point3::new(2., 3., 4.)).unwrap();
    Handle::new(GeomBezierSurface::new(&poles).unwrap())
}

fn transformed_bilinear() -> GeomAdaptorTransformedSurface {
    let surface: Handle<dyn GeomSurface> = bilinear_fixture().cast();
    let shift = Transform3::translation([4., -2., 8.]);
    GeomAdaptorTransformedSurface::with_surface(&surface, shift).unwrap()
}

fn assert_bilinear_calculus(sample: GeomSurfaceD3, u: f64, v: f64) {
    let point = sample.point;
    assert_eq!(
        [point.x(), point.y(), point.z()],
        [4. + 2. * u, -2. + 3. * v, 8. + 4. * u * v]
    );
    assert_eq!(xyz(sample.d1u), [2., 0., 4. * v]);
    assert_eq!(xyz(sample.d1v), [0., 3., 4. * u]);
    assert_eq!(xyz(sample.d2uv), [0., 0., 4.]);
    for vector in [
        sample.d2u,
        sample.d2v,
        sample.d3u,
        sample.d3v,
        sample.d3uuv,
        sample.d3uvv,
    ] {
        assert_eq!(xyz(vector), [0.; 3]);
    }
}

fn check_bilinear_grid(adaptor: &GeomAdaptorTransformedSurface) {
    for row in 0..=8 {
        for column in 0..=8 {
            let (u, v) = (f64::from(row) / 8., f64::from(column) / 8.);
            assert_bilinear_calculus(adaptor.eval_d3(u, v).unwrap(), u, v);
        }
    }
}

#[test]
fn transformed_bilinear_cache_has_exact_dyadic_polynomial_calculus() {
    let adaptor = transformed_bilinear();
    adaptor.init_transformed_cache().unwrap();
    check_bilinear_grid(&adaptor);
    adaptor.invalidate_transformed_cache();
    check_bilinear_grid(&adaptor);
}

fn replace_both_adaptor_members(source: &GeomAdaptorTransformedSurface) {
    let replacement: Handle<dyn GeomSurface> =
        Handle::new(GeomPlane::from_pln(Plane3::default())).cast();
    let transformed = source.adaptor_surface_transformed().unwrap();
    transformed.load(&replacement).unwrap();
    source
        .adaptor_surface_original()
        .load(&replacement)
        .unwrap();
}

#[test]
fn copied_surface_cache_keeps_authored_geometry_after_original_member_replacement() {
    let source = transformed_bilinear();
    source.init_transformed_cache().unwrap();
    let copied: Handle<GeomAdaptorTransformedSurface> = source.shallow_copy().unwrap().cast();
    replace_both_adaptor_members(&source);
    assert_ne!(
        source.adaptor_surface_original(),
        copied.adaptor_surface_original()
    );
    let original_cache = source.adaptor_surface_transformed().unwrap();
    let copied_cache = copied.adaptor_surface_transformed().unwrap();
    assert_ne!(original_cache, copied_cache);
    check_bilinear_grid(&copied);
}
