//! Truncation bounds and coefficients retain their original shared allocation.
use super::CurveProjectionFrame;
use crate::adv_app2var::math_base::{TruncationPrecisionAccess, TruncationRealAccess, mzsnorm_};
use martensite_tkernel::standard::Failure;

pub(super) struct CurveTruncationReals<'view, Precision> {
    workspace: &'view mut CurveProjectionFrame,
    column: usize,
    precision: &'view mut Precision,
}
impl<'view, Precision> CurveTruncationReals<'view, Precision> {
    pub(super) fn new(
        workspace: &'view mut CurveProjectionFrame,
        column: usize,
        precision: &'view mut Precision,
    ) -> Self {
        Self {
            workspace,
            column,
            precision,
        }
    }
}
impl<Precision: TruncationPrecisionAccess> TruncationRealAccess
    for CurveTruncationReals<'_, Precision>
{
    fn coefficient(&self, index: i32) -> Result<f64, Failure> {
        Ok(self
            .workspace
            .coefficient_offset(self.column as i64 + i64::from(index)))
    }
    fn bound(&self, index: i32) -> Result<f64, Failure> {
        Ok(self.workspace.truncation_bound(index))
    }
    fn set_bound(&mut self, index: i32, value: f64) -> Result<(), Failure> {
        self.workspace.set_truncation_bound(index, value);
        Ok(())
    }
    fn bound_norm(&mut self, dimensions: i32) -> Result<f64, Failure> {
        Ok(mzsnorm_(dimensions, self.workspace.truncation_bounds()))
    }
    fn precision(&self) -> Result<f64, Failure> {
        Ok(self.precision.precision())
    }
    fn set_error(&mut self, value: f64) -> Result<(), Failure> {
        self.precision.set_error(value);
        Ok(())
    }
}
