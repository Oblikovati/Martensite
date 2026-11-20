//! Format the owned payloads; formatting groups do not replace typed projection.
use super::*;
use std::fmt::{Display, Formatter, Result};
type Message<'a> = (&'static str, &'a dyn Display);
impl Display for ConeCause {
    fn fmt(&self, out: &mut Formatter<'_>) -> Result {
        if let Some((label, cause)) = self.native_message().or_else(|| self.math_message()) {
            return write!(out, "{label}: {cause}");
        }
        self.context_message(out)
    }
}
impl ConeCause {
    fn native_message(&self) -> Option<Message<'_>> {
        match self {
            Budget(cause) => Some(("publication budget", cause.as_ref())),
            Native(cause) => Some(("native construction", cause.as_ref())),
            ActualBinding(cause) => Some(("actual conical native binding", cause.as_ref())),
            Storage(cause) => Some(("retained storage", cause.as_ref())),
            Coverage(cause) => Some(("retained coverage", cause.as_ref())),
            History(cause) => Some(("source history", cause.as_ref())),
            _ => self.source_message(),
        }
    }
    fn source_message(&self) -> Option<Message<'_>> {
        match self {
            Planar(cause) => Some(("authored planar material", cause.as_ref())),
            Candidate(cause) => Some(("candidate fit", cause)),
            Coefficients(cause) => Some(("candidate coefficients", cause)),
            _ => None,
        }
    }
    fn math_message(&self) -> Option<Message<'_>> {
        match self {
            Boundary(cause) => Some(("source boundary", cause.as_ref())),
            Radius(cause) => Some(("axis clearance", cause.as_ref())),
            Sector(cause) => Some(("single-turn sector", cause.as_ref())),
            Domain(cause) => Some(("source-bound chart domain", cause.as_ref())),
            ChartPlan(cause) => Some(("local chart planning", cause.as_ref())),
            Joins(cause) => Some(("source joins", cause.as_ref())),
            ChartJoins(cause) => Some(("chart source joins", cause.as_ref())),
            TrimPair(cause) => Some(("analytic cap trim pair", cause.as_ref())),
            ChartTrimPair(cause) => Some(("chart analytic cap trim pair", cause.as_ref())),
            _ => self.extended_math_message(),
        }
    }
    fn extended_math_message(&self) -> Option<Message<'_>> {
        match self {
            ActualPatch(cause) | ActualPair { cause, .. } => {
                Some(("complete actual conical patch", cause.as_ref()))
            }
            DirectedRows(cause) => Some(("checked directed boundary rows", cause.as_ref())),
            Clearance(_) | Flux(_) | Authored(_) => self.integral_message(),
            Material(_) | DevelopedMaterial(_) | Reference(_) | ReferenceVolume(_) => {
                self.material_message()
            }
            _ => None,
        }
    }
    fn integral_message(&self) -> Option<Message<'_>> {
        match self {
            Clearance(cause) => Some(("nonadjacent clearance", cause.as_ref())),
            Flux(cause) => Some(("represented boundary flux", cause.as_ref())),
            Authored(cause) => Some(("authored normal volume", cause.as_ref())),
            _ => None,
        }
    }
    fn material_message(&self) -> Option<Message<'_>> {
        match self {
            Material(cause) => Some(("filled UV material", cause.as_ref())),
            DevelopedMaterial(cause) => {
                Some(("source-bound developed UV material", cause.as_ref()))
            }
            Reference(cause) => Some(("reference body", cause.as_ref())),
            ReferenceVolume(cause) => Some(("reference normal volume", cause.as_ref())),
            _ => None,
        }
    }
    fn context_message(&self, out: &mut Formatter<'_>) -> Result {
        match self {
            Support(_)
            | ProofOptions(_)
            | InputBoundProofOptions { .. }
            | ExecutionProof(_)
            | Depths(_)
            | SourceAnchor { .. } => self.admission_message(out),
            Scalar { .. } => self.scalar_message(out),
            Limit { .. } => self.limit_message(out),
            Missing(region) => write!(out, "region={region:?}; expected one retained owner"),
            NativeQualification(cause) => qualification(out, cause),
            Encoding(cause) => write!(out, "frozen encoding: {cause:?}"),
            ActualValidation(cause) => write!(
                out,
                "ordered native validation={cause:?}; expected a valid report under the selected check policy"
            ),
            Binding { .. } => self.binding_message(out),
            Cancelled(point) => write!(out, "cancelled at {point:?}"),
            _ => write!(out, "{self:?}"),
        }
    }
    fn admission_message(&self, out: &mut Formatter<'_>) -> Result {
        match self {
            Support(actual) => expected(out, "support", actual, "canonical apex/axis/radial frame"),
            ProofOptions(actual) => proof_policy(out, actual),
            InputBoundProofOptions { actual, ceiling } => write!(
                out,
                "input-bound proof policy={actual:?}; expected precision 8..4096, aggregate arithmetic <={ceiling}, finite integration and authored-volume settings"
            ),
            ExecutionProof(actual) => expected(
                out,
                "directed planar proof",
                actual,
                "explicit FilteredCompositeBernsteinV5 selected-material authority",
            ),
            Depths(actual) => depths(out, actual),
            SourceAnchor { adjustment } => anchor(out, adjustment),
            _ => write!(out, "{self:?}"),
        }
    }
    fn scalar_message(&self, out: &mut Formatter<'_>) -> Result {
        match self {
            Scalar {
                field,
                actual,
                expected,
            } => scalar(out, field, *actual, expected),
            _ => write!(out, "{self:?}"),
        }
    }
    fn limit_message(&self, out: &mut Formatter<'_>) -> Result {
        match self {
            Limit {
                field,
                actual,
                limit,
            } => write!(out, "{field}={actual}; expected at most {limit}"),
            _ => write!(out, "{self:?}"),
        }
    }
    fn binding_message(&self, out: &mut Formatter<'_>) -> Result {
        match self {
            Binding {
                field,
                actual,
                expected,
            } => binding(out, field, actual, expected),
            _ => write!(out, "{self:?}"),
        }
    }
}
fn expected(
    out: &mut Formatter<'_>,
    field: &str,
    actual: &dyn std::fmt::Debug,
    expected: &str,
) -> Result {
    write!(out, "{field}={actual:?}; expected {expected}")
}
fn scalar(out: &mut Formatter<'_>, field: &str, actual: f64, expected: &str) -> Result {
    write!(out, "{field}={actual}; expected {expected}")
}
fn qualification(out: &mut Formatter<'_>, cause: &Refusal) -> Result {
    write!(
        out,
        "native qualification stage={:?} region={:?}: {}",
        cause.stage, cause.region, cause.cause
    )
}
fn binding(out: &mut Formatter<'_>, field: &str, actual: &[u64], expected: &[u64]) -> Result {
    write!(out, "{field}={actual:?}; expected {expected:?}")
}

fn proof_policy(out: &mut Formatter<'_>, actual: &WrappedConeProofOptions) -> Result {
    expected(
        out,
        "proof policy",
        actual,
        "precision 8..4096, arithmetic <=1000000, finite integration and authored-volume settings",
    )
}
fn depths(out: &mut Formatter<'_>, actual: &[f64; 2]) -> Result {
    expected(
        out,
        "depths",
        actual,
        "increasing finite caller-relative depths",
    )
}
fn anchor(out: &mut Formatter<'_>, actual: &Option<i8>) -> Result {
    expected(out, "anchor adjustment", actual, "certified zero")
}
