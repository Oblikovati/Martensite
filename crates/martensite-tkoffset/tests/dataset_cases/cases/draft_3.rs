//! Script case tables.
#[allow(unused_imports)]
use crate::conditions::{Cond, Value};
#[allow(unused_imports)]
use crate::extra_steps::*;
#[allow(unused_imports)]
use crate::groups::{End, WireEnd};
#[allow(unused_imports)]
use crate::steps::*;
#[allow(unused_imports)]
use crate::support::{properties::Prop, session::ViewStep};

offset_case!(
    draft_angle_g7,
    "tests/draft/angle/G7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 7.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 10.0, 0.0, 7.0, 7.0, 0.0, 7.0, 3.0, 0.0, 7.0, 0.0, 0.0, 7.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Revol(
            "pr",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "-4.0", "-4.0", "0.0", "l", "8.0", "d", "0.0", "1.0", "l", "8.0", "d", "-1.0",
                "0.0", "l", "8.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 0.0, 0.0, 0.0, 15.0])),
        Step::Sweep(Sweep::Pipe("p", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 3.0, [0.0, 0.0, 14.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1323.39, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g8,
    "tests/draft/angle/G8",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 Linux Windows: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 22803 Linux Windows: Error : The area of result shape is"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 3.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 10.0, 0.0, 3.0, 7.0, 0.0, 2.0, 3.0, 0.0, 3.0, 0.0, 0.0, 3.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Revol(
            "p",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "0.0", "-5.0", "1.0", "p", "1.0", "-1.0", "0.0", "1.0", "1.0", "0.0", "l",
                "7.0", "t", "0.0", "7.0", "t", "-5.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pf",
            [-1.0, -7.0, 0.0, 0.0, 0.0, 1.0, 70.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::If(
            Cond::Failed(&Step::Draft(Draft {
                result: "result",
                shape: "f",
                direction: [0.0, 0.0, 1.0],
                faces: &[("f_4", 3.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])]
            })),
            &[Step::Puts("Error in depouille")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 492.768, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g9,
    "tests/draft/angle/G9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 8.0, 0.0, 0.0, 12.0, 0.0, 0.0, 0.0, 4.0,
                0.0, 4.0, 4.0, 1.0, 8.0, 4.0, 0.0, 12.0, 4.0, 0.0, 0.0, 8.0, 0.0, 4.0, 8.0, 0.0,
                8.0, 8.0, -1.0, 12.0, 8.0, 0.0, 0.0, 12.0, 0.0, 4.0, 12.0, 0.0, 8.0, 12.0, 0.0,
                12.0, 12.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::Prim(Prim::Box("bx", &[3.0, 3.0, 2.0, 6.0, 6.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_4", 5.0, [0.0, 0.0, 12.0, 0.0, 0.0, 1.0]),
                ("f_10", 5.0, [0.0, 0.0, 12.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 697.476, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h1,
    "tests/draft/angle/H1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 5.0,
                0.0, 5.0, 5.0, 2.0, 10.0, 5.0, 0.0, 15.0, 5.0, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0, 0.0,
                10.0, 10.0, 1.0, 15.0, 10.0, 0.0, 0.0, 15.0, 0.0, 5.0, 15.0, 0.0, 10.0, 15.0, 0.0,
                15.0, 15.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[8.0, 8.0, 1.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "pc", "pc", &[5.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 11.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 926.278, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h2,
    "tests/draft/angle/H2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 5.0,
                0.0, 5.0, 5.0, 0.0, 10.0, 5.0, 0.0, 15.0, 5.0, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0, 0.0,
                10.0, 10.0, 0.0, 15.0, 10.0, 0.0, 0.0, 15.0, 0.0, 5.0, 15.0, 0.0, 10.0, 15.0, 0.0,
                15.0, 15.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[8.0, 8.0, 1.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "pc",
            "pc",
            &[5.0, 4.0, 10.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 3.0, [0.0, 0.0, 11.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 898.725, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h3,
    "tests/draft/angle/H3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 5.0,
                0.0, 5.0, 5.0, -2.0, 10.0, 5.0, 2.0, 15.0, 5.0, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0,
                0.0, 10.0, 10.0, 0.0, 15.0, 10.0, 0.0, 0.0, 15.0, 0.0, 5.0, 15.0, 0.0, 10.0, 15.0,
                0.0, 15.0, 15.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "5.0", "1.0", "l", "2.0", "c", "3.0", "90.0", "l", "2.0", "d", "-1.0",
                "0.0", "l", "5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 5.0, [0.0, 0.0, 21.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1076.83, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h4,
    "tests/draft/angle/H4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 5.0,
                0.0, 5.0, 5.0, 0.0, 10.0, 5.0, 0.0, 15.0, 5.0, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0, 0.0,
                10.0, 10.0, 0.0, 15.0, 10.0, 0.0, 0.0, 15.0, 0.0, 5.0, 15.0, 0.0, 10.0, 15.0, 0.0,
                15.0, 15.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("pr", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "4.0", "4.0", "2.0", "l", "4.0", "c", "3.0", "90.0", "l", "4.0", "d", "-1.0",
                "0.0", "l", "7.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 2.0, 0.0, 0.0, 20.0])),
        Step::Sweep(Sweep::Pipe("p", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 2.0, [0.0, 0.0, 18.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1154.28, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h5,
    "tests/draft/angle/H5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bs",
            &[
                4.0, 4.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 15.0, 0.0, 0.0, 0.0, 5.0,
                0.0, 5.0, 5.0, -1.0, 10.0, 5.0, 1.0, 15.0, 5.0, 0.0, 0.0, 10.0, 0.0, 5.0, 10.0,
                1.0, 10.0, 10.0, -1.0, 15.0, 10.0, 0.0, 0.0, 15.0, 0.0, 5.0, 15.0, 0.0, 10.0, 15.0,
                0.0, 15.0, 15.0, 0.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "10.0", "7.0", "1.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "3.0", "t", "0.0", "15.0", "t", "-2.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pf",
            [8.0, 7.0, 0.0, 0.0, 0.0, 1.0, 120.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", -1.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 980.006, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h6,
    "tests/draft/angle/H6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::BSpline(
            "bs",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0,
                0.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 1.0, 10.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 1.0,
                0.0, 5.0, 0.0, 1.0, 5.0, 5.0, 1.0, 1.0, 10.0, 5.0, 0.0, 1.0, 15.0, 5.0, 0.0, 1.0,
                0.0, 10.0, 0.0, 1.0, 5.0, 10.0, 0.0, 1.0, 10.0, 10.0, 1.0, 1.0, 15.0, 10.0, 0.0,
                1.0, 0.0, 15.0, 0.0, 1.0, 5.0, 15.0, 0.0, 1.0, 10.0, 15.0, 0.0, 1.0, 15.0, 15.0,
                0.0, 1.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[7.5, 7.5, 1.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "pc", "pc", &[5.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 11.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 931.411, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h7,
    "tests/draft/angle/H7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::BSpline(
            "bs",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0,
                0.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 1.0, 10.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 1.0,
                0.0, 5.0, 0.0, 1.0, 5.0, 5.0, -1.0, 1.0, 10.0, 5.0, 0.0, 1.0, 15.0, 5.0, 0.0, 1.0,
                0.0, 10.0, 0.0, 1.0, 5.0, 10.0, 0.0, 1.0, 10.0, 10.0, 1.0, 1.0, 15.0, 10.0, 0.0,
                1.0, 0.0, 15.0, 0.0, 1.0, 5.0, 15.0, 0.0, 1.0, 10.0, 15.0, 0.0, 1.0, 15.0, 15.0,
                0.0, 1.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[8.0, 8.0, 1.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "pc",
            "pc",
            &[5.0, 4.0, 15.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 1.0, [0.0, 0.0, 11.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1050.36, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h8,
    "tests/draft/angle/H8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::BSpline(
            "bs",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0,
                0.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 1.0, 10.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 1.0,
                0.0, 5.0, 0.0, 1.0, 5.0, 5.0, 0.0, 1.0, 10.0, 5.0, 0.0, 1.0, 15.0, 5.0, 0.0, 1.0,
                0.0, 10.0, 0.0, 1.0, 5.0, 10.0, 0.0, 1.0, 10.0, 10.0, 0.0, 1.0, 15.0, 10.0, 0.0,
                1.0, 0.0, 15.0, 0.0, 1.0, 5.0, 15.0, 0.0, 1.0, 10.0, 15.0, 0.0, 1.0, 15.0, 15.0,
                0.0, 1.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("pr", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "4.0", "4.0", "2.0", "l", "4.0", "c", "3.0", "90.0", "l", "4.0", "d", "-1.0",
                "0.0", "l", "7.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 2.0, 0.0, 0.0, 20.0])),
        Step::Sweep(Sweep::Pipe("p", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 2.0, [0.0, 0.0, 19.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1160.66, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_h9,
    "tests/draft/angle/H9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::BSpline(
            "bs",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0,
                0.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 1.0, 10.0, 0.0, 0.0, 1.0, 15.0, 0.0, 0.0, 1.0,
                0.0, 5.0, 0.0, 1.0, 5.0, 5.0, -2.0, 1.0, 10.0, 5.0, 0.0, 1.0, 15.0, 5.0, 0.0, 1.0,
                0.0, 10.0, 0.0, 1.0, 5.0, 10.0, 0.0, 1.0, 10.0, 10.0, -1.0, 1.0, 15.0, 10.0, 0.0,
                1.0, 0.0, 15.0, 0.0, 1.0, 5.0, 15.0, 0.0, 1.0, 10.0, 15.0, 0.0, 1.0, 15.0, 15.0,
                0.0, 1.0
            ]
        ))),
        Step::More(More::Face("bs", "bs", None)),
        Step::Sweep(Sweep::Prism("p", "bs", [0.0, 0.0, 5.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "10.0", "7.0", "0.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "2.0", "t", "0.0", "15.0", "t", "-2.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pf",
            [10.0, 7.0, 0.0, 0.0, 0.0, 1.0, 180.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 3.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 863.389, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i1,
    "tests/draft/angle/I1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 10.0
            ]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 10.0, 15.0, 0.0, 11.0, 5.0, 0.0, 9.0, 0.0, 0.0, 10.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 20.0, 0.0])),
        Step::Prim(Prim::Box("bx", &[5.0, 5.0, 2.0, 10.0, 10.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 3.0, [5.0, 5.0, 22.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2082.25, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i2,
    "tests/draft/angle/I2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 10.0
            ]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 10.0, 15.0, 0.0, 11.0, 5.0, 0.0, 9.0, 0.0, 0.0, 10.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 20.0, 0.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[10.0, 10.0, 0.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "pc", "pc", &[5.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1994.6, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i3,
    "tests/draft/angle/I3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 5.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 5.0, 15.0, 0.0, 6.0, 5.0, 0.0, 4.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 20.0, 0.0])),
        Step::More(More::Surface(Surface::Plane(
            "pc",
            &[10.0, 10.0, 0.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "pc",
            "pc",
            &[5.0, 4.0, 10.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 1.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1325.75, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i4,
    "tests/draft/angle/I4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 7.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 7.0, 15.0, 0.0, 5.0, 5.0, 0.0, 5.0, 0.0, 0.0, 7.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 20.0, 0.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "5.0", "1.0", "l", "10.0", "d", "0.0", "1.0", "l", "10.0", "d", "-1.0",
                "0.0", "l", "10.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 12.0])),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 5.0, [7.0, 7.0, 10.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1610.35, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i5,
    "tests/draft/angle/I5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 5.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 5.0, 15.0, 0.0, 6.0, 5.0, 0.0, 4.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 20.0, 0.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "7.0", "7.0", "0.0", "l", "7.0", "d", "0.0", "1.0", "l", "7.0", "d", "-1.0",
                "0.0", "l", "7.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 0.0, 0.0, 0.0, 15.0])),
        Step::Sweep(Sweep::Pipe("p", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "pr", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 3.0, [0.0, 0.0, 14.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1510.45, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i6,
    "tests/draft/angle/I6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "p",
            &[0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 0.0, 5.0]
        )),
        Step::Curve(Curve::Bezier(
            "bc",
            &[
                4.0, 20.0, 0.0, 5.0, 15.0, 0.0, 6.0, 5.0, 0.0, 4.0, 0.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("bc", "bc", None)),
        Step::Profile(Profile::Wire("p", &["p", "bc"])),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 20.0, 0.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "12.0", "9.0", "0.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "3.0", "t", "0.0", "15.0", "t", "-3.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pf",
            [10.0, 9.0, 0.0, 0.0, 0.0, 1.0, 180.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 1.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1482.57, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i7,
    "tests/draft/angle/I7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Bezier(
            "bz1",
            &[
                3.0, 3.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0, 0.0, 10.0, 0.0, 0.0, 10.0,
                10.0, 0.0, 10.0, 20.0, 0.0, 20.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0
            ]
        ))),
        Step::More(More::Face("f1", "bz1", None)),
        Step::Sweep(Sweep::Prism("rr", "f1", [0.0, 0.0, 12.0])),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "rr",
            shape: "rr",
            value: -1.0,
            tolerance: None,
            complete: true,
            faces: &[]
        })),
        Step::Topo(Topo::ForceSameParameter("rr", Some(0.001))),
        Step::Prim(Prim::Box("bx", &[5.0, 5.0, 10.0, 10.0, 10.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "rr", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 2.0, [20.0, 20.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1727.74, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i8,
    "tests/draft/angle/I8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "cb",
            &[
                4.0, 10.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0, 10.0, 30.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[10.0, 30.0, 0.0, 10.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("ed", "cb", None)),
        Step::Profile(Profile::Wire("ww", &["ed", "pl"])),
        Step::Profile(Profile::Plane("ww", "ww", false)),
        Step::Sweep(Sweep::Revol(
            "rv",
            "ww",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -180.0]
        )),
        Step::Prim(Prim::Box("bx", &[-5.0, 10.0, 12.0, 10.0, 10.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "rv", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_3", 2.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3257.56, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_i9,
    "tests/draft/angle/I9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "cb",
            &[
                4.0, 10.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0, 10.0, 30.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[10.0, 30.0, 0.0, 10.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("ed", "cb", None)),
        Step::Profile(Profile::Wire("ww", &["ed", "pl"])),
        Step::Profile(Profile::Plane("ww", "ww", false)),
        Step::Sweep(Sweep::Revol(
            "rv",
            "ww",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -180.0]
        )),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 15.0, 12.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[5.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "rv", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 3.0, [0.0, 0.0, 27.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3164.69, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j1,
    "tests/draft/angle/J1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "cb",
            &[
                4.0, 2.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0, 2.0, 30.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[2.0, 30.0, 0.0, 2.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("ed", "cb", None)),
        Step::Profile(Profile::Wire("ww", &["ed", "pl"])),
        Step::Profile(Profile::Plane("ww", "ww", false)),
        Step::Sweep(Sweep::Revol(
            "rv",
            "ww",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -180.0]
        )),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 15.0, 12.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cone, "p", "p", &[7.0, 5.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "rv", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 10.0, [0.0, 0.0, 27.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2510.01, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j2,
    "tests/draft/angle/J2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "cb",
            &[
                4.0, 2.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0, 2.0, 30.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[2.0, 30.0, 0.0, 2.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("ed", "cb", None)),
        Step::Profile(Profile::Wire("ww", &["ed", "pl"])),
        Step::Profile(Profile::Plane("ww", "ww", false)),
        Step::Sweep(Sweep::Revol(
            "rv",
            "ww",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -180.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "-5.0", "10.0", "5.0", "l", "4.0", "c", "3.0", "90.0", "l", "4.0", "d",
                "-1.0", "0.0", "l", "7.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "rv", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_8", 5.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2208.65, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j3,
    "tests/draft/angle/J3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "cb",
            &[
                4.0, 2.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0, 2.0, 30.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[2.0, 30.0, 0.0, 2.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("ed", "cb", None)),
        Step::Profile(Profile::Wire("ww", &["ed", "pl"])),
        Step::Profile(Profile::Plane("ww", "ww", false)),
        Step::Sweep(Sweep::Revol(
            "rv",
            "ww",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -180.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "2.0", "15.0", "10.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "3.0", "t", "0.0", "15.0", "t", "-3.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pf",
            [0.0, 15.0, 10.0, 0.0, 0.0, 1.0, 300.0]
        )),
        Step::Bool(Bool::Fuse("f", "rv", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_6", 1.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2498.26, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j4,
    "tests/draft/angle/J4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[10.0, 20.0])),
        Step::Prim(Prim::Box("bx", &[5.0, 0.5, 0.0, 10.0, 5.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 5.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2204.48, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j5,
    "tests/draft/angle/J5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -0.5, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[10.0, 20.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[10.0, -0.2, 5.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p2", "p2", &[4.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 4.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2018.64, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j6,
    "tests/draft/angle/J6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[15.0, 20.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "1.0", "7.0", "l", "10.0", "c", "2.0", "90.0", "l", "5.0", "d", "-1.0",
                "0.0", "l", "14.0", "d", "0.0", "-1.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 23.0])),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 5.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3960.45, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j7,
    "tests/draft/angle/J7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[15.0, 20.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "1.0", "5.0", "l", "10.0", "c", "2.0", "90.0", "l", "5.0", "d", "-1.0",
                "0.0", "l", "14.0", "d", "0.0", "-1.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 5.0, 0.0, 0.0, 30.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 5.0, [0.0, 0.0, 29.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3953.7, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j8,
    "tests/draft/angle/J8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[12.0, 10.0, 20.0]
        )),
        Step::Prim(Prim::Box("bx", &[5.0, 0.5, 0.0, 10.0, 5.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 5.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2442.47, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_j9,
    "tests/draft/angle/J9",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[15.0, 14.0, 20.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "0.0", "5.0", "l", "10.0", "c", "2.0", "90.0", "l", "5.0", "d", "-1.0",
                "0.0", "l", "14.0", "d", "0.0", "-1.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 5.0, 0.0, 0.0, 30.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 5.0, [0.0, 0.0, 29.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3814.99, 0.01, None)),
    ]
);
