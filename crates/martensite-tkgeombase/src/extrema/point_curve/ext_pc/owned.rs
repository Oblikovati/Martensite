//! Shared curve lifetime for persistent point-curve extrema without call replay.
use super::GGExtPC;
use crate::extrema::curve_reference::CurveReference;
use crate::extrema::curve_tool::ExtremaCurveTool;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};

impl<T: ExtremaCurveTool> GGExtPC<'_, T>
where
    T::Curve: RuntimeTyped + Send + Sync,
{
    /// Retains the adaptor while applying the existing initialization state transition.
    /// `extrema.initialize_shared(&curve, 0.0, 1.0, 1e-10)?;`
    pub fn initialize_shared(
        &mut self,
        curve: &Handle<T::Curve>,
        uinf: f64,
        usup: f64,
        tol_f: f64,
    ) -> Result<(), Failure> {
        let reference = CurveReference::owned(curve.clone())?;
        self.initialize_reference(reference, uinf, usup, tol_f)
    }
}
