//! Completion describes ordinary finite-interval injectivity, with no endpoint quotient.
use super::super::closed_extrusion::ClosedExtrusionCompletion;
use super::{OpenExtrusionCause, OpenExtrusionProjectionContact};

/// A support-only proof, not represented-face acceptance; inspect `report.completion()`.
#[derive(Clone, Debug, PartialEq)]
pub enum OpenExtrusionCompletion {
    /// Every self and pair obligation on the entire interval was proved.
    Certified,
    /// A projected contact prevents this sufficient injectivity proof.
    NotExcluded(OpenExtrusionProjectionContact),
    /// A required obligation remains unfinished.
    Incomplete(OpenExtrusionCause),
}
impl From<&ClosedExtrusionCompletion> for OpenExtrusionCompletion {
    fn from(completion: &ClosedExtrusionCompletion) -> Self {
        match completion {
            ClosedExtrusionCompletion::Certified => Self::Certified,
            ClosedExtrusionCompletion::NotExcluded(contact) => Self::NotExcluded(*contact),
            ClosedExtrusionCompletion::Incomplete(cause) => Self::Incomplete(cause.clone()),
        }
    }
}
