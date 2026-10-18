//! Count storage uses the same initialized integer owner as caller scalar roles.
use super::*;
fn missing_count_origin() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: coefficient origin=None; expected configured initialized caller coefficient-count storage",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Select an initialized count origin, retaining signed in-owner prefix access;
    /// `let frame=frame.with_coefficient_count_origin(64)?`.
    pub fn with_coefficient_count_origin(mut self, origin: usize) -> Result<Self, Failure> {
        self.integers.coefficient_slot(origin, 0)?;
        self.coefficient_origin = Some(origin);
        Ok(self)
    }
    /// Read a signed caller count offset after return or unwind;
    /// `frame.coefficient_count(-1)?` reads initialized padding before its origin.
    pub fn coefficient_count(&self, index: i32) -> Result<i32, Failure> {
        let origin = self.coefficient_origin.ok_or_else(missing_count_origin)?;
        let slot = self.integers.coefficient_slot(origin, index)?;
        Ok(self.integers.output_slot(slot))
    }
    pub(super) fn validate_count_mirror(&self, values: &[i32]) -> Result<(), Failure> {
        if self.coefficient_origin.is_none() {
            return Ok(());
        }
        for (index, value) in values.iter().enumerate() {
            let actual = self.coefficient_count(index as i32)?;
            if actual != *value {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarIsoOperationFrame: coefficient mirror[{index}]={value}; expected initialized caller value={actual}"
                    ),
                ));
            }
        }
        Ok(())
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    fn coefficient_slot(&self, index: i32) -> Result<usize, Failure> {
        let origin = self.coefficient_origin.ok_or_else(missing_count_origin)?;
        crate::adv_app2var::integer_frame::coefficient_slot(
            origin,
            index,
            self.integers.values.len(),
            self.integers.bindings,
        )
    }
    /// Read the actual initialized coefficient count, including scalar incidence;
    /// `call.coefficient_count(0)?`.
    pub fn coefficient_count(&self, index: i32) -> Result<i32, Failure> {
        Ok(self.integers.integer_slot(self.coefficient_slot(index)?))
    }
    /// Write through the actual caller integer identity with immediate visibility;
    /// `call.set_coefficient_count(0,1)?`.
    pub fn set_coefficient_count(&mut self, index: i32, value: i32) -> Result<(), Failure> {
        let slot = self.coefficient_slot(index)?;
        self.integers.set_integer_slot(slot, value);
        Ok(())
    }
}
impl OwnedCurveIntegers<'_, '_> {
    pub(super) fn count(&self, fallback: &[i32], index: i32) -> i32 {
        self.with_frame(|frame| {
            frame.coefficient_origin.map_or_else(|| fallback[index as usize], |_| frame.coefficient_count(index).expect(
                "AdvApp2VarIsoOperationFrame: coefficient index outside initialized owner; expected valid caller storage"
            ))
        })
    }
    pub(super) fn write_count(
        &self,
        fallback: &mut [i32],
        index: i32,
        value: i32,
        request: &mut IsoApproxRequest<'_>,
    ) {
        self.with_frame(|frame| {
            let Some(origin) = frame.coefficient_origin else { fallback[index as usize] = value; return; };
            let slot = frame.integers.coefficient_slot(origin, index).expect("AdvApp2VarIsoOperationFrame: coefficient index outside initialized owner; expected valid caller storage");
            frame.integers.set_output_slot(slot, value);
            frame.integers.refresh_iso_request(request);
        });
    }
}
