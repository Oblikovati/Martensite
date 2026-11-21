//! Preserve publicly downcastable mathematical and native causes through the private adapter.
use super::*;
pub(super) fn cause(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    if let Some(source) = mathematical(cause) {
        return Some(source);
    }
    match cause {
        Budget(cause) => Some(cause.as_ref()),
        Native(cause) => Some(cause.as_ref()),
        NativeQualification(cause) => native_qualification(cause),
        Storage(cause) => Some(cause.as_ref()),
        Planar(cause) => planar(cause),
        Boundary(_) | Radius(_) | Sector(_) | Joins(_) | TrimPair(_) => None,
        Clearance(_) | Flux(_) | Authored(_) | Material(_) | Reference(_) => None,
        ReferenceVolume(_) | Support(_) | ProofOptions(_) | Depths(_) => None,
        SourceAnchor { .. } | Scalar { .. } | Limit { .. } | Missing(_) => None,
        Encoding(_) | Coverage(_) | History(_) | Candidate(_) => None,
        Coefficients(_) | Binding { .. } | Cancelled(_) => None,
    }
}
fn mathematical(cause: &ConeCause) -> Option<&(dyn std::error::Error + 'static)> {
    match cause {
        Boundary(cause) => Some(cause.as_ref()),
        Radius(cause) => Some(cause.as_ref()),
        Sector(cause) => Some(cause.as_ref()),
        Joins(cause) => Some(cause.as_ref()),
        TrimPair(cause) => Some(cause.as_ref()),
        Clearance(cause) => Some(cause.as_ref()),
        Flux(cause) => Some(cause.as_ref()),
        Authored(cause) => Some(cause.as_ref()),
        Material(cause) => Some(cause.as_ref()),
        Reference(cause) => Some(cause.as_ref()),
        ReferenceVolume(cause) => Some(cause.as_ref()),
        Budget(_) | Native(_) | NativeQualification(_) | Storage(_) | Planar(_) => None,
        Support(_) | ProofOptions(_) | Depths(_) | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Encoding(_) | Coverage(_) => None,
        History(_) | Candidate(_) | Coefficients(_) | Binding { .. } | Cancelled(_) => None,
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
