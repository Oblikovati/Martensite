//! Whole hover cases share the existing real session, picker and image contracts.
mod cases;
mod circle;
#[cfg(test)]
mod controls;
mod cycle;
mod marker;
mod occlusion;
mod pixel_contract;

// This single existing source also supplies unrelated axis helpers to its original host.
#[allow(dead_code)]
#[path = "../../vselect_quadric_cases/datum_objects.rs"]
mod reused_datums;

use crate::case_view::viewer_session::{DisplayRequest, ViewParameters, ViewerSession};
use crate::primitive_shapes::{box_at, box_sized};
use crate::session_run::{SessionRun, banner, check_point};
pub(crate) use cases::{free_edge_camera_pick, occluded_box_projections, overlapping_box_cycle};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions::program_error};
use martensite_tkv3d::ais::AisInteractiveObject;
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

fn hover_object(
    session: &ViewerSession,
    name: &str,
) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    session.object(name).cloned().ok_or_else(|| {
        program_error(&format!(
            "hover operand {name:?}; expected an actual displayed binding"
        ))
    })
}
