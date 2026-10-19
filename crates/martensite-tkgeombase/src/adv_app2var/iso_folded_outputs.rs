//! Folded output identities are read and written in their initialized real owner.
use super::*;
use crate::adv_app2var::AdvApp2VarFoldedOutput as Folded;
fn missing_folded() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: folded origins=None; expected initialized owned sum/difference storage",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Bind actual sum/difference origins, permitting the same initialized origin;
    /// `let frame=frame.with_folded_output_origins([192,192])?`.
    pub fn with_folded_output_origins(mut self, origins: [usize; 2]) -> Result<Self, Failure> {
        self.reals.configure_output_origins(origins)?;
        self.folded_origins = Some(origins);
        Ok(self)
    }
    /// Read actual folded storage after return or unwind; `frame.folded_output(Folded::SymmetricSum,0)?`.
    pub fn folded_output(&self, table: Folded, index: i32) -> Result<f64, Failure> {
        let origins = self.folded_origins.ok_or_else(missing_folded)?;
        self.reals.output_value(origins[table.index()], index)
    }
    pub(super) fn validate_folded_mirrors(
        &self,
        sum: &[f64],
        difference: &[f64],
    ) -> Result<(), Failure> {
        self.validate_real_output_mirrors(
            self.folded_origins,
            [Folded::SymmetricSum, Folded::AntisymmetricDifference],
            [sum, difference],
        )
    }
    pub(super) fn synchronize_folded_mirrors(
        &self,
        sum: &mut [f64],
        difference: &mut [f64],
    ) -> Result<(), Failure> {
        self.synchronize_real_output_mirrors(self.folded_origins, [sum, difference])
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read one actual folded sample; `call.folded_output(Folded::SymmetricSum,0)?`.
    pub fn folded_output(&self, table: Folded, index: i32) -> Result<f64, Failure> {
        let origins = self.folded_origins.ok_or_else(missing_folded)?;
        self.reals.output_value(origins[table.index()], index)
    }
    /// Write one shared folded sample with immediate input visibility;
    /// `call.set_folded_output(Folded::SymmetricSum,0,2.)?`.
    pub fn set_folded_output(
        &mut self,
        table: Folded,
        index: i32,
        value: f64,
    ) -> Result<(), Failure> {
        let origins = self.folded_origins.ok_or_else(missing_folded)?;
        self.reals
            .set_output_value(origins[table.index()], index, value)
    }
}
