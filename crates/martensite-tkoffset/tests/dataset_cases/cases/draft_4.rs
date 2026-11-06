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
    draft_angle_k1,
    "tests/draft/angle/K1",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Sphere("p", &[20.0, 0.0, 90.0])),
        Step::Prim(Prim::Box("bx", &[5.0, 0.5, 10.0, 10.0, 5.0, 20.0])),
        Step::Bool(Bool::Fuse("f", "p", "bx")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_5", 5.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 4173.36, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k2,
    "tests/draft/angle/K2",
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
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "p", "p", &[20.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[0.0, 0.0, 15.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p2", "p2", &[7.0, 15.0])),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 12.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5536.07, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k3,
    "tests/draft/angle/K3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "p",
            &[0.0, 0.0, 0.0, 0.0, 0.5, -1.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "p", "p", &[20.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[0.0, 0.0, 15.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p2",
            "p2",
            &[9.0, 7.0, 15.0]
        )),
        Step::Bool(Bool::Fuse("f", "p", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_1", 12.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5536.05, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k4,
    "tests/draft/angle/K4",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Surface(Surface::Plane(
            "ps",
            &[10.0, -3.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.2, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "ps", "ps", &[20.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "1.0", "5.0", "l", "10.0", "c", "2.0", "90.0", "l", "5.0", "d", "-1.0",
                "0.0", "l", "14.0", "d", "0.0", "-1.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 30.0])),
        Step::Bool(Bool::Fuse("f", "ps", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 4.0, [0.0, 0.0, 30.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5719.09, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k5,
    "tests/draft/angle/K5",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_"),
        Step::More(More::Surface(Surface::Plane(
            "ps",
            &[10.0, -3.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.2, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Sphere, "ps", "ps", &[20.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "5.0", "-0.5", "10.0", "l", "10.0", "c", "5.0", "90.0", "l", "5.0", "d",
                "-1.0", "0.0", "l", "14.0", "d", "0.0", "-1.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 10.0, 0.0, 0.0, 35.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "ps", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", -5.0, [0.0, 0.0, 35.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 5465.24, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k6,
    "tests/draft/angle/K6",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Torus("pt", &[9.0, 7.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[8.0, 4.0, 4.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Cylinder, "p2", "p2", &[3.0, 10.0])),
        Step::Bool(Bool::Fuse("f", "pt", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_2", 10.0, [0.0, 0.0, 15.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2642.63, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k7,
    "tests/draft/angle/K7",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 22803 Linux Windows: Error : The area of result shape is"),
        Step::More(More::Surface(Surface::Plane(
            "pt",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Torus, "pt", "pt", &[9.0, 7.0])),
        Step::More(More::Surface(Surface::Plane(
            "p2",
            &[2.0, 2.0, 10.0, 0.0, 0.0, 1.0]
        ))),
        Step::Prim(Prim::OnPlane(
            PlaneKind::Cone,
            "p2",
            "p2",
            &[2.0, 3.0, 15.0]
        )),
        Step::Bool(Bool::Fuse("f", "pt", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 1.0, [0.0, 0.0, 25.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 2046.52, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k8,
    "tests/draft/angle/K8",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to"),
        Step::More(More::Surface(Surface::Plane(
            "pt",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Torus, "pt", "pt", &[15.0, 12.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "-5.0", "0.5", "15.0", "l", "10.0", "c", "2.0", "90.0", "l", "5.0", "d",
                "-1.0", "0.0", "l", "14.0", "d", "0.0", "-1.0", "l", "5.0", "c", "2.0", "90.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pf", [0.0, 0.0, 25.0])),
        Step::Bool(Bool::Fuse("f", "pt", "pr")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_4", 5.0, [0.0, 0.0, 40.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 9447.73, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_k9,
    "tests/draft/angle/K9",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to"),
        Step::More(More::Surface(Surface::Plane(
            "pt",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        ))),
        Step::Prim(Prim::OnPlane(PlaneKind::Torus, "pt", "pt", &[15.0, 15.0])),
        Step::More(More::Moves(
            "pf",
            &[
                "o", "-8.0", "-12.0", "15.0", "l", "5.0", "c", "2.0", "90.0", "l", "20.0", "c",
                "2.0", "90.0", "l", "5.0"
            ]
        )),
        Step::Profile(Profile::Polyline("pl", &[0.0, 0.0, 15.0, 0.0, 0.0, 45.0])),
        Step::Sweep(Sweep::Pipe("p2", "pl", "pf", 0, false)),
        Step::Bool(Bool::Fuse("f", "pt", "p2")),
        Step::Topo(Topo::NumberedExplode("f", "f")),
        Step::Draft(Draft {
            result: "result",
            shape: "f",
            direction: [0.0, 0.0, 1.0],
            faces: &[("f_6", 5.0, [0.0, 0.0, 44.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 9979.48, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l2,
    "tests/draft/angle/L2",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CFE903_cts21ggr.rle", "base")),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Topo(Topo::SharedCopy("base_14", "facdep")),
        Step::Profile(Profile::Vertex(
            "v1",
            [12.9980200374864, -13.0, 16.2268812577167]
        )),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, -1.0, 0.0],
            faces: &[(
                "facdep",
                2.0,
                [12.9980200374864, -13.0, 16.2268812577167, 0.0, -1.0, 0.0]
            )]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 8004.37, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l3,
    "tests/draft/angle/L3",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Cylinder("base", &[5.0, 10.0])),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, 1.0],
            faces: &[("base_1", 0.41, [5.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 466.767, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l4,
    "tests/draft/angle/L4",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Topo(Topo::Restore("brep/CFE903_cts20ggq.rle", "base")),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [1.0, 0.0, 0.0],
            faces: &[
                ("base_2", 5.0, [0.0, -250.0, 250.0, 1.0, 0.0, 0.0]),
                ("base_3", 5.0, [0.0, -250.0, 250.0, 1.0, 0.0, 0.0]),
                ("base_4", 5.0, [0.0, -250.0, 250.0, 1.0, 0.0, 0.0]),
                ("base_5", 5.0, [0.0, -250.0, 250.0, 1.0, 0.0, 0.0])
            ]
        }),
        Step::Topo(Topo::ForceSameParameter("result", None)),
        Step::Check(Check::Props(Prop::Area, "result", 688897.0, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l5,
    "tests/draft/angle/L5",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CFE903_pro16ggt.rle", "base")),
        Step::Topo(Topo::NumberedExplode("base", "face")),
        Step::Profile(Profile::Vertex(
            "v1",
            [56.3468437194825, 6.86845636367778, 100.0]
        )),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, -1.0],
            faces: &[(
                "base_5",
                1.0,
                [56.3468437194825, 6.86845636367778, 100.0, 0.0, 0.0, -1.0]
            )]
        }),
        Step::Topo(Topo::ForceSameParameter("result", None)),
        Step::Check(Check::Props(Prop::Area, "result", 39615.1, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l7,
    "tests/draft/angle/L7",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CFE903_pro16ggz.rle", "base")),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, 1.0],
            faces: &[
                ("base_3", 1.0, [-20.0, -120.0, 52.00897, 0.0, 0.0, 1.0]),
                ("base_4", 1.0, [-20.0, -120.0, 52.00897, 0.0, 0.0, 1.0])
            ]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 9217.99, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_l9,
    "tests/draft/angle/L9",
    Draft,
    600,
    End::Draft,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CFE903_cts21ggu.rle", "base")),
        Step::Topo(Topo::NumberedExplode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, 1.0],
            faces: &[("base_9", 3.0, [0.0, 0.0, 81.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 180498.0, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_m1,
    "tests/draft/angle/M1",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Topo(Topo::Restore("brep/ger60805_base.brep", "base")),
        Step::Topo(Topo::ForceSameParameter("base", None)),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, 1.0],
            faces: &[("base_2", 10.0, [0.0, 0.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 278770.0, 0.01, None)),
    ]
);

offset_case!(
    draft_angle_m4,
    "tests/draft/angle/M4",
    Draft,
    600,
    End::Draft,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22803 All:Faulty shapes in variables faulty_1 to faulty_"),
        Step::Topo(Topo::Restore("brep/CFE903_pro16gha.rle", "base")),
        Step::Topo(Topo::Explode("base", "face")),
        Step::Draft(Draft {
            result: "result",
            shape: "base",
            direction: [0.0, 0.0, -1.0],
            faces: &[(
                "base_1",
                1.0,
                [115.16843, -2.03233999999999, -119.85872, 0.0, 0.0, 1.0]
            )]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10170.4, 0.01, None)),
    ]
);
