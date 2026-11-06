//! Complete child-selection regressions, without synthetic owners.
#[cfg(test)]
mod controls;
mod parent_display;
mod recompute;

pub(crate) use parent_display::child_selected_through_parent;
pub(crate) use recompute::child_selection_recomputed;

use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::ais::{AisInteractiveContext, AisShape};
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

struct ChildSelectionScene {
    parent: Handle<AisShape>,
    children: Vec<Handle<AisShape>>,
}

fn child_selection_begin(border: &str, title: &str) -> Result<SessionRun, Box<dyn Error>> {
    banner(&[border, title, border, ""]);
    // Both cases reach VInit directly; their QA helpers require this real context.
    SessionRun::new()
}
