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
    draft_angle_d4,
    "tests/draft/angle/D4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 15.0])),
        Step::More(More::Moves(
            "p",
            &[
                "o", "15.0", "10.0", "5.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "c",
                "5.0", "90.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "c", "5.0", "90.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "p", [0.0, 0.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "b1", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_12", 10.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6245.73, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d5,
    "tests/draft/angle/D5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 15.0])),
        Step::More(More::Moves(
            "p",
            &[
                "o", "15.0", "10.0", "5.0", "l", "10.0", "c", "10.0", "90.0", "l", "10.0", "c",
                "5.0", "90.0", "l", "10.0", "c", "10.0", "90.0", "l", "10.0", "c", "5.0", "90.0"
            ]
        )),
        Step::Profile(Profile::Polyline("p2", &[0.0, 0.0, 5.0, 0.0, 0.0, 25.0])),
        Step::Sweep(Sweep::Pipe("p", "p2", "p", 0, false)),
        Step::Bool(Bool::Fuse("f", "b1", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_12", 10.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6377.54, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d6,
    "tests/draft/angle/D6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b1", &[40.0, 40.0, 15.0])),
        Step::More(More::Moves(
            "p",
            &[
                "o", "25.0", "20.0", "10.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "10.0", "t", "0.0", "30.0", "t", "-7.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "p",
            "p",
            [20.0, 20.0, 0.0, 0.0, 0.0, 1.0, 330.0]
        )),
        Step::Bool(Bool::Fuse("f", "b1", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_8", 3.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0]),
                ("f_7", 5.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 9087.81, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d7,
    "tests/draft/angle/D7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("p", &[10.0, 20.0])),
        Step::Place(Place::Rotate(&["p"], [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 90.0])),
        Step::Prim(Prim::Box("bx", &[5.0, -5.0, 0.0, 10.0, 10.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_3", 5.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0]),
                ("f_7", 3.0, [0.0, 0.0, 20.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2323.25, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d8,
    "tests/draft/angle/D8",
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
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[10.0, 0.0, 5.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p2", "p2", &[5.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 5.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2056.88, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_d9,
    "tests/draft/angle/D9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[10.0, 20.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[10.0, 0.0, 5.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p2",
            "p2",
            &[7.0, 5.0, 20.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 10.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2441.77, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e1,
    "tests/draft/angle/E1",
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
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[15.0, 30.0])),
        Step::More(More::Moves(
            "p2",
            &[
                "o", "10.0", "-10.0", "5.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "d",
                "-1.0", "0.0", "l", "20.0", "d", "0.0", "-1.0", "l", "10.0", "c", "5.0", "90.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p2", "p2", [0.0, 0.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 1.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4980.52, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e2,
    "tests/draft/angle/E2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[25.0, 35.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "12.0", "-10.0", "20.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "c",
                "5.0", "90.0", "l", "10.0", "c", "5.0", "90.0", "l", "10.0", "c", "5.0", "90.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 20.0, 0.0, 0.0, 40.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_11", 2.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10572.7, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e3,
    "tests/draft/angle/E3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[20.0, 30.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "20.0", "4.0", "10.0", "p", "1.0", "0.0", "0.0", "0.0", "1.0", "0.0", "l",
                "7.0", "t", "0.0", "30.0", "t", "-7.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "p2",
            "pf",
            [20.0, 0.0, 0.0, 0.0, 0.0, 1.0, 170.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_3", 3.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 7525.77, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e4,
    "tests/draft/angle/E4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[-10.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "pn",
            "p",
            &[5.0, 10.0, 20.0]
        )),
        Step::Prim(Prim::Box("bx", &[-5.0, -5.0, 0.0, 10.0, 10.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "bx", "pn")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_7", 5.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0]),
                ("f_3", 4.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1703.96, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e5,
    "tests/draft/angle/E5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[15.0, 18.0, 20.0]
        )),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[10.0, 0.0, 15.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p2", "p2", &[5.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 5.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4099.12, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e6,
    "tests/draft/angle/E6",
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
            &[15.0, 18.0, 20.0, 90.0]
        )),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[10.0, -7.0, 10.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p2",
            "p2",
            &[5.0, 4.0, 20.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_6", 1.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2002.16, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e7,
    "tests/draft/angle/E7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[15.0, 18.0, 25.0, 180.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "-7.0", "10.0", "l", "10.0", "c", "2.0", "90.0", "l", "10.0", "d",
                "-1.0", "0.0", "l", "12.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p2", "pf", [0.0, 0.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_10", 2.0, [5.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3673.56, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e8,
    "tests/draft/angle/E8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[15.0, 18.0, 30.0, 180.0]
        )),
        Step::More(More::Moves(
            "p2",
            &[
                "o", "5.0", "-7.0", "10.0", "d", "1.0", "0.0", "l", "10.0", "c", "5.0", "90.0",
                "l", "10.0", "x", "-15.0"
            ]
        )),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 10.0, 0.0, 0.0, 25.0])),
        Step::Sweep(Sweep::Pipe("p2", "w", "p2", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_9", 4.0, [0.0, 0.0, 22.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3972.17, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_e9,
    "tests/draft/angle/E9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p",
            "p",
            &[15.0, 17.0, 25.0, 180.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "15.0", "4.0", "10.0", "p", "1.0", "0.0", "0.0", "0.0", "1.0", "0.0", "l",
                "5.0", "t", "0.0", "20.0", "t", "-5.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "p2",
            "pf",
            [15.0, 0.0, 0.0, 0.0, 0.0, 1.0, 170.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_8", -3.0, [5.0, 0.0, 30.0, 0.0, 0.0, 1.0]),
                ("f_6", -3.0, [5.0, 0.0, 30.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 3608.6, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f1,
    "tests/draft/angle/F1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Sphere("p", &[20.0, 0.0, 90.0])),
        Step::Prim(Prim::Box("p2", &[-10.0, -10.0, 10.0, 20.0, 20.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "e")),
        Step::Fillet(Fillet::Blend("f", "f", &[(2.0, "f_10")])),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_8", 2.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4827.83, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f2,
    "tests/draft/angle/F2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "p", "p", &[20.0])),
        Step::Prim(Prim::Cylinder("p2", &[10.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 5.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6506.68, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f3,
    "tests/draft/angle/F3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "p", "p", &[20.0])),
        Step::Prim(Prim::Cone("p2", &[10.0, 12.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", -10.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6658.61, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f4,
    "tests/draft/angle/F4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Sphere,
            "p",
            "p",
            &[20.0, 0.0, 90.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "-4.0", "-4.0", "1.0", "l", "8.0", "c", "2.0", "90.0", "l", "8.0", "d",
                "-1.0", "0.0", "l", "8.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p2", "pf", [0.0, 0.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4630.78, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f5,
    "tests/draft/angle/F5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Sphere,
            "p",
            "p",
            &[20.0, 0.0, 90.0]
        )),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "2.0", "-5.0", "10.0", "l", "10.0", "d", "0.0", "1.0", "l", "10.0", "d",
                "-1.0", "0.0", "l", "10.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 10.0, 0.0, 0.0, 35.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 32.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4466.62, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f6,
    "tests/draft/angle/F6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Sphere,
            "p",
            "p",
            &[20.0, 0.0, 90.0]
        )),
        Step::More(More::Moves(
            "pr",
            &[
                "o", "2.0", "0.0", "5.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "10.0", "t", "0.0", "30.0", "t", "-7.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pr",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", -10.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5966.54, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f7,
    "tests/draft/angle/F7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Torus("p", &[25.0, 20.0, 90.0])),
        Step::Prim(Prim::Box("bx", &[10.0, 10.0, 10.0, 18.0, 18.0, 30.0])),
        Step::Bool(Bool::Fuse("f", "bx", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_3", 5.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 9010.79, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f8,
    "tests/draft/angle/F8",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "pt",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Torus, "pt", "pt", &[25.0, 24.0])),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 40.0, 0.0, 0.0, 1.0, 1.0, -1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p", "p", &[10.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "pt", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_3", 5.0, [0.0, 0.0, 80.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 25894.6, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_f9,
    "tests/draft/angle/F9",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Puts("TODO 22803 All :Error : The area of result shape is"),
        Step::Prim(Prim::Torus("pt", &[25.0, 24.0, 90.0])),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[20.0, 20.0, 10.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cone, "p", "p", &[10.0, 8.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "pt", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 1.0, [0.0, 0.0, 50.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g1,
    "tests/draft/angle/G1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Torus("pt", &[25.0, 24.0, 90.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "15.0", "15.0", "10.0", "l", "5.0", "c", "5.0", "90.0", "l", "5.0", "d",
                "-1.0", "0.0", "l", "10.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("p", "pf", [0.0, 0.0, 40.0])),
        Step::Bool(Bool::Fuse("f", "pt", "p")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_7", 5.0, [0.0, 0.0, 50.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10629.3, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g2,
    "tests/draft/angle/G2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Torus("pt", &[25.0, 24.0, 90.0])),
        Step::More(More::Moves(
            "pr",
            &[
                "o", "20.0", "18.0", "5.0", "p", "0.0", "-1.0", "0.0", "1.0", "0.0", "0.0", "l",
                "10.0", "t", "0.0", "30.0", "t", "-7.0", "0.0"
            ]
        )),
        Step::Sweep(Sweep::Revol(
            "pr",
            "pr",
            [18.0, 18.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Bool(Bool::Fuse("f", "pt", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("f_5", 3.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0]),
                ("f_6", 3.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10776.9, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g3,
    "tests/draft/angle/G3",
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
        Step::Curve(Curve::BSpline(
            "bc",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 10.0, 0.0, 7.0, 1.0, 7.0, 0.0, 7.0, 1.0,
                3.0, 0.0, 8.0, 1.0, 0.0, 0.0, 7.0, 1.0
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
        Step::Prim(Prim::Box("bx", &[-4.0, -4.0, 4.0, 8.0, 8.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 2.0, [0.0, 0.0, 14.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1276.03, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g4,
    "tests/draft/angle/G4",
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
                4.0, 10.0, 0.0, 7.0, 7.0, 0.0, 8.0, 3.0, 0.0, 7.0, 0.0, 0.0, 7.0
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
        Step::Prim(Prim::Cylinder("pc", &[5.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 3.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1306.33, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g5,
    "tests/draft/angle/G5",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All: Error in depouille"),
        Step::Puts("TODO 22803 All: Error : The skin cannot be built."),
        Step::Puts("TODO 22803 All: Error: The command cannot be built"),
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
            "p",
            "p",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Prim(Prim::Cone("pc", &[5.0, 4.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "p", "pc")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::If(
            Cond::Failed(&Step::Draft(Draft {
                result: "result",
                shape: "f",
                direction: [0.0, 0.0, 1.0],
                faces: &[("f_4", 1.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
            })),
            &[Step::Puts("Error in depouille")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_g6,
    "tests/draft/angle/G6",
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
                4.0, 10.0, 0.0, 7.0, 7.0, 0.0, 5.0, 3.0, 0.0, 7.0, 0.0, 0.0, 7.0
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
                "o", "-4.0", "-4.0", "0.0", "l", "8.0", "d", "0.0", "1.0", "l", "8.0", "d", "-1.0",
                "0.0", "l", "8.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "p", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 3.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 1357.29, 0.01, None)),
    ]
);
