//! Exact source selection queries; topology bindings are distinct from displayed AIS identities.
mod cases;
#[cfg(test)]
mod controls;
mod geometry;
mod objects;
pub(crate) use objects::state_text;
mod predicates;
mod selected;
mod topology;
mod wire;

use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
pub(crate) use cases::{
    line_shape_discrimination, local_and_neutral_rows, selected_topology_roundtrip,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions::program_error};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisSelectedOwner};
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

fn query_named_object(
    session: &ViewerSession,
    name: &str,
) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    session.object(name).cloned().ok_or_else(|| {
        program_error(&format!(
            "selection query object {name:?}; expected a live named AIS binding"
        ))
    })
}
