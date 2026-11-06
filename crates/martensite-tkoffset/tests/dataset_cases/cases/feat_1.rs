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
    feat_featlf_a2,
    "tests/feat/featlf/A2",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[1.0, 1.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("b", "f")),
        Step::Draft(Draft {
            result: "b",
            shape: "b",
            direction: [1.0, 0.0, 0.0],
            faces: &[("b_4", 3.0, [0.5, 0.5, 1.0, 1.0, 0.0, 0.0])]
        }),
        Step::More(More::Moves("pp", &["c", "0.3", "180.0", "ww"])),
        Step::Place(Place::Rotate(
            &["pp"],
            [0.0, 0.15, 0.0, 0.0, 1.0, 0.0, -90.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.3, 0.3, 0.9])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.3, 0.6, 1.0, -1.0, 0.0, 0.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "b",
            wire: "pp",
            plane: "pl",
            direct: [0.3, 0.0, 0.0],
            other: [-0.2, 0.0, 0.0],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 6.23832, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_b4,
    "tests/feat/featlf/B4",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[1.0, 1.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("b", "f")),
        Step::Draft(Draft {
            result: "b",
            shape: "b",
            direction: [0.0, 1.0, 0.0],
            faces: &[("b_6", 10.0, [0.5, 0.5, 1.0, 0.0, 1.0, 0.0])]
        }),
        Step::Profile(Profile::Vertex("v1", [0.5, 0.5, 1.2])),
        Step::Profile(Profile::Vertex("v2", [0.5, 0.5, 0.5])),
        Step::Profile(Profile::Vertex("v3", [1.2, 0.5, 0.5])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::Wire("w", &["e1", "e2"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.5, 0.5, 0.5, 0.0, 1.0, 0.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "b",
            wire: "w",
            plane: "pl",
            direct: [0.0, 0.3, 0.0],
            other: [0.0, 0.0, 0.0],
            fuse: 0,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 6.88666, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_c3,
    "tests/feat/featlf/C3",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "pr",
            &["x", "1.0", "y", "1.0", "x", "-0.5", "tt", "0.0", "1.2"]
        )),
        Step::Sweep(Sweep::Prism("pr", "pr", [0.0, 0.0, 1.0])),
        Step::Place(Place::Rotate(&["pr"], [0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Place(Place::Translate(&["pr"], [0.0, 1.0, 0.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_6", 10.0, [1.0, 0.5, 1.0, 0.0, 1.0, 0.0])]
        }),
        Step::More(More::Moves("pp", &["c", "0.3", "280.0", "ww"])),
        Step::Place(Place::Rotate(
            &["pp"],
            [0.0, 0.15, 0.0, 0.0, 1.0, 0.0, -90.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.3, 0.3, 0.9])),
        Step::Place(Place::Rotate(
            &["pp"],
            [0.3, 0.3, 0.9, 0.0, 0.0, 1.0, -90.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.4, 0.0, 0.0])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[1.0, 0.3, 1.0, 0.0, 1.0, 0.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.3, 0.0],
            other: [0.0, 0.0, 0.0],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 6.90921, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_c4,
    "tests/feat/featlf/C4",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "pr",
            &["x", "1.0", "y", "1.0", "x", "-0.5", "tt", "0.0", "1.2"]
        )),
        Step::Sweep(Sweep::Prism("pr", "pr", [0.0, 0.0, 1.0])),
        Step::Place(Place::Rotate(&["pr"], [0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Place(Place::Translate(&["pr"], [0.0, 1.0, 0.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_6", 10.0, [1.0, 0.0, 1.0, 0.0, 1.0, 0.0])]
        }),
        Step::More(More::Moves("pp", &["c", "0.3", "280.0", "ww"])),
        Step::Place(Place::Rotate(
            &["pp"],
            [0.0, 0.15, 0.0, 0.0, 1.0, 0.0, -90.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.3, 0.3, 0.9])),
        Step::Place(Place::Rotate(
            &["pp"],
            [0.3, 0.3, 0.9, 0.0, 0.0, 1.0, -90.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.4, 0.0, 0.0])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[1.0, 0.3, 1.0, 0.0, 1.0, 0.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.3, 0.0],
            other: [0.0, 0.0, 0.0],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 6.86189, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_c8,
    "tests/feat/featlf/C8",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-4.0", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 0.0, 1.0],
            faces: &[("pr_3", 10.0, [1.0, 3.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::More(More::Moves("pp", &["x", "2.0", "y", "-2.0", "ww"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [0.5, 2.5, 0.3])),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.2],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 42.03, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_c9,
    "tests/feat/featlf/C9",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-4.0", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 0.0, 1.0],
            faces: &[("pr_3", 10.0, [1.0, 3.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::More(More::Moves(
            "pp",
            &["x", "2.0", "c", "1.0", "90.0", "y", "2.0", "ww"]
        )),
        Step::Place(Place::Rotate(
            &["pp"],
            [3.0, 3.0, 0.0, -1.0, 0.0, 0.0, 180.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.5, -2.5, 0.3])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.2],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 49.5019, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_d2,
    "tests/feat/featlf/D2",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-4.0", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_4", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::More(More::Moves("pp", &["x", "2.0", "y", "-2.0", "ww"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [0.5, 2.5, 0.3])),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.1],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 43.7592, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_d3,
    "tests/feat/featlf/D3",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-4.0", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_4", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend("pr", "pr", &[(0.3, "pr_7")])),
        Step::More(More::Moves("pp", &["x", "2.0", "y", "-2.0", "ww"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [0.5, 2.5, 0.3])),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.1],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 43.6826, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_d4,
    "tests/feat/featlf/D4",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-4.0", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_4", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend("pr", "pr", &[(0.3, "pr_7")])),
        Step::More(More::Moves("pp", &["x", "4.0", "y", "-4.0", "ww"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [1.0, 5.0, 0.3])),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.1],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 71.1826, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_d6,
    "tests/feat/featlf/D6",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "pro",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "pro", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 0.0, 1.0],
            faces: &[("pr_5", 10.0, [1.0, 3.0, 0.0, 0.0, 0.0, 1.0])]
        }),
        Step::More(More::Moves(
            "pp",
            &["x", "2.0", "c", "1.0", "90.0", "y", "2.0", "ww"]
        )),
        Step::Place(Place::Rotate(
            &["pp"],
            [3.0, 3.0, 0.0, -1.0, 0.0, 0.0, 180.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.5, -2.5, 0.3])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.2],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 50.3873, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_e1,
    "tests/feat/featlf/E1",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_8", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend(
            "pr",
            "pr",
            &[
                (0.1, "pr_20"),
                (0.1, "pr_17"),
                (0.1, "pr_14"),
                (0.1, "pr_11"),
                (0.1, "pr_8")
            ]
        )),
        Step::More(More::Moves(
            "pp",
            &["x", "3.0", "c", "1.0", "90.0", "y", "3.0", "ww"]
        )),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [1.0, 5.0, 0.3])),
        Step::Place(Place::Rotate(
            &["pp"],
            [5.0, 5.0, 0.3, 1.0, 0.0, 0.0, 180.0]
        )),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.2],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 71.1883, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_e2,
    "tests/feat/featlf/E2",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_5", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend(
            "pr",
            "pr",
            &[
                (0.1, "pr_13"),
                (0.1, "pr_8"),
                (0.1, "pr_5"),
                (0.1, "pr_19"),
                (0.1, "pr_22"),
                (0.3, "pr_30"),
                (0.3, "pr_18")
            ]
        )),
        Step::More(More::Moves(
            "pp",
            &["x", "3.3", "c", "1.0", "90.0", "y", "3.3", "ww"]
        )),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Translate(&["pp"], [0.5, 5.1, 0.3])),
        Step::Place(Place::Rotate(
            &["pp"],
            [5.0, 5.0, 0.3, 1.0, 0.0, 0.0, 180.0]
        )),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.2],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 67.8037, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_e3,
    "tests/feat/featlf/E3",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_5", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend(
            "pr",
            "pr",
            &[
                (0.1, "pr_13"),
                (0.1, "pr_8"),
                (0.1, "pr_5"),
                (0.1, "pr_19"),
                (0.1, "pr_22"),
                (0.3, "pr_30"),
                (0.3, "pr_18")
            ]
        )),
        Step::More(More::Moves("pp", &["x", "5.0", "ww"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.0, 0.0, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::Place(Place::Rotate(
            &["pp"],
            [5.0, 1.0, 0.0, 0.0, 0.0, 1.0, -45.0]
        )),
        Step::Place(Place::Translate(&["pp"], [0.3, 0.3, 0.3])),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "pp",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, -0.3],
            fuse: 1,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 48.2152, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_e4,
    "tests/feat/featlf/E4",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_7", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend(
            "pr",
            "pr",
            &[
                (0.1, "pr_20"),
                (0.1, "pr_17"),
                (0.1, "pr_14"),
                (0.1, "pr_11"),
                (0.1, "pr_8")
            ]
        )),
        Step::Profile(Profile::Vertex("v1", [0.2, 6.0, 0.3])),
        Step::Profile(Profile::Vertex("v2", [0.2, 0.2, 0.3])),
        Step::Profile(Profile::Vertex("v3", [3.0, 0.2, 0.3])),
        Step::Profile(Profile::Vertex("v4", [4.0, 2.0, 0.3])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v4")),
        Step::Profile(Profile::Wire("w", &["e1", "e2", "e3"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.2, 0.2, 0.3, 0.0, 0.0, -1.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "w",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, 0.0],
            fuse: 0,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 44.8099, 0.01, None)),
    ]
);

offset_case!(
    feat_featlf_e5,
    "tests/feat/featlf/E5",
    Feat,
    600,
    End::Feat,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "5.0", "y", "1.0", "x", "-3.0", "y", "-0.5", "x", "-1.5", "y", "0.5", "x",
                "0.5", "y", "4.0", "x", "-1.0", "y", "-5.0"
            ]
        )),
        Step::Sweep(Sweep::Prism("pr", "p", [0.0, 0.0, 1.0])),
        Step::Topo(Topo::NumberedExplode("pr", "f")),
        Step::Draft(Draft {
            result: "pr",
            shape: "pr",
            direction: [0.0, 1.0, 0.0],
            faces: &[("pr_7", -5.0, [1.0, 1.0, 0.0, 0.0, 1.0, 0.0])]
        }),
        Step::Topo(Topo::NumberedExplode("pr", "e")),
        Step::Fillet(Fillet::Blend(
            "pr",
            "pr",
            &[
                (0.1, "pr_20"),
                (0.1, "pr_17"),
                (0.1, "pr_14"),
                (0.1, "pr_11"),
                (0.1, "pr_8")
            ]
        )),
        Step::Profile(Profile::Vertex("v1", [0.2, 6.0, 0.3])),
        Step::Profile(Profile::Vertex("v2", [0.2, 0.2, 0.3])),
        Step::Profile(Profile::Vertex("v3", [3.0, 0.2, 0.3])),
        Step::Profile(Profile::Vertex("v4", [4.0, -2.0, 0.3])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v4")),
        Step::Profile(Profile::Wire("w", &["e1", "e2", "e3"])),
        Step::More(More::Surface(Surface::Plane(
            "pl",
            &[0.2, 0.2, 0.3, 0.0, 0.0, 1.0]
        ))),
        Step::More(More::LinearForm(LinearForm {
            base: "pr",
            wire: "w",
            plane: "pl",
            direct: [0.0, 0.0, 0.3],
            other: [0.0, 0.0, 0.0],
            fuse: 0,
            modify: true
        })),
        Step::More(More::PerformLinearForm("result")),
        Step::Check(Check::Props(Prop::Area, "result", 50.9297, 0.01, None)),
    ]
);
