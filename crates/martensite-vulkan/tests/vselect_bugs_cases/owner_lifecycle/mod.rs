//! Whole direct-context selection-owner regressions.
#[cfg(test)]
mod controls;
mod objects;
mod obsolete_shape;
mod removed_detection;
mod selected_object;

pub(crate) use obsolete_shape::obsolete_shape_owner;
pub(crate) use removed_detection::removed_object_detection;
pub(crate) use selected_object::direct_selected_object;

use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, exceptions::program_error};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

fn lifecycle_begin(border: &str, title: &str) -> Result<SessionRun, Box<dyn Error>> {
    banner(&[border, title, border, ""]);
    // Each isolated worker reaches VInit without a preceding VClear command.
    SessionRun::new()
}
