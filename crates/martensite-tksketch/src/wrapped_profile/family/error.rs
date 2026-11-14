use super::*;
#[derive(Debug)]
pub(in super::super) enum FamilyFailure {
    Selection(Box<selection::SelectionFailure>),
    Native {
        region: Option<SurfaceRegionId>,
        cause: Box<Failure>,
    },
    Encoding(Box<crate::surface_sketch::SurfaceFailureCause>),
    Coverage(Box<coverage::CoverageFailure>),
    Member(Box<Refusal>),
    Binding(Box<planar_clearance::ClearanceFailure>),
    History(Box<history::HistoryFailure>),
}
impl std::fmt::Display for FamilyFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Selection(cause) => write!(f, "family selection: {cause}"),
            Self::Native { region, cause } => write!(f, "family member={region:?}: {cause}"),
            Self::Encoding(cause) => write!(f, "family encoding: {cause:?}"),
            Self::Coverage(cause) => write!(f, "{cause}"),
            Self::Member(cause) => write!(
                f,
                "family region={:?} stage={:?}: {}",
                cause.region, cause.stage, cause.cause
            ),
            Self::History(cause) => write!(f, "family {cause}"),
            Self::Binding(cause) => write!(f, "family retained binding: {cause}"),
        }
    }
}
