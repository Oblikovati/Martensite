//! Terminal transfer preserves the exact coordinator preparation/transform methods.
//! The released line constructor owns Rc-backed tools, so its kernel stays on the
//! worker. These owned fields are everything the existing postprocessors read;
//! this private terminal state is never used to restart an intersection.
use super::{Curve, FaceFace, GeomAdaptorSurface, Handle, PntOn2Faces};

/// Owns terminal postprocessing inputs after the worker's kernel and context drop.
/// Example: `let state = intersection.into_postprocessing();` before leaving a worker.
pub(crate) struct FaceFacePostprocessing {
    done: bool,
    tangent: bool,
    surfaces: [Handle<GeomAdaptorSurface>; 2],
    curves: Vec<Curve>,
    points: Vec<PntOn2Faces>,
}

impl FaceFace {
    /// Moves actual results and adaptor owners, without sending the worker-local kernel.
    /// Example: `let packet = intersection.into_postprocessing();`.
    pub(crate) fn into_postprocessing(self) -> FaceFacePostprocessing {
        FaceFacePostprocessing {
            done: self.done,
            tangent: self.tangent,
            surfaces: self.surfaces,
            curves: self.curves,
            points: self.points,
        }
    }
}

impl FaceFacePostprocessing {
    /// Restores only terminal state for unchanged curve preparation and transformation.
    /// Example: `let mut terminal = packet.into_intersector(); terminal.prepare_lines_3d(false)?;`.
    pub(crate) fn into_intersector(self) -> FaceFace {
        FaceFace {
            done: self.done,
            tangent: self.tangent,
            surfaces: self.surfaces,
            curves: self.curves,
            points: self.points,
            ..FaceFace::default()
        }
    }
}

#[cfg(test)]
#[path = "postprocessing_contract.rs"]
mod contract;
