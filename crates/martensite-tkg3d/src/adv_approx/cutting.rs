//! How an approximation splits an interval whose error is too large:
//! `AdvApproxCutting` and its two retained strategies.
use crate::geom::errors;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::arrays::rebased;

// Minimum length of a parametric interval: 10 * Precision::PConfusion() (retained `lgmin`).
fn lgmin() -> f64 {
    10. * precision::p_confusion()
}

/// A strategy that chooses where to cut `[a, b]` (`AdvApproxCutting`).
pub trait AdvApproxCutting {
    /// The cutting parameter, and whether the cut is accepted; the value is
    /// computed even when refused (`Value(a, b, cuttingvalue)`);
    /// `let (accepted, cut) = tool.value(0.0, 1.0);`.
    fn value(&self, a: f64, b: f64) -> (bool, f64);
}

/// Cut in the middle while the interval is long enough (`AdvApproxDichoCutting`);
/// `AdvApproxDichoCutting.value(0.0, 1.0) == (true, 0.5)`.
#[derive(Clone, Copy, Debug, Default)]
pub struct AdvApproxDichoCutting;

impl AdvApproxDichoCutting {
    /// The strategy (`AdvApproxDichoCutting()`); `AdvApproxDichoCutting::new()`.
    pub fn new() -> Self {
        Self
    }
}

impl AdvApproxCutting for AdvApproxDichoCutting {
    fn value(&self, a: f64, b: f64) -> (bool, f64) {
        let cuttingvalue: f64 = (a + b) / 2.;
        ((b - a).abs() >= 2. * lgmin(), cuttingvalue)
    }
}

/// Cut at a preferred point near the middle, else at a recommended point, else
/// in the middle (`AdvApproxPrefAndRec`);
/// `AdvApproxPrefAndRec::new(&recommended, &preferred, 5.0)?`.
#[derive(Clone, Debug)]
pub struct AdvApproxPrefAndRec {
    rec_cutting: Array1<'static, f64>,
    pref_cutting: Array1<'static, f64>,
    weight: f64,
}

impl AdvApproxPrefAndRec {
    /// Copies the cut tables (rebased to 1); StandardDomainError when
    /// `weight <= 1` (`AdvApproxPrefAndRec(RecomendedCut, PrefferedCut, Weight = 5)`);
    /// `AdvApproxPrefAndRec::new(&rec, &pref, 5.0)?`.
    pub fn new(
        recommended_cut: &Array1<'_, f64>,
        preferred_cut: &Array1<'_, f64>,
        weight: f64,
    ) -> Result<Self, Failure> {
        let tool = Self {
            rec_cutting: rebased(recommended_cut)?,
            pref_cutting: rebased(preferred_cut)?,
            weight,
        };
        if tool.weight <= 1. {
            return Err(errors::domain(&format!(
                "PrefAndRec : Weight is too small: {weight}; expected Weight > 1"
            )));
        }
        Ok(tool)
    }

    // The preferred point nearest the middle, when one lies within the weighted band.
    fn preferred(&self, a: f64, b: f64, mil: f64) -> Option<f64> {
        let mut dist: f64 = ((a * self.weight + b) / (1. + self.weight) - mil).abs();
        let mut cut: Option<f64> = None;
        for &point in self.pref_cutting.as_slice() {
            if dist > (mil - point).abs() {
                cut = Some(point);
                dist = (mil - point).abs();
            }
        }
        cut
    }

    // The recommended point nearest the middle, strictly inside the interval.
    fn recommended(&self, a: f64, b: f64, mil: f64) -> f64 {
        let mut dist: f64 = ((a - b) / 2.).abs();
        let mut cut: f64 = mil;
        for &point in self.rec_cutting.as_slice() {
            if (dist - lgmin()) > (mil - point).abs() {
                cut = point;
                dist = (mil - cut).abs();
            }
        }
        cut
    }
}

impl AdvApproxCutting for AdvApproxPrefAndRec {
    fn value(&self, a: f64, b: f64) -> (bool, f64) {
        let mil: f64 = (a + b) / 2.;
        let cut: f64 = match self.preferred(a, b, mil) {
            Some(cut) => cut,
            None => self.recommended(a, b, mil),
        };
        (
            (cut - a).abs() >= lgmin() && (b - cut).abs() >= lgmin(),
            cut,
        )
    }
}
