use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::bilinear_patch::{
    BilinearCertificate, BilinearInconclusive, BilinearInputError, BilinearPatchInput,
};
use std::fmt;

/// The unsupported representation, without asserting geometric invalidity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BilinearSplineLayout {
    /// U and V polynomial degrees.
    pub degrees: [i32; 2],
    /// Numbers of poles in U and V.
    pub poles: [usize; 2],
    /// Numbers of stored weights in U and V.
    pub weights: [usize; 2],
    /// Whether an external evaluation representation replaces the stored net.
    pub evaluation_override: bool,
    /// Periodicity flags in U and V.
    pub periodic: [bool; 2],
    /// Distinct knot counts in U and V.
    pub knots: [usize; 2],
    /// Stored multiplicities in U and V.
    pub multiplicities: [Vec<i32>; 2],
}
/// A geometric representation that has not established self-exclusion.
#[derive(Clone, Debug, PartialEq)]
pub enum BilinearSplineInconclusive {
    /// Only a nonperiodic degree-1, 2x2, clamped single span is supported.
    UnsupportedLayout(BilinearSplineLayout),
    /// The stored net does not determine the externally supplied evaluator.
    EvaluationOverride(BilinearSplineLayout),
    /// An owned bilinear input did not establish an injective projection.
    Patch {
        /// The coherently copied numeric support.
        input: BilinearPatchInput,
        /// The admission or projection condition that was not established.
        reason: BilinearInconclusive,
    },
}
/// The conclusion for one coherently copied numeric support, never a live handle.
#[derive(Clone, Debug, PartialEq)]
pub enum BilinearSplineExclusion {
    /// The owned continuous support is globally injective on its rectangle.
    Certified(BilinearSplineCertificate),
    /// No exclusion certificate was established.
    Inconclusive(BilinearSplineInconclusive),
}
/// An owned polynomial proof and the representation admission that produced it.
#[derive(Clone, Debug, PartialEq)]
pub struct BilinearSplineCertificate {
    pub(super) polynomial: BilinearCertificate,
    pub(super) layout: BilinearSplineLayout,
}
impl BilinearSplineCertificate {
    /// Read the exact numeric support; `proof.input().corners()`.
    pub fn input(&self) -> &BilinearPatchInput {
        self.polynomial.input()
    }
    /// Read admission metadata, including override absence; `proof.layout()`.
    pub fn layout(&self) -> &BilinearSplineLayout {
        &self.layout
    }
    /// Read the owned mathematical certificate; `proof.polynomial()`.
    pub fn polynomial(&self) -> &BilinearCertificate {
        &self.polynomial
    }
}
/// Invalid numeric input or inconsistent storage, separate from inconclusiveness.
#[derive(Debug)]
pub enum BilinearSplineError {
    /// A supported patch failed numeric input validation.
    Input(BilinearInputError),
    /// A stored value is outside its required numeric domain.
    StoredValue {
        /// The stored numeric component being checked.
        component: &'static str,
        /// Zero-based component index.
        index: usize,
        /// Supplied value, including any nonfinite payload.
        value: f64,
        /// Whether the component also requires strict positivity.
        positive: bool,
    },
    /// Checked storage access failed.
    Storage(Failure),
}
impl fmt::Display for BilinearSplineError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(error) => write!(out, "{error}"),
            Self::Storage(error) => write!(out, "bilinear support storage: {error}"),
            Self::StoredValue {
                component,
                index,
                value,
                positive,
            } => write!(
                out,
                "{component} index {index} = {value}; expected {}finite values",
                if *positive { "positive " } else { "" }
            ),
        }
    }
}
impl std::error::Error for BilinearSplineError {}
