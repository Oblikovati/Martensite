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
    bugs_modalg_5_bug22323,
    "tests/bugs/modalg_5/bug22323",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR22323"),
        Step::Puts("============"),
        Step::Prim(Prim::Cylinder("cy", &[100.0, 200.0])),
        Step::Topo(Topo::Explode("cy", "f")),
        Step::View(ViewStep::Donly(&["cy", "cy_2"])),
        Step::View(ViewStep::Fit),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "res",
            shape: "cy",
            value: -1.0,
            tolerance: None,
            complete: false,
            faces: &["cy_2"]
        })),
        Step::View(ViewStep::Donly(&["res"])),
    ]
);

offset_case!(
    bugs_modalg_5_bug23706_1,
    "tests/bugs/modalg_5/bug23706_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("23706"),
        Step::Puts("============"),
        Step::Curve(Curve::BSpline(
            "r2",
            &[
                4.0, 3.0, 1.0, 5.0, 2.0, 1.0, 3.0, 5.0, 0.0, 8.0, 0.0, 1.0, 2.0, 8.0, 2.0, 1.0,
                4.0, 8.0, 3.0, 1.0, 4.0, 8.0, 3.0, 1.0, 6.0, 8.0, 4.0, 1.0, 10.0, 8.0, 10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("spine", "r2", None)),
        Step::Profile(Profile::Wire("spine", &["spine"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 8.0, 0.0, 1.0, 0.0, 1.0, 0.2]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("p", "spine", "profile", 0, false)),
        Step::Topo(Topo::Explode("p", "f")),
        Step::More(More::SurfaceOfFace("ss1", "p_1")),
        Step::More(More::SurfaceOfFace("ss2", "p_2")),
        Step::More(More::SurfaceOfFace("ss3", "p_3")),
        Step::Extra(Extra::OffsetSurface("o1", "ss1", -0.1)),
        Step::Extra(Extra::OffsetSurface("o2", "ss2", -0.1)),
        Step::Extra(Extra::OffsetSurface("o3", "ss3", -0.1)),
        Step::More(More::Face("res", "o2", None)),
        Step::Check(Check::Props(Prop::Area, "res", 9.00819, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_5_bug23706_2,
    "tests/bugs/modalg_5/bug23706_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("23706"),
        Step::Puts("============"),
        Step::Curve(Curve::BSpline(
            "r2",
            &[
                4.0, 3.0, 1.0, 5.0, 2.0, 1.0, 3.0, 5.0, 0.0, 8.0, 0.0, 1.0, 2.0, 8.0, 2.0, 1.0,
                4.0, 8.0, 3.0, 1.0, 4.0, 8.0, 3.0, 1.0, 6.0, 8.0, 4.0, 1.0, 10.0, 8.0, 10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("spine", "r2", None)),
        Step::Profile(Profile::Wire("spine", &["spine"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "spine",
            vertex: None,
            translate: false,
            rotate: true,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "spine",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::Explode("spine", "f")),
        Step::More(More::SurfaceOfFace("ss", "spine_1")),
        Step::Extra(Extra::OffsetSurface("o1", "ss", 2.0)),
        Step::More(More::Face("res", "o1", None)),
        Step::Check(Check::Props(Prop::Area, "res", 245.495, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_5_bug23706_3,
    "tests/bugs/modalg_5/bug23706_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("23706"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("geom/bug23706_c2.draw", "c")),
        Step::Profile(Profile::Edge("e1", "c", None)),
        Step::Profile(Profile::Wire("w", &["e1"])),
        Step::Profile(Profile::Polyline(
            "pl",
            &[3.9, 4.9, 5.0, 4.0, 5.1, 5.0, 4.1, 3.9, 5.0, 3.9, 4.9, 5.0]
        )),
        Step::Profile(Profile::Plane("pl", "pl", false)),
        Step::Sweep(Sweep::Pipe("result", "w", "pl", 0, false)),
        Step::View(ViewStep::Donly(&["result"])),
        Step::View(ViewStep::Fit),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 1.67216, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_5_bug23706_8,
    "tests/bugs/modalg_5/bug23706_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("23706"),
        Step::Puts("============"),
        Step::Curve(Curve::BSpline(
            "r2",
            &[
                2.0, 4.0, 1.0, 3.0, 2.0, 2.0, 3.0, 1.0, 4.0, 3.0, 0.0, 8.0, 0.0, 1.0, 2.0, 8.0,
                2.0, 1.0, 4.0, 8.0, 3.0, 1.0, 4.0, 8.0, 3.0, 1.0, 6.0, 8.0, 4.0, 1.0, 10.0, 8.0,
                10.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("spine", "r2", None)),
        Step::Profile(Profile::Wire("spine", &["spine"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 8.0, 0.0, 1.0, 0.0, 1.0, 0.51]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("p", "spine", "profile", 0, false)),
        Step::Topo(Topo::Explode("p", "f")),
        Step::More(More::SurfaceOfFace("ss", "p_2")),
        Step::Curve(Curve::BSpline(
            "r3",
            &[
                2.0,
                5.0,
                1.0,
                3.0,
                2.0,
                1.0,
                3.0,
                1.0,
                4.0,
                1.0,
                5.0,
                3.0,
                9.0,
                7.0,
                3.0,
                1.0,
                5.447213595499958,
                9.0,
                2.105572809000084,
                1.0,
                4.223606797749979,
                8.0,
                2.552786404500042,
                1.0,
                4.223606797749979,
                8.0,
                2.552786404500042,
                1.0,
                3.0,
                7.0,
                3.0,
                1.0,
                2.0,
                5.0,
                3.0,
                1.0
            ]
        )),
        Step::Run(crate::one_off::two_intersections),
    ]
);

offset_case!(
    bugs_modalg_5_bug23820_1,
    "tests/bugs/modalg_5/bug23820_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Profile(Profile::Vertex(
            "v1",
            [297.706909179688, 19.969066619873, 9.07486343383789]
        )),
        Step::Profile(Profile::Vertex(
            "v2",
            [282.750762939453, 24.8436107635498, 6.85506725311279]
        )),
        Step::Profile(Profile::Vertex(
            "v3",
            [284.397491455078, 7.2792820930481, -0.00160308415070176]
        )),
        Step::Profile(Profile::Vertex(
            "v4",
            [301.926666259766, 5.67953395843506, 0.0583421923220158]
        )),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v4")),
        Step::Profile(Profile::EdgeOfVertices("e4", "v4", "v1")),
        Step::Profile(Profile::Wire("w", &["e1", "e2", "e3", "e4"])),
        Step::Topo(Topo::Restore("brep/bug23820_non_planar.brep", "shell")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["w"],
            surface: "shell",
            unlimited: false,
            max_distance: None,
            tolerance: 1.0e-4,
            continuity: 2,
            max_degree: 14,
            max_segments: 16
        })),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 14),
                ("WIRE", 0),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 39)
            ]
        )),
        Step::Check(Check::Props(Prop::Length, "result", 83.0174, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_5_bug23820_2,
    "tests/bugs/modalg_5/bug23820_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Profile(Profile::Vertex(
            "v1",
            [297.706909179688, 19.969066619873, 9.07486343383789]
        )),
        Step::Profile(Profile::Vertex(
            "v2",
            [282.750762939453, 24.8436107635498, 6.85506725311279]
        )),
        Step::Profile(Profile::Vertex(
            "v3",
            [284.397491455078, 7.2792820930481, -0.00160308415070176]
        )),
        Step::Profile(Profile::Vertex(
            "v4",
            [301.926666259766, 5.67953395843506, 0.0583421923220158]
        )),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v4")),
        Step::Profile(Profile::EdgeOfVertices("e4", "v4", "v1")),
        Step::Profile(Profile::Wire("w", &["e1", "e2", "e3", "e4"])),
        Step::Topo(Topo::Restore("brep/bug23820_non_planar.brep", "shell")),
        Step::More(More::Project(Project {
            result: "result",
            shapes: &["w"],
            surface: "shell",
            unlimited: false,
            max_distance: Some(2.0),
            tolerance: 1.0e-4,
            continuity: 2,
            max_degree: 14,
            max_segments: 16
        })),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 6),
                ("WIRE", 0),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 19)
            ]
        )),
        Step::Check(Check::Props(Prop::Length, "result", 73.9585, 0.01, None)),
    ]
);

offset_case!(
    bugs_modalg_5_bug24532,
    "tests/bugs/modalg_5/bug24532",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("24532"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "w1",
            &[
                -4.0, -3.0, 0.0, 4.0, -3.0, 0.0, 4.0, 3.0, 0.0, -4.0, 3.0, 0.0, -4.0, -3.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                4.0, 3.0, 80.0, -4.0, 3.0, 80.0, -4.0, -3.0, 80.0, 4.0, -3.0, 80.0, 4.0, 3.0, 80.0
            ]
        )),
        Step::Profile(Profile::Polyline("sp", &[0.0, 0.0, 0.0, 0.0, 0.0, 80.0])),
        Step::More(More::Surface(Surface::Cylinder(
            "cc",
            &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -4.0, -3.0, 0.0],
            5.0
        ))),
        Step::Curve(Curve::Line("ll", &[0.0, 0.0, 3.141592653589793, 80.0])),
        Step::More(More::TrimCurve("ll", "ll", [0.0, 80.06166126430983])),
        Step::Topo(Topo::Explode("w1", "v")),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Extra(Extra::EdgeBetween("ee", "ll", Some("cc"), "w1_1", "w2_1")),
        Step::Profile(Profile::Wire("gg", &["ee"])),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Guide("gg", false, 0)),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'M',
            solid: true
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2343.46, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 12),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 34)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug24667,
    "tests/bugs/modalg_5/bug24667",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("24667"),
        Step::Puts("============"),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, -50.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("w", &["e"])),
        Step::Extra(Extra::PipeWithEnds("r", "w", "v3")),
        Step::More(More::Compound(&["r", "r_f", "r_l"], "result")),
        Step::Check(Check::Props(Prop::Length, "result", 100.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 1),
                ("WIRE", 1),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 1),
                ("SHAPE", 5)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug24738,
    "tests/bugs/modalg_5/bug24738",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("24738"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug24738_path.brep", "sp")),
        Step::Topo(Topo::Restore("brep/bug24738_profile.brep", "pr")),
        Step::Sweep(Sweep::Pipe("result", "sp", "pr", 0, false)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 8)
            ]
        )),
        Step::Run(crate::one_off::gravity_center),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_06,
    "tests/bugs/modalg_5/bug25298_06",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("cc", &[0.0, 0.0, 0.0, 10.0])),
        Step::More(More::TrimCurve(
            "cc",
            "cc",
            [3.141592653589793, 4.71238898038469]
        )),
        Step::Profile(Profile::Edge("e1", "cc", None)),
        Step::Profile(Profile::Polyline(
            "pp",
            &[0.0, -10.0, 0.0, -10.0, -10.0, 0.0, -10.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("ww", &["e1", "pp"])),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "ww",
                count: 1,
                step: 2.0,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 49.1327, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 5),
                        ("EDGE", 5),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 11)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_07,
    "tests/bugs/modalg_5/bug25298_07",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("cc", &[0.0, 0.0, 0.0, 10.0])),
        Step::More(More::TrimCurve(
            "cc",
            "cc",
            [3.141592653589793, 4.71238898038469]
        )),
        Step::Profile(Profile::Edge("e1", "cc", None)),
        Step::Profile(Profile::Polyline(
            "pp",
            &[0.0, -10.0, 0.0, -10.0, -10.0, 0.0, -10.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("ww", &["e1", "pp"])),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "ww",
                count: 1,
                step: -0.2,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 24.2333, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 3),
                        ("EDGE", 3),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 7)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_08,
    "tests/bugs/modalg_5/bug25298_08",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 6.0, 10.0, 0.0, 6.0, 15.0, 0.0,
                4.0, 15.0, 0.0, 4.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "pp",
                count: 1,
                step: 1.0,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 58.0, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 8),
                        ("EDGE", 8),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 17)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_09,
    "tests/bugs/modalg_5/bug25298_09",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 6.0, 10.0, 0.0, 6.0, 15.0, 0.0,
                4.0, 15.0, 0.0, 4.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "pp",
                count: 1,
                step: -1.0,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 39.5, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 6),
                        ("EDGE", 7),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 14)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_10,
    "tests/bugs/modalg_5/bug25298_10",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 6.0, 10.0, 0.0, 6.0, 15.0, 0.0,
                4.0, 15.0, 0.0, 4.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "pp",
                count: 1,
                step: -0.2,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 48.4, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 8),
                        ("EDGE", 8),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 17)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_11,
    "tests/bugs/modalg_5/bug25298_11",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 6.0, 10.0, 0.0, 6.0, 15.0, 0.0,
                4.0, 15.0, 0.0, 4.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "pp",
                count: 1,
                step: -2.0,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 24.0, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 5),
                        ("EDGE", 5),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 11)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_12,
    "tests/bugs/modalg_5/bug25298_12",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "a",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 20.0, 0.0, 5.0, 5.0, 0.0, 0.0, 20.0, 0.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: 0.5,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 92.9473, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 5),
                        ("EDGE", 5),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 11)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_13,
    "tests/bugs/modalg_5/bug25298_13",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "a",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 20.0, 0.0, 5.0, 5.0, 0.0, 0.0, 20.0, 0.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: -0.2,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 77.093, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 5),
                        ("EDGE", 5),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 11)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25298_14,
    "tests/bugs/modalg_5/bug25298_14",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25298"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "a",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 20.0, 0.0, 5.0, 5.0, 0.0, 0.0, 20.0, 0.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: -2.5,
                approx: false,
                join: 'i',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 10.0, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 6),
                        ("EDGE", 6),
                        ("WIRE", 2),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 1),
                        ("SHAPE", 15)
                    ]
                )),
                Step::View(ViewStep::Fit)
            ]
        ),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25339,
    "tests/bugs/modalg_5/bug25339",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("=========="),
        Step::Puts("25339"),
        Step::Puts("=========="),
        Step::Topo(Topo::Restore("brep/bug25339_face1.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "r",
            base: "a",
            count: 1,
            step: 1.0,
            approx: false,
            join: 'i',
            altitude: 0.0,
            open: false
        })),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Topo(Topo::Rename("r_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 36.1042, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25453,
    "tests/bugs/modalg_5/bug25453",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("25453"),
        Step::Puts("========"),
        Step::View(ViewStep::SmallView),
        Step::Topo(Topo::Restore(
            "brep/OCC25453_sweep9-draw-Sketch001.brep",
            "Sketch001"
        )),
        Step::Topo(Topo::Explode("Sketch001", "E")),
        Step::Profile(Profile::Wire("Sweep-0-spine", &["Sketch001_1"])),
        Step::Shell(PipeShell::Start("Sweep-0-spine")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Topo(Topo::Restore(
            "brep/OCC25453_sweep9-draw-Sketch.brep",
            "Sketch"
        )),
        Step::Place(Place::Rotate(
            &["Sketch"],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0]
        )),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Sketch",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep",
            mode: 'C',
            solid: false
        })),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25509_1,
    "tests/bugs/modalg_5/bug25509_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_1"),
        Step::Puts("============"),
        Step::Puts("25509"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::ExplodeChildren("b")),
        Step::More(More::EmptyShape("result", "so")),
        Step::Topo(Topo::Add("b_1", "result")),
        Step::Topo(Topo::Add("b_1", "result")),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25509_3,
    "tests/bugs/modalg_5/bug25509_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_1"),
        Step::Puts("============"),
        Step::Puts("25509"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[4.0, 4.0, 4.0, 4.0, 4.0, 4.0])),
        Step::Topo(Topo::ExplodeChildren("b1")),
        Step::Topo(Topo::ExplodeChildren("b2")),
        Step::More(More::EmptyShape("result", "so")),
        Step::Topo(Topo::Add("b1_1", "result")),
        Step::Topo(Topo::Add("b2_1", "result")),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25509_4,
    "tests/bugs/modalg_5/bug25509_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_1"),
        Step::Puts("============"),
        Step::Puts("25509"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::ExplodeChildren("b1")),
        Step::Profile(Profile::Vertex("v1", [11.0, 11.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [11.0, 11.0, 11.0])),
        Step::Profile(Profile::EdgeOfVertices("e2", "v1", "v2")),
        Step::More(More::EmptyShape("result", "so")),
        Step::Topo(Topo::Add("b1_1", "result")),
        Step::Topo(Topo::Add("e2", "result")),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25509_5,
    "tests/bugs/modalg_5/bug25509_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_1"),
        Step::Puts("============"),
        Step::Puts("25509"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::ExplodeChildren("b1")),
        Step::Profile(Profile::Vertex("v1", [11.0, 11.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [11.0, 11.0, 11.0])),
        Step::More(More::EmptyShape("result", "so")),
        Step::Topo(Topo::Add("b1_1", "result")),
        Step::Topo(Topo::Add("v1", "result")),
        Step::Topo(Topo::Add("v2", "result")),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25509_6,
    "tests/bugs/modalg_5/bug25509_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("REQUIRED All: Faulty shapes in variables faulty_1 to faulty_1"),
        Step::Puts("============"),
        Step::Puts("25509"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Topo(Topo::ExplodeChildren("b1")),
        Step::Profile(Profile::Vertex("v1", [11.0, 11.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [11.0, 11.0, 11.0])),
        Step::Prim(Prim::Box("b3", &[3.0, 3.0, 3.0, 4.0, 4.0, 4.0])),
        Step::Topo(Topo::ExplodeChildren("b3")),
        Step::Topo(Topo::ExplodeChildren("b1_1")),
        Step::Topo(Topo::Orient("b1_1_1", 'I')),
        Step::More(More::EmptyShape("x", "sh")),
        Step::Topo(Topo::Add("b1_1_1", "x")),
        Step::More(More::EmptyShape("result", "so")),
        Step::Topo(Topo::Add("b1_1", "result")),
        Step::Topo(Topo::Add("v1", "result")),
        Step::Topo(Topo::Add("v2", "result")),
        Step::Topo(Topo::Add("b3_1", "result")),
        Step::Topo(Topo::Add("x", "result")),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25568,
    "tests/bugs/modalg_5/bug25568",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25568"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore(
            "brep/bug25568_loft-inv1-draw-Loft-00-section.brep",
            "Loft-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug25568_loft-inv1-draw-Loft-01-section.brep",
            "Loft-01-section"
        )),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft-00-section", "Loft-01-section"]
        }),
        Step::If(
            Cond::Unbound("result"),
            &[
                Step::Extra(Extra::FixShape("Loft-00-section_fixed", "Loft-00-section")),
                Step::Loft(Loft {
                    result: "result",
                    solid: false,
                    ruled: false,
                    check: true,
                    mutable_input: true,
                    sections: &["Loft-00-section_fixed", "Loft-01-section"]
                }),
                Step::If(
                    Cond::Unbound("result"),
                    &[Step::Puts(
                        "Error : thrusection is failed even after fixing arguments"
                    )],
                    &[]
                )
            ],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 25.236, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25582,
    "tests/bugs/modalg_5/bug25582",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25582"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore(
            "brep/bug25582_filletloft-draw-Loft-00-section.brep",
            "Loft-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug25582_filletloft-draw-Loft-01-section.brep",
            "Loft-01-section"
        )),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft-00-section", "Loft-01-section"]
        }),
        Step::If(
            Cond::Unbound("result"),
            &[Step::Puts("Error : thrusections failed")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 1.5708, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25704_1,
    "tests/bugs/modalg_5/bug25704_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25704"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("a", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("a", "a", None)),
        Step::Profile(Profile::Wire("a", &["a"])),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: 2.0,
                approx: false,
                join: 'a',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 75.3982, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 1),
                        ("EDGE", 1),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 3)
                    ]
                ))
            ]
        ),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    bugs_modalg_5_bug25704_2,
    "tests/bugs/modalg_5/bug25704_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25704"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("a", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("a", "a", None)),
        Step::Profile(Profile::Wire("a", &["a"])),
        Step::If(
            Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                result: "resoffset",
                base: "a",
                count: 1,
                step: -3.0,
                approx: false,
                join: 'a',
                altitude: 0.0,
                open: false
            }))),
            &[Step::Puts("Error : mkoffset is wrong")],
            &[
                Step::Topo(Topo::Rename("resoffset_1", "result")),
                Step::Check(Check::Props(Prop::Length, "result", 43.9823, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::Check(Check::Section("result")),
                Step::Check(Check::Counts(
                    "result",
                    &[
                        ("VERTEX", 1),
                        ("EDGE", 1),
                        ("WIRE", 1),
                        ("FACE", 0),
                        ("SHELL", 0),
                        ("SOLID", 0),
                        ("COMPSOLID", 0),
                        ("COMPOUND", 0),
                        ("SHAPE", 3)
                    ]
                ))
            ]
        ),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);
