//! Fitting and aspect regressions over explicitly displayed box subshapes.
mod aspects;
mod fitting;

pub(crate) use aspects::edge_type_and_hollow_interior;
pub(crate) use fitting::{box_edges_fit, box_vertices_fit, shrunk_box_bounds_redisplay};

mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::primitive_shapes::box_sized;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;
}
