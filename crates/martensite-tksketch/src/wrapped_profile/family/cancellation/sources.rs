//! Shared cylinder failure causes are owned beside the family operation (#526).
use super::*;
use crate::surface_sketch::wrapped_profile as wrapped;

impl CancellationCause for wrapped::planar::PlanarFailure {
    fn cancelled(&self) -> bool {
        match self {
            #[cfg(test)]
            Self::Capture(..) => false,
            Self::WorkCapture(cause) => cause.is_cancelled(),
            Self::Selection { .. }
            | Self::SplitSource { .. }
            | Self::NonAuthoredVertex(..)
            | Self::EmptyMaterial
            | Self::Join(..) => false,
        }
    }
}

impl CancellationCause for wrapped::selection::material::MaterialFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Policy(..) | Self::Source(..) | Self::Members { .. } => false,
            Self::Cancelled => true,
            Self::Admission(cause) => cause.cancelled(),
            Self::Clearance(cause) => cause.cancelled(),
            Self::Probe(cause) => cause.is_cancelled(),
            Self::Limit { .. } => false,
            Self::Winding { cause, .. } => cause.is_cancelled(),
            Self::NonJordan { .. } | Self::Overlap { .. } => false,
        }
    }
}

impl CancellationCause for wrapped::selection::SelectionFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Cancelled => true,
            Self::Material(cause) => cause.cancelled(),
            Self::Limit { .. } | Self::Admission { .. } => false,
            Self::Region { cause, .. } => cause.cancelled(),
            Self::Capture(cause) => cause.cancelled(),
            Self::Planning { cause, .. } => cause.source_cause().is_cancelled(),
            Self::Clearance(cause) => cause.cancelled(),
            Self::DuplicateRegion(..)
            | Self::DuplicateCurve { .. }
            | Self::DuplicateVertex { .. }
            | Self::Context { .. } => false,
        }
    }
}

impl CancellationCause for wrapped::planar_clearance::ClearanceFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Authored(cause) => cause.is_cancelled(),
            Self::EmptySelection
            | Self::OriginalBinding
            | Self::SelectionBinding { .. }
            | Self::DuplicateSource(..) => false,
            Self::Sources(cause) => cause.cancelled(),
            Self::Certificate { cause, .. } => cause.is_cancelled(),
            Self::Binding { .. } => false,
        }
    }
}

impl CancellationCause for wrapped::planar_cones::ConeFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::Authored(cause) => cause.is_cancelled(),
            Self::Binding { .. } | Self::CapJoin { .. } | Self::Membership { .. } => false,
            Self::Interval { cause, .. } => cause.is_cancelled(),
            Self::Certificate { cause, .. } => cause.is_cancelled(),
        }
    }
}

impl CancellationCause for wrapped::history::HistoryFailure {
    fn cancelled(&self) -> bool {
        match self {
            Self::SourceBinding => false,
            Self::Clearance(cause) => cause.cancelled(),
            Self::DuplicateTarget(..) | Self::Corners { .. } | Self::Targets { .. } => false,
        }
    }
}
