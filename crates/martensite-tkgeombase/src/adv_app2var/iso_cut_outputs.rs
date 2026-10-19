//! Cut-table identities share actual caller real slots with mapped scalar inputs.
use super::*;
fn missing_cuts() -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: cut origin=None; expected initialized owned cut parameter storage",
    )
}
impl AdvApp2VarIsoOperationFrame {
    /// Select a cut-table origin in initialized caller real storage;
    /// `let frame=frame.with_cut_parameter_origin(448)?`.
    pub fn with_cut_parameter_origin(mut self, origin: usize) -> Result<Self, Failure> {
        self.reals.output_slot(origin, 0)?;
        self.cut_origin = Some(origin);
        Ok(self)
    }
    /// Inspect actual signed cut storage after return or unwind; `frame.cut_parameter(-1)?`.
    pub fn cut_parameter(&self, index: i32) -> Result<f64, Failure> {
        self.reals
            .output_value(self.cut_origin.ok_or_else(missing_cuts)?, index)
    }
    pub(super) fn validate_cut_mirror(&self, values: &[f64]) -> Result<(), Failure> {
        if self.cut_origin.is_none() {
            return Ok(());
        }
        for (index, value) in values.iter().enumerate() {
            let actual = self.cut_parameter(index as i32)?;
            if actual.to_bits() != value.to_bits() {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarIsoOperationFrame: cut mirror[{index}] bits={:#x}; expected initialized caller bits={:#x}",
                        value.to_bits(),
                        actual.to_bits()
                    ),
                ));
            }
        }
        Ok(())
    }
}
impl<Parameters> AdvApp2VarCombinedEvaluation<'_, '_, Parameters> {
    /// Read a signed actual caller cut, including domain incidence; `call.cut_parameter(0)?`.
    pub fn cut_parameter(&self, index: i32) -> Result<f64, Failure> {
        self.reals
            .output_value(self.cut_origin.ok_or_else(missing_cuts)?, index)
    }
    /// Write a caller cut with immediate mapped-input visibility; `call.set_cut_parameter(0,0.)?`.
    pub fn set_cut_parameter(&mut self, index: i32, value: f64) -> Result<(), Failure> {
        self.reals
            .set_output_value(self.cut_origin.ok_or_else(missing_cuts)?, index, value)
    }
}
impl crate::adv_app2var::approx_f2var::curve_cut_storage::CurveCutParameters
    for OwnedCurveIntegers<'_, '_>
{
    fn cut_parameter(&self, values: &[f64], index: i32) -> f64 {
        self.with_frame(|frame| {
            frame.cut_origin.map_or_else(
                || values[index as usize],
                |_| {
                    frame.cut_parameter(index).expect(
            "AdvApp2VarIsoOperationFrame: invalid cut index; expected initialized caller storage")
                },
            )
        })
    }
    fn set_cut_parameter(
        &mut self,
        values: &mut [f64],
        index: i32,
        value: f64,
        request: &mut IsoApproxRequest<'_>,
    ) {
        self.with_frame(|frame|{
            let Some(origin)=frame.cut_origin else {values[index as usize]=value;return;};
            frame.reals.set_output_value(origin,index,value).expect("AdvApp2VarIsoOperationFrame: invalid cut write; expected initialized caller storage");
            frame.reals.write_request(request);
        });
    }
    fn shift_cut_parameters(
        &mut self,
        bytes: i32,
        values: &mut [f64],
        source: i32,
        target: i32,
        request: &mut IsoApproxRequest<'_>,
    ) {
        self.with_frame(|frame|{
            let Some(origin)=frame.cut_origin else {crate::adv_app2var::sys_base::mcrfill_within(bytes,values,source as usize,target as usize);return;};
            frame.reals.shift_output_values(origin,source,target,bytes).expect("AdvApp2VarIsoOperationFrame: invalid cut copy; expected initialized caller storage");
            frame.reals.write_request(request);
        });
    }
}
