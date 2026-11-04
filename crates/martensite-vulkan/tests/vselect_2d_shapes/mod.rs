//! Full arbitrary-shape 2D persistence with all authored compound objects and placements.
mod geometry;
mod scene;
use crate::case_view::viewer_session::ViewerSession;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use std::error::Error;
type Outcome = Result<(), Box<dyn Error>>;

const SHAPES_BANNER: &[&str] = &[
    "========",
    "Allow 3D objects with Graphic3dTMF_2d flag",
    "========",
];

/// Execute all five placements and the final named-color predicate; `persistent_shapes()?`.
pub(crate) fn persistent_shapes() -> Outcome {
    banner(SHAPES_BANNER);
    let shapes = geometry::source_shapes()?;
    let mut run = SessionRun::new()?;
    scene::initial(run.session(), &shapes)?;
    run.dump("bug27796_center.png")?;
    corner_images(&mut run)?;
    run.session().move_cursor(110, 385)?;
    run.session().select_point(110, 385)?;
    if run.session().pixel_color_name(110, 385)? != "GRAY89" {
        println!("Error: wrong object is highlighted");
    }
    run.dump("bug27796.png")?;
    run.finish()
}

fn corner_images(run: &mut SessionRun) -> Outcome {
    for (corner, image) in [
        (Corner::LEFT_UPPER, "bug27796_tl.png"),
        (Corner::RIGHT_UPPER, "bug27796_tr.png"),
        (Corner::RIGHT_LOWER, "bug27796_br.png"),
        (Corner::LEFT_LOWER, "bug27796_bl.png"),
    ] {
        scene::reanchor(run.session(), corner)?;
        run.dump(image)?;
    }
    Ok(())
}

#[cfg(test)]
mod controls;
