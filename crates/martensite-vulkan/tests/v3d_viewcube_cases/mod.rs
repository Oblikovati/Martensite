//! The view cube: clicks on sides, edges and corners that turn the camera, far-origin text,
//! orthographic persistence in a perspective view and per-view visibility.
mod clicks;
mod placement;
mod style;
pub(crate) use clicks::{default_cube_clicks, orthographic_cube_clicks};
pub(crate) use placement::{far_origin_cube, per_view_visibility};
pub(crate) use style::styled_cubes;

/// What every view cube program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::{DisplayRequest, ViewerSession};
    pub(super) use crate::primitive_shapes::box_sized;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkernel::standard::Handle;
    pub(super) use martensite_tkv3d::ais::*;
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;

    /// Print `message` when pixel (x, y) is (or, with `equal` false, is not) named `expected`.
    /// Example: `expect_pixel(session, (70, 340), "CYAN", true, "Error: ...")?`.
    pub(super) fn expect_pixel(
        session: &ViewerSession,
        (x, y): (i32, i32),
        expected: &str,
        equal: bool,
        message: &str,
    ) -> Outcome {
        if (session.pixel_color_name(x, y)? == expected) != equal {
            println!("{message}");
        }
        Ok(())
    }

    /// Display the cube `name` with its fixed animation loop and a zero animation duration,
    /// as the clicking regressions configure it; `animated_cube(session, false)?`.
    pub(super) fn instant_cube(
        session: &mut ViewerSession,
        orthographic: bool,
    ) -> Result<Handle<AisViewCube>, Box<dyn Error>> {
        let cube = session.view_cube("vc")?;
        cube.set_fixed_animation_loop(true);
        cube.set_duration(0.0)?;
        if orthographic {
            super::placement::persist_orthographic(&cube)?;
        }
        session.display_object("vc", cube.clone().cast())?;
        Ok(cube)
    }
}
