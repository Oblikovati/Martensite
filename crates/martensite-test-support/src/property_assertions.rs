//! Development comparisons for integrated mass properties.
use crate::floating::within_absolute_error as near;
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
#[cfg(test)]
mod tests {
    use super::{GProps, expect_properties_near};
    use martensite_tkmath::gp::{Matrix3, Point3};

    fn point_mass_fixture() -> GProps {
        GProps {
            dim: 2.,
            g: Point3::new(1., 2., 3.),
            loc: Point3::default(),
            inertia: Matrix3::new([[26., -4., -6.], [-4., 20., -12.], [-6., -12., 10.]]),
        }
    }

    #[test]
    fn copied_point_mass_and_empty_properties_match_exactly() {
        let fixture = point_mass_fixture();
        expect_properties_near(&fixture, &fixture, 0.);
        expect_properties_near(&GProps::new(), &GProps::new(), 0.);
    }

    #[test]
    #[should_panic]
    fn changed_mass_is_rejected() {
        let mut actual = point_mass_fixture();
        actual.dim = 3.;
        expect_properties_near(&actual, &point_mass_fixture(), 0.);
    }

    #[test]
    #[should_panic]
    fn changed_center_is_rejected() {
        let mut actual = point_mass_fixture();
        actual.g = Point3::new(2., 2., 3.);
        expect_properties_near(&actual, &point_mass_fixture(), 0.);
    }

    #[test]
    #[should_panic]
    fn changed_inertia_is_rejected() {
        let mut actual = point_mass_fixture();
        actual.inertia.set_value(1, 1, 27.).unwrap();
        expect_properties_near(&actual, &point_mass_fixture(), 0.);
    }
}
