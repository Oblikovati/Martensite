//! Independent quadrature and explicitly separated rounding estimates.
use super::{admission::Admitted, failure::*, result::VolumeErrorEstimates};
use martensite_tkbo::measured::{IntegrationMethod, Measured};
use martensite_tkbrep::topods;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};
pub(super) fn measure(
    solid: &topods::Shape,
    admitted: &Admitted,
) -> Checked<(Measured, VolumeErrorEstimates)> {
    let (value, status) = integrate(solid, admitted.options.volume_relative_error)?;
    let errors = VolumeErrorEstimates {
        integration: absolute_error(value, status, admitted.options.volume_relative_error)?,
        construction: surface_area(admitted) * admitted.coordinate_error,
    };
    let error = errors.integration + errors.construction;
    accept_measure(value, error, admitted)?;
    let measured = Measured {
        value,
        error_bound: error,
        method: IntegrationMethod::AdaptiveQuadrature,
    };
    Ok((measured, errors))
}
fn absolute_error(value: f64, status: f64, requested: f64) -> Checked<f64> {
    let absolute = if value > martensite_tkernel::standard::real::epsilon(1.) {
        status * value
    } else {
        status
    };
    require_integration(value, status, absolute / value, requested)?;
    Ok(absolute + 64. * f64::EPSILON * value)
}
fn surface_area(admitted: &Admitted) -> f64 {
    let height = admitted.rectangle.z[1] - admitted.rectangle.z[0];
    let thickness = admitted.depths[1] - admitted.depths[0];
    let sum = admitted.radii[0] + admitted.radii[1];
    admitted.angle * sum * (height + thickness) + 2. * height * thickness
}
fn comparison_budget(admitted: &Admitted) -> f64 {
    surface_area(admitted) * (admitted.coordinate_error + admitted.options.resolution.weld)
        + 64. * f64::EPSILON * admitted.expected_volume
}

fn integrate(solid: &topods::Shape, requested: f64) -> Checked<(f64, f64)> {
    let mut props = GProps::new();
    let status = BRepGProp::volume_properties_gk(
        solid,
        &mut props,
        requested,
        true,
        VolumeGKFlags::default(),
    )
    .map_err(|error| native("Gauss-Kronrod volume", error))?;
    Ok((props.mass(), status))
}
fn accept_measure(value: f64, error: f64, admitted: &Admitted) -> Checked<()> {
    let requested = value * admitted.options.volume_relative_error;
    if !error.is_finite() || error > requested {
        return Err(mismatch(value, admitted.expected_volume, error, requested));
    }
    let budget = comparison_budget(admitted);
    if (value - admitted.expected_volume).abs() > budget + error {
        return Err(mismatch(value, admitted.expected_volume, error, budget));
    }
    Ok(())
}
fn mismatch(actual: f64, expected: f64, error: f64, budget: f64) -> SurfaceFailureCause {
    SurfaceFailureCause::VolumeMismatch {
        actual,
        expected,
        error,
        budget,
    }
}

fn require_integration(volume: f64, status: f64, relative: f64, requested: f64) -> Checked<()> {
    let valid = volume.is_finite()
        && volume > 0.
        && status.is_finite()
        && status >= 0.
        && relative.is_finite()
        && relative <= requested;
    if !valid {
        return Err(SurfaceFailureCause::IntegrationRejected {
            volume,
            status,
            relative,
            requested,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_error_units_switch_at_epsilon_without_losing_small_volume_error() {
        let epsilon = martensite_tkernel::standard::real::epsilon(1.);
        let tiny = epsilon / 2.;
        assert!(absolute_error(tiny, tiny * 1e-10, 1e-9).unwrap() >= tiny * 1e-10);
        assert!(absolute_error(tiny, 1e-10, 1e-9).is_err());
        assert!(absolute_error(epsilon * 2., 1e-10, 1e-9).is_ok());
        for (value, status) in [(1., -1.), (0., 0.), (f64::INFINITY, 0.), (1., f64::NAN)] {
            assert!(absolute_error(value, status, 1e-9).is_err());
        }
    }
}
