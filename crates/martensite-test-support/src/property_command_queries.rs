//! Shared property queries exercised by native command regression translations.
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Direction3};

/// Executes the property command's kernel queries and constructs its principal frame.
///
/// ```
/// use martensite_test_support::property_command_queries::property_command_axes;
/// use martensite_tkgeombase::gprop::GProps;
/// let axes = property_command_axes(&GProps::new())?;
/// assert_eq!(axes.location().x(), 0.0);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub fn property_command_axes(properties: &GProps) -> Result<Axis2, Failure> {
    let _mass = properties.mass();
    let center = properties.centre_of_mass();
    let _matrix = properties.matrix_of_inertia();
    let principal = properties.principal_properties()?;
    let _moments = principal.moments();
    let normal = Direction3::from_vector(principal.third_axis_of_inertia())?;
    let x_direction = Direction3::from_vector(principal.first_axis_of_inertia())?;
    Axis2::with_x_direction(center, normal, x_direction)
}

#[cfg(test)]
mod tests {
    use super::property_command_axes;
    use martensite_tkgeombase::gprop::GProps;
    use martensite_tkmath::gp::{Matrix3, Point3};

    #[test]
    fn empty_property_queries_preserve_origin_and_orthogonal_axes() {
        let axes = property_command_axes(&GProps::new()).unwrap();
        assert_eq!(axes.location(), Point3::new(0., 0., 0.));
        assert_eq!(axes.direction().dot(axes.x_direction()), 0.);
        assert_eq!(axes.direction().dot(axes.y_direction()), 0.);
        assert_eq!(axes.x_direction().dot(axes.y_direction()), 0.);
    }

    #[test]
    fn diagonal_inertia_produces_the_expected_principal_axis_lines() {
        let location = Point3::new(1.25, -3.5, 7.);
        let properties = GProps {
            dim: 1.,
            loc: location,
            inertia: Matrix3::new([[2., 0., 0.], [0., 3., 0.], [0., 0., 4.]]),
            ..GProps::new()
        };
        let axes = property_command_axes(&properties).unwrap();
        assert_eq!(axes.location(), location);
        assert_eq!(axes.direction().x().abs(), 1.);
        assert_eq!(axes.x_direction().z().abs(), 1.);
    }
}
