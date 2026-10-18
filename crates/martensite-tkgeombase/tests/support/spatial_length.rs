//! Shared production-adaptor dispatch for the original spatial length command.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{geom::GeomCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
#[path = "length_policy.rs"]
mod length_policy;
use length_policy::{LENGTH_COMPUTE_TOLERANCE, record_original_length};

/// Apply the original spatial length command; `record_spatial_curve_length("case", curve, 5.0)`.
pub fn record_spatial_curve_length(case: &str, curve: Handle<dyn GeomCurve>, expected: f64) {
    let adaptor = GeomAdaptorCurve::with_curve(&curve).unwrap();
    record_original_length(
        case,
        AbscissaPoint::length_with_tolerance(&adaptor, LENGTH_COMPUTE_TOLERANCE).unwrap(),
        expected,
    );
}
