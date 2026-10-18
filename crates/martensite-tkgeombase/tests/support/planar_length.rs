//! Shared production-adaptor dispatch for the original planar length command.
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::{geom2d::Geom2dCurve, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkgeombase::gcpnts::AbscissaPoint;
#[path = "length_policy.rs"]
mod length_policy;
use length_policy::{LENGTH_COMPUTE_TOLERANCE, record_original_length};

/// Apply the original planar length command; `record_planar_curve_length("case", curve, 5.0)`.
pub fn record_planar_curve_length(case: &str, curve: Handle<dyn Geom2dCurve>, expected: f64) {
    let adaptor = Geom2dAdaptorCurve::with_curve(&curve).unwrap();
    record_original_length(
        case,
        AbscissaPoint::length_with_tolerance(&adaptor, LENGTH_COMPUTE_TOLERANCE).unwrap(),
        expected,
    );
}
