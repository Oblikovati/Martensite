//! Preserve publicly downcastable mathematical and native causes through the private adapter.
use super::*;
pub(super) fn cause(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    mathematical(cause).or_else(|| native_source(cause))
}
fn native_source(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        Budget(cause) => Some(cause.as_ref()),
        Native(cause) => Some(cause.as_ref()),
        ActualBinding(cause) => Some(cause.as_ref()),
        DirectedCap(_) => None,
        NativeQualification(cause) => native_qualification(cause),
        Storage(cause) => Some(cause.as_ref()),
        Planar(cause) => planar(cause),
        Boundary(_) | Radius(_) | Sector(_) | Domain(_) | Joins(_) | ChartJoins(_)
        | TrimPair(_) | ChartTrimPair(_) => None,
        Clearance(_) | Flux(_) | Authored(_) | Material(_) | DevelopedMaterial(_)
        | Reference(_) => None,
        ReferenceVolume(_)
        | Support(_)
        | ProofOptions(_)
        | InputBoundProofOptions { .. }
        | ExecutionProof(_)
        | Depths(_) => None,
        SourceAnchor { .. } | Scalar { .. } | Limit { .. } | Missing(_) => None,
        Encoding(_) | Coverage(_) | History(_) | Candidate(_) | ActualValidation(_) => None,
        Coefficients(_) | Binding { .. } | Cancelled(_) => None,
        ActualPatch(_) | ActualPair { .. } | DirectedRows(_) => None,
        ChartPlan(_) | Atlas(_) => None,
    }
}
fn mathematical(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        Boundary(cause) => Some(cause.as_ref()),
        Radius(cause) => Some(cause.as_ref()),
        Sector(cause) => Some(cause.as_ref()),
        Domain(cause) => Some(cause.as_ref()),
        ChartPlan(cause) => Some(cause.as_ref()),
        Atlas(cause) => Some(cause.as_ref()),
        Joins(cause) => Some(cause.as_ref()),
        ChartJoins(cause) => Some(cause.as_ref()),
        TrimPair(cause) => Some(cause.as_ref()),
        ChartTrimPair(cause) => Some(cause.as_ref()),
        _ => extended_mathematical(cause),
    }
}
fn extended_mathematical(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        ActualPatch(cause) | ActualPair { cause, .. } => Some(cause.as_ref()),
        DirectedRows(cause) => Some(cause.as_ref()),
        DirectedCap(cause) => Some(cause.as_ref()),
        Clearance(_) | Flux(_) | Authored(_) => integral_source(cause),
        Material(_) | DevelopedMaterial(_) | Reference(_) | ReferenceVolume(_) => {
            material_source(cause)
        }
        Boundary(_) | Radius(_) | Sector(_) | Domain(_) | Joins(_) | ChartJoins(_)
        | TrimPair(_) | ChartTrimPair(_) => None,
        Budget(_)
        | Native(_)
        | NativeQualification(_)
        | Storage(_)
        | Planar(_)
        | ActualBinding(_)
        | ActualValidation(_) => None,
        Support(_)
        | ProofOptions(_)
        | InputBoundProofOptions { .. }
        | ExecutionProof(_)
        | Depths(_)
        | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Encoding(_) | Coverage(_) => None,
        History(_) | Candidate(_) | Coefficients(_) | Binding { .. } | Cancelled(_) => None,
        ChartPlan(_) | Atlas(_) => None,
    }
}

fn planar(cause: &planar::PlanarFailure) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        planar::PlanarFailure::WorkCapture(cause) => Some(cause.as_ref()),
        #[cfg(test)]
        planar::PlanarFailure::Capture(cause) => Some(cause.as_ref()),
        planar::PlanarFailure::Selection { .. }
        | planar::PlanarFailure::SplitSource { .. }
        | planar::PlanarFailure::NonAuthoredVertex(_)
        | planar::PlanarFailure::MissingVertex(_)
        | planar::PlanarFailure::EmptyMaterial
        | planar::PlanarFailure::Join(_) => None,
    }
}
fn native_qualification(cause: &Refusal) -> Option<&(dyn std::error::Error + 'static)> {
    match &cause.cause {
        RefusalCause::Native(cause) => Some(cause.as_ref()),
        RefusalCause::Storage(cause) => Some(cause.as_ref()),
        RefusalCause::Coverage(_) | RefusalCause::Cones(_) | RefusalCause::NormalMeasure(_) => None,
        #[cfg(test)]
        RefusalCause::Planning(_) | RefusalCause::Planar(_) | RefusalCause::Clearance(_) => None,
    }
}

fn material_source(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        Material(cause) => Some(cause.as_ref()),
        DevelopedMaterial(cause) => Some(cause.as_ref()),
        Reference(cause) => Some(cause.as_ref()),
        ReferenceVolume(cause) => Some(cause.as_ref()),
        _ => None,
    }
}

fn integral_source(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        Clearance(cause) => Some(cause.as_ref()),
        Flux(cause) => Some(cause.as_ref()),
        Authored(cause) => Some(cause.as_ref()),
        _ => None,
    }
}
