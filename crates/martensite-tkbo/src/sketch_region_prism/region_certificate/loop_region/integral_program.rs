// SPDX-License-Identifier: GPL-2.0-only
//! Explicit graph-local loans to source-bound Math integral programs.
mod binding;
mod error;
use binding::components;
pub(crate) use binding::{matches_boundary, matches_face};
pub(crate) use error::IntegralLedgerError;
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundaryIntegralProgramQuery, CertifiedIntegralProgram, FaceIntegralProgramQuery,
    IntegralProgramError, IntegralProgramLoop, IntegralProgramOptions, OrientedMetricCurve,
    certify_boundary_integral_program, certify_face_integral_program,
};

pub(crate) struct IntegralLedger {
    options: IntegralProgramOptions,
    consumed: u64,
    inconsistent_report: Option<u64>,
}
impl IntegralLedger {
    pub(crate) fn new(options: IntegralProgramOptions) -> Self {
        Self {
            options,
            consumed: 0,
            inconsistent_report: None,
        }
    }
    pub(crate) fn consumed(&self) -> u64 {
        self.consumed
    }
    pub(crate) fn options(&self) -> IntegralProgramOptions {
        self.options
    }
    pub(crate) fn boundary(
        &mut self,
        curves: &[OrientedMetricCurve<'_>],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<CertifiedIntegralProgram, IntegralLedgerError> {
        let options = self.loan()?;
        let parts = components(curves, options.integration.max_curves);
        let query = BoundaryIntegralProgramQuery {
            components: &parts,
            options,
        };
        self.settle(certify_boundary_integral_program(query, cancelled))
    }
    pub(crate) fn face(
        &mut self,
        loops: &[&[OrientedMetricCurve<'_>]],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<CertifiedIntegralProgram, IntegralLedgerError> {
        let options = self.loan()?;
        let parts: Vec<_> = loops
            .iter()
            .map(|curves| components(curves, options.integration.max_curves))
            .collect();
        let loops: Vec<_> = parts
            .iter()
            .map(|components| IntegralProgramLoop { components })
            .collect();
        let query = FaceIntegralProgramQuery {
            loops: &loops,
            options,
        };
        self.settle(certify_face_integral_program(query, cancelled))
    }
    fn loan(&self) -> Result<IntegralProgramOptions, IntegralLedgerError> {
        if let Some(reported) = self.inconsistent_report {
            return Err(self.accounting(reported));
        }
        let remaining = self
            .options
            .max_total_subintervals
            .checked_sub(self.consumed)
            .ok_or_else(|| self.accounting(0))?;
        Ok(IntegralProgramOptions {
            max_total_subintervals: remaining,
            ..self.options
        })
    }
    fn record(&mut self, reported: u64) -> Result<(), IntegralLedgerError> {
        let combined = self.consumed.checked_add(reported);
        if let Some(consumed) =
            combined.filter(|total| *total <= self.options.max_total_subintervals)
        {
            self.consumed = consumed;
            return Ok(());
        }
        self.inconsistent_report = Some(reported);
        Err(self.accounting(reported))
    }
    fn settle(
        &mut self,
        result: Result<CertifiedIntegralProgram, IntegralProgramError>,
    ) -> Result<CertifiedIntegralProgram, IntegralLedgerError> {
        match result {
            Ok(proof) => {
                self.record(proof.integration_subintervals())?;
                Ok(proof)
            }
            Err(cause) => {
                // The original mathematical failure remains primary; record poisons any inconsistent later loan.
                let _accounting = self.record(cause.integration_subintervals);
                Err(IntegralLedgerError::Math(Box::new(cause)))
            }
        }
    }
    fn accounting(&self, reported: u64) -> IntegralLedgerError {
        IntegralLedgerError::Accounting {
            grant: self.options.max_total_subintervals,
            consumed: self.consumed,
            reported,
        }
    }
}
#[cfg(test)]
mod tests;
