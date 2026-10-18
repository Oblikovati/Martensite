//! Precision reads and error writes retain their caller-storage order.
pub(crate) trait TruncationPrecisionAccess {
    fn precision(&self) -> f64;
    fn set_error(&mut self, value: f64);
}

pub(super) struct BorrowedTruncationPrecision<'a> {
    pub(super) precision: f64,
    pub(super) error: &'a mut f64,
}
impl TruncationPrecisionAccess for BorrowedTruncationPrecision<'_> {
    fn precision(&self) -> f64 {
        self.precision
    }
    fn set_error(&mut self, value: f64) {
        *self.error = value;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_precision_keeps_the_existing_value_and_output_independent() {
        let mut error = 7.;
        let mut access = BorrowedTruncationPrecision {
            precision: 1e-6,
            error: &mut error,
        };
        access.set_error(0.);
        assert_eq!(access.precision(), 1e-6);
        access.set_error(0.25);
        assert_eq!(error, 0.25);
    }
}

struct IndexedTruncationPrecision<'a> {
    values: &'a mut [f64],
    precision_slot: usize,
    error_slot: usize,
}
impl<'a> IndexedTruncationPrecision<'a> {
    fn new(
        values: &'a mut [f64],
        precision_slot: usize,
        error_slot: usize,
    ) -> Result<Self, martensite_tkernel::standard::Failure> {
        if precision_slot >= values.len() || error_slot >= values.len() {
            return Err(martensite_tkernel::standard::Failure::with_message(
                "Standard_OutOfRange",
                &format!(
                    "mmtrpjj_with_precision_slots_: precision_slot={precision_slot}, error_slot={error_slot}; expected both indices in 0..{} initialized real slots",
                    values.len()
                ),
            ));
        }
        Ok(Self {
            values,
            precision_slot,
            error_slot,
        })
    }
}
impl TruncationPrecisionAccess for IndexedTruncationPrecision<'_> {
    fn precision(&self) -> f64 {
        self.values[self.precision_slot]
    }
    fn set_error(&mut self, value: f64) {
        self.values[self.error_slot] = value;
    }
}

/// Truncates with live precision/error identities in initialized caller storage.
/// Coefficients and per-component bounds remain separate slices; this does not
/// represent their intersections with either selected scalar slot.
///
/// ```
/// use martensite_tkgeombase::adv_app2var::math_base::mmtrpjj_with_precision_slots_;
/// let mut slots = vec![1e-6];
/// let (mut bounds, mut count) = ([7.], 0);
/// mmtrpjj_with_precision_slots_(3, 1, 3, -1, &[1., 0., 1e-8],
///     &mut bounds, &mut slots, 0, 0, &mut count).unwrap();
/// assert_eq!(count, 3);
/// ```
pub fn mmtrpjj_with_precision_slots_(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    iordre: i32,
    crvlgd: &[f64],
    ycvmax: &mut [f64],
    initialized_reals: &mut [f64],
    precision_slot: usize,
    error_slot: usize,
    ncfnew: &mut i32,
) -> Result<i32, martensite_tkernel::standard::Failure> {
    let mut access =
        IndexedTruncationPrecision::new(initialized_reals, precision_slot, error_slot)?;
    Ok(super::mmtrpjj_with_precision_access(
        ncofmx,
        ndimen,
        ncoeff,
        iordre,
        crvlgd,
        ycvmax,
        &mut access,
        ncfnew,
    ))
}

#[cfg(test)]
#[path = "truncation_precision_tests.rs"]
mod indexed_tests;
