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
    bugs_modalg_7_bug29861,
    "tests/bugs/modalg_7/bug29861",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("29861"),
        Step::Puts("============"),
        Step::More(More::Surface(Surface::Cylinder(
            "s1",
            &[0.0, 360.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            160.0
        ))),
        Step::More(More::Surface(Surface::Cylinder(
            "s2",
            &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            80.0
        ))),
        Step::More(More::Face(
            "f1",
            "s1",
            Some([0.0, 6.283185307179586, -113.2000012, 113.2000012])
        )),
        Step::More(More::Face(
            "f2",
            "s2",
            Some([0.0, 6.283185307179586, 168.4, 600.0])
        )),
        Step::More(More::Compound(&["f1", "f2"], "s")),
        Step::Extra(Extra::RollingBall("result", "s", 30.0, &["f1", "f2"])),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 10),
                ("EDGE", 17),
                ("WIRE", 7),
                ("FACE", 6),
                ("SHELL", 1)
            ]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 246256.0, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug30140,
    "tests/bugs/modalg_7/bug30140",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("0030140: Modeling Algorithms - Access Null Pointer"),
        Step::Puts("========"),
        Step::Curve(Curve::Line("l_path", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::More(More::TrimCurve("l_path", "l_path", [0.0, 10.0])),
        Step::Curve(Curve::Circle(
            "c_profile",
            &[0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "c_profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Extra(Extra::Point("dir", [0.0, 1.0, 0.0])),
        Step::Extra(Extra::OffsetCurve("offset_path", "l_path", 2.0, "dir")),
        Step::Profile(Profile::Edge("path", "offset_path", None)),
        Step::Profile(Profile::Wire("path", &["path"])),
        Step::Sweep(Sweep::Pipe("result", "path", "profile", 0, false)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 314.159, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug30150,
    "tests/bugs/modalg_7/bug30150",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts(
            "0030150: Modeling Algorithms - Removal of BRepAlgo_BooleanOperations and BRepAlgo_DSAccess classes"
        ),
        Step::Puts("========"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b1", "f")),
        Step::More(More::EmptyShape("sx", "Sh")),
        Step::Topo(Topo::Add("b1_1", "sx")),
        Step::Topo(Topo::Add("b1_2", "sx")),
        Step::Topo(Topo::Add("b1_3", "sx")),
        Step::Topo(Topo::Add("b1_4", "sx")),
        Step::Topo(Topo::Add("b1_5", "sx")),
        Step::Prim(Prim::Box("b2", &[-50.0, -50.0, 30.0, 100.0, 100.0, 10.0])),
        Step::Topo(Topo::Explode("b2", "f")),
        Step::Extra(Extra::Draft("r1", "sx", [0.0, 0.0, 1.0], 0.2, "b2", -1)),
        Step::Extra(Extra::Draft("r2", "sx", [0.0, 0.0, 1.0], 0.2, "b2_6", -1)),
        Step::Extra(Extra::Draft("r3", "sx", [0.0, 0.0, 1.0], 0.2, "b2", 0)),
        Step::Check(Check::Shape("r1")),
        Step::Let("answer_2_bopcheck", Value::Bopcheck("r1")),
        Step::If(
            Cond::Lacks("answer_2_bopcheck", "OK"),
            &[Step::Puts("Error: r1 is self-interfered shape")],
            &[]
        ),
        Step::Check(Check::Shape("r2")),
        Step::Let("answer_2_bopcheck", Value::Bopcheck("r2")),
        Step::If(
            Cond::Lacks("answer_2_bopcheck", "OK"),
            &[Step::Puts("Error: r2 is self-interfered shape")],
            &[]
        ),
        Step::Check(Check::Shape("r3")),
        Step::Let("answer_2_bopcheck", Value::Bopcheck("r3")),
        Step::If(
            Cond::Lacks("answer_2_bopcheck", "OK"),
            &[Step::Puts("Error: r3 is self-interfered shape")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "r1", 25319.3, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "r1", 105060.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "r1",
            &[
                ("VERTEX", 20),
                ("EDGE", 32),
                ("WIRE", 16),
                ("FACE", 15),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::Check(Check::Props(Prop::Area, "r2", 11977.8, 0.01, None)),
        Step::Check(Check::Counts(
            "r2",
            &[
                ("VERTEX", 16),
                ("EDGE", 24),
                ("WIRE", 11),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 0)
            ]
        )),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "r3", 1975.12, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "r3", 5059.99, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "r3",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("r3"),
    ]
);

offset_case!(
    bugs_modalg_7_bug30204_1,
    "tests/bugs/modalg_7/bug30204_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("30204"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug30204_path.brep", "path")),
        Step::Topo(Topo::Restore("brep/bug30204_base.brep", "base")),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::BiNormal([0.0, 1.0, 0.0])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "base",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug30204_2,
    "tests/bugs/modalg_7/bug30204_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("30204"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug30204_path.brep", "path")),
        Step::Topo(Topo::Restore("brep/bug30204_base.brep", "base")),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "base",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("SHELL", 1),
                ("FACE", 20),
                ("WIRE", 20),
                ("EDGE", 44),
                ("VERTEX", 24)
            ]
        )),
        Step::Let("tolres", Value::CheckedMaxTolerance("result")),
        Step::If(
            Cond::Above("tolres", 2e-07),
            &[Step::Puts("Error: bad tolerance of result")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 6851.98, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug30391_1,
    "tests/bugs/modalg_7/bug30391_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================================================================"),
        Step::Puts("30391: Improvement of the method BRepOffsetTool::EnLargeFace"),
        Step::Puts("================================================================"),
        Step::Topo(Topo::Restore("brep/bug30391_face1.brep", "a")),
        Step::Extra(Extra::EnlargeFace("result", "a", [0.0, 0.0, 20.0, 20.0])),
        Step::Check(Check::Props(Prop::Area, "result", 98393.6, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug30391_2,
    "tests/bugs/modalg_7/bug30391_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================================================================"),
        Step::Puts("30391: Improvement of the method BRepOffsetTool::EnLargeFace"),
        Step::Puts("================================================================"),
        Step::Topo(Topo::Restore("brep/bug30391_face2.brep", "a")),
        Step::Extra(Extra::EnlargeFace("result", "a", [0.0, 10.0, 10.0, 0.0])),
        Step::Check(Check::Props(Prop::Area, "result", 172862.0, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug30391_3,
    "tests/bugs/modalg_7/bug30391_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================================================================"),
        Step::Puts("30391: Improvement of the method BRepOffsetTool::EnLargeFace"),
        Step::Puts("================================================================"),
        Step::Topo(Topo::Restore("brep/bug30391_face3.brep", "a")),
        Step::Extra(Extra::EnlargeFace("result", "a", [1.0, 1.0, 1.0, 1.0])),
        Step::Check(Check::Props(Prop::Area, "result", 1.60173, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_7_bug30795,
    "tests/bugs/modalg_7/bug30795",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts(
            "0030795: BRepOffsetAPIMakePipeShell: hangs on the attached model and produces infinite rails"
        ),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug30795.brep", "a")),
        Step::Topo(Topo::ExplodeChildren("a")),
        Step::Shell(PipeShell::Start("a_1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "a_2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: true
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug30958,
    "tests/bugs/modalg_7/bug30958",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts(
            "=========================================================================================="
        ),
        Step::Puts(
            "30958: BRepOffsetAPIThruSections operation raises an except-ion while building history"
        ),
        Step::Puts(
            "=========================================================================================="
        ),
        Step::Topo(Topo::Restore("brep/bug30958_CompWires.brep", "ww")),
        Step::Topo(Topo::ExplodeChildren("ww")),
        Step::Loft(Loft {
            result: "res",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["ww_1", "ww_2"]
        }),
        Step::More(More::History(HistoryStep::Save("hist"))),
        Step::Topo(Topo::ExplodeChildren("ww_1")),
        Step::More(More::History(HistoryStep::Generated(
            "r1", "hist", "ww_1_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2", "hist", "ww_1_2"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 22.4404, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 81.2396, 0.01, None)),
        Step::Topo(Topo::Explode("ww_1", "v")),
        Step::More(More::History(HistoryStep::Generated(
            "rr1", "hist", "ww_1_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "rr2", "hist", "ww_1_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "rr3", "hist", "ww_1_3"
        ))),
        Step::Run(crate::one_off::generated_edges_touch),
    ]
);

offset_case!(
    bugs_modalg_7_bug31120,
    "tests/bugs/modalg_7/bug31120",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================================================="),
        Step::Puts("31120: Memory leak in BRepOffsetAPIMakeOffset"),
        Step::Puts("================================================="),
        Step::Topo(Topo::Restore("brep/bug31120_wire.brep", "w")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "w",
            count: 1,
            step: 0.5,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_1,
    "tests/bugs/modalg_7/bug31845_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_1.brep", "a")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("a", 0.2, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 271.847, 0.01, None)),
        Step::Check(Check::TypedCounts(
            "result",
            &[
                ("EDGE", 12),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_2,
    "tests/bugs/modalg_7/bug31845_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_1.brep", "a")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("a", -0.2, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 210.825, 0.01, None)),
        Step::Check(Check::TypedCounts(
            "result",
            &[
                ("EDGE", 12),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_3,
    "tests/bugs/modalg_7/bug31845_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_1.brep", "a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("a", 0.2, &["a_5", "a_6"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 423.33, 0.01, None)),
        Step::Check(Check::TypedCounts(
            "result",
            &[
                ("EDGE", 24),
                ("WIRE", 12),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_4,
    "tests/bugs/modalg_7/bug31845_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_1.brep", "a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("a", -0.2, &["a_5", "a_6"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 388.879, 0.01, None)),
        Step::Check(Check::TypedCounts(
            "result",
            &[
                ("EDGE", 24),
                ("WIRE", 12),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_f,
    "tests/bugs/modalg_7/bug31845_f",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_f.brep", "a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("a", 0.001, &["a_4", "a_5"])),
        Step::Offset(Offset::Perform("result")),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_h,
    "tests/bugs/modalg_7/bug31845_h",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 31845 All: ERROR: null result"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_h.brep", "a")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("a", -0.01, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::If(
            Cond::NullShape("result"),
            &[Step::Puts("ERROR: null result")],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_7_bug31845_i,
    "tests/bugs/modalg_7/bug31845_i",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts("31845: BRepOffsetAPIMakeThickSolid fails"),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug31845_h.brep", "a")),
        Step::Topo(Topo::Explode("a", "f")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("a", 0.001, &["a_3", "a_4"])),
        Step::Offset(Offset::Perform("result")),
    ]
);

offset_case!(
    bugs_modalg_7_bug32444,
    "tests/bugs/modalg_7/bug32444",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032444: BRepOffsetAPIMakePipeShell crash when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug32444.brep", "s")),
        Step::Topo(Topo::Restore("brep/bug32444_1.brep", "p1")),
        Step::Topo(Topo::Restore("brep/bug32444_2.brep", "p2")),
        Step::Topo(Topo::Restore("brep/bug32444_3.brep", "p3")),
        Step::Topo(Topo::Restore("brep/bug32444_4.brep", "p4")),
        Step::Topo(Topo::Restore("brep/bug32444_5.brep", "p5")),
        Step::Topo(Topo::Restore("brep/bug32444_6.brep", "p6")),
        Step::Extra(Extra::ShapeDivide("s", "s", 0.001)),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p3",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p4",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p5",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p6",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
    ]
);

offset_case!(
    bugs_modalg_7_bug32445,
    "tests/bugs/modalg_7/bug32445",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032445: BRepOffsetAPIMakePipeShell crash when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug32445_spine.brep", "s")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_1.brep", "p1")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_2.brep", "p2")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_3.brep", "p3")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_4.brep", "p4")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_5.brep", "p5")),
        Step::Topo(Topo::Restore("brep/bug32445_profile_6.brep", "p6")),
        Step::Extra(Extra::ShapeDivide("s", "s", 0.01)),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p3",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p4",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p5",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p6",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::Check(Check::Shape("r")),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32446,
    "tests/bugs/modalg_7/bug32446",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032446: BRepOffsetAPIMakePipeShell crash when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug32446_spine.brep", "s")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_1.brep", "p1")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_2.brep", "p2")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_3.brep", "p3")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_4.brep", "p4")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_5.brep", "p5")),
        Step::Topo(Topo::Restore("brep/bug32446_profile_6.brep", "p6")),
        Step::Extra(Extra::ShapeDivide("s", "s", 0.05)),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p3",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p4",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p5",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p6",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::Check(Check::Shape("r")),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32447_1,
    "tests/bugs/modalg_7/bug32447_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032447: BRepOffsetAPIMakePipeShell result in inaccurate shape when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug32447-spine.brep", "s")),
        Step::Topo(Topo::Restore("brep/bug32447-profile.brep", "p")),
        Step::Shell(PipeShell::StartCorrected("s", 5.0, 0.001)),
        Step::Topo(Topo::Explode("s", "e")),
        Step::More(More::History(HistoryStep::Save("cor_hist"))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_1", "cor_hist", "s_1"
        ))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_2", "cor_hist", "s_2"
        ))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_3", "cor_hist", "s_4"
        ))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_4", "cor_hist", "s_6"
        ))),
        Step::Check(Check::Props(Prop::Length, "sc_1", 0.1816, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "sc_2", 0.265078, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "sc_3", 0.261167, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "sc_4", 0.253542, 0.01, None)),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sc_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sc_2"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r3",
            "sweep_hist",
            "sc_3"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r4",
            "sweep_hist",
            "sc_4"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 0.0483501, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 0.060163, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r3", 0.0577468, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r4", 0.0658619, 0.01, None)),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts("r", &[("FACE", 18)])),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32447_2,
    "tests/bugs/modalg_7/bug32447_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032447: BRepOffsetAPIMakePipeShell result in inaccurate shape when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Topo(Topo::Restore("brep/bug32447-spine.brep", "s")),
        Step::Topo(Topo::Restore("brep/bug32447-profile.brep", "p")),
        Step::Shell(PipeShell::StartCorrected("s", 1.0, 0.1)),
        Step::Topo(Topo::Explode("s", "e")),
        Step::More(More::History(HistoryStep::Save("cor_hist"))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_1", "cor_hist", "s_1"
        ))),
        Step::More(More::History(HistoryStep::Modified(
            "sc_2", "cor_hist", "s_2"
        ))),
        Step::Check(Check::Props(Prop::Length, "sc_1", 0.1816, 0.01, None)),
        Step::Check(Check::Props(Prop::Length, "sc_2", 0.779758, 0.01, None)),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sc_1"
        ))),
        Step::More(More::History(HistoryStep::Generated(
            "r2",
            "sweep_hist",
            "sc_2"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 0.0483501, 0.01, None)),
        Step::Check(Check::Props(Prop::Area, "r2", 0.184231, 0.01, None)),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts("r", &[("FACE", 10)])),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32447_3,
    "tests/bugs/modalg_7/bug32447_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032447: BRepOffsetAPIMakePipeShell result in inaccurate shape when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Curve(Curve::Circle("c", &[0.0, 0.0, 0.0, 1.0])),
        Step::More(More::TrimCurve("c", "c", [0.0, 1.0])),
        Step::Extra(Extra::ConvertCurve("c", "c")),
        Step::Profile(Profile::Edge("c", "c", None)),
        Step::Curve(Curve::Line("l", &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0])),
        Step::More(More::TrimCurve("l", "l", [-1.0, 0.0])),
        Step::Extra(Extra::ConvertCurve("l", "l")),
        Step::Profile(Profile::Edge("l", "l", None)),
        Step::Profile(Profile::Wire("s", &["l", "c"])),
        Step::Curve(Curve::Circle("c", &[0.9, 0.0, 0.0, 0.0, 1.0, 0.0, 0.1])),
        Step::Profile(Profile::Edge("p", "c", None)),
        Step::Profile(Profile::Wire("p", &["p"])),
        Step::Shell(PipeShell::StartCorrected("s", 5.0, 0.001)),
        Step::Topo(Topo::Explode("s", "e")),
        Step::More(More::History(HistoryStep::Save("cor_hist"))),
        Step::More(More::History(HistoryStep::Modified(
            "sc", "cor_hist", "s_1"
        ))),
        Step::Check(Check::Props(Prop::Length, "sc", 2.0, 0.01, None)),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sc"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 1.19393, 0.01, None)),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts("r", &[("FACE", 3)])),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32447_4,
    "tests/bugs/modalg_7/bug32447_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============================================"),
        Step::Puts(
            "0032447: BRepOffsetAPIMakePipeShell result in inaccurate shape when using -DT mode and DT_ShapeDivide on the spine"
        ),
        Step::Puts("============================================"),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::More(More::TrimCurve("c1", "c1", [0.0, 1.0])),
        Step::Extra(Extra::ConvertCurve("c1", "c1")),
        Step::Profile(Profile::Edge("c1", "c1", None)),
        Step::Curve(Curve::Circle("c2", &[-1000.0, 0.0, 0.0, 1010.0])),
        Step::More(More::TrimCurve("c2", "c2", [-0.1, 0.0])),
        Step::Extra(Extra::ConvertCurve("c2", "c2")),
        Step::Profile(Profile::Edge("c2", "c2", None)),
        Step::Profile(Profile::Wire("s", &["c2", "c1"])),
        Step::Curve(Curve::Circle("c", &[9.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0])),
        Step::Profile(Profile::Edge("p", "c", None)),
        Step::Profile(Profile::Wire("p", &["p"])),
        Step::Shell(PipeShell::StartCorrected("s", 5.0, 0.1)),
        Step::Topo(Topo::Explode("s", "e")),
        Step::More(More::History(HistoryStep::Save("cor_hist"))),
        Step::More(More::History(HistoryStep::Modified(
            "sc", "cor_hist", "s_1"
        ))),
        Step::Check(Check::Props(Prop::Length, "sc", 111.0, 0.01, None)),
        Step::Shell(PipeShell::Discrete),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "r",
            mode: 'C',
            solid: true
        })),
        Step::More(More::History(HistoryStep::Save("sweep_hist"))),
        Step::More(More::History(HistoryStep::Generated(
            "r1",
            "sweep_hist",
            "sc"
        ))),
        Step::Check(Check::Props(Prop::Area, "r1", 691.348, 0.01, None)),
        Step::Run(crate::one_off::swept_face_pin),
        Step::Check(Check::Shape("r")),
        Step::Check(Check::Counts("r", &[("FACE", 3)])),
        Step::CheckView("r"),
    ]
);

offset_case!(
    bugs_modalg_7_bug32691,
    "tests/bugs/modalg_7/bug32691",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("=============================================="),
        Step::Puts(
            "0032691: Modeling Algorithms - Crash when trying to build solid during sweep algorithm"
        ),
        Step::Puts("=============================================="),
        Step::Puts("REQUIRED ALL: Buildsweep : Not Done"),
        Step::Curve(Curve::BSpline(
            "s",
            &[
                3.0, 8.0, 0.0, 4.0, 1.0, 3.0, 2.0, 3.0, 3.0, 3.0, 4.0, 3.0, 5.0, 3.0, 6.0, 3.0,
                7.0, 4.0, 65.1, 7.3076, 2.1, 1.0, 65.1, 7.3027, 1.9217, 1.0, 65.1, 7.2978, 1.7434,
                1.0, 65.1, 7.2928, 1.5652, 1.0, 65.1, 7.2899, 1.4588, 1.0, 65.1, 7.2869, 1.3524,
                1.0, 65.1, 7.284, 1.2461, 1.0, 65.1, 7.272, 1.2388, 1.0, 65.1, 7.26, 1.2315, 1.0,
                65.1, 7.248, 1.2243, 1.0, 65.1, 7.1717, 1.1778, 1.0, 65.1, 7.0954, 1.1314, 1.0,
                65.1, 7.019, 1.0849, 1.0, 65.1, 6.9424, 1.0379, 1.0, 65.1, 6.8657, 0.9909, 1.0,
                65.1, 6.789, 0.9438, 1.0, 65.1, 6.7782, 0.9372, 1.0, 65.1, 6.7673, 0.9305, 1.0,
                65.1, 6.7565, 0.9238, 1.0, 65.1, 6.7436, 0.9159, 1.0, 65.1, 6.7307, 0.908, 1.0,
                65.1, 6.7178, 0.9, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("s", "s", None)),
        Step::Profile(Profile::Wire("s", &["s"])),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                65.1,
                7.3076,
                2.1,
                65.0855660928097,
                7.16839889506023,
                2.10383597533476,
                65.0746247111087,
                7.16953256986072,
                2.10380518704116,
                65.089058618299,
                7.30873367480049,
                2.0999692117064,
                65.1,
                7.3076,
                2.1
            ]
        )),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "p",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Discrete),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "r",
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Error: exception is raised")],
            &[]
        ),
    ]
);

offset_case!(
    bugs_modalg_7_bug32877,
    "tests/bugs/modalg_7/bug32877",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================================================="),
        Step::Puts("0032877: Modeling algorithms - Sweeping issue"),
        Step::Puts("================================================="),
        Step::Profile(Profile::Polyline(
            "f",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 5.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "f", false)),
        Step::Profile(Profile::Polyline("path", &[9.0, 1.0, 0.01, 5.0, 2.5, 4.0])),
        Step::Sweep(Sweep::Pipe("result", "path", "f", 0, false)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[("SHELL", 1), ("FACE", 6), ("WIRE", 6), ("EDGE", 12)]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_7_bug5081_1,
    "tests/bugs/modalg_7/bug5081_1",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 5081 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts(
            "TODO 5081 ALL: Error: BRepOffset_MakeOffsetShape algorithm does not return valid result"
        ),
        Step::Puts("======="),
        Step::Puts("5081"),
        Step::Puts("======="),
        Step::Topo(Topo::Restore("brep/bug5582_shape1.brep", "comp")),
        Step::Topo(Topo::Explode("comp", "so")),
        Step::Offset(Offset::Parameter(1e-05, false, 'a', false)),
        Step::Offset(Offset::Load("comp_1", 1.0, &[])),
        Step::Offset(Offset::Perform("res")),
        Step::Check(Check::Shape("res")),
        Step::Let("Log", Value::Bopcheck("res")),
        Step::If(
            Cond::Lacks("Log", "This shape seems to be OK"),
            &[Step::Puts(
                "Error: BRepOffset_MakeOffsetShape algorithm does not return valid result"
            )],
            &[]
        ),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_7_bug5081_2,
    "tests/bugs/modalg_7/bug5081_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("======="),
        Step::Puts("5081"),
        Step::Puts("======="),
        Step::Topo(Topo::Restore("brep/bug5081_shape2.brep", "comp")),
        Step::Topo(Topo::Explode("comp", "so")),
        Step::Offset(Offset::Parameter(1e-05, false, 'i', false)),
        Step::Offset(Offset::Load("comp_1", 2.0, &[])),
        Step::Offset(Offset::Perform("res")),
        Step::Check(Check::Shape("res")),
        Step::Let("Log", Value::Bopcheck("res")),
        Step::If(
            Cond::Lacks("Log", "This shape seems to be OK"),
            &[Step::Puts(
                "Error: BRepOffset_MakeOffsetShape algorithm does not return valid result"
            )],
            &[]
        ),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_7_bug5582,
    "tests/bugs/modalg_7/bug5582",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 5582 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("======="),
        Step::Puts("5582"),
        Step::Puts("======="),
        Step::Topo(Topo::Restore("brep/bug5582_shape1.brep", "comp")),
        Step::Check(Check::Shape("comp")),
        Step::Topo(Topo::Explode("comp", "so")),
        Step::Offset(Offset::Parameter(1e-05, false, 'a', false)),
        Step::Offset(Offset::Load("comp_1", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Shape("result")),
    ]
);
