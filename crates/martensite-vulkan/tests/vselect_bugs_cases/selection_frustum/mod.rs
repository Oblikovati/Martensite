//! Complete ranked-wire and perspective-frustum cases, with source text comparisons.
#[cfg(test)]
mod controls;
mod grid;
mod persistent_point;
mod perspective;
mod state_rows;
mod wire_depth;

pub(crate) use persistent_point::persistent_ranked_points;
pub(crate) use perspective::{perspective_front_point, perspective_grid_regions};
pub(crate) use wire_depth::wire_edge_characteristics;

use super::prelude::*;
use crate::case_view::viewer_session::{ViewParameters, ViewerSession};
use crate::viewer_objects::point_object;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle, exceptions::program_error},
    storage::format_general,
};
use martensite_tkv3d::ais::AisInteractiveObject;

type Outcome = Result<(), Box<dyn Error>>;
// Bound presentation owners stay confined to this invocation's existing ViewerSession.
type FrustumBindings = Vec<(String, Handle<dyn AisInteractiveObject>)>;

fn frustum_operand(
    session: &ViewerSession,
    name: &str,
) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    session.object(name).cloned().ok_or_else(|| {
        program_error(&format!(
            "frustum operand {name:?}; expected a displayed bound object"
        ))
    })
}

fn frustum_bound_name(
    object: &Handle<dyn AisInteractiveObject>,
    bindings: &FrustumBindings,
) -> Result<String, Failure> {
    bindings
        .iter()
        .find(|(_, bound)| bound == object)
        .map(|(name, _)| name.clone())
        .ok_or_else(|| {
            program_error(&format!(
                "frustum owner {:?}; expected one of the actual named operands",
                object.object_address()
            ))
        })
}
