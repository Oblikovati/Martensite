//! Selection regressions: cursor detection, picking by point and area, selection modes.
//! Each program runs the regression commands in order on one viewer session.
pub(crate) mod checked;
pub(crate) mod edge;
pub(crate) mod edge_face;
pub(crate) mod edge_face_more;
pub(crate) mod edge_solid;
pub(crate) mod edge_solid_more;
pub(crate) mod face;
pub(crate) mod vertex;
pub(crate) mod vertex_edge;
pub(crate) mod vertex_face;
pub(crate) mod vertex_solid;
pub(crate) mod vertex_wire;
pub(crate) mod wire;
pub(crate) mod wire_solid;

/// What every selection regression program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, ViewParameters};
    pub(super) use crate::primitive_shapes::{box_at, box_sized};
    pub(super) use crate::session_run::SessionRun;
    pub(super) use martensite_tkv3d::ais::AisSelectionScheme;
    pub(super) use std::error::Error;
}

pub(crate) mod infinite_face;
