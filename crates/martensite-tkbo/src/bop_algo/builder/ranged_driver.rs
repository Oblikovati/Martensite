//! Source-weighted private orchestration retains the selected actual receiver and child outcome.
use super::perform::{BuilderHooks, perform_on_using};
use crate::bop_algo::{BOPAlgoPaveFiller, ProgressCompletion};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;

/// Selects fixed native operation weights, never caller-supplied progress arithmetic.
/// Example: `BuilderProgressProfile::MakerVolume { intersect: false }` reserves0.5/9.5.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BuilderProgressProfile {
    GeneralFuse,
    Splitter,
    Section,
    Boolean {
        operation: crate::bop_algo::Operation,
    },
    MakerVolume {
        intersect: bool,
    },
}

impl BuilderProgressProfile {
    fn scope_name(self) -> &'static str {
        match self {
            Self::GeneralFuse | Self::Section => "Performing General Fuse operation",
            Self::Splitter => "Performing Split operation",
            Self::Boolean { .. } => "Performing Boolean operation",
            Self::MakerVolume { .. } => "Performing MakeVolume operation",
        }
    }

    fn intersection_steps(self) -> f64 {
        match self {
            Self::MakerVolume { intersect: false } => 0.5,
            Self::GeneralFuse
            | Self::Section
            | Self::Splitter
            | Self::Boolean { .. }
            | Self::MakerVolume { intersect: true } => 9.0,
        }
    }
}

/// Actual lower receiver entry; its explicit completion is forwarded without reinterpretation.
pub(crate) type RangedIntersector =
    fn(&mut BOPAlgoPaveFiller, &ProgressRange<'_>) -> ProgressCompletion;

/// Actual-H builder callback keeps native report/poll decisions at the owning stage.
pub(crate) type RangedBuilder<H> = fn(
    &mut H,
    &mut BOPAlgoPaveFiller,
    ProgressCompletion,
    &ProgressRange<'_>,
) -> Result<ProgressCompletion, Failure>;

/// Allocates native child ranges while preserving the existing pipeline and typed stop reason.
/// Example: `perform_on_progress(hooks, shapes, range, profile, intersect, build)`.
/// Closing a child advances presentation only; it never changes the returned completion.
pub(crate) fn perform_on_progress<H: BuilderHooks>(
    hooks: &mut H,
    shapes: List<Shape>,
    range: &ProgressRange<'_>,
    profile: BuilderProgressProfile,
    intersect: RangedIntersector,
    build: RangedBuilder<H>,
) -> Result<ProgressCompletion, Failure> {
    let scope = ProgressScope::with_literal(range, profile.scope_name(), 10.0, false);
    let interference_steps = profile.intersection_steps();
    perform_on_using(
        hooks,
        shapes,
        |pf| intersect(pf, &scope.next(interference_steps)),
        |h, pf, status| build(h, pf, status, &scope.next(10.0 - interference_steps)),
    )
}

#[cfg(test)]
#[path = "ranged_driver_contract.rs"]
mod contract;
