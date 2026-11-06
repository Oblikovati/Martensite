//! `transformed_circle`: four filled circles in tilted planes, seen from below; the highlight
//! colour of the detected circle and the selection images (surface normals, then depth with
//! the sensitive entities displayed).
use super::circles::{circle_banner, show_circle};
use super::prelude::*;
use crate::session_run::SessionRun;
use martensite_tkv3d::std_select::StdSelectTypeOfSelectionImage as SelectionImage;

/// Point names, circle name and the three points of each filled circle.
const TILTED_CIRCLES: [([&str; 4], [[f64; 3]; 3]); 4] = [
    (
        ["C1P1", "C1P2", "C1P3", "c1"],
        [[10., 10., 10.], [60., 60., -10.], [35., 100., 10.]],
    ),
    (
        ["C2P1", "C2P2", "C2P3", "c2"],
        [[10., -10., -10.], [60., -60., 10.], [35., -100., 10.]],
    ),
    (
        ["C3P1", "C3P2", "C3P3", "c3"],
        [[-80., -10., 10.], [-140., -50., -10.], [-110., -100., -10.]],
    ),
    (
        ["C4P1", "C4P2", "C4P3", "c4"],
        [[-80., 10., 10.], [-140., 60., 10.], [-110., 100., -10.]],
    ),
];

/// `transformed_circle`: the detected circle is drawn TURQUOISE, then both selection images.
pub(crate) fn transformed_circle_images() -> Outcome {
    circle_banner();
    let mut run = SessionRun::new()?;
    for (names, points) in TILTED_CIRCLES {
        show_circle(&mut run, names, points, true, None)?;
    }
    run.session()
        .set_standard_projection(Orientation::ZUP_BOTTOM)?;
    run.session().fit()?;
    run.session().move_cursor(100, 100)?;
    if run.session().pixel_color_name(100, 100)? != "TURQUOISE" {
        println!("ERROR: the circle should be highlighted");
    }
    run.session().move_cursor(200, 200)?;
    run.selection_dump(
        "transformed_circle_selnorm.png",
        SelectionImage::SURFACE_NORMAL,
    )?;
    run.session().display_active_sensitive()?;
    run.untyped_selection_dump("transformed_circle.png")?;
    run.finish_without_image()
}
