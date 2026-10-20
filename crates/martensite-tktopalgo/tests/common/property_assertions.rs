#![allow(dead_code)]
//! Development comparisons for integrated mass properties.
use super::floating::within_absolute_error as near;
use martensite_tkgeombase::gprop::GProps;

/// Compares integrated mass, center and inertia with one inclusive absolute tolerance.
/// `expect_properties_near(&GProps::new(), &GProps::new(), 0.0);`
pub fn expect_properties_near(actual: &GProps, expected: &GProps, tol: f64) {
    assert!(near(actual.mass(), expected.mass(), tol));
    let (a, e) = (actual.centre_of_mass(), expected.centre_of_mass());
    assert!(near(a.x(), e.x(), tol) && near(a.y(), e.y(), tol) && near(a.z(), e.z(), tol));
    let (ma, me) = (actual.matrix_of_inertia(), expected.matrix_of_inertia());
    for row in 1..=3 {
        for column in 1..=3 {
            let (x, y) = (
                ma.value(row, column).unwrap(),
                me.value(row, column).unwrap(),
            );
            assert!(near(x, y, tol), "inertia ({row}, {column}): {x} vs {y}");
        }
    }
}
