//! 3d radius text that flips to stay readable after the view turns half around one axis
//! (`bug24288_1` to `_3`).
use super::prelude::*;

const CYAN: [f64; 3] = [0., 1., 1.];

/// The axis of the half turn.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FlipAxis {
    X,
    Y,
    Z,
}

/// Per axis: the image, the turn, the cursor and checked pixels and the subtitle.
struct Flip {
    image: &'static str,
    turn: [f64; 3],
    cursor: (i32, i32),
    checked: (i32, i32),
    name: &'static str,
}

fn flip(axis: FlipAxis) -> Flip {
    // The cases write pi as "3,14" and pi/2 as "1,57": the command host's number parser stops at the
    // comma, so the view turns by 3 and -1 radians.
    let (image, turn, cursor, checked, name) = match axis {
        FlipAxis::X => ("bug24288_1.png", [3., 0., 0.], (208, 200), (295, 113), "x"),
        FlipAxis::Y => ("bug24288_2.png", [0., 3., 0.], (159, 246), (113, 294), "y"),
        FlipAxis::Z => ("bug24288_3.png", [0., 0., 3.], (221, 217), (209, 208), "z"),
    };
    Flip {
        image,
        turn,
        cursor,
        checked,
        name,
    }
}

/// The radius of a circle with 3d text before and after a half turn about `axis`.
/// Example: `flipped_radius(FlipAxis::X)?`.
pub(crate) fn flipped_radius(axis: FlipAxis) -> Outcome {
    let flip = flip(axis);
    let subtitle = format!(
        "Test for text flipping after {} rotation",
        flip.name.to_uppercase()
    );
    dimension_banner(&["CR24288: Provide a text flipping", &subtitle]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    prepare_flipped_radius(&mut run)?;
    highlight(&mut run, (110, 111), (196, 196), "before flipping")?;
    let [x, y, z] = flip.turn;
    run.session().rotate(x, y, z)?;
    let after = format!("with flipping after {} rotation", flip.name);
    highlight(&mut run, flip.cursor, flip.checked, &after)?;
    run.dump(flip.image)?;
    run.finish()
}

/// Fit, detect at `cursor` and check `checked` is cyan, with the original's message.
fn highlight(run: &mut SessionRun, (x, y): (i32, i32), checked: (i32, i32), when: &str) -> Outcome {
    run.session().fit()?;
    run.session().move_cursor(x, y)?;
    let error = format!("Error : Highlighting of radius dimension with 3d text {when} is wrong.");
    check_or_report(run, checked, CYAN, &error)
}

fn prepare_flipped_radius(run: &mut SessionRun) -> Outcome {
    let points = [[0., 0., 0.], [50., 50., 0.], [100., 0., 0.]];
    for (name, point) in ["radP1", "radP2", "radP3"].into_iter().zip(points) {
        display_point(run, name, point)?;
    }
    display_circle(run, "circle", points)?;
    run.session().rotate(0., -1., 0.)?;
    run.session().erase(&["radP1", "radP2", "radP3"])?;
    let operands = objects(&["circle"]);
    let request = DimensionRequest {
        font: Some("SansFont"),
        text_3d: Some(true),
        ..DimensionRequest::new(DimensionKind::Radius, &operands)
    };
    display_dimension(run.session(), "dim", &request)?;
    Ok(())
}
