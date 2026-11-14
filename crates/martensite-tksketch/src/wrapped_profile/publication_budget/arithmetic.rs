//! Outward nonnegative products shared by cap-domain and homotopy bounds.
use super::*;

pub(super) fn scale_upper(value: f64, factor: f64, quantity: Quantity) -> Checked<f64> {
    volume::valid_bound(value, quantity)?;
    volume::valid_bound(factor, quantity)?;
    if value == 0. || factor == 0. {
        return Ok(0.);
    }
    if factor == 1. {
        return Ok(value);
    }
    let upper = (value * factor).next_up();
    volume::valid_bound(upper, quantity)?;
    Ok(upper)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn scale(value: f64, factor: f64) -> Checked<f64> {
        scale_upper(value, factor, Quantity::CapDomain)
    }
    #[test]
    fn nonnegative_products_enclose_exact_controls_including_underflow() {
        assert_eq!(scale(0., 3.).unwrap(), 0.);
        assert_eq!(scale(2., 0.).unwrap(), 0.);
        assert_eq!(scale(0.1, 1.).unwrap(), 0.1);
        assert!(scale(0.5, 3.).unwrap() >= 1.5);
        assert_eq!(scale(f64::from_bits(1), 0.5).unwrap(), f64::from_bits(1));
    }
    #[test]
    fn invalid_or_overflowed_products_refuse_instead_of_publishing_infinity() {
        for (a, b) in [
            (f64::MAX, 2.),
            (1., -1.),
            (f64::NAN, 1.),
            (1., f64::INFINITY),
        ] {
            assert!(scale(a, b).is_err());
        }
    }
}
