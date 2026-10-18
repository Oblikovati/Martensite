//! Error output identities are read and written in their initialized real owner.
use super::*;
use crate::adv_app2var::AdvApp2VarErrorOutput as ErrorRole;
fn missing_error() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: error origins=None; expected initialized owned maximum/RMS storage",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Bind actual maximum/RMS origins, permitting the same initialized origin;
    /// `let frame=frame.with_error_output_origins([512,576])?`.
    pub fn with_error_output_origins(mut self, origins: [usize; 2]) -> Result<Self, Failure> {
        self.reals.configure_output_origins(origins)?;
        self.error_origins = Some(origins);
        Ok(self)
    }
    /// Read actual error storage after return or unwind; `frame.error_output(AdvApp2VarErrorOutput::Maximum,0)?`.
    pub fn error_output(&self, table: ErrorRole, index: i32) -> Result<f64, Failure> {
        let origins = self.error_origins.ok_or_else(missing_error)?;
        self.reals.output_value(origins[table.index()], index)
    }
    pub(super) fn validate_error_mirrors(
        &self,
        maximum: &[f64],
        mean: &[f64],
    ) -> Result<(), Failure> {
        self.validate_real_output_mirrors(
            self.error_origins,
            [ErrorRole::Maximum, ErrorRole::RootMeanSquare],
            [maximum, mean],
        )
    }
    pub(super) fn synchronize_error_mirrors(
        &self,
        maximum: &mut [f64],
        mean: &mut [f64],
    ) -> Result<(), Failure> {
        self.synchronize_real_output_mirrors(self.error_origins, [maximum, mean])
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read one actual error value; `call.error_output(AdvApp2VarErrorOutput::Maximum,0)?`.
    pub fn error_output(&self, table: ErrorRole, index: i32) -> Result<f64, Failure> {
        let origins = self.error_origins.ok_or_else(missing_error)?;
        self.reals.output_value(origins[table.index()], index)
    }
    /// Write one shared error value with immediate input visibility;
    /// `call.set_error_output(AdvApp2VarErrorOutput::Maximum,0,2.)?`.
    pub fn set_error_output(
        &mut self,
        table: ErrorRole,
        index: i32,
        value: f64,
    ) -> Result<(), Failure> {
        let origins = self.error_origins.ok_or_else(missing_error)?;
        self.reals
            .set_output_value(origins[table.index()], index, value)
    }
}
