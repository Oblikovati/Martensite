//! Native replay of the GDML truncated-cone boolean construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{
    CreateCone, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis, Translate,
};
use martensite_tkmath::gp::Point3;

const ALIGNMENT_AXIS: [f64; 3] = [
    0.90882954575006403,
    0.41716766026590796,
    2.7755575615628907e-17,
];
const REORIENT_AXIS: [f64; 3] = [
    -0.37211395835926747,
    0.76355699062743898,
    0.52774228943506762,
];
const PLACEMENT: [f64; 3] = [-194.945, 678.75574884, 127.];

fn place(mut shape: martensite_tkbrep::topods::Shape) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), ALIGNMENT_AXIS, 90.);
    Translate(&shape, PLACEMENT[0], PLACEMENT[1], PLACEMENT[2])
}

#[test]
fn GdmlPublic__A5() {
    let source = "tests/boolean/gdml_public/A5";
    let outer = place(Translate(
        &CreateCone(148.6909644, 515.32406046, 798.7363878495737),
        0.,
        0.,
        323.93388673042631,
    ));

    let mut lower = CreateCone(2.9651215899955146, 148.69096439977511, 0.010431115384777485);
    RotateAroundAxis(
        &mut lower,
        Point3::default(),
        REORIENT_AXIS,
        6.0267193763611507e-15,
    );
    lower = Translate(
        &lower,
        2.2054877035770915e-8,
        1.0123585525434464e-8,
        323.92324335930391,
    );
    let lower = place(lower);

    let mut upper = CreateCone(2.8788700799980997, 515.35313995965976, 0.036683119294047425);
    RotateAroundAxis(
        &mut upper,
        Point3::default(),
        REORIENT_AXIS,
        6.0267193763611507e-15,
    );
    upper = Translate(
        &upper,
        7.4577883424353786e-8,
        3.4232556345159537e-8,
        1122.6333874656173,
    );
    let upper = place(upper);

    let mut cylinder = Translate(
        &CreateCylinder(2.90794958, 798.71014002),
        0.,
        0.,
        -399.35507001,
    );
    RotateAroundAxis(&mut cylinder, Point3::default(), ALIGNMENT_AXIS, 180.);
    let cylinder = place(cylinder);

    let fused = PerformFuse(&outer, &lower);
    assert!(!fused.is_null(), "{source}: fuse result");
    let upper_cut = PerformCut(&fused, &upper);
    assert!(!upper_cut.is_null(), "{source}: upper cut");
    let result = PerformCut(&upper_cut, &cylinder);
    assert!(!result.is_null(), "{source}: cylinder cut");
}
