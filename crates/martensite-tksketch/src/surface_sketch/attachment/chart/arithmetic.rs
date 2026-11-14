//! Directed rounding encloses the ideal binary64 chart expression, including underflow.
use super::*;
use martensite_tkbo::measured::IntegrationMethod;

pub(super) fn arc(attempt: &Attempt, radius: f64, u: f64, seam: f64) -> Checked<Measured> {
    let difference = u - seam;
    let value = radius * difference;
    let lower = (radius * difference.next_down()).next_down();
    let upper = (radius * difference.next_up()).next_up();
    let error_bound = (value - lower).max(upper - value).next_up();
    if !value.is_finite() || !error_bound.is_finite() || radius <= 0. {
        return Err(attempt.reject(AttachmentFailureCause::ChartValue {
            field: "developed arc length",
            value,
        }));
    }
    Ok(Measured {
        value,
        error_bound,
        method: IntegrationMethod::CertifiedEnclosure,
    })
}
pub(super) fn axial(value: f64) -> Measured {
    Measured {
        value,
        error_bound: 0.,
        method: IntegrationMethod::CertifiedEnclosure,
    }
}
