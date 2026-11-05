use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

pub(super) fn require_closed_form(shape: &Shape, area: f64, volume: f64) {
    let mut surface = GProps::default();
    let area_error = BRepGProp::surface_properties_eps(shape, &mut surface, 1e-10, false).unwrap();
    let mut solid = GProps::default();
    let volume_error =
        BRepGProp::volume_properties_gk(shape, &mut solid, 1e-10, true, VolumeGKFlags::default())
            .unwrap();
    require_measure(surface.mass(), area, area_error);
    require_measure(solid.mass(), volume, volume_error);
    println!(
        "{{\"area\":{},\"volume\":{},\"area_error_estimate\":{},\"volume_error_estimate\":{},\"requested_relative_target\":1e-10,\"evidence\":\"quadrature-estimate-plus-independent-analytic-box\"}}",
        surface.mass(),
        solid.mass(),
        area_error,
        volume_error
    );
}

fn require_measure(value: f64, expected: f64, reported_error: f64) {
    assert!(
        reported_error.is_finite() && reported_error >= 0.,
        "quadrature error={reported_error}"
    );
    assert!(
        reported_error <= 1e-10,
        "quadrature error={reported_error}, expected <=1e-10"
    );
    assert!(
        (value - expected).abs() <= 1e-10,
        "measurement={value}, expected {expected}"
    );
}
