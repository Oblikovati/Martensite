//! Trihedrons: object trihedrons and their options, plane trihedrons on faces, the view
//! trihedron and the graduated trihedron.
mod datums;
mod object_trihedrons;
mod trihedron_options;
mod view_trihedrons;
pub(crate) use datums::{
    face_plane_trihedron, labelled_plane_trihedron, selected_circle_and_trihedron,
    shape_type_filter, single_mode_trihedron,
};
pub(crate) use object_trihedrons::{
    default_trihedron, hidden_label_selection, moved_box_detection, point_highlight,
};
pub(crate) use trihedron_options::trihedron_options;
pub(crate) use view_trihedrons::{
    graduated_options, graduated_sphere, scaled_view_trihedron, view_trihedron_positions,
    wireframe_view_trihedron,
};

/// What every trihedron program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::primitive_shapes::{box_sized, sphere};
    pub(super) use crate::session_run::{ImageControl, SessionRun, banner};
    pub(super) use martensite_tkernel::quantity::{Color, NamedColor};
    pub(super) use martensite_tkernel::standard::Handle;
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// Face `index` (1-based) of a shape; `face_of(&b, 1)`.
    pub(super) fn face_of(
        shape: &martensite_tkbrep::topods::Shape,
        index: usize,
    ) -> martensite_tkbrep::topods::Shape {
        use martensite_tkg3d::top_abs::ShapeEnum;
        crate::boolean_typed_explode::all_distinct(shape, ShapeEnum::Face)[index - 1].clone()
    }
}
