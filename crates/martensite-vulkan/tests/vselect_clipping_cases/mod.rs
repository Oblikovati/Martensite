//! Complete six-case clipping-selection family; original GPU observations stay explicit.
mod bug26973;
mod bug29791;
mod bug30695;
mod bug30777;
mod bug30906;
mod bug31650;
#[cfg(test)]
mod case_controls;
#[cfg(test)]
mod cpu_scene;
mod plane_context;
#[cfg(test)]
mod plane_context_tests;
mod selected_state;
pub(crate) use selected_state::occurrence_rank;
#[cfg(test)]
mod selected_state_tests;

pub(crate) use bug26973::four_plane_targets;
pub(crate) use bug29791::orthogonal_and_negative_depth;
pub(crate) use bug30695::rectangle_containment_overlap;
pub(crate) use bug30777::clipped_sphere_point;
pub(crate) use bug30906::override_does_not_leak;
pub(crate) use bug31650::shared_global_local_plane;

mod prelude {
    pub(super) use super::plane_context::ClippingPlanes;
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
    pub(super) use crate::primitive_shapes::{box_at, box_sized};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkernel::standard::{Failure, Handle};
    pub(super) use martensite_tkv3d::v3d::V3dTypeOfOrientation;
    pub(super) use std::error::Error;

    pub(super) fn shaded_highlighted() -> DisplayRequest {
        DisplayRequest::default()
            .with_display_mode(1)
            .with_highlight_mode(1)
    }

    pub(super) fn named_object(
        session: &ViewerSession,
        name: &str,
    ) -> Result<Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>, Failure> {
        session.object(name).cloned().ok_or_else(|| {
            Failure::with_message(
                "Standard_ProgramError",
                &format!("clipping object {name:?}; expected a displayed object"),
            )
        })
    }

    pub(super) fn object_plane(
        planes: &mut ClippingPlanes,
        session: &ViewerSession,
        name: &str,
        target: &str,
        override_global: bool,
    ) -> Result<(), Failure> {
        planes.on_object(name, &named_object(session, target)?, override_global)?;
        session.context().update_current_viewer();
        Ok(())
    }

    pub(super) fn pixel(
        session: &ViewerSession,
        x: i32,
        y: i32,
        color: &str,
        error: &str,
    ) -> Result<(), Failure> {
        if session.pixel_color_name(x, y)? != color {
            println!("{error}");
        }
        Ok(())
    }

    pub(super) fn count(session: &ViewerSession, expected: i32, error: &str) {
        if session.selected_count() != expected {
            println!("{error}");
        }
    }
}
