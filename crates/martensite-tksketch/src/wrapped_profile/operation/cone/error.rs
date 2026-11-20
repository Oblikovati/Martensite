//! Cone refusals keep the actual source owner and original typed leaf cause.
use super::*;
use ConeCause::*;
mod display;
mod projection;
mod sources;
#[cfg(test)]
mod tests;
use crate::surface_sketch::HistorySource;
use crate::surface_sketch::wrapped_profile::math_cancellation::MathCancellation;
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum ConeCause {
    // Admission snapshots stay off the inline fit-refusal path.
    Support(Box<ConeSupport>),
    ProofOptions(Box<WrappedConeProofOptions>),
    InputBoundProofOptions {
        actual: Box<WrappedConeProofOptions>,
        ceiling: u32,
    },
    ExecutionProof(Box<martensite_tkbo::profile_extrusion::PlanarProofVersion>),
    Depths([f64; 2]),
    ActualPatch(Box<ConicalActualPatchError>),
    ActualPair {
        cause: Box<ConicalActualPatchError>,
        entities: Vec<HistorySource>,
    },
    DirectedRows(Box<ConicalBoundaryZRowError>),
    ActualBinding(Box<martensite_tkbo::body_check::ConicalBindingError>),
    ActualValidation(Box<martensite_tkbo::body_check::ValidationReport>),
    Material(Box<ConicalMaterialError>),
    DevelopedMaterial(Box<ConicalDevelopedMaterialError>),
    Reference(Box<ConicalReferenceError>),
    ReferenceVolume(Box<ConicalReferenceVolumeError>),
    Budget(Box<publication_budget::BudgetFailure>),
    SourceAnchor {
        adjustment: Option<i8>,
    },
    Scalar {
        field: &'static str,
        actual: f64,
        expected: &'static str,
    },
    Limit {
        field: &'static str,
        actual: u32,
        limit: u32,
    },
    Missing(SurfaceRegionId),
    Native(Box<Failure>),
    NativeQualification(Box<Refusal>),
    Storage(Box<crate::surface_sketch::wrapped_profile::storage::StorageFailure>),
    Encoding(crate::surface_sketch::failure::SurfaceFailureCause),
    Coverage(Box<coverage::CoverageFailure>),
    History(Box<history::HistoryFailure>),
    Planar(Box<planar::PlanarFailure>),
    Candidate(cone_mapping::fit::FitFailure),
    Coefficients(cone_mapping::ConeFitFailure),
    Boundary(Box<ConicalBoundaryError>),
    Radius(Box<ConeRequestError>),
    Sector(Box<ConicalSectorFailure>),
    Domain(Box<ConicalChartDomainError>),
    ChartPlan(Box<ConicalChartPlanError>),
    Atlas(Box<material::atlas::Refusal>),
    Joins(Box<ConicalJoinsFailure>),
    ChartJoins(Box<ConicalChartJoinsError>),
    TrimPair(Box<ConicalTrimPairError>),
    ChartTrimPair(Box<ConicalChartTrimPairError>),
    Clearance(Box<CylindricalClearanceError>),
    Flux(Box<AnalyticConicalFluxError>),
    Authored(Box<ConicalAuthoredFluxError>),
    Binding {
        field: &'static str,
        actual: Vec<u64>,
        expected: Vec<u64>,
    },
    Cancelled(family::Checkpoint),
}
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) struct ConeFailure {
    pub stage: WrappedStage,
    pub class: WrappedFailureClass,
    pub region: Option<SurfaceRegionId>,
    pub span: Option<CurveSpan>,
    pub cause: ConeCause,
}
impl ConeFailure {
    pub(super) fn new(
        region: Option<SurfaceRegionId>,
        span: Option<CurveSpan>,
        class: WrappedFailureClass,
        cause: ConeCause,
    ) -> Self {
        Self {
            stage: WrappedStage::Construction,
            class,
            region,
            span,
            cause,
        }
    }
    pub(super) fn at(mut self, stage: WrappedStage) -> Self {
        self.stage = stage;
        self
    }
    pub(super) fn public(self) -> WrappedNormalFailure {
        let (class, mut entities) = projection::context(&self);
        entities.extend(self.region.into_iter().map(HistorySource::Region));
        entities.extend(self.span.map(|span| HistorySource::Curve(span.curve)));
        let class = if self.cancelled() {
            WrappedFailureClass::Cancelled
        } else {
            class
        };
        super::super::error::with_entities(
            self.stage,
            class,
            entities,
            super::super::error::Cause::Cone(Box::new(self)),
        )
    }

    fn cancelled(&self) -> bool {
        match &self.cause {
            Cancelled(_) | Planar(_) | Storage(_) => self.capture_cancelled(),
            Material(_) | DevelopedMaterial(_) => self.material_cancelled(),
            ConeCause::Reference(cause) => cause.is_cancelled(),
            ConeCause::ReferenceVolume(cause) => reference_volume_cancelled(cause),
            ConeCause::Budget(cause) => budget_cancelled(cause),
            ActualPatch(_) | ActualPair { .. } | DirectedRows(_) => self.actual_cancelled(),
            Boundary(_) | Radius(_) | Sector(_) | Domain(_) | Joins(_) | ChartJoins(_)
            | TrimPair(_) | ChartTrimPair(_) => self.local_cancelled(),
            ChartPlan(cause) => cause.is_cancelled(),
            Atlas(cause) => cause.cancelled(),
            ConeCause::Clearance(cause) => cause.is_cancelled(),
            ConeCause::Flux(cause) => flux_cancelled(cause),
            ConeCause::Authored(cause) => cause.is_cancelled(),
            ProofOptions(_)
            | InputBoundProofOptions { .. }
            | ExecutionProof(_)
            | Support(_)
            | SourceAnchor { .. }
            | Depths(_) => false,
            Scalar { .. } | Limit { .. } | Missing(_) | Native(_) | Encoding(_) => false,
            NativeQualification(_) | Coverage(_) | History(_) | Candidate(_) => false,
            Coefficients(_) | Binding { .. } | ActualBinding(_) | ActualValidation(_) => false,
        }
    }
    fn material_cancelled(&self) -> bool {
        match &self.cause {
            Material(cause) => cause.cause.is_cancelled(),
            DevelopedMaterial(cause) => cause.is_cancelled(),
            _ => false,
        }
    }
    fn actual_cancelled(&self) -> bool {
        match &self.cause {
            ActualPatch(cause) | ActualPair { cause, .. } => cause.is_cancelled(),
            DirectedRows(cause) => cause.is_cancelled(),
            _ => false,
        }
    }
    fn local_cancelled(&self) -> bool {
        match &self.cause {
            Boundary(cause) => cause.is_cancelled(),
            Radius(cause) => cause.is_cancelled(),
            Sector(cause) => cause.is_cancelled(),
            Domain(cause) => cause.is_cancelled(),
            Joins(cause) => cause.is_cancelled(),
            ChartJoins(cause) => cause.is_cancelled(),
            TrimPair(cause) => cause.cause.cause.is_cancelled(),
            ChartTrimPair(cause) => cause.is_cancelled(),
            _ => false,
        }
    }
    fn capture_cancelled(&self) -> bool {
        match &self.cause {
            Cancelled(_) => true,
            Planar(cause) => cause.cancelled(),
            Storage(cause) => cause.cancelled(),
            _ => false,
        }
    }
}
impl super::super::error::ConeFailureCause for ConeFailure {
    fn planar_work_failure(
        &self,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarRegionWorkFailure> {
        if let ConeCause::Planar(cause) = &self.cause
            && let planar::PlanarFailure::WorkCapture(work) = cause.as_ref()
        {
            return Some(work.as_ref());
        }
        None
    }
}

impl From<family::Checkpoint> for ConeFailure {
    fn from(point: family::Checkpoint) -> Self {
        Self::new(
            point.region,
            None,
            WrappedFailureClass::Cancelled,
            ConeCause::Cancelled(point),
        )
    }
}
impl std::fmt::Display for ConeFailure {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            out,
            "cone region={:?} span={:?}: {}",
            self.region, self.span, self.cause
        )
    }
}
impl std::error::Error for ConeFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        sources::cause(&self.cause)
    }
}
fn flux_cancelled(cause: &AnalyticConicalFluxError) -> bool {
    use crate::surface_sketch::wrapped_profile::math_cancellation::MathCancellation;
    use AnalyticConicalFluxFailure as F;
    match &cause.cause {
        F::Arithmetic { cause, .. } => cause.is_cancelled(),
        F::Input { cause, .. } => cause.is_cancelled(),
        F::Coefficients { cause, .. } => cause.is_cancelled(),
        F::Cap { cause, .. } => cause.is_cancelled(),
        F::Integration { cause, .. } => cause.is_cancelled(),
        F::ArithmeticPolicy(_) | F::Carrier { .. } => false,
    }
}

fn reference_volume_cancelled(cause: &ConicalReferenceVolumeError) -> bool {
    match cause.cause.as_ref() {
        ConicalReferenceVolumeFailure::Input { .. }
        | ConicalReferenceVolumeFailure::Shape { .. }
        | ConicalReferenceVolumeFailure::Policy(_) => false,
        ConicalReferenceVolumeFailure::Arithmetic(cause) => cause.is_cancelled(),
        ConicalReferenceVolumeFailure::Trigonometric(cause) => cause.is_cancelled(),
        ConicalReferenceVolumeFailure::Integration { cause, .. } => cause.is_cancelled(),
    }
}

fn budget_cancelled(cause: &publication_budget::BudgetFailure) -> bool {
    use publication_budget::BudgetFailure as B;
    match cause {
        B::Cones { cause, .. } => cause.cancelled(),
        B::Empty | B::Policy { .. } | B::Owners { .. } | B::Inventory { .. } | B::Spans { .. } => {
            false
        }
        B::Source(_) | B::Coverage { .. } | B::Native { .. } | B::Invalid { .. } => false,
        B::Limit { .. } | B::Overflow { .. } | B::Clearance { .. } => false,
    }
}
