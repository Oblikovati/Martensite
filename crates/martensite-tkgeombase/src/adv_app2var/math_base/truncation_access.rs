//! One numerical pipeline can read either borrowed or common-owner real roles.
use super::{TruncationPrecisionAccess, mzsnorm_};
use martensite_tkernel::standard::Failure;

pub(crate) trait TruncationRealAccess {
    fn coefficient(&self, index: i32) -> Result<f64, Failure>;
    fn bound(&self, index: i32) -> Result<f64, Failure>;
    fn set_bound(&mut self, index: i32, value: f64) -> Result<(), Failure>;
    fn bound_norm(&mut self, dimensions: i32) -> Result<f64, Failure>;
    fn precision(&self) -> Result<f64, Failure>;
    fn set_error(&mut self, value: f64) -> Result<(), Failure>;
}

pub(super) struct BorrowedTruncationReals<'a, Precision> {
    coefficients: &'a [f64],
    bounds: &'a mut [f64],
    precision: &'a mut Precision,
}
impl<'a, Precision> BorrowedTruncationReals<'a, Precision> {
    pub(super) fn new(
        coefficients: &'a [f64],
        bounds: &'a mut [f64],
        precision: &'a mut Precision,
    ) -> Self {
        Self {
            coefficients,
            bounds,
            precision,
        }
    }
}
impl<Precision: TruncationPrecisionAccess> TruncationRealAccess
    for BorrowedTruncationReals<'_, Precision>
{
    fn coefficient(&self, index: i32) -> Result<f64, Failure> {
        Ok(self.coefficients[index as usize])
    }
    fn bound(&self, index: i32) -> Result<f64, Failure> {
        Ok(self.bounds[index as usize])
    }
    fn set_bound(&mut self, index: i32, value: f64) -> Result<(), Failure> {
        self.bounds[index as usize] = value;
        Ok(())
    }
    fn bound_norm(&mut self, dimensions: i32) -> Result<f64, Failure> {
        Ok(mzsnorm_(dimensions, self.bounds))
    }
    fn precision(&self) -> Result<f64, Failure> {
        Ok(self.precision.precision())
    }
    fn set_error(&mut self, value: f64) -> Result<(), Failure> {
        self.precision.set_error(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::truncation_precision::BorrowedTruncationPrecision;
    use super::*;
    #[test]
    fn borrowed_roles_preserve_reads_stores_and_existing_norm() {
        let mut error = 7.;
        let mut precision = BorrowedTruncationPrecision {
            precision: 0.25,
            error: &mut error,
        };
        let mut bounds = [3., 4.];
        let mut access = BorrowedTruncationReals::new(&[2.], &mut bounds, &mut precision);
        assert_eq!(access.coefficient(0).unwrap(), 2.);
        assert_eq!(access.bound(1).unwrap(), 4.);
        assert_eq!(access.bound_norm(2).unwrap(), 5.);
        access.set_bound(0, 0.).unwrap();
        assert_eq!(access.bound_norm(2).unwrap(), 4.);
        assert_eq!(access.precision().unwrap(), 0.25);
        access.set_error(0.125).unwrap();
        assert_eq!(error, 0.125);
    }
}
