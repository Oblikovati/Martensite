// SPDX-License-Identifier: GPL-2.0-only
//! Give operation-local proof wrappers a standard cause chain.
use super::*;
use crate::surface_sketch::wrapped_profile::{planar, subdivision};

type Source<'a> = Option<&'a (dyn Error + 'static)>;

impl Error for selection::SelectionFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::Admission { cause, .. } => Some(cause),
            Self::Region { cause, .. } | Self::Capture(cause) => Some(cause.as_ref()),
            Self::Directed(cause) => Some(cause.as_ref()),
            Self::Planning { cause, .. } => Some(cause.as_ref()),
            Self::Clearance(cause) => Some(cause.as_ref()),
            Self::Material(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl Error for selection::MaterialFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::Admission(cause) => Some(cause.as_ref()),
            Self::Clearance(cause) => Some(cause.as_ref()),
            Self::Probe(cause) => Some(cause.as_ref()),
            Self::Winding { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl Error for planar::PlanarFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::WorkCapture(cause) => Some(cause.as_ref()),
            #[cfg(test)]
            Self::Capture(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl Error for subdivision::PlanningFailure {
    fn source(&self) -> Source<'_> {
        Some(self.source_cause())
    }
}

impl Error for planar_clearance::ClearanceFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::Authored(cause) => Some(cause.as_ref()),
            Self::Sources(cause) => Some(cause.as_ref()),
            Self::Certificate { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}

impl Error for planar_cones::ConeFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::Authored(cause) => Some(cause.as_ref()),
            Self::Interval { cause, .. } => Some(cause),
            Self::Certificate { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

impl Error for history::HistoryFailure {
    fn source(&self) -> Source<'_> {
        match self {
            Self::Clearance(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
