// SPDX-License-Identifier: GPL-2.0-only
//! Ordered straight-line wire recipes and detached native consumer exports.
mod failure;
mod native;
mod recipe;

pub use failure::{WireFailure, WirePhase};
pub use native::{DetachedWire, NativeEdgeBinding, NativePointBinding, NativeWireValidation};
pub use recipe::{WireRecipe, WireRecipeValidation, WireSegment};

use crate::{LineId, PointId};

/// Direction of a use relative to the line's authored order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineDirection {
    /// Ascending parameter from the authored first point.
    Forward,
    /// Descending parameter from the authored last point.
    Reversed,
}

impl LineDirection {
    fn endpoints(self, authored: [PointId; 2]) -> [PointId; 2] {
        match self {
            Self::Forward => authored,
            Self::Reversed => [authored[1], authored[0]],
        }
    }
}

/// Ordered use of a sketch line. Each line can occur once in a wire recipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectedLineUse {
    /// Authored line and stable occurrence identity within this recipe.
    pub line: LineId,
    /// Direction of the occurrence.
    pub direction: LineDirection,
}

/// Required connectivity at the ends of the ordered wire.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WireClosure {
    /// The final point and first point have different identities.
    Open,
    /// The final point and first point have the same identity.
    Closed,
}

fn poll(cancelled: &mut impl FnMut() -> bool, phase: WirePhase) -> Result<(), WireFailure> {
    if cancelled() {
        return Err(WireFailure::Cancelled(phase));
    }
    Ok(())
}
