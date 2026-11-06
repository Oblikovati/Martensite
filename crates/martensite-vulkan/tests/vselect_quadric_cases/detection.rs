//! Detection of quadric sensitives under the cursor and the depth of the picked point from
//! several standard views.
use super::prelude::*;
use crate::session_run::check_point;

/// The cursor pixel at the centre of the 600x400 view.
const CENTRE: (i32, i32) = (300, 200);

/// A cursor move after view steps: the picked point must match `expected` within `tolerance`.
struct Probe {
    steps: &'static [ViewStep],
    label: &'static str,
    expected: [f64; 3],
    tolerance: f64,
}

/// Move the cursor to the centre and print `message` unless an entity of type `entity` is
/// picked; returns the detected point.
fn detect(run: &mut QuadricRun, entity: &str, message: &str) -> Result<[f64; 3], Box<dyn Error>> {
    run.run.session().move_cursor(CENTRE.0, CENTRE.1)?;
    let point = run.run.session().detected_point()?;
    if !run
        .run
        .session()
        .picked_entity_types()?
        .iter()
        .any(|name| name.contains(entity))
    {
        println!("{message}");
    }
    Ok(point)
}

/// Run each probe: its view steps, a detection at the centre and the depth check.
fn probe(run: &mut QuadricRun, probes: &[Probe], entity: &str, message: &str) -> Outcome {
    for probe in probes {
        run.view(probe.steps)?;
        let point = detect(run, entity, message)?;
        check_point(probe.label, point, probe.expected, probe.tolerance);
    }
    Ok(())
}

const TOP_FIT: &[ViewStep] = &[ViewStep::Project(Orientation::ZUP_TOP), ViewStep::Fit];
const BOTTOM: &[ViewStep] = &[ViewStep::Project(Orientation::ZUP_BOTTOM)];
const RIGHT_FIT: &[ViewStep] = &[ViewStep::Project(Orientation::ZUP_RIGHT), ViewStep::Fit];
const LEFT: &[ViewStep] = &[ViewStep::Project(Orientation::ZUP_LEFT)];

/// The cylinder probes from the default, top, bottom, right and left views.
#[allow(clippy::excessive_precision)] // The expected points are the original's, digit for digit.
const CYLINDER_PROBES: [Probe; 5] = [
    Probe {
        steps: &[],
        label: "center_cyl_p",
        expected: [7.0710678118654755, -7.0710678118654755, 16.970067811865476],
        tolerance: 1e-4,
    },
    Probe {
        steps: TOP_FIT,
        label: "top_cyl_p",
        expected: [0., -0.050500000000000045, 20.],
        tolerance: 1e-4,
    },
    Probe {
        steps: BOTTOM,
        label: "bottom_cyl_p",
        expected: [0., 0.050500000000000052, 0.],
        tolerance: 1e-4,
    },
    Probe {
        steps: RIGHT_FIT,
        label: "right_cyl_p",
        expected: [10., 0., 9.9495000000000005],
        tolerance: 1e-4,
    },
    Probe {
        steps: LEFT,
        label: "left_cyl_p",
        expected: [-10., 0., 9.9495000000000005],
        tolerance: 1e-4,
    },
];

/// The cone probes from the axonometric, top, bottom, right and left views.
#[allow(clippy::excessive_precision)]
const CONE_PROBES: [Probe; 5] = [
    Probe {
        steps: &[],
        label: "center_cone_p",
        expected: [2.6384203410087546, -2.6384203410087546, 12.537420341008755],
        tolerance: 1e-4,
    },
    Probe {
        steps: TOP_FIT,
        label: "top_cone_p",
        expected: [0., -0.050500000000000045, 19.899000000000001],
        tolerance: 1e-4,
    },
    Probe {
        steps: BOTTOM,
        label: "bottom_cone_p",
        expected: [0., 0.050500000000000052, 0.],
        tolerance: 1e-4,
    },
    Probe {
        steps: RIGHT_FIT,
        label: "right_cone_p",
        expected: [5.0252500000000007, 0., 9.9495000000000005],
        tolerance: 1e-4,
    },
    Probe {
        steps: LEFT,
        label: "left_cone_p",
        expected: [-5.0252500000000007, 0., 9.9495000000000005],
        tolerance: 1e-4,
    },
];

const CYLINDER_BANNER: [&str; 2] = [
    "=================================",
    "0032281: Visualization - add Select3DSensitiveCylinder",
];
const SPHERE_BANNER: [&str; 2] = [
    "=================================",
    "0032182: Visualization - add Select3DSensitiveSphere",
];
const CYLINDER_ENTITY: &str = "Select3DSensitiveCylinder";
const SPHERE_ENTITY: &str = "Select3DSensitiveSphere";

/// `cone_cylinder/check_depth`: picked depths of a cylinder, then of a cone, from five views.
pub(crate) fn cone_cylinder_depth() -> Outcome {
    let [rule, title] = CYLINDER_BANNER;
    banner(&[
        rule,
        title,
        "Tests depth value returned by Select3DSensitiveCylinder",
        rule,
    ]);
    let mut run = QuadricRun::shaded(&[("cyl", cylinder(10., 20.)?)])?;
    probe(
        &mut run,
        &CYLINDER_PROBES,
        CYLINDER_ENTITY,
        "Error: cylinder should be detected",
    )?;
    run.remove("cyl")?;
    run.show("cone", &cone(10., 0., 20.)?)?;
    run.run.session().set_axonometric()?;
    run.view(&[ViewStep::Fit])?;
    probe(
        &mut run,
        &CONE_PROBES,
        CYLINDER_ENTITY,
        "Error: cone should be detected",
    )?;
    run.run.finish_without_image()
}

/// `cone_cylinder/detecting`: a cylinder, a cone and a truncated cone each detected in turn.
pub(crate) fn cone_cylinder_detecting() -> Outcome {
    let [rule, title] = CYLINDER_BANNER;
    banner(&[
        rule,
        title,
        "Tests detecting Select3DSensitiveCylinder",
        rule,
    ]);
    let mut run = QuadricRun::shaded(&[("cyl", cylinder(10., 20.)?)])?;
    detect(
        &mut run,
        CYLINDER_ENTITY,
        "Error: cylinder should be detected",
    )?;
    run.remove("cyl")?;
    run.show("cone", &cone(10., 0., 20.)?)?;
    run.view(&[ViewStep::Fit])?;
    detect(&mut run, CYLINDER_ENTITY, "Error: cone should be detected")?;
    run.remove("cone")?;
    run.show("tr_cone", &cone(10., 5., 10.)?)?;
    run.view(&[ViewStep::Fit])?;
    detect(
        &mut run,
        CYLINDER_ENTITY,
        "Error: truncated cone should be detected",
    )?;
    run.run.finish_without_image()
}

/// The sphere probes from the top, bottom, right, front and left views.
#[allow(clippy::excessive_precision)]
const SPHERE_PROBES: [Probe; 5] = [
    Probe {
        steps: &[ViewStep::Project(Orientation::ZUP_TOP)],
        label: "top_p",
        expected: [
            0.0013453695513163666,
            -0.0090115971854718303,
            0.99995849333074871,
        ],
        tolerance: 1e-4,
    },
    Probe {
        steps: BOTTOM,
        label: "bottom_p",
        expected: [
            0.0013453695513163666,
            0.0090115971854718303,
            -0.99995849333074871,
        ],
        tolerance: 1e-4,
    },
    Probe {
        steps: &[ViewStep::Project(Orientation::ZUP_RIGHT)],
        label: "right_p",
        expected: [
            0.99995849333074871,
            0.0013453695513163666,
            -0.0090115971854718268,
        ],
        tolerance: 1e-4,
    },
    Probe {
        steps: &[ViewStep::Project(Orientation::ZUP_FRONT)],
        label: "front_p",
        expected: [
            0.0013453695513163666,
            -0.99995832904743187,
            -0.0090115971854718285,
        ],
        tolerance: 0.01,
    },
    Probe {
        steps: LEFT,
        label: "left_p",
        expected: [
            -0.99995817565020639,
            -0.0013453695513163666,
            -0.0090115971854718268,
        ],
        tolerance: 1e-4,
    },
];

/// `sphere/check_depth`: picked depths of a unit sphere from five views.
pub(crate) fn sphere_depth() -> Outcome {
    let [rule, title] = SPHERE_BANNER;
    banner(&[
        rule,
        title,
        "Tests depth value returned by Select3DSenstiveSphere",
        rule,
    ]);
    let mut run = QuadricRun::shaded(&[("s", sphere(1.)?)])?;
    probe(
        &mut run,
        &SPHERE_PROBES,
        SPHERE_ENTITY,
        "Error: sphere should be detected",
    )?;
    run.run.finish_without_image()
}

/// `sphere/detecting`: a unit sphere detected at the view centre.
pub(crate) fn sphere_detecting() -> Outcome {
    let [rule, title] = SPHERE_BANNER;
    banner(&[rule, title, "Tests detecting Select3DSenstiveSphere", rule]);
    let mut run = QuadricRun::shaded(&[("s", sphere(1.)?)])?;
    detect(&mut run, SPHERE_ENTITY, "Error: sphere should be detected")?;
    run.run.finish_without_image()
}
