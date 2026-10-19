//! Owned adaptors keep persistent curve-curve calculations free of self-borrows.
use super::ExtCC;
use crate::extrema::curve_reference::CurveReference;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;

impl ExtCC<'_> {
    /// Retains one adaptor without clearing previously calculated results.
    /// `extrema.set_curve_shared(1, &curve)?;`
    pub fn set_curve_shared(
        &mut self,
        rank: i32,
        curve: &Handle<dyn Adaptor3dCurve>,
    ) -> Result<(), Failure> {
        let index = Self::rank_index("Extrema_ExtCC::SetCurve()", rank)?;
        self.curves[index] = Some(CurveReference::owned(curve.clone())?);
        Ok(())
    }

    /// Retains an adaptor and its range in the same order as SetCurve then SetRange.
    /// `extrema.set_curve_range_shared(2, &curve, 0.0, 1.0)?;`
    pub fn set_curve_range_shared(
        &mut self,
        rank: i32,
        curve: &Handle<dyn Adaptor3dCurve>,
        uinf: f64,
        usup: f64,
    ) -> Result<(), Failure> {
        self.set_curve_shared(rank, curve)?;
        self.set_range(rank, uinf, usup)
    }
}
