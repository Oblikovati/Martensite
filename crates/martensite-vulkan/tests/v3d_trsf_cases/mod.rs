//! Local transformations, instances and assemblies, and transform persistence.
mod assembly_edits;
mod connected;
mod labelled_assembly;
mod labels;
mod persistent;
mod persistent_picking;
mod plane_size;
mod screen_boxes;
pub(crate) use assembly_edits::{instance_exact_hidden_lines, nested_assembly_edits};
pub(crate) use connected::{
    assembly_after_reopened_view, moved_instance_chain, selected_assembly, transformed_assemblies,
};
pub(crate) use labelled_assembly::labelled_assembly;
pub(crate) use labels::{aligned_zoomable_labels, flipped_label_selection, flipped_plane_labels};
pub(crate) use persistent::{
    centered_trihedron_box, stereo_corner_trihedrons, trihedron_face_highlight,
};
pub(crate) use persistent_picking::{persistent_box_selection, zoom_persistent_pick_depth};
pub(crate) use plane_size::plane_minimum_size;
pub(crate) use screen_boxes::screen_boxes_detection;

/// What every transformation program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::{
        ConnectedPart, DisplayRequest, corner_persistence,
    };
    pub(super) use crate::primitive_shapes::{box_at, box_sized};
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
