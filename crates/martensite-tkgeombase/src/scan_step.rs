// Rust addition: the guard of the retained float-step scans
// `for (t = First; t <= Last; t += Step)` (ProjLib, Extrema, BRepTools)

//! The increment of a retained float-step scan, refused where the retained loop stalls.
//!
//! Rust deviation: the geometry contract samples parameter ranges with loops such as
//! `for (T = First; T <= Last; T += Step)` or `while (T < Last) { T += Step; ... }`.
//! When the increment does not move `T` the loop state never changes, so the retained
//! call never returns: a zero-length range gives `Step = 0`, a range of a few ulps a
//! step that rounds back to `T`, a huge parameter a step below its half ulp, and an
//! infinite bound `T = +inf` that stays `+inf`. Rounding is monotone, so an increment
//! that moves `T` always moves it towards the loop exit, and a NaN leaves every such
//! loop at once; the first increment that leaves `T` unchanged is therefore exactly
//! where the endless loop starts. [`next_scan_parameter`] refuses it there with a
//! domain failure; every scan that ends natively takes the same samples.
use crate::native_failure::failure;
use martensite_tkernel::standard::Failure;

/// `par + step`, or `None` when the increment leaves `par` unchanged (the retained loop
/// would then repeat its last sample forever).
///
/// ```
/// use martensite_tkgeombase::scan_step::advanced_scan_parameter;
/// assert_eq!(advanced_scan_parameter(1.0, 0.5), Some(1.5));
/// assert_eq!(advanced_scan_parameter(1.0, 0.0), None);
/// assert_eq!(advanced_scan_parameter(f64::INFINITY, 1.0), None);
/// ```
pub fn advanced_scan_parameter(par: f64, step: f64) -> Option<f64> {
    let next: f64 = par + step;
    // `!=` and not `>`: a NaN is a new state that ends the retained loop, and a
    // descending scan passes a negative step.
    (next != par).then_some(next)
}

/// The next sample `par + step` of the scan of `range` by the retained `owner`, else
/// domain failure naming the owner, the range, the step and the parameter.
///
/// ```
/// use martensite_tkgeombase::scan_step::next_scan_parameter;
/// let next = next_scan_parameter("ComputeApprox", 0.2, 0.1, [0.0, 1.0]);
/// assert_eq!(next.unwrap(), 0.30000000000000004);
/// let stalled = next_scan_parameter("ComputeApprox", 1.0, 0.0, [1.0, 1.0]);
/// assert!(stalled.unwrap_err().message().contains("never ends"));
/// ```
pub fn next_scan_parameter(
    owner: &str,
    par: f64,
    step: f64,
    range: [f64; 2],
) -> Result<f64, Failure> {
    advanced_scan_parameter(par, step).ok_or_else(|| {
        failure(
            "Standard_DomainError",
            &format!(
                "{owner}: the scan of the range [{}, {}] with the step {step} does not move \
                 past the parameter {par}, so the native sampling loop never ends; expected \
                 a range long enough for the step to advance",
                range[0], range[1]
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_that_move_are_kept_bit_for_bit() {
        let (first, last) = (0.25, 1.75);
        let step: f64 = (last - first) / 10.;
        let mut samples: Vec<f64> = Vec::new();
        let mut t: f64 = first;
        while t <= last {
            samples.push(t);
            t = next_scan_parameter("test", t, step, [first, last]).unwrap();
        }
        let mut native: Vec<f64> = Vec::new();
        let mut u: f64 = first;
        while u <= last {
            native.push(u);
            u += step;
        }
        assert_eq!(samples, native);
    }

    #[test]
    fn stalls_are_refused_and_nan_ends_the_scan() {
        let one_ulp: f64 = 1.0 + f64::EPSILON;
        let step: f64 = (one_ulp - 1.0) / 10.;
        let failure = next_scan_parameter("IsoIsDeg", 1.0, step, [1.0, one_ulp]).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(
            failure
                .message()
                .starts_with("IsoIsDeg: the scan of the range [1, ")
        );
        assert!(advanced_scan_parameter(1e20, 1.0).is_none());
        assert!(
            advanced_scan_parameter(f64::NEG_INFINITY, f64::INFINITY)
                .unwrap()
                .is_nan()
        );
        assert_eq!(advanced_scan_parameter(2.0, -0.5), Some(1.5));
    }
}
