//! Independent behavior and supporting operations.
use crate::construction_corpus::{
    common::Reader, finite_element_driver::read_array2, fit_constraints_points::array,
    linear_tokens::read_vector,
};
use martensite_tkernel::{
    ncollection::{Array2, HArray1, HArray2},
    standard::Handle,
};
use martensite_tkgeombase::fem_tool::{
    Assembly, AssemblyTable, Curve, ElementaryCriterion, ElementsOfRefMatrix, LinearTension,
    ProfileMatrix, SparseMatrix,
};
use martensite_tkmath::{
    geom_abs::Shape,
    math::{Matrix, Vector},
    plib::HermitJacobi,
};
fn system_matrix() -> Matrix<'static> {
    let mut m = Matrix::filled(1, 2, 1, 2, 0.).unwrap();
    for (i, j, value) in [(1, 1, 4.), (1, 2, 2.), (2, 1, 2.), (2, 2, 5.)] {
        m[(i, j)] = value;
    }
    m
}
fn linear_coefficients() -> Array2<'static, f64> {
    read_array2(&mut Reader::new(&[0., 1., 2., 1., 0., 2.]))
}
#[test]
fn linear_hermite_reference_elements_are_basis_products() {
    let basis = HermitJacobi::new(3, Shape::C0).unwrap();
    let u = Vector::filled(1, 1, 0.).unwrap();
    for (order, expected) in [(0, [0.25, 0.25, 0.25]), (1, [0.25, -0.25, 0.25])] {
        let e = ElementsOfRefMatrix::new(&basis, order).unwrap();
        assert_eq!((e.nb_variables(), e.nb_equations()), (1, 10));
        let mut output = Vector::filled(1, 10, -7.).unwrap();
        assert!(e.evaluate(&u, &mut output).unwrap());
        assert_eq!([output[1], output[2], output[5]], expected);
    }
}
#[test]
fn straight_piecewise_curve_has_linear_values_and_constant_derivatives() {
    let basis = HermitJacobi::new(3, Shape::C0).unwrap();
    let curve = Curve::new(1, 1, &basis, 1e-7).unwrap();
    curve.knots()[1] = 0.;
    curve.knots()[2] = 1.;
    let coefficients = read_array2(&mut Reader::new(&[0., 1., 4., 1., 0., 2., 0., 0.]));
    curve.set_element(1, &coefficients).unwrap();
    for t in [-0.5, 0., 0.25, 0.5, 1., 1.5] {
        let mut output = array(1, &[-7.]);
        curve.d0(t, &mut output).unwrap();
        assert_eq!(output[1], 2. * t);
        curve.d1(t, &mut output).unwrap();
        assert_eq!(output[1], 2.);
        curve.d2(t, &mut output).unwrap();
        assert_eq!(output[1], 0.);
    }
}
#[test]
fn profile_matrix_solves_an_independently_known_two_variable_system() {
    let profile = ProfileMatrix::new(&array(1, &[1, 1])).unwrap();
    profile.init(0.);
    let matrix = system_matrix();
    for (i, j) in [(1, 1), (2, 1), (2, 2)] {
        *profile.change_value(i, j).unwrap() = matrix[(i, j)];
    }
    let x = read_vector(&mut Reader::new(&[1., 2., 1., 2.]));
    let mut product = Vector::filled(1, 2, -7.).unwrap();
    profile.multiplied(&x, &mut product).unwrap();
    assert_eq!([product[1], product[2]], [8., 12.]);
    assert!(profile.decompose().unwrap());
    let mut solved = Vector::filled(1, 2, -7.).unwrap();
    profile.solve(&product, &mut solved).unwrap();
    assert_eq!([solved[1], solved[2]], [1., 2.]);
}
fn assembly_table() -> AssemblyTable {
    let table = Handle::new(HArray2::with_bounds(1, 1, 1, 1).unwrap());
    let indices = Handle::new(HArray1::from_owned(array(1, &[1, 2])));
    table.change_array2().set_value(1, 1, indices).unwrap();
    table
}
#[test]
fn assembly_uses_shared_incidence_to_solve_the_known_system() {
    let mut dep = Array2::with_bounds(1, 1, 1, 1).unwrap();
    dep.init(1);
    let mut assembly = Assembly::new(&dep, &assembly_table()).unwrap();
    assert_eq!(assembly.nb_glob_var(), 2);
    assembly.add_matrix(1, 1, 1, &system_matrix()).unwrap();
    let rhs = read_vector(&mut Reader::new(&[1., 2., 8., 12.]));
    assembly.add_vector(1, 1, &rhs).unwrap();
    assert!(assembly.solve().unwrap());
    let mut output = Vector::filled(1, 2, -7.).unwrap();
    assembly.solution(&mut output).unwrap();
    assert_eq!([output[1], output[2]], [1., 2.]);
}
#[test]
fn straight_line_tension_matches_the_integral_of_squared_slope() {
    let criterion = LinearTension::new(1, Shape::C0).unwrap();
    criterion.set_coeff(&Handle::new(HArray2::from_owned(linear_coefficients())));
    criterion.set(0., 1.);
    // The independent integral is integral_0^1(2^2) du = 4; this supplement
    // does not relax the original frozen bit comparison in any of the 1230 rows.
    assert!((criterion.value().unwrap() - 4.).abs() < 1e-12);
}
