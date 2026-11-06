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
    draft_angle_a1,
    "tests/draft/angle/A1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("bx", &[10.0, 20.0, 30.0])),
        Step::Topo(Topo::NumberedExplode("bx", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "bx",
            direction: [0.0, 0.0, 1.0],
            faces: &[("bx_6", 5.0, [5.0, 10.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2333.52, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a2,
    "tests/draft/angle/A2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("bx", &[10.0, 20.0, 30.0])),
        Step::Topo(Topo::NumberedExplode("bx", "e")),
        Step::Fillet(Fillet::Blend("bx", "bx", &[(2.0, "bx_6"), (3.0, "bx_7")])),
        Step::Topo(Topo::NumberedExplode("bx", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "bx",
            direction: [1.0, 0.0, 0.0],
            faces: &[
                ("bx_3", 10.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_6", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2011.72, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a3,
    "tests/draft/angle/A3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("bx", &[10.0, 20.0, 30.0])),
        Step::Topo(Topo::NumberedExplode("bx", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "bx",
            direction: [1.0, 0.0, 0.0],
            faces: &[
                ("bx_2", 4.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_5", 6.0, [10.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_4", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2191.56, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a4,
    "tests/draft/angle/A4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("bx", &[10.0, 20.0, 30.0])),
        Step::Topo(Topo::NumberedExplode("bx", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "bx",
            direction: [1.0, 0.0, 0.0],
            faces: &[
                ("bx_2", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_5", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_4", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("bx_3", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2084.26, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a5,
    "tests/draft/angle/A5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("p", &[10.0, 25.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [1.0, 0.0, 0.0],
            faces: &[("p_2", 5.0, [-10.0, 0.0, 25.0, 1.0, 0.0, 0.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2145.34, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a6,
    "tests/draft/angle/A6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("p", &[10.0, 25.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [1.0, 0.0, 0.0],
            faces: &[
                ("p_2", 10.0, [0.0, 0.0, 25.0, 1.0, 1.0, 0.0]),
                ("p_1", -10.0, [0.0, 0.0, 0.0, 1.0, 1.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2218.96, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a7,
    "tests/draft/angle/A7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("p", &[10.0, 25.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_3", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1910.28, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a8,
    "tests/draft/angle/A8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("p", &[10.0, 25.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_3", 5.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2150.96, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_a9,
    "tests/draft/angle/A9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cone("p", &[0.0, 10.0, 20.0])),
        Step::Topo(Topo::Explode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [1.0, 0.0, 0.0],
            faces: &[("p_2", 5.0, [0.0, 0.0, 0.0, 1.0, 1.0, 0.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1024.98, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b1,
    "tests/draft/angle/B1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cone("p", &[10.0, 2.0, 20.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [1.0, 1.0, 0.0],
            faces: &[
                ("p_2", 5.0, [-2.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
                ("p_1", 5.0, [20.0, 0.0, 0.0, 0.0, 1.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1143.37, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b2,
    "tests/draft/angle/B2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cone("p", &[10.0, 2.0, 20.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_3", -5.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 987.866, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b3,
    "tests/draft/angle/B3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cone("p", &[10.0, 2.0, 20.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_3", 0.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1138.79, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b4,
    "tests/draft/angle/B4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Wedge("p", &[1.0, 1.0, 5.0, 5.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [1.0, 0.0, 0.0],
            faces: &[("p_4", 5.0, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 59.4075, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b5,
    "tests/draft/angle/B5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Wedge("p", &[1.0, 1.0, 5.0, 5.0])),
        Step::Topo(Topo::NumberedExplode("p", "e")),
        Step::Fillet(Fillet::Blend("p", "p", &[(0.2, "p_12"), (0.5, "p_1")])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_8", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_2", 3.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 42.9257, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b6,
    "tests/draft/angle/B6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Wedge("p", &[2.0, 2.0, 4.0, 4.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 1.0, 0.0],
            faces: &[
                ("p_4", 5.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
                ("p_6", 5.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
                ("p_3", 5.0, [0.0, 2.0, 0.0, 0.0, 1.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 39.0405, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b7,
    "tests/draft/angle/B7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Wedge("p", &[2.0, 2.0, 4.0, 4.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 1.0, 0.0],
            faces: &[
                ("p_4", 5.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
                ("p_6", 5.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
                ("p_3", 5.0, [0.0, 2.0, 0.0, 0.0, 1.0, 0.0]),
                ("p_1", 5.0, [0.0, 0.0, 0.0, 0.0, 1.0, 0.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 38.0505, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b8,
    "tests/draft/angle/B8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "d", "1.0", "0.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "x", "-15.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 0.0, 12.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_2", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1083.89, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_b9,
    "tests/draft/angle/B9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "d", "1.0", "0.0", "l", "10.0", "c", "5.0", "90.0", "l", "5.0", "c", "2.0", "90.0",
                "l", "13.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 0.0, 12.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_8", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_1", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 885.039, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c1,
    "tests/draft/angle/C1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "t", "5.0", "-5.0", "c", "5.0", "90.0", "t", "5.0", "5.0", "t", "0.0", "5.0", "t",
                "-5.0", "5.0", "c", "5.0", "90.0", "t", "-5.0", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 0.0, 12.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_1", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_4", 4.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_7", 7.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1005.01, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c2,
    "tests/draft/angle/C2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "t", "5.0", "-5.0", "c", "5.0", "90.0", "t", "5.0", "5.0", "t", "0.0", "5.0", "t",
                "-5.0", "5.0", "c", "5.0", "90.0", "t", "-5.0", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 0.0, 12.0])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_1", 10.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_4", 4.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_7", 7.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_10", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 987.189, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c3,
    "tests/draft/angle/C3",
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
                2.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 20.0, 2.0, 0.0, 20.0, 2.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Revol(
            "p",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 180.0]
        )),
        Step::Topo(Topo::NumberedExplode("p", "e")),
        Step::Fillet(Fillet::Blend("p", "p", &[(2.0, "p_1"), (2.0, "p_12")])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_8", 3.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1198.82, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c4,
    "tests/draft/angle/C4",
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
                2.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 20.0, 2.0, 0.0, 20.0, 2.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("p", "p", false)),
        Step::Sweep(Sweep::Revol(
            "p",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 180.0]
        )),
        Step::Topo(Topo::NumberedExplode("p", "e")),
        Step::Fillet(Fillet::Blend("p", "p", &[(2.0, "p_12"), (1.0, "p_4")])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_8", 2.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_1", 1.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1273.54, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c5,
    "tests/draft/angle/C5",
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
                2.0, 0.0, 0.0, 15.0, 0.0, 0.0, 15.0, 0.0, 20.0, 2.0, 0.0, 20.0, 2.0, 0.0, 0.0
            ]
        )),
        Step::More(More::Moves(
            "p",
            &[
                "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "f", "2.0", "0.0", "l", "15.0",
                "t", "0.0", "20.0", "t", "-5.0", "0.0", "t", "0.0", "-18.0", "t", "-5.0", "0.0",
                "t", "0.0", "18.0", "t", "-5.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "p",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 180.0]
        )),
        Step::Topo(Topo::NumberedExplode("p", "e")),
        Step::Fillet(Fillet::Blend("p", "p", &[(2.0, "p_8"), (2.0, "p_24")])),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_2", 2.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_12", 3.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_3", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3349.7, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c6,
    "tests/draft/angle/C6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "l", "5.0", "c", "2.0", "90.0", "l", "5.0", "c", "2.0", "90.0", "l", "5.0", "c",
                "2.0", "90.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 0.0, 0.0, 10.0])),
        Step::Sweep(Sweep::Pipe("p", "w", "p", 0, false)),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[("p_7", -5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 540.527, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c7,
    "tests/draft/angle/C7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "l", "7.0", "c", "2.0", "135.0", "c", "-2.0", "90.0", "c", "2.0", "135.0", "l",
                "7.0"
            ]
        )),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 0.0, 0.0, 12.0])),
        Step::Sweep(Sweep::Pipe("p", "w", "p", 0, false)),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_8", 4.0, [0.0, 0.0, 10.0, 0.0, 0.0, 1.0]),
                ("p_1", 3.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 618.869, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c8,
    "tests/draft/angle/C8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "d", "1.0", "0.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "x", "-15.0"
            ]
        )),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 0.0, 0.0, 10.0])),
        Step::Sweep(Sweep::Pipe("p", "w", "p", 0, false)),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_1", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_5", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_6", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 939.057, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_c9,
    "tests/draft/angle/C9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "c", "5.0", "180.0", "l", "5.0", "t", "0.0", "5.0", "d", "-1.0", "0.0", "c", "5.0",
                "180.0", "l", "5.0"
            ]
        )),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 0.0, 0.0, 10.0])),
        Step::Sweep(Sweep::Pipe("p", "w", "p", 0, false)),
        Step::Topo(Topo::NumberedExplode("p", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "p",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("p_1", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_7", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_8", 5.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
                ("p_2", 1.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 659.647, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d1,
    "tests/draft/angle/D1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[10.0, 10.0, 10.0, 20.0, 20.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "b1", "b2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 5.0, [20.0, 20.0, 20.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5592.02, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d2,
    "tests/draft/angle/D2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 15.0])),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[20.0, 20.0, 5.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[15.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "b1", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_6", 5.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6488.81, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d3,
    "tests/draft/angle/D3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 15.0])),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[20.0, 20.0, 5.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[17.0, 15.0, 20.0]
        )),
        Step::Bool(Bool::Fuse("f", "b1", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_6", 15.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6387.78, 0.01, None)),
    ]
);
