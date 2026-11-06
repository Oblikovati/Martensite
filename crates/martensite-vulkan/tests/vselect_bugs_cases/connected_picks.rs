//! Detection of every sub-shape kind on a shape and on its connected instance.
use super::prelude::*;
use crate::case_view::viewer_session::ConnectedPart;
use crate::dataset_shape::read_dataset_shape;

/// The detected colour every sub-shape check expects (cyan).
const DETECTED: [f64; 3] = [0., 1., 1.];

/// Per selection mode 1..=6 (vertex to solid), the pixel on the shape, then on its instance.
const MODE_PIXELS: [[(i32, i32); 2]; 6] = [
    [(330, 212), (184, 268)],
    [(296, 155), (205, 240)],
    [(298, 209), (56, 236)],
    [(169, 146), (204, 238)],
    [(224, 148), (230, 268)],
    [(329, 218), (168, 256)],
];

/// The colours at the shape's and the instance's check pixels.
fn pair_colors(run: &mut SessionRun) -> Result<[String; 2], Box<dyn Error>> {
    let obj = run.session().pixel_color(300, 187)?;
    let new = run.session().pixel_color(183, 269)?;
    Ok([format!("{obj:?}"), format!("{new:?}")])
}

/// `bug12121`: the shape and its connected instance highlight alike, and each sub-shape
/// selection mode detects both.
pub(crate) fn connected_instance_sub_shapes() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC12121:  Optimization of existing selection classes",
        "================",
        "",
    ]);
    let obj = read_dataset_shape("OCC12121-CrankArm.brep")?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("obj", &obj, &DisplayRequest::default())?;
    run.session()
        .connect_instance("new", [50., 50., 50.], ConnectedPart::object("obj"))?;
    run.session().fit()?;
    run.session().move_cursor(226, 154)?;
    let detected = pair_colors(&mut run)?;
    run.session().select_point(226, 154)?;
    let selected = pair_colors(&mut run)?;
    println!("Check vconnect command");
    let same = detected[0] == detected[1] && selected[0] == selected[1];
    run.session().select_point(10, 10)?;
    detect_each_mode(&mut run)?;
    run.session().set_selection_mode(0, true)?;
    run.session().select_point(10, 10)?;
    println!();
    println!("{} OCC12121", if same { "Faulty" } else { "OK" });
    run.dump("bug12121.png")?;
    run.finish()
}

/// Activate each sub-shape mode alone and check the detected colour on both objects.
fn detect_each_mode(run: &mut SessionRun) -> Result<(), Box<dyn Error>> {
    for (mode, pixels) in (1..).zip(MODE_PIXELS) {
        run.session().set_selection_mode(mode, true)?;
        for (x, y) in pixels {
            run.session().move_cursor(x, y)?;
            run.session().check_color(x, y, DETECTED)?;
        }
        run.session().set_selection_mode(mode, false)?;
    }
    Ok(())
}
