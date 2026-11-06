//! Dimension text moved to a custom position, kept after the flyout changes (`bug24412_1`
//! and `_2`).
use super::prelude::*;

const INITIAL_TEXT_POSITION_ERROR: &str = "Error : Text position of dimension is incorrect.";

const TEXT_GREEN: [f64; 3] = [0.46, 0.9, 0.];

/// Move the text of `name`, fit and check it, dump; flip the flyout, fit, check, dump.
fn move_and_flip(
    run: &mut SessionRun,
    name: &str,
    to: [i32; 3],
    checks: [(i32, i32); 2],
    images: [&'static str; 2],
) -> Outcome {
    run.session().move_dimension_text(name, to)?;
    check_moved_dump(run, checks[0], INITIAL_TEXT_POSITION_ERROR, images[0])?;
    let flipped = DimensionParams {
        flyout: Some(-5.),
        ..DimensionParams::default()
    };
    set_dimension_params(run.session(), name, &flipped)?;
    let error = "Error : Text position of dimension after layout changing is incorrect.";
    check_moved_dump(run, checks[1], error, images[1])
}

fn moved_banner(subtitle: &str) {
    dimension_banner(&[
        "CR24412: TKV3d, Dimensions - Possibility to customize text position was lost",
        subtitle,
    ]);
}

/// `bug24412_1`: the diameter of a displayed circle.
/// Example: `moved_diameter_text()?`.
pub(crate) fn moved_diameter_text() -> Outcome {
    moved_banner("Test position setting for linear dimension");
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    prepare_moved_diameter(&mut run)?;
    move_and_flip(
        &mut run,
        "diam",
        [15, 15, 15],
        [(166, 226), (213, 179)],
        ["bug24412_1_1.png", "bug24412_1_2.png"],
    )?;
    run.finish()
}

/// `bug24412_2`: the angle at the middle of three points.
/// Example: `moved_angle_text()?`.
pub(crate) fn moved_angle_text() -> Outcome {
    moved_banner("Test position setting for angular dimension");
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    prepare_moved_angle(&mut run)?;
    move_and_flip(
        &mut run,
        "ang",
        [5, 0, 5],
        [(60, 189), (206, 312)],
        ["bug24412_2_1.png", "bug24412_2_2.png"],
    )?;
    run.finish()
}

fn check_moved_dump(
    run: &mut SessionRun,
    checked: (i32, i32),
    error: &str,
    image: &'static str,
) -> Outcome {
    run.session().fit()?;
    check_or_report(run, checked, TEXT_GREEN, error)?;
    run.dump(image)
}

fn prepare_moved_diameter(run: &mut SessionRun) -> Outcome {
    let points = [[0., 10., 30.], [30., 10., 0.], [60., 10., 30.]];
    for (name, point) in ["circ_p1", "circ_p2", "circ_p3"].into_iter().zip(points) {
        display_point(run, name, point)?;
    }
    display_circle(run, "circ", points)?;
    run.session().display_again("circ")?;
    let operands = objects(&["circ"]);
    let request = DimensionRequest {
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Diameter, &operands)
    };
    display_dimension(run.session(), "diam", &request)?;
    Ok(())
}

fn prepare_moved_angle(run: &mut SessionRun) -> Outcome {
    let points = [[0., 0., 30.], [30., 0., 0.], [60., 0., 30.]];
    for (name, point) in ["ang_p1", "ang_p2", "ang_p3"].into_iter().zip(points) {
        display_point(run, name, point)?;
    }
    let operands = objects(&["ang_p1", "ang_p2", "ang_p3"]);
    let request = DimensionRequest {
        font: Some("SansFont"),
        ..DimensionRequest::new(DimensionKind::Angle, &operands)
    };
    display_dimension(run.session(), "ang", &request)?;
    Ok(())
}
