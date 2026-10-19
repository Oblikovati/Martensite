//! Exact line/plane algebra pins the distance objectives and state policies independently.
use super::super::construction_corpus::{
    extrema_pc::{Space, Space3},
    extrema_ps::surface,
    print::Reader,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, geom_adaptor::GeomAdaptorSurface};
use martensite_tkgeombase::extrema::{
    ExtCS, ExtSS, FuncExtCS, FuncExtSS, GenExtCS, GenExtSS, GlobOptFuncCQuadric, GlobOptFuncCS,
    GlobOptFuncConicS,
};
use martensite_tkmath::{
    gp::Point3,
    math::{Matrix, Vector},
};

fn plane(z: f64) -> GeomAdaptorSurface {
    let fields = [
        0., 0., 0., 0., z, 0., 0., 1., 1., 0., 0., 1., -6., 6., -6., 6.,
    ];
    surface(&mut Reader::new(&fields)).unwrap()
}
fn line(origin: [f64; 3], direction: [f64; 3]) -> Handle<dyn Adaptor3dCurve> {
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
fn vertical() -> Handle<dyn Adaptor3dCurve> {
    line([0., 0., 0.], [0., 0., 1.])
}
fn vector(values: &[f64]) -> Vector<'static> {
    let mut result = Vector::with_bounds(1, values.len() as i32).unwrap();
    for (index, value) in values.iter().enumerate() {
        result[index as i32 + 1] = *value;
    }
    result
}
fn matrix_entries(matrix: &Matrix<'_>, size: i32) -> Vec<f64> {
    (1..=size)
        .flat_map(|i| (1..=size).map(move |j| matrix[(i, j)]))
        .collect()
}
#[test]
fn global_line_plane_distance_has_retained_half_gradient_and_half_hessian() {
    let (c, s) = (vertical(), plane(0.));
    let function = GlobOptFuncCS::new(&*c, &s);
    let mut shifted = Vector::with_bounds(-2, 0).unwrap();
    (shifted[-2], shifted[-1], shifted[0]) = (1., 2., 3.);
    let (mut value, mut gradient, mut hessian) = (
        7.5,
        vector(&[7.5; 3]),
        Matrix::filled(1, 3, 1, 3, 7.5).unwrap(),
    );
    assert!(
        function
            .values_hessian(&shifted, &mut value, &mut gradient, &mut hessian)
            .unwrap()
    );
    assert_eq!(value, 14.);
    assert_eq!([gradient[1], gradient[2], gradient[3]], [1., 2., 3.]);
    assert_eq!(
        matrix_entries(&hessian, 3),
        [1., 0., 0., 0., 1., 0., 0., 0., 1.]
    );
}
#[test]
fn global_line_plane_range_failure_preserves_every_output() {
    let (c, s) = (vertical(), plane(0.));
    let function = GlobOptFuncCS::new(&*c, &s);
    let (mut value, mut gradient, mut hessian) = (
        7.5,
        vector(&[11., 12., 13.]),
        Matrix::filled(1, 3, 1, 3, 14.).unwrap(),
    );
    assert!(
        !function
            .values_hessian(
                &vector(&[7., 0., 0.]),
                &mut value,
                &mut gradient,
                &mut hessian
            )
            .unwrap()
    );
    assert_eq!(
        (value, gradient[1], gradient[2], gradient[3]),
        (7.5, 11., 12., 13.)
    );
    assert_eq!(matrix_entries(&hessian, 3), [14.; 9]);
}
#[test]
fn stationary_line_plane_residual_has_the_signed_geometric_jacobian() {
    let (c, s) = (vertical(), plane(0.));
    let mut function = FuncExtCS::with_curve_surface(&*c, &s);
    let (mut residual, mut jacobian) =
        (vector(&[7.5; 3]), Matrix::filled(1, 3, 1, 3, 7.5).unwrap());
    assert!(
        function
            .values(&vector(&[1., 2., 3.]), &mut residual, &mut jacobian)
            .unwrap()
    );
    assert_eq!([residual[1], residual[2], residual[3]], [1., -2., -3.]);
    assert_eq!(
        matrix_entries(&jacobian, 3),
        [1., 0., 0., 0., -1., 0., 0., 0., -1.]
    );
}
#[test]
fn curve_surface_saved_state_deduplicates_only_the_curve_parameter() {
    let (c, s) = (vertical(), plane(0.));
    let mut function = FuncExtCS::with_curve_surface(&*c, &s);
    for uv in [[0., 1., 2.], [0., 2., 3.]] {
        function.value(&vector(&uv), &mut vector(&[0.; 3])).unwrap();
        function.get_state_number().unwrap();
    }
    assert_eq!(function.nb_ext(), 1);
    assert_eq!(function.square_distance(1).unwrap(), 5.);
    assert_eq!(
        *function.point_on_surface(1).unwrap().value(),
        Point3::new(1., 2., 0.)
    );
    function.initialize(&*c, &s);
    assert_eq!(function.nb_ext(), 0);
}
#[test]
fn conic_projection_selects_the_nearest_trimmed_line_endpoint() {
    let (c, s) = (vertical(), plane(0.));
    let full = GlobOptFuncConicS::with_curve(&*c, &s).unwrap();
    let mut trimmed = GlobOptFuncConicS::with_bounds(&s, (-6., 6.), (-6., 6.));
    trimmed.load_conic(&*c, 2., 4.).unwrap();
    let (uv, mut value) = (vector(&[2., 3.]), 7.5);
    assert!(full.value(&uv, &mut value).unwrap());
    assert_eq!(value, 13.);
    assert_eq!(full.conic_parameter(&uv).unwrap(), 0.);
    assert!(trimmed.value(&uv, &mut value).unwrap());
    assert_eq!(value, 17.);
    assert_eq!(trimmed.conic_parameter(&uv).unwrap(), 2.);
}
#[test]
fn quadric_projection_recovers_the_plane_foot_and_rejects_external_curve_parameter() {
    let (c, s) = (vertical(), plane(0.));
    let function = GlobOptFuncCQuadric::with_surface(&*c, &s).unwrap();
    let (mut value, mut uv) = (7.5, vector(&[7.5; 2]));
    assert!(function.value(&vector(&[4.]), &mut value).unwrap());
    assert_eq!(value, 16.);
    function
        .quadric_parameters(&vector(&[4.]), &mut uv)
        .unwrap();
    assert_eq!((uv[1], uv[2]), (0., 0.));
    value = 7.5;
    assert!(!function.value(&vector(&[7.]), &mut value).unwrap());
    assert_eq!(value, 7.5);
}
#[test]
fn trimmed_quadric_policy_checks_corners_without_inventing_a_boundary_edge_search() {
    let c = line([2., 0., 0.], [0., 0., 1.]);
    let s = plane(0.);
    let mut function = GlobOptFuncCQuadric::with_range(&*c, -6., 6.);
    function.load_quad(&s, (-1., 1.), (-1., 1.)).unwrap();
    let (mut value, mut uv) = (7.5, vector(&[7.5; 2]));
    assert!(function.value(&vector(&[0.]), &mut value).unwrap());
    assert_eq!(value, 2.);
    function
        .quadric_parameters(&vector(&[0.]), &mut uv)
        .unwrap();
    assert_eq!((uv[1], uv[2]), (1., -1.));
}
#[test]
fn vertical_line_plane_extrema_returns_the_unique_intersection() {
    let (c, s) = (vertical(), plane(0.));
    let result = ExtCS::with_curve(&*c, &s, 1e-9, 1e-9).unwrap();
    assert!(result.is_done());
    assert!(!result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 0.);
    let (pc, ps) = result.points(1).unwrap();
    assert_eq!(
        (*pc.value(), *ps.value()),
        (Point3::new(0., 0., 0.), Point3::new(0., 0., 0.))
    );
}
#[test]
fn general_curve_surface_search_recovers_the_unique_intersection() {
    let (c, s) = (vertical(), plane(0.));
    let result = GenExtCS::with_curve(&*c, &s, (8, 8, 8), 1e-9, 1e-9).unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 0.);
}
#[test]
fn parallel_line_plane_extrema_preserves_distance_without_selecting_a_pair() {
    let c = line([0., 0., 3.], [1., 0., 0.]);
    let s = plane(0.);
    let result = ExtCS::with_curve(&*c, &s, 1e-9, 1e-9).unwrap();
    assert!(result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    assert_eq!(
        result.points(1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn parallel_plane_stationary_residual_and_four_variable_jacobian_are_exact() {
    let (a, b) = (plane(0.), plane(3.));
    let mut function = FuncExtSS::with_surfaces(&a, &b);
    let (mut residual, mut jacobian) =
        (vector(&[7.5; 4]), Matrix::filled(1, 4, 1, 4, 7.5).unwrap());
    assert!(
        function
            .values(&vector(&[1., 2., 3., 4.]), &mut residual, &mut jacobian)
            .unwrap()
    );
    assert_eq!(
        [residual[1], residual[2], residual[3], residual[4]],
        [-2.; 4]
    );
    assert_eq!(
        matrix_entries(&jacobian, 4),
        [
            1., 0., -1., 0., 0., 1., 0., -1., 1., 0., -1., 0., 0., 1., 0., -1.
        ]
    );
}
#[test]
fn surface_pair_registration_keeps_duplicates_and_reinitialization_clears_them() {
    let (a, b) = (plane(0.), plane(3.));
    let mut function = FuncExtSS::with_surfaces(&a, &b);
    function
        .value(&vector(&[1., 2., 3., 4.]), &mut vector(&[0.; 4]))
        .unwrap();
    for count in [1, 2] {
        assert_eq!(function.get_state_number().unwrap(), 0);
        assert_eq!(function.nb_ext(), count);
        assert_eq!(function.square_distance(count).unwrap(), 17.);
    }
    function.initialize(&b, &a);
    assert_eq!(function.nb_ext(), 0);
}
#[test]
fn parallel_plane_extrema_returns_distance_without_a_selected_point_pair() {
    let (a, b) = (plane(0.), plane(3.));
    let result = ExtSS::with_surfaces(&a, &b, 1e-9, 1e-9).unwrap();
    assert!(result.is_done());
    assert!(result.is_parallel().unwrap());
    assert_eq!(result.nb_ext().unwrap(), 1);
    assert_eq!(result.square_distance(1).unwrap(), 9.);
    assert_eq!(
        result.points(1).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
}
#[test]
fn general_plane_pair_search_retains_both_stationary_solver_records() {
    let (a, b) = (plane(0.), plane(3.));
    let result = GenExtSS::with_surfaces(&a, &b, 8, 8, 1e-9, 1e-9).unwrap();
    assert!(result.is_done());
    assert_eq!(result.nb_ext().unwrap(), 2);
    for index in 1..=2 {
        assert_eq!(result.square_distance(index).unwrap(), 9.);
    }
}
#[test]
fn uninitialized_function_sets_preserve_output_sentinels() {
    let mut cs = FuncExtCS::new();
    let mut three = vector(&[7.5; 3]);
    assert_eq!(
        cs.value(&vector(&[0.; 3]), &mut three)
            .unwrap_err()
            .exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!([three[1], three[2], three[3]], [7.5; 3]);
    let mut ss = FuncExtSS::new();
    let mut four = vector(&[11.; 4]);
    assert_eq!(
        ss.value(&vector(&[0.; 4]), &mut four)
            .unwrap_err()
            .exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!([four[1], four[2], four[3], four[4]], [11.; 4]);
}
#[test]
fn default_result_access_requires_a_completed_computation() {
    assert_eq!(
        ExtCS::new().nb_ext().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
    assert_eq!(
        ExtSS::new().is_parallel().unwrap_err().exception_type(),
        "StdFail_NotDone"
    );
}
