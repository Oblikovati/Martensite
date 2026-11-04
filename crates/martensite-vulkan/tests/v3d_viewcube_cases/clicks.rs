//! Highlighting and clicking the view cube's side, edge and corner turn the camera.
use super::prelude::*;
use crate::session_run::ScriptedControl;
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;

const OCC28954: &str =
    "0028954: Visualization - implement interactive object AisViewCube for camera manipulations";
const OCC28954_PERSPECTIVE: &str = "0028954: Visualization - compare AisViewCube on perspective view with and without orthographic persistence";
const RULE: &str = "==================================";

/// A shaded 15 x 20 x 70 box in a fitted axonometric view; `shaded_box(&mut run)?`.
fn shaded_box(run: &mut SessionRun) -> Outcome {
    let b = box_sized([15., 20., 70.])?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display("b", &b, &request)?;
    run.session().set_axonometric()?;
    run.session().fit()?;
    Ok(())
}

/// One click on the cube and the pixel colours expected after the camera turned.
struct Click {
    at: (i32, i32),
    checks: &'static [((i32, i32), &'static str)],
    message: &'static str,
    image: &'static str,
}

/// Click, check every pixel and dump; `click(&mut run, &side)?`.
fn click(run: &mut SessionRun, click: &Click) -> Outcome {
    click_and_check(run, click)?;
    run.dump(click.image)
}

fn click_and_check(run: &mut SessionRun, click: &Click) -> Outcome {
    run.session().select_point(click.at.0, click.at.1)?;
    for &(pixel, expected) in click.checks {
        expect_pixel(run.session(), pixel, expected, true, click.message)?;
    }
    Ok(())
}

/// The cube `vc` displayed again with axis cylinders of `radius`.
/// A zero radius collapses every cylinder triangle to zero area, so the control
/// hides the axis shafts and keeps the labels, cones and centre sphere: the `uncovered` control
/// of a shaft on a pixel-centre tie over a label glyph.
fn cube_axes_radius(session: &mut ViewerSession, radius: f64) -> Outcome {
    let cube = session.view_cube("vc")?;
    cube.set_axes_radius(radius);
    session.display_object("vc", cube.cast())?;
    Ok(())
}

/// Click, check, dump, then dump the `uncovered` control with the cube's axis shafts collapsed:
/// a shaft row on a pixel-centre tie uncovers the axis label glyph under it (batch `controls`).
fn click_with_axis_control(run: &mut SessionRun, click: &Click) -> Outcome {
    click_and_check(run, click)?;
    let control = ScriptedControl {
        suffix: "uncovered",
        apply: &|session| cube_axes_radius(session, 0.),
        restore: &|session| cube_axes_radius(session, 1.),
    };
    run.dump_with_scripted_controls(click.image, &[control])
}

/// `default`: hover highlight, then clicks on the front side, front-top edge and a corner.
pub(crate) fn default_cube_clicks() -> Outcome {
    banner(&[
        RULE,
        OCC28954,
        "Display and erase with default settings",
        RULE,
    ]);
    let mut run = SessionRun::new()?;
    shaded_box(&mut run)?;
    instant_cube(run.session(), false)?;
    run.session().move_cursor(70, 340)?;
    let message = "Error: Highlighting of view cube Side is wrong.";
    expect_pixel(run.session(), (70, 340), "CYAN", true, message)?;
    run.session().move_cursor(0, 0)?;
    run.dump("default_axo.png")?;
    let [side, rest @ ..] = DEFAULT_CLICKS;
    click_with_axis_control(&mut run, side)?;
    for step in rest {
        click(&mut run, step)?;
    }
    run.finish()
}

const FRONT: &str = "Error: Position of FRONT camera is wrong.";
const FRONT_TOP: &str = "Error: Position of FRONT-TOP camera is wrong.";
const CORNER: &str = "Error: Position of TOP-FRONT-RIGHT camera is wrong.";

const DEFAULT_CLICKS: &[Click; 3] = &[
    Click {
        at: (70, 340),
        checks: &[((255, 300), "BLACK")],
        message: FRONT,
        image: "default_side.png",
    },
    Click {
        at: (100, 270),
        checks: &[((100, 300), "GRAY52"), ((100, 310), "CYAN")],
        message: FRONT_TOP,
        image: "default_edge.png",
    },
    Click {
        at: (140, 310),
        checks: &[
            ((100, 290), "GRAY47"),
            ((100, 310), "CYAN"),
            ((100, 320), "GRAY59"),
        ],
        message: CORNER,
        image: "default_corner.png",
    },
];

/// `orthopers`: the same clicks in a perspective view on a cube kept orthographic.
pub(crate) fn orthographic_cube_clicks() -> Outcome {
    banner(&[RULE, OCC28954_PERSPECTIVE, RULE]);
    let mut run = SessionRun::new()?;
    run.session()
        .set_projection(Graphic3dCameraProjection::Perspective)?;
    shaded_box(&mut run)?;
    instant_cube(run.session(), true)?;
    run.session().move_cursor(70, 350)?;
    let message = "Error: Highlighting of view cube Side is wrong.";
    expect_pixel(run.session(), (95, 350), "GRAY62", true, message)?;
    run.session().move_cursor(0, 0)?;
    run.dump("orthopers_axo.png")?;
    let [side, rest @ ..] = ORTHOGRAPHIC_CLICKS;
    click_with_axis_control(&mut run, side)?;
    for step in rest {
        click(&mut run, step)?;
    }
    run.finish()
}

const ORTHOGRAPHIC_CLICKS: &[Click; 3] = &[
    Click {
        at: (70, 340),
        checks: &[((255, 300), "BLACK")],
        message: FRONT,
        image: "orthopers_side.png",
    },
    Click {
        at: (110, 270),
        checks: &[((100, 320), "GRAY57"), ((100, 310), "CYAN")],
        message: FRONT_TOP,
        image: "orthopers_edge.png",
    },
    Click {
        at: (140, 310),
        checks: &[
            ((100, 290), "GRAY41"),
            ((100, 310), "CYAN"),
            ((100, 320), "GRAY62"),
        ],
        message: CORNER,
        image: "orthopers_corner.png",
    },
];
