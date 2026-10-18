//! Endpoint output identities are read and written in their initialized real owner.
use super::*;
use crate::adv_app2var::AdvApp2VarEndpointOutput as EndpointRole;
fn missing_endpoint() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: endpoint origins=None; expected initialized owned lower/upper endpoint storage",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Bind actual lower/upper endpoint origins, permitting the same initialized origin;
    /// `let frame=frame.with_endpoint_output_origins([320,384])?`.
    pub fn with_endpoint_output_origins(mut self, origins: [usize; 2]) -> Result<Self, Failure> {
        self.reals.configure_output_origins(origins)?;
        self.endpoint_origins = Some(origins);
        Ok(self)
    }
    /// Read actual endpoint storage after return or unwind; `frame.endpoint_output(AdvApp2VarEndpointOutput::Lower,0)?`.
    pub fn endpoint_output(&self, table: EndpointRole, index: i32) -> Result<f64, Failure> {
        let origins = self.endpoint_origins.ok_or_else(missing_endpoint)?;
        self.reals.output_value(origins[table.index()], index)
    }
    pub(super) fn validate_endpoint_mirrors(
        &self,
        lower: &[f64],
        upper: &[f64],
    ) -> Result<(), Failure> {
        self.validate_real_output_mirrors(
            self.endpoint_origins,
            [EndpointRole::Lower, EndpointRole::Upper],
            [lower, upper],
        )
    }
    pub(super) fn synchronize_endpoint_mirrors(
        &self,
        lower: &mut [f64],
        upper: &mut [f64],
    ) -> Result<(), Failure> {
        self.synchronize_real_output_mirrors(self.endpoint_origins, [lower, upper])
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read one actual endpoint value; `call.endpoint_output(AdvApp2VarEndpointOutput::Lower,0)?`.
    pub fn endpoint_output(&self, table: EndpointRole, index: i32) -> Result<f64, Failure> {
        let origins = self.endpoint_origins.ok_or_else(missing_endpoint)?;
        self.reals.output_value(origins[table.index()], index)
    }
    /// Write one shared endpoint value with immediate input visibility;
    /// `call.set_endpoint_output(AdvApp2VarEndpointOutput::Lower,0,2.)?`.
    pub fn set_endpoint_output(
        &mut self,
        table: EndpointRole,
        index: i32,
        value: f64,
    ) -> Result<(), Failure> {
        let origins = self.endpoint_origins.ok_or_else(missing_endpoint)?;
        self.reals
            .set_output_value(origins[table.index()], index, value)
    }
}
