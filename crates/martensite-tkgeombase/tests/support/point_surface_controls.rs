//! Plane and cylinder identities supplement captured baselines with independent expected values.
use super::super::construction_corpus::{
    extrema_pc::{Space, Space3},
    extrema_ps::surface,
    print::Reader,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dCurve,
    geom_adaptor::{
        GeomAdaptorSurface, GeomAdaptorSurfaceOfLinearExtrusion, GeomAdaptorSurfaceOfRevolution,
    },
};
use martensite_tkgeombase::extrema::{
    ExtAlgo, ExtFlag, ExtPExtS, ExtPRevS, ExtPS, FuncPSDist, FuncPSNorm, GenExtPS, GenLocateExtPS,
    Sampling,
};
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3},
    math::{Matrix, Vector},
};
use std::f64::consts::TAU;

fn plane() -> GeomAdaptorSurface {
    let fields = [
        0., 0., 0., 0., 0., 0., 0., 1., 1., 0., 0., 1., -6., 6., -6., 6.,
    ];
    surface(&mut Reader::new(&fields)).unwrap()
}
fn pair(u: f64, v: f64) -> Vector<'static> {
    let mut result = Vector::with_bounds(1, 2).unwrap();
    (result[1], result[2]) = (u, v);
    result
}
fn sampling(nb_u: i32, nb_v: i32) -> Sampling {
    Sampling {
        nb_u,
        nb_v,
        tol_u: 1e-9,
        tol_v: 1e-9,
    }
}
fn cylinder_circle() -> Handle<dyn Adaptor3dCurve> {
    let fields = [1., 0., 0., 0., 0., 0., 1., 1., 0., 0., 2., 0.];
    Space3::adaptor(&mut Reader::new(&fields)).unwrap()
}
fn cylinder_meridian() -> Handle<dyn Adaptor3dCurve> {
    let fields = [0., 2., 0., 0., 0., 0., 1., 1., -6., 6.];
    Space3::adaptor(&mut Reader::new(&fields)).unwrap()
}
#[test]
fn stationary_plane_function_has_exact_residual_and_identity_jacobian() {
    let s = plane();
    let mut function = FuncPSNorm::with_point_surface(Point3::new(3., 4., 5.), &s);
    let (mut residual, mut jacobian) = (pair(7.5, 7.5), Matrix::filled(1, 2, 1, 2, 7.5).unwrap());
    assert!(
        function
            .values(&pair(1., 2.), &mut residual, &mut jacobian)
            .unwrap()
    );
    assert_eq!((residual[1], residual[2]), (-2., -2.));
    assert_eq!(
        [
            jacobian[(1, 1)],
            jacobian[(1, 2)],
            jacobian[(2, 1)],
            jacobian[(2, 2)]
        ],
        [1., 0., 0., 1.]
    );
}
#[test]
fn plane_distance_value_preserves_retained_negative_half_gradient_convention() {
    let s = plane();
    let function = FuncPSDist::new(&s, Point3::new(3., 4., 5.));
    let (mut value, mut gradient) = (7.5, pair(7.5, 7.5));
    assert!(
        function
            .values(&pair(1., 2.), &mut value, &mut gradient)
            .unwrap()
    );
    assert_eq!((value, gradient[1], gradient[2]), (33., 2., 2.));
}
#[test]
fn plane_distance_function_rejects_outside_bounds_without_mutation() {
    let s = plane();
    let function = FuncPSDist::new(&s, Point3::new(3., 4., 5.));
    let (mut value, mut gradient) = (7.5, pair(11., 12.));
    assert!(
        !function
            .values(&pair(7., 0.), &mut value, &mut gradient)
            .unwrap()
    );
    assert_eq!((value, gradient[1], gradient[2]), (7.5, 11., 12.));
    assert!(function.value(&pair(-6., 6.), &mut value).unwrap());
    assert_eq!(value, 110.);
}
#[test]
fn uninitialized_normal_function_rejects_before_touching_outputs() {
    let mut function = FuncPSNorm::new();
    let mut residual = pair(7.5, 11.);
    assert_eq!(
        function
            .value(&pair(0., 0.), &mut residual)
            .unwrap_err()
            .exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!((residual[1], residual[2]), (7.5, 11.));
    assert_eq!(
        function.get_state_number().unwrap_err().exception_type(),
        "Standard_TypeMismatch"
    );
}
#[test]
fn normal_registration_deduplicates_parameters_and_point_reset_clears_results() {
    let s = plane();
    let mut function = FuncPSNorm::with_point_surface(Point3::new(3., 4., 5.), &s);
    assert!(function.value(&pair(0., 0.), &mut pair(0., 0.)).unwrap());
    for _ in 0..2 {
        assert_eq!(function.get_state_number().unwrap(), 0);
    }
    assert_eq!(function.nb_ext(), 1);
    assert_eq!(function.square_distance(1).unwrap(), 50.);
    function.set_point(Point3::new(0., 0., 3.));
    assert_eq!(function.nb_ext(), 0);
    assert!(function.value(&pair(0., 0.), &mut pair(0., 0.)).unwrap());
    function.get_state_number().unwrap();
    assert_eq!(function.square_distance(1).unwrap(), 9.);
    function.initialize(&s);
    assert_eq!(function.nb_ext(), 0);
}
#[test]
fn normal_projection_recovers_the_unique_plane_foot() {
    let s = plane();
    let mut result = GenLocateExtPS::new(&s, 1e-9, 1e-9);
    assert!(!result.is_done());
    assert_eq!(
        result.point().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    result
        .perform(Point3::new(3., 4., 5.), 0., 0., false)
        .unwrap();
    assert!(result.is_done());
    assert_eq!(result.square_distance().unwrap(), 25.);
    assert_eq!(result.point().unwrap().parameter(), (3., 4.));
    assert_eq!(*result.point().unwrap().value(), Point3::new(3., 4., 0.));
}
#[test]
fn plane_normal_foot_is_a_local_minimum_but_a_shifted_parameter_is_not() {
    let s = plane();
    let point = Point3::new(3., 4., 5.);
    assert!(GenLocateExtPS::is_min_dist(point, &s, 3., 4.).unwrap());
    assert!(!GenLocateExtPS::is_min_dist(point, &s, 2., 4.).unwrap());
}
#[test]
fn generic_grid_and_tree_searches_recover_the_same_plane_minimum() {
    let s = plane();
    let point = Point3::new(3., 4., 5.);
    let mut result =
        GenExtPS::with_point(point, &s, sampling(8, 8), ExtFlag::Min, ExtAlgo::Grad).unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 25.);
    result.set_algo(ExtAlgo::Tree);
    result.perform(point).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 25.);
}
#[test]
fn generic_surface_sampler_rejects_single_sample_in_either_direction() {
    let s = plane();
    let mut result = GenExtPS::new();
    for (nu, nv) in [(1, 8), (8, 1), (0, 0)] {
        assert_eq!(
            result
                .initialize(&s, sampling(nu, nv))
                .unwrap_err()
                .exception_type(),
            "Standard_OutOfRange"
        );
    }
}
#[test]
fn analytic_plane_extrema_has_the_unique_closed_form_foot() {
    let s = plane();
    let result = ExtPS::with_range(
        Point3::new(0., 0., 3.),
        &s,
        (-1., 1.),
        (-1., 1.),
        (1e-9, 1e-9),
        (ExtFlag::MinMax, ExtAlgo::Grad),
    )
    .unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    assert_eq!(*result.point(1).unwrap().value(), Point3::new(0., 0., 0.));
}
#[test]
fn default_initialized_corner_cache_remains_uncomputed_after_plane_projection() {
    let s = plane();
    let mut result = ExtPS::new();
    let initial = result.trimmed_square_distances();
    assert_eq!(initial, ([0.; 4], [Point3::new(0., 0., 0.); 4]));
    result
        .initialize(&s, (-1., 1.), (-1., 1.), (1e-9, 1e-9))
        .unwrap();
    result.perform(Point3::new(0., 0., 3.)).unwrap();
    assert_eq!(result.trimmed_square_distances(), initial);
}
#[test]
fn reusing_analytic_plane_extrema_replaces_the_point_and_distance() {
    let s = plane();
    let mut result = ExtPS::with_surface(
        Point3::new(3., 4., 5.),
        &s,
        1e-9,
        1e-9,
        ExtFlag::MinMax,
        ExtAlgo::Grad,
    )
    .unwrap();
    assert_eq!(result.square_distance(1).unwrap(), 25.);
    result.perform(Point3::new(1., 2., 3.)).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    assert_eq!(*result.point(1).unwrap().value(), Point3::new(1., 2., 0.));
}
#[test]
fn circle_extrusion_cylinder_has_exact_near_and_far_radial_distances() {
    let surface = Handle::new(
        GeomAdaptorSurfaceOfLinearExtrusion::with_curve_and_direction(
            &cylinder_circle(),
            Direction3::new(0., 0., 1.).unwrap(),
        ),
    );
    let result = ExtPExtS::with_range(
        Point3::new(3., 0., 1.),
        &surface,
        (0., TAU),
        (-6., 6.),
        (1e-9, 1e-9),
    )
    .unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 2);
    let mut distances = [
        result.square_distance(1).unwrap(),
        result.square_distance(2).unwrap(),
    ];
    distances.sort_by(f64::total_cmp);
    assert_eq!(distances, [1., 25.]);
}
#[test]
fn line_revolution_cylinder_has_the_same_independent_radial_distances() {
    let surface = Handle::new(
        GeomAdaptorSurfaceOfRevolution::with_curve_and_axis(&cylinder_meridian(), Axis1::default())
            .unwrap(),
    );
    let result = ExtPRevS::with_range(
        Point3::new(3., 0., 1.),
        &surface,
        (0., TAU),
        (-6., 6.),
        (1e-9, 1e-9),
    )
    .unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 2);
    let mut distances = [
        result.square_distance(1).unwrap(),
        result.square_distance(2).unwrap(),
    ];
    distances.sort_by(f64::total_cmp);
    assert_eq!(distances, [1., 25.]);
}
#[test]
fn unperformed_swept_and_analytic_results_reject_distance_access() {
    assert_eq!(
        ExtPExtS::new()
            .square_distance(1)
            .unwrap_err()
            .exception_type(),
        "StdFail_NotDone"
    );
    assert_eq!(
        ExtPRevS::new()
            .square_distance(1)
            .unwrap_err()
            .exception_type(),
        "StdFail_NotDone"
    );
    assert_eq!(
        ExtPS::new().nb_ext().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
}
