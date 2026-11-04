//! Regression programs of the V3d bugs group; inputs, commands and checks follow the cases.
mod transform_persistence;
pub(crate) use transform_persistence::{
    zoom_persistent_anchor_moves, zoom_persistent_box_survives_view_resize,
    zoom_persistent_boxes_fit, zoom_persistent_normals_stay_unit,
};
