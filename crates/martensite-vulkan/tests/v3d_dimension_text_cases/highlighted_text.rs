//! Highlighting of dimensions with 2d text in a first viewer and 3d text in a second one
//! (`bug24133_2` to `_4`): the cursor detects the dimension and its pixels turn cyan.
use super::prelude::*;

const CYAN: [f64; 3] = [0., 1., 1.];
const ANGLE: [[f64; 3]; 3] = [[0., 0., 0.], [50., 50., 50.], [50., 50., 100.]];
const CIRCLE: [[f64; 3]; 3] = [[0., 0., 0.], [50., 50., 0.], [100., 0., 0.]];

/// One viewer's detection: the cursor pixel, the checked pixel and the message on failure.
struct Detection {
    cursor: (i32, i32),
    checked: (i32, i32),
    error: &'static str,
}

/// Fit, move the cursor over the dimension and check the highlighted pixel.
fn detect(run: &mut SessionRun, detection: &Detection) -> Outcome {
    run.session().fit()?;
    let (x, y) = detection.cursor;
    run.session().move_cursor(x, y)?;
    check_or_report(run, detection.checked, CYAN, detection.error)
}

/// The dimension `name` of `kind` over `shapes` with SansFont, 3d text when `text_3d`.
fn sans_dimension(
    run: &mut SessionRun,
    name: &str,
    kind: DimensionKind,
    shapes: &[&str],
    text_3d: bool,
) -> Outcome {
    let operands = objects(shapes);
    let request = DimensionRequest {
        font: Some("SansFont"),
        text_3d: text_3d.then_some(true),
        ..DimensionRequest::new(kind, &operands)
    };
    display_dimension(run.session(), name, &request)
}

/// `bug24133_2`: an angle at three points with 2d text, then with 3d text in `Viewer2`.
/// Example: `angle_text_highlight()?`.
pub(crate) fn angle_text_highlight() -> Outcome {
    dimension_banner(&[
        "CR24133: Development of improvement of dimensions implementation - Angle dimension",
    ]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    for (viewer, text_3d) in [(1, false), (2, true)] {
        show_angle_view(&mut run, viewer, text_3d)?;
    }
    run.dump("bug24133_2.png")?;
    run.finish()
}

const ANGLE_DETECTIONS: [Detection; 2] = [
    Detection {
        cursor: (249, 206),
        checked: (249, 200),
        error: "Error : Highlighting of angle dimension with 2d text is wrong.",
    },
    Detection {
        cursor: (263, 251),
        checked: (335, 319),
        error: "Error : Highlighting of angle dimension with 3d text is wrong.",
    },
];

/// A radius (`bug24133_3`) or diameter (`bug24133_4`) of the circle through three points,
/// with 2d text, then with 3d text on a second circle of the same points in `Viewer2`.
/// Example: `circle_text_highlight(DimensionKind::Radius)?`.
pub(crate) fn circle_text_highlight(kind: DimensionKind) -> Outcome {
    let (prefix, title, image, detections) = match kind {
        DimensionKind::Radius => ("rad", "Radius", "bug24133_3.png", &RADIUS_DETECTIONS),
        _ => ("diam", "Diameter", "bug24133_4.png", &DIAMETER_DETECTIONS),
    };
    circle_highlight_banner(title);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    first_circle_points(&mut run, prefix)?;
    sans_dimension(&mut run, "dim1", kind, &["circle1"], false)?;
    detect(&mut run, &detections[0])?;
    run.session().open_viewer()?;
    display_circle(&mut run, "circle2", CIRCLE)?;
    sans_dimension(&mut run, "dim2", kind, &["circle2"], true)?;
    detect(&mut run, &detections[1])?;
    run.dump(image)?;
    run.finish()
}

const RADIUS_DETECTIONS: [Detection; 2] = [
    Detection {
        cursor: (123, 158),
        checked: (112, 151),
        error: "Error : Highlighting of radius dimension with 2d text is wrong.",
    },
    Detection {
        cursor: (191, 196),
        checked: (112, 151),
        error: "Error : Highlighting of radius dimension with 3d text is wrong.",
    },
];

const DIAMETER_DETECTIONS: [Detection; 2] = [
    Detection {
        cursor: (208, 205),
        checked: (296, 257),
        error: "Error : Highlighting of diameter dimension with 2d text is wrong.",
    },
    Detection {
        cursor: (208, 205),
        checked: (296, 257),
        error: "Error : Highlighting of diameter dimension with 3d text is wrong.",
    },
];

fn show_angle_view(run: &mut SessionRun, viewer: usize, text_3d: bool) -> Outcome {
    if text_3d {
        run.session().open_viewer()?;
    }
    let names = [1, 2, 3].map(|i| format!("angle{viewer}P{i}"));
    for (name, point) in names.iter().zip(ANGLE) {
        display_point(run, name, point)?;
    }
    let shapes = names.each_ref().map(String::as_str);
    sans_dimension(
        run,
        &format!("dim{viewer}"),
        DimensionKind::Angle,
        &shapes,
        text_3d,
    )?;
    detect(run, &ANGLE_DETECTIONS[viewer - 1])?;
    Ok(())
}

fn circle_highlight_banner(title: &str) {
    let title = format!(
        "CR24133: Development of improvement of dimensions implementation - {title} dimension"
    );
    dimension_banner(&[&title]);
}

fn first_circle_points(run: &mut SessionRun, prefix: &str) -> Outcome {
    let points = [1, 2, 3].map(|i| format!("{prefix}P{i}"));
    for (name, point) in points.iter().zip(CIRCLE) {
        display_point(run, name, point)?;
    }
    display_circle(run, "circle1", CIRCLE)?;
    run.session()
        .erase(&points.each_ref().map(String::as_str))?;
    Ok(())
}
