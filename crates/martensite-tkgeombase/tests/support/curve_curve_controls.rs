//! Closed-form line pairs pin extrema and the intentionally distinct objective/derivative contracts.
use super::super::construction_corpus::extrema_pc::{Space, Space2, Space3};
use super::super::construction_corpus::print::Reader;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::extrema::{
    CCLocFOfLocECC, Curve2dTool, CurveTool, ExtCC, ExtCC2d, GGenExtCC, GlobOptFuncCCC0,
    GlobOptFuncCCC1, GlobOptFuncCCC2, LocECC, LocateExtCC,
};
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::math::{Matrix, Vector};

fn spatial_line(origin: [f64; 3], direction: [f64; 3]) -> Handle<dyn Adaptor3dCurve> {
    let fields = [
        0.,
        origin[0],
        origin[1],
        origin[2],
        direction[0],
        direction[1],
        direction[2],
        1.,
        -6.,
        6.,
    ];
    Space3::adaptor(&mut Reader::new(&fields)).unwrap()
}
fn planar_line(origin: [f64; 2], direction: [f64; 2]) -> Handle<dyn Adaptor2dCurve2d> {
    let fields = [
        0.,
        origin[0],
        origin[1],
        direction[0],
        direction[1],
        1.,
        -6.,
        6.,
    ];
    Space2::adaptor(&mut Reader::new(&fields)).unwrap()
}
fn skew_lines() -> (Handle<dyn Adaptor3dCurve>, Handle<dyn Adaptor3dCurve>) {
    (
        spatial_line([0., 0., 0.], [1., 0., 0.]),
        spatial_line([0., 0., 3.], [0., 1., 0.]),
    )
}
fn crossing_lines() -> (Handle<dyn Adaptor2dCurve2d>, Handle<dyn Adaptor2dCurve2d>) {
    (
        planar_line([0., 0.], [1., 0.]),
        planar_line([0., 0.], [0., 1.]),
    )
}
fn pair(u: f64, v: f64) -> Vector<'static> {
    let mut result = Vector::with_bounds(1, 2).unwrap();
    (result[1], result[2]) = (u, v);
    result
}
fn entries(matrix: &Matrix<'_>) -> [f64; 4] {
    [
        matrix[(1, 1)],
        matrix[(1, 2)],
        matrix[(2, 1)],
        matrix[(2, 2)],
    ]
}
#[test]
fn skew_line_extrema_and_four_trimmed_distances_are_exact() {
    let (a, b) = skew_lines();
    let result = ExtCC::with_ranges(&*a, &*b, (-1., 1.), (-1., 1.), 1e-10, 1e-10).unwrap();
    assert!(result.is_done());
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    let (first, second) = result.points(1).unwrap();
    assert_eq!((first.parameter(), second.parameter()), (0., 0.));
    assert_eq!(
        (*first.value(), *second.value()),
        (Point3::new(0., 0., 0.), Point3::new(0., 0., 3.))
    );
    assert_eq!(result.trimmed_square_distances().0, [11.; 4]);
}
#[test]
fn planar_crossing_has_zero_minimum_and_exact_trimmed_distances() {
    let (a, b) = crossing_lines();
    let result = ExtCC2d::with_ranges(&*a, &*b, (-1., 1.), (-1., 1.), 1e-10, 1e-10).unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.square_distance(1).unwrap(), 0.);
    let (first, second) = result.points(1).unwrap();
    assert_eq!(
        (*first.value(), *second.value()),
        (Point2::new(0., 0.), Point2::new(0., 0.))
    );
    assert_eq!(result.trimmed_square_distances().0, [2.; 4]);
}
#[test]
fn finite_parallel_overlap_reports_distance_without_a_selected_pair() {
    let a = spatial_line([0., 0., 0.], [1., 0., 0.]);
    let b = spatial_line([0., 3., 0.], [1., 0., 0.]);
    let result = ExtCC::with_ranges(&*a, &*b, (-1., 1.), (-1., 1.), 1e-10, 1e-10).unwrap();
    assert!(result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    assert_eq!(
        result.points(1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn parallel_ranges_touching_at_one_parameter_return_the_unique_pair() {
    let a = spatial_line([0., 0., 0.], [1., 0., 0.]);
    let b = spatial_line([0., 3., 0.], [1., 0., 0.]);
    let result = ExtCC::with_ranges(&*a, &*b, (-1., 0.), (0., 1.), 1e-10, 1e-10).unwrap();
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    let (first, second) = result.points(1).unwrap();
    assert_eq!(
        (*first.value(), *second.value()),
        (Point3::new(0., 0., 0.), Point3::new(0., 3., 0.))
    );
}
#[test]
fn repeated_perform_replaces_the_solution_and_preserves_single_solution_option() {
    let (a, b) = skew_lines();
    let mut result = ExtCC::new(1e-9, 1e-8);
    result.set_single_solution_flag(true);
    result
        .initialize_ranges(&*a, &*b, (-1., 1.), (-1., 1.), (1e-9, 1e-8))
        .unwrap();
    for _ in 0..2 {
        result.perform().unwrap();
        assert!(result.get_single_solution_flag());
        assert_eq!(result.nb_ext().unwrap(), 1);
        assert_eq!(result.square_distance(1).unwrap(), 9.);
    }
}
#[test]
fn unperformed_and_out_of_range_result_access_have_distinct_failures() {
    let result = ExtCC::new(1e-9, 1e-8);
    assert_eq!(
        result.is_parallel().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    assert_eq!(
        result.nb_ext().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    let (a, b) = skew_lines();
    let done = ExtCC::with_curves(&*a, &*b, 1e-10, 1e-10).unwrap();
    assert_eq!(
        done.square_distance(0).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        done.points(2).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn generic_curve_search_recovers_the_closed_form_skew_distance() {
    let (a, b) = skew_lines();
    let mut result = GGenExtCC::<CurveTool>::with_bounds(&*a, &*b, (-1., 1.), (-1., 1.));
    result.perform().unwrap();
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
}
#[test]
fn both_local_search_interfaces_recover_the_skew_minimum() {
    let (a, b) = skew_lines();
    let generic = LocECC::new::<CurveTool>(&*a, &*b, (0.25, -0.75), (1e-9, 1e-9)).unwrap();
    let located = LocateExtCC::new(&*a, &*b, 0.25, -0.75).unwrap();
    assert!(generic.is_done() && located.is_done());
    assert_eq!(generic.square_distance().unwrap(), 9.);
    assert_eq!(located.square_distance().unwrap(), 9.);
    let (first, second) = generic.point().unwrap();
    assert_eq!((first.parameter(), second.parameter()), (0., 0.));
}
#[test]
fn stationary_line_residual_and_jacobian_are_not_the_global_objective_gradient() {
    let (a, b) = skew_lines();
    let mut function = CCLocFOfLocECC::with_curves(&*a, &*b, 1e-10).unwrap();
    let mut residual = pair(7.5, 7.5);
    let mut jacobian = Matrix::filled(1, 2, 1, 2, 7.5).unwrap();
    assert!(
        function
            .values(&pair(0.5, -0.5), &mut residual, &mut jacobian)
            .unwrap()
    );
    assert_eq!((residual[1], residual[2]), (-0.5, -0.5));
    assert_eq!(entries(&jacobian), [-1., 0., 0., 1.]);
}
#[test]
fn explicit_stationary_state_registration_retains_each_call() {
    let (a, b) = skew_lines();
    let mut function = CCLocFOfLocECC::with_curves(&*a, &*b, 1e-10).unwrap();
    assert!(function.value(&pair(0., 0.), &mut pair(7.5, 7.5)).unwrap());
    for count in [1, 2] {
        assert_eq!(function.get_state_number(), 0);
        assert_eq!(function.nb_ext(), count);
        assert_eq!(function.square_distance(count).unwrap(), 9.);
    }
}
#[test]
fn curve_rank_identity_and_tolerance_search_use_the_registered_curve() {
    let (a, b) = skew_lines();
    let mut function = CCLocFOfLocECC::with_curves(&*a, &*b, 1e-10).unwrap();
    assert!(std::ptr::addr_eq(
        function.curve_ptr(1).unwrap().unwrap(),
        &*a
    ));
    assert_eq!(function.search_of_tolerance(&*a).unwrap(), 1e-12);
    for rank in [0, 3] {
        assert_eq!(
            function.curve_ptr(rank).err().unwrap().exception_type(),
            "Standard_OutOfRange"
        );
        assert_eq!(
            function.set_curve(rank, &*b).unwrap_err().exception_type(),
            "Standard_OutOfRange"
        );
    }
    function.set_curve(1, &*b).unwrap();
    assert!(std::ptr::addr_eq(
        function.curve_ptr(1).unwrap().unwrap(),
        &*b
    ));
    let empty = CCLocFOfLocECC::new(1e-10);
    assert_eq!(empty.search_of_tolerance(&*a).unwrap(), 1e-20);
}
#[test]
fn spatial_objective_is_distance_but_its_derivatives_are_for_squared_distance() {
    let (a, b) = skew_lines();
    let (f0, f1, f2) = (
        GlobOptFuncCCC0::<CurveTool>::new(&*a, &*b),
        GlobOptFuncCCC1::<CurveTool>::new(&*a, &*b),
        GlobOptFuncCCC2::<CurveTool>::new(&*a, &*b),
    );
    let (mut value, mut gradient, mut hessian) = (
        7.5,
        pair(7.5, 7.5),
        Matrix::filled(1, 2, 1, 2, 7.5).unwrap(),
    );
    assert!(f0.value(&pair(4., 0.), &mut value).unwrap());
    assert_eq!(value, 5.);
    assert!(f1.values(&pair(4., 0.), &mut value, &mut gradient).unwrap());
    assert_eq!((value, gradient[1], gradient[2]), (5., 8., 0.));
    assert!(
        f2.values_hessian(&pair(4., 0.), &mut value, &mut gradient, &mut hessian)
            .unwrap()
    );
    assert_eq!(value, 5.);
    assert_eq!(entries(&hessian), [2., 0., 0., 2.]);
}
#[test]
fn planar_objective_retains_the_same_distance_and_squared_derivative_convention() {
    let (a, b) = crossing_lines();
    let function = GlobOptFuncCCC2::<Curve2dTool>::new(&*a, &*b);
    let (mut value, mut gradient, mut hessian) = (
        7.5,
        pair(7.5, 7.5),
        Matrix::filled(1, 2, 1, 2, 7.5).unwrap(),
    );
    assert!(
        function
            .values_hessian(&pair(3., 4.), &mut value, &mut gradient, &mut hessian)
            .unwrap()
    );
    assert_eq!((value, gradient[1], gradient[2]), (5., 6., 8.));
    assert_eq!(entries(&hessian), [2., 0., 0., 2.]);
}
#[test]
fn global_objective_rejects_outside_ranges_without_mutating_outputs() {
    let (a, b) = skew_lines();
    let function = GlobOptFuncCCC2::<CurveTool>::new(&*a, &*b);
    let (mut value, mut gradient, mut hessian) = (
        7.5,
        pair(11., 12.),
        Matrix::filled(1, 2, 1, 2, 13.).unwrap(),
    );
    assert!(
        !function
            .values_hessian(&pair(7., 0.), &mut value, &mut gradient, &mut hessian)
            .unwrap()
    );
    assert_eq!((value, gradient[1], gradient[2]), (7.5, 11., 12.));
    assert_eq!(entries(&hessian), [13.; 4]);
    assert!(function.value(&pair(-6., 6.), &mut value).unwrap());
    assert_eq!(value, 9.);
}
