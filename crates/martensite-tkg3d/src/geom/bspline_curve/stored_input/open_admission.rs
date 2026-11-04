//! Open support admission preserves stored evaluator metadata without identifying endpoints.
use super::{StoredSplineInput, admission};
use crate::geom::{ClosedSplineExtrusionError, ClosedSplineExtrusionLayout};
use martensite_tkmath::predicates::roots::open_extrusion::{
    OpenExtrusionInputError, OpenQuadraticExtrusionInput,
};

/// Immutable stored representation and its admitted open support; `input.numeric()`.
#[derive(Clone, Debug)]
pub struct StoredOpenExtrusionInput {
    source: StoredSplineInput,
    numeric: OpenQuadraticExtrusionInput,
    layout: ClosedSplineExtrusionLayout,
}
impl StoredOpenExtrusionInput {
    /// Read the original stored representation; `input.source().evaluation_override()` is false.
    pub fn source(&self) -> &StoredSplineInput {
        &self.source
    }
    /// Read the admitted numeric input without geometry lookup; `input.numeric().clone()`.
    pub fn numeric(&self) -> &OpenQuadraticExtrusionInput {
        &self.numeric
    }
    /// Read unchanged requested direction/domain and evaluator layout; `input.layout()`.
    pub fn layout(&self) -> &ClosedSplineExtrusionLayout {
        &self.layout
    }
}
/// Admission refusal preserves invalid storage versus unsupported evaluator distinctions.
#[derive(Clone, Debug, PartialEq)]
pub enum StoredOpenExtrusionFailure {
    /// An external evaluator prevents proof from stored coefficients.
    EvaluationOverride(ClosedSplineExtrusionLayout),
    /// Degree, periodicity or array indexing is unsupported.
    UnsupportedLayout(ClosedSplineExtrusionLayout),
    /// Shared stored multiplicity or flat-knot validation failed.
    Storage(ClosedSplineExtrusionError),
    /// Actual numeric values fail the open operation's input contract.
    Numeric(OpenExtrusionInputError),
    /// Stored evaluator bounds differ from the represented active knot interval.
    Domain {
        /// Actual bounds retained from the stored evaluator.
        actual: [f64; 2],
        /// Active knot interval required by the numeric operation.
        expected: [f64; 2],
    },
}
impl std::fmt::Display for StoredOpenExtrusionFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "stored open support={self:?}; expected coherent finite clamped quadratic storage"
        )
    }
}
impl std::error::Error for StoredOpenExtrusionFailure {}
impl StoredSplineInput {
    /// Admit without identifying curve endpoints; `stored.into_open_extrusion_input([0.,0.,1.], [0.,1.])?`.
    pub fn into_open_extrusion_input(
        self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<StoredOpenExtrusionInput, StoredOpenExtrusionFailure> {
        let layout = admitted_layout(&self, direction, interval)?;
        let numeric = numeric(&self, direction, interval)?;
        validate_storage(&self, &numeric)?;
        Ok(StoredOpenExtrusionInput {
            source: self,
            numeric,
            layout,
        })
    }
}
fn admitted_layout(
    source: &StoredSplineInput,
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<ClosedSplineExtrusionLayout, StoredOpenExtrusionFailure> {
    let layout = source.layout(direction, interval);
    if layout.basis_override {
        return Err(StoredOpenExtrusionFailure::EvaluationOverride(layout));
    }
    if !admission::supported(&layout) {
        return Err(StoredOpenExtrusionFailure::UnsupportedLayout(layout));
    }
    Ok(layout)
}

fn numeric(
    source: &StoredSplineInput,
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<OpenQuadraticExtrusionInput, StoredOpenExtrusionFailure> {
    let counts = admission::checked_multiplicities(&source.multiplicities)
        .map_err(StoredOpenExtrusionFailure::Storage)?;
    OpenQuadraticExtrusionInput::capture(
        source.poles.clone(),
        source.weights.clone(),
        source.knots.clone(),
        counts,
        direction,
        interval,
    )
    .map_err(StoredOpenExtrusionFailure::Numeric)
}
fn validate_storage(
    source: &StoredSplineInput,
    numeric: &OpenQuadraticExtrusionInput,
) -> Result<(), StoredOpenExtrusionFailure> {
    admission::validate_flat_sequence(
        numeric.knots(),
        numeric.multiplicities(),
        &source.flat_knots,
    )
    .map_err(StoredOpenExtrusionFailure::Storage)?;
    if !same_bits(&source.domain, &numeric.curve_interval()) {
        return Err(StoredOpenExtrusionFailure::Domain {
            actual: source.domain,
            expected: numeric.curve_interval(),
        });
    }
    Ok(())
}
fn same_bits(first: &[f64], second: &[f64]) -> bool {
    first
        .iter()
        .map(|v| v.to_bits())
        .eq(second.iter().map(|v| v.to_bits()))
}
#[cfg(test)]
mod tests;
