//! All twelve original polygon/rectangle operations, including the weak final conjunction.
use super::{
    source_checks::{has_cylinder, weak_hollow_error},
    transformed_input::hollow_faces,
};
use crate::{
    case_view::viewer_session::DisplayRequest,
    session_run::{SessionRun, banner},
};
use std::error::Error;

/// The original hollow face selection source, without an invented image; `hollow_cone_cyl()`.
pub(crate) fn hollow_cone_cyl() -> Result<(), Box<dyn Error>> {
    banner(&[
        "=================================",
        "0032547: Visualization, Select3DSensitiveCylinder - implement picking of a hollow cylinder",
        " ",
        "=================================",
    ]);
    let [cylinder_face, cone_face] = hollow_faces()?;
    let mut run = SessionRun::new()?;
    let shaded = DisplayRequest::default().with_display_mode(1);
    run.session().display("cyl_1", &cylinder_face, &shaded)?;
    run.session().display("cone_1", &cone_face, &shaded)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    region_probes(&mut run)?;
    crossing_probes(&mut run)?;
    run.finish_without_image()
}

fn region_probes(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    for region in [
        (60, 220, 140, 190, "cylinder"),
        (270, 210, 310, 160, "cone"),
    ] {
        region_pair(run, region)?;
    }
    Ok(())
}

fn region_pair(
    run: &mut SessionRun,
    (x1, y1, x2, y2, label): (i32, i32, i32, i32, &str),
) -> Result<(), Box<dyn Error>> {
    for polygon in [true, false] {
        let pixels = if polygon {
            vec![(x1, y1), (x2, y1), (x2, y2), (x1, y2)]
        } else {
            vec![(x1, y1), (x2, y2)]
        };
        region_modes(run, &pixels, label)?;
    }
    Ok(())
}

fn region_modes(
    run: &mut SessionRun,
    pixels: &[(i32, i32)],
    label: &str,
) -> Result<(), Box<dyn Error>> {
    for overlap in [false, true] {
        run.session().select_pixels(pixels, overlap)?;
        if has_cylinder(&run.session().picked_entity_state_types()?) {
            println!("Error: {label} should not be detected");
        }
    }
    Ok(())
}

fn crossing_probes(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    for pixels in [
        vec![(60, 210), (310, 210), (310, 200), (60, 200)],
        vec![(60, 210), (310, 200)],
    ] {
        run.session().select_pixels(&pixels, false)?;
        if has_cylinder(&run.session().picked_entity_state_types()?) {
            println!("Error: cone should not be detected");
        }
        run.session().select_pixels(&pixels, true)?;
        if weak_hollow_error(&run.session().picked_entity_state_types()?) {
            println!("Error: cone should be detected");
        }
    }
    Ok(())
}
