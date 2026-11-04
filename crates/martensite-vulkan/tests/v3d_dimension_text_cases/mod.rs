//! Dimension text and relations: 2d and 3d text highlighted in a second viewer, flipped 3d
//! text, label positions and arrow orientations, moved text, custom text values, and the
//! relations between selected sub-shapes.
#[path = "../support/dimension_input.rs"]
mod dimension_input;
#[path = "../support/dimension_params.rs"]
mod dimension_params;
#[path = "../support/dimension_request.rs"]
mod dimension_request;
mod flipped_text;
mod highlighted_text;
#[path = "../support/identic_relation_controls.rs"]
mod identic_relation_controls;
mod local_selection;
mod moved_text;
mod relations;
mod shaped_lengths;
mod text_layout;
mod valued_text;
pub(crate) use dimension_request::DimensionKind;
pub(crate) use flipped_text::{FlipAxis, flipped_radius};
pub(crate) use highlighted_text::{angle_text_highlight, circle_text_highlight};
pub(crate) use local_selection::local_part_highlight;
pub(crate) use moved_text::{moved_angle_text, moved_diameter_text};
pub(crate) use relations::{
    concentric_circles, equal_distances, equal_radii, fixed_edge, identic_shapes, offset_faces,
    parallel_edges, perpendicular_edges, symmetric_shapes,
};
pub(crate) use shaped_lengths::{face_edge_lengths, face_point_length};
pub(crate) use text_layout::{angle_layouts, circle_layouts, length_layouts};
pub(crate) use valued_text::{custom_text_values, fixed_size_arrows};

/// What every dimension text program uses.
mod prelude {
    pub(super) use super::dimension_input::{display_point, register_sans_font};
    pub(super) use super::dimension_params::{
        DimensionParams, DimensionStyle, set_dimension_params,
    };
    pub(super) use super::dimension_request::{
        DimensionKind, DimensionRequest, Measured, WorkingPlane, display_dimension,
    };
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// The outline circle through three displayed points.
    /// Example: `display_circle(&mut run, "c", [[0.; 3], [1., 1., 0.], [2., 0., 0.]])?`.
    pub(super) fn display_circle(
        run: &mut SessionRun,
        name: &str,
        points: [[f64; 3]; 3],
    ) -> Outcome {
        use crate::viewer_objects::{circle_object, circle_through};
        let circle = circle_through(points)?;
        run.session()
            .display_object(name, circle_object(&circle, false, None))?;
        Ok(())
    }

    /// Check the pixel colour at (x, y) and print the case's message when it fails.
    pub(super) fn check_or_report(
        run: &mut SessionRun,
        (x, y): (i32, i32),
        rgb: [f64; 3],
        error: &str,
    ) -> Outcome {
        if !run.session().check_color(x, y, rgb)? {
            println!("{error}");
        }
        Ok(())
    }

    /// The operands of a dimension over displayed objects; `objects(&["p1", "p2"])`.
    pub(super) fn objects<'a>(names: &[&'a str]) -> Vec<Measured<'a>> {
        names.iter().map(|name| Measured::Object(name)).collect()
    }

    /// The standard banner of the dimension cases: the `title` lines between rules.
    pub(super) fn dimension_banner(title: &[&str]) {
        let rule = "============";
        let lines: Vec<&str> = [rule]
            .into_iter()
            .chain(title.iter().copied())
            .chain([rule, ""])
            .collect();
        banner(&lines);
    }
}
