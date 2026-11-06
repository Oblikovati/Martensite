//! Highlighting of dimension parts in local selection (`bug24389`): eleven dimensions with 3d
//! text on the XOY plane, each detected at its picks, its local modes detected only by their
//! own parts, then whole-object, line and text selections in a rectangle.
use super::prelude::*;
use martensite_tkv3d::ais::AisSelectionScheme as Scheme;
use martensite_tkv3d::prs3d::{
    Prs3dDimensionArrowOrientation as Arrow, Prs3dDimensionTextHorizontalPosition as HPos,
};
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

const CYAN: [f64; 3] = [0., 1., 1.];
const BLACK: [f64; 3] = [0., 0., 0.];

/// One dimension: kind, name, its new points (a circle's three points for radii and
/// diameters; none when it reuses earlier points), its operands, arrows, label and flyout.
struct Spec {
    kind: DimensionKind,
    name: &'static str,
    points: &'static [(&'static str, [f64; 3])],
    operands: &'static [&'static str],
    arrow: Arrow,
    label: HPos,
    flyout: Option<f64>,
}

/// Display each new point and erase it, then display the circle for a circle dimension.
fn new_points(run: &mut SessionRun, spec: &Spec) -> Outcome {
    if spec.points.is_empty() {
        return Ok(());
    }
    for (name, at) in spec.points {
        display_point(run, name, *at)?;
    }
    let names: Vec<&str> = spec.points.iter().map(|(name, _)| *name).collect();
    run.session().erase(&names)?;
    if matches!(spec.kind, DimensionKind::Radius | DimensionKind::Diameter) {
        let [a, b, c] = [0, 1, 2].map(|i| spec.points[i].1);
        display_circle(run, spec.operands[0], [a, b, c])?;
    }
    Ok(())
}

fn show(run: &mut SessionRun, spec: &Spec) -> Outcome {
    new_points(run, spec)?;
    let shapes = objects(spec.operands);
    let style = DimensionStyle {
        arrow: Some(spec.arrow),
        label: Some(spec.label),
        ..DimensionStyle::default()
    };
    let request = DimensionRequest {
        text_3d: Some(true),
        plane: Some(WorkingPlane::Xoy),
        font: Some("SansFont"),
        flyout: spec.flyout,
        style,
        ..DimensionRequest::new(spec.kind, &shapes)
    };
    display_dimension(run.session(), spec.name, &request)
}

/// `check_picking`: each pick highlights the checked pixel.
fn check_picking(
    run: &mut SessionRun,
    picks: &[(i32, i32)],
    check: (i32, i32),
    name: &str,
) -> Outcome {
    for &(x, y) in picks {
        run.session().move_cursor(x, y)?;
        let error = format!(
            "Error : Highlighting of {name} at pick coordinates ({x}, {y}) check coordinates ({}, {})",
            check.0, check.1
        );
        check_or_report(run, check, CYAN, &error)?;
    }
    Ok(())
}

/// `check_cross_picking`: with local mode i on, the i-th of the last two picks highlights
/// itself and leaves the other one black.
fn check_cross_picking(
    run: &mut SessionRun,
    picks: &[(i32, i32)],
    object: &str,
    name: &str,
) -> Outcome {
    run.session().set_selection_mode(0, true)?;
    let last_two = &picks[picks.len() - 2..];
    for (i, &(x, y)) in last_two.iter().enumerate() {
        let mode = i as i32 + 1;
        run.session()
            .set_object_selection_mode(object, mode, true)?;
        check_local_dimension_part(run, last_two, i, (x, y), name)?;
        run.session()
            .set_object_selection_mode(object, mode, false)?;
    }
    run.session().set_selection_mode(0, true)?;
    Ok(())
}

/// The picks, checked pixel, object and name of one dimension's checks.
type Check = (
    &'static [(i32, i32)],
    (i32, i32),
    &'static str,
    &'static str,
);

/// The checks of the eleven dimensions, in the original's order.
const CHECKS: [Check; 11] = [
    (
        &[(100, 133), (106, 124), (76, 130)],
        (123, 125),
        "len1",
        "length dimension (len1)",
    ),
    (
        &[(100, 167), (126, 177), (187, 180)],
        (106, 177),
        "len2",
        "length dimension (len2)",
    ),
    (
        &[(100, 114), (106, 99), (120, 97)],
        (153, 109),
        "len3",
        "length dimension (len3)",
    ),
    (
        &[(247, 151), (184, 150)],
        (236, 151),
        "diam1",
        "diameter dimension (diam1)",
    ),
    (
        &[(222, 99), (285, 99)],
        (239, 99),
        "diam2",
        "diameter dimension (diam2)",
    ),
    (
        &[(205, 225), (263, 242), (224, 242)],
        (205, 233),
        "diam3",
        "diameter dimension (diam3)",
    ),
    (
        &[(288, 158), (310, 157)],
        (288, 158),
        "rad1",
        "radius dimension (rad1)",
    ),
    (
        &[(320, 242), (362, 241)],
        (320, 242),
        "rad2",
        "radius dimension (rad2)",
    ),
    (
        &[(113, 229), (96, 221), (85, 256)],
        (86, 250),
        "ang1",
        "angle dimension (ang1)",
    ),
    (
        &[(140, 311), (91, 310), (50, 278)],
        (105, 318),
        "ang2",
        "angle dimension (ang2)",
    ),
    (
        &[(172, 303), (163, 327), (243, 287)],
        (183, 326),
        "ang3",
        "angle dimension (ang3)",
    ),
];

/// Select the rectangle (-500, -500)..(500, 500) with `mode` on for every object, then dump.
fn select_all_and_dump(run: &mut SessionRun, mode: i32, image: &'static str) -> Outcome {
    run.session().set_selection_mode(mode, true)?;
    run.session()
        .select_rectangle((-500, -500), (500, 500), Scheme::Replace)?;
    run.dump(image)
}

/// `bug24389`: the eleven dimensions, their picking checks and three selection dumps.
/// Example: `local_part_highlight()?`.
pub(crate) fn local_part_highlight() -> Outcome {
    dimension_banner(&["CR24389: Invalid highlight of AIS dimension line in local selection"]);
    register_sans_font()?;
    let mut run = SessionRun::new()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    fit_local_dimensions(&mut run)?;
    for (picks, check, object, name) in CHECKS {
        check_picking(&mut run, picks, check, name)?;
        check_cross_picking(&mut run, picks, object, name)?;
    }
    select_all_and_dump(&mut run, 0, "bug24389_1.png")?;
    clear_local_selection(&mut run)?;
    select_all_and_dump(&mut run, 1, "bug24389_2.png")?;
    clear_local_selection(&mut run)?;
    run.session().set_selection_mode(0, true)?;
    select_all_and_dump(&mut run, 2, "bug24389_3.png")?;
    run.finish()
}

use DimensionKind::{Angle, Diameter, Length, Radius};

const SPECS: [Spec; 11] = [
    Spec {
        kind: Length,
        name: "len1",
        points: &[("len_p1", [0., 0., 0.]), ("len_p2", [40., 0., 0.])],
        operands: &["len_p1", "len_p2"],
        arrow: Arrow::External,
        label: HPos::Left,
        flyout: Some(20.),
    },
    Spec {
        kind: Length,
        name: "len2",
        points: &[],
        operands: &["len_p1", "len_p2"],
        arrow: Arrow::External,
        label: HPos::Right,
        flyout: Some(-20.),
    },
    Spec {
        kind: Length,
        name: "len3",
        points: &[("len_p3", [0., 20., 0.]), ("len_p4", [40., 20., 0.])],
        operands: &["len_p3", "len_p4"],
        arrow: Arrow::Internal,
        label: HPos::Center,
        flyout: Some(20.),
    },
    Spec {
        kind: Diameter,
        name: "diam1",
        points: &[
            ("diam_p1", [90., 0., 0.]),
            ("diam_p2", [120., 0., 0.]),
            ("diam_p3", [105., -15., 0.]),
        ],
        operands: &["diam_c1"],
        arrow: Arrow::External,
        label: HPos::Left,
        flyout: None,
    },
    Spec {
        kind: Diameter,
        name: "diam2",
        points: &[
            ("diam_p4", [90., 40., 0.]),
            ("diam_p5", [120., 40., 0.]),
            ("diam_p6", [105., 25., 0.]),
        ],
        operands: &["diam_c2"],
        arrow: Arrow::External,
        label: HPos::Right,
        flyout: None,
    },
    Spec {
        kind: Diameter,
        name: "diam3",
        points: &[
            ("diam_p7", [80., -40., 0.]),
            ("diam_p8", [120., -40., 0.]),
            ("diam_p9", [100., -60., 0.]),
        ],
        operands: &["diam_c3"],
        arrow: Arrow::External,
        label: HPos::Center,
        flyout: Some(30.),
    },
    Spec {
        kind: Radius,
        name: "rad1",
        points: &[
            ("rad_p1", [140., -5., 0.]),
            ("rad_p2", [180., -45., 0.]),
            ("rad_p3", [220., -5., 0.]),
        ],
        operands: &["rad_c1"],
        arrow: Arrow::Internal,
        label: HPos::Center,
        flyout: Some(0.),
    },
    Spec {
        kind: Radius,
        name: "rad2",
        points: &[
            ("rad_p4", [180., -70., 0.]),
            ("rad_p5", [160., -90., 0.]),
            ("rad_p6", [140., -70., 0.]),
        ],
        operands: &["rad_c2"],
        arrow: Arrow::External,
        label: HPos::Left,
        flyout: None,
    },
    Spec {
        kind: Angle,
        name: "ang1",
        points: &[
            ("ang_p1", [0., -50., 0.]),
            ("ang_p2", [25., -75., 0.]),
            ("ang_p3", [0., -100., 0.]),
        ],
        operands: &["ang_p1", "ang_p2", "ang_p3"],
        arrow: Arrow::Internal,
        label: HPos::Center,
        flyout: Some(35.),
    },
    Spec {
        kind: Angle,
        name: "ang2",
        points: &[
            ("ang_p4", [0., -120., 0.]),
            ("ang_p5", [30., -80., 0.]),
            ("ang_p6", [30., -120., 0.]),
        ],
        operands: &["ang_p4", "ang_p5", "ang_p6"],
        arrow: Arrow::External,
        label: HPos::Left,
        flyout: Some(55.),
    },
    Spec {
        kind: Angle,
        name: "ang3",
        points: &[
            ("ang_p8", [55., -120., 0.]),
            ("ang_p9", [55., -80., 0.]),
            ("ang_p10", [95., -120., 0.]),
        ],
        operands: &["ang_p8", "ang_p9", "ang_p10"],
        arrow: Arrow::External,
        label: HPos::Right,
        flyout: Some(55.),
    },
];

fn check_local_dimension_part(
    run: &mut SessionRun,
    last_two: &[(i32, i32)],
    i: usize,
    (x, y): (i32, i32),
    name: &str,
) -> Outcome {
    for (j, &(cx, cy)) in last_two.iter().enumerate() {
        run.session().move_cursor(x, y)?;
        let (expected, what) = if i == j {
            (CYAN, "No local")
        } else {
            (BLACK, "Unwanted")
        };
        let error = local_highlight_error(what, name, (x, y), (cx, cy));
        check_or_report(run, (cx, cy), expected, &error)?;
    }
    Ok(())
}

fn fit_local_dimensions(run: &mut SessionRun) -> Outcome {
    for spec in &SPECS {
        show(run, spec)?;
    }
    display_point(run, "fit1", [-75., 0., 0.])?;
    display_point(run, "fit2", [235., 0., 0.])?;
    run.session().fit()?;
    Ok(())
}

fn local_highlight_error(
    what: &str,
    name: &str,
    (x, y): (i32, i32),
    (cx, cy): (i32, i32),
) -> String {
    format!(
        "Error : {what} highlighting of {name} at pick coordinates ({x}, {y}) check coordinates ({cx}, {cy})"
    )
}

fn clear_local_selection(run: &mut SessionRun) -> Outcome {
    run.session()
        .select_rectangle((0, 0), (0, 0), Scheme::Replace)?;
    Ok(())
}

#[cfg(test)]
mod local_highlight_contract {
    use super::*;

    #[test]
    fn local_selection_failure_keeps_the_actual_pick_and_check_coordinates() {
        let error = local_highlight_error("Unwanted", "length", (4, 7), (9, 11));
        assert_eq!(
            error,
            "Error : Unwanted highlighting of length at pick coordinates (4, 7) check coordinates (9, 11)"
        );
    }
}
