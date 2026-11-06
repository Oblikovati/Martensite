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
    bugs_modalg_2_bug2083_1,
    "tests/bugs/modalg_2/bug2083_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("2083"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC2083-f1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/OCC2083-f2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/OCC2083-w.brep", "s")),
        Step::Check(Check::Shape("f1")),
        Step::Check(Check::Shape("f2")),
        Step::Check(Check::Shape("s")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f1_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f2_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 2083")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 36302.3, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug2083_2,
    "tests/bugs/modalg_2/bug2083_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("2083"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC2083-f1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/OCC2083-f2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/OCC2083-w.brep", "s")),
        Step::Check(Check::Shape("f1")),
        Step::Check(Check::Shape("f2")),
        Step::Check(Check::Shape("s")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f1_1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f2_1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 2083")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 36583.8, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug2083_3,
    "tests/bugs/modalg_2/bug2083_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("2083"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC2083-f1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/OCC2083-f2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/OCC2083-w.brep", "s")),
        Step::Check(Check::Shape("f1")),
        Step::Check(Check::Shape("f2")),
        Step::Check(Check::Shape("s")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f1_1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f2_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 2083")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 36546.8, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug2083_4,
    "tests/bugs/modalg_2/bug2083_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("2083"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC2083-f1.brep", "f1")),
        Step::Topo(Topo::Restore("brep/OCC2083-f2.brep", "f2")),
        Step::Topo(Topo::Restore("brep/OCC2083-w.brep", "s")),
        Step::Check(Check::Shape("f1")),
        Step::Check(Check::Shape("f2")),
        Step::Check(Check::Shape("s")),
        Step::Topo(Topo::Explode("f1", "w")),
        Step::Topo(Topo::Explode("f2", "w")),
        Step::Shell(PipeShell::Start("s")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f1_1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "f2_1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 2083")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 36339.5, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug22361,
    "tests/bugs/modalg_2/bug22361",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("22361"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 7.0])),
        Step::Curve(Curve::Circle(
            "c2",
            &[5.28270543795374, 13.2067635948844, 26.4135271897687, 21.0]
        )),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0,
                0.0,
                0.0,
                5.28270543795374,
                13.2067635948844,
                26.4135271897687
            ]
        )),
        Step::Profile(Profile::Edge("c1", "c1", None)),
        Step::Profile(Profile::Wire("c1", &["c1"])),
        Step::Profile(Profile::Edge("c2", "c2", None)),
        Step::Profile(Profile::Wire("c2", &["c2"])),
        Step::View(ViewStep::Fit),
        Step::Shell(PipeShell::Start("pp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "c1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "c2",
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
        Step::Check(Check::Props(Prop::Area, "result", 2745.88, 0.01, None)),
        Step::Check(Check::Shape("result")),
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
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug22864_3,
    "tests/bugs/modalg_2/bug22864_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("22864"),
        Step::Puts("============"),
        Step::More(More::Surface(Surface::Sphere("s1", &[], 10.0))),
        Step::More(More::Surface(Surface::Sphere("s2", &[], 10.0))),
        Step::More(More::Face("f1", "s1", None)),
        Step::More(More::Face("f2", "s2", None)),
        Step::Extra(Extra::Deform("f1", "f1", [2.0, 2.0, 1.0])),
        Step::Extra(Extra::Deform("f2", "f2", [0.5, 0.5, 2.0])),
        Step::More(More::Compound(&["f1", "f2"], "s")),
        Step::Extra(Extra::RollingBall("result", "s", 2.0, &["f1", "f2"])),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 11),
                ("WIRE", 5),
                ("FACE", 5),
                ("SHELL", 1)
            ]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 3750.55, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug22864_4,
    "tests/bugs/modalg_2/bug22864_4",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 29957 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("22864"),
        Step::Puts("============"),
        Step::More(More::Surface(Surface::Cone(
            "s1",
            &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            [30.0, 0.0]
        ))),
        Step::More(More::Face(
            "f1",
            "s1",
            Some([0.0, 6.283185307179586, -40.0, 0.0])
        )),
        Step::More(More::Surface(Surface::Cone(
            "s2",
            &[25.0, 0.0, -15.0, 1.0, 0.0, 0.0],
            [10.0, 0.0]
        ))),
        Step::More(More::Face(
            "f2",
            "s2",
            Some([0.0, 6.283185307179586, -55.0, 0.0])
        )),
        Step::More(More::Compound(&["f1", "f2"], "s")),
        Step::Extra(Extra::RollingBall("result", "s", 2.0, &["f1", "f2"])),
        Step::Check(Check::Props(Prop::Area, "result", 3523.97, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_1,
    "tests/bugs/modalg_2/bug23162_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 100."),
        Step::Profile(Profile::Vertex("v1", [50.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [100.0, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 100.0])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 100.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 14572.5, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_2,
    "tests/bugs/modalg_2/bug23162_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 10."),
        Step::Profile(Profile::Vertex("v1", [5.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 10.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 145.725, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_3,
    "tests/bugs/modalg_2/bug23162_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 1."),
        Step::Profile(Profile::Vertex("v1", [0.5, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [1.0, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 1.0])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 1.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 1.45725, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_4,
    "tests/bugs/modalg_2/bug23162_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 0.1"),
        Step::Profile(Profile::Vertex("v1", [0.05, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.1, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 0.1])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 0.1])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 0.0145725, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_5,
    "tests/bugs/modalg_2/bug23162_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 0.01"),
        Step::Profile(Profile::Vertex("v1", [0.005, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.01, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 0.01])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 0.01])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 0.000145725, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23162_6,
    "tests/bugs/modalg_2/bug23162_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("================"),
        Step::Puts("23162"),
        Step::Puts("================"),
        Step::Puts("------------- Radius 0.001"),
        Step::Profile(Profile::Vertex("v1", [0.0005, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.001, 0.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Sweep(Sweep::Revol(
            "rq",
            "e",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0]
        )),
        Step::More(More::Scale("rq2", "rq", 0.2)),
        Step::Place(Place::Translate(&["rq2"], [0.0, 0.0, 0.001])),
        Step::Topo(Topo::Explode("rq", "W")),
        Step::Topo(Topo::Explode("rq2", "W")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 0.001])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("spine", &["e"])),
        Step::Shell(PipeShell::Start("spine")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq_1",
            vertex: Some("v1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "rq2_1",
            vertex: Some("v2"),
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
        Step::Check(Check::Props(Prop::Area, "result", 1.45725e-06, 0.01, None)),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_1,
    "tests/bugs/modalg_2/bug23367_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Prim(Prim::Box("b", &[10.0, 100.0, 10.0])),
        Step::Topo(Topo::Explode("b", "f")),
        Step::View(ViewStep::Donly(&["b", "b_3", "b_4"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "b", "b_3", "b_4")),
        Step::Check(Check::Props(Prop::Length, "result", 100.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_10,
    "tests/bugs/modalg_2/bug23367_10",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0, 50.0, 200.0, 0.0, 100.0, 100.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pp",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "a",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::Explode("a", "e")),
        Step::Profile(Profile::Wire("w1", &["a_2", "a_5", "a_8"])),
        Step::Profile(Profile::Wire("w2", &["a_24", "a_22", "a_20"])),
        Step::View(ViewStep::Donly(&["a", "w1", "w2"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "w1", "w2")),
        Step::Check(Check::Props(Prop::Length, "result", 288.824, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_13,
    "tests/bugs/modalg_2/bug23367_13",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0, 10.0, 0.0, 20.0, 20.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 100.0, -50.0, 50.0, 200.0, -100.0, 100.0, 100.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pp",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "a",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::Explode("a", "e")),
        Step::Profile(Profile::Wire("w1", &["a_2", "a_5", "a_8"])),
        Step::Profile(Profile::Wire("w2", &["a_24", "a_27", "a_30"])),
        Step::View(ViewStep::Donly(&["a", "w1", "w2"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "w1", "w2")),
        Step::Check(Check::Props(Prop::Length, "result", 352.512, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_14,
    "tests/bugs/modalg_2/bug23367_14",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, -10.0, 0.0, 20.0, 0.0, 0.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 50.0, -100.0, 0.0, 0.0, -50.0, 0.0, -50.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pp",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "a",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::Explode("a", "e")),
        Step::Profile(Profile::Wire("ww", &["a_11", "a_8", "a_5", "a_2"])),
        Step::View(ViewStep::Donly(&["a", "ww"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "ww", "ww")),
        Step::Check(Check::Props(Prop::Length, "result", 365.161, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_15,
    "tests/bugs/modalg_2/bug23367_15",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 10.0, -10.0, 0.0, 20.0, 0.0, 0.0, 10.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "sp",
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 50.0, -100.0, 0.0, 0.0, -50.0, 0.0, -50.0
            ]
        )),
        Step::Shell(PipeShell::Start("sp")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pp",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "a",
            mode: 'R',
            solid: false
        })),
        Step::Topo(Topo::Explode("a", "e")),
        Step::Profile(Profile::Wire("w1", &["a_11", "a_8", "a_5", "a_2"])),
        Step::Profile(Profile::Wire("w2", &["a_34", "a_36", "a_28", "a_31"])),
        Step::View(ViewStep::Donly(&["a", "w1", "w2"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "w1", "w2")),
        Step::Check(Check::Props(Prop::Length, "result", 270.149, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_2,
    "tests/bugs/modalg_2/bug23367_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Puts("TODO 24156 MacOS: Error : The length of result shape is"),
        Step::Prim(Prim::Cylinder("a", &[10.0, 30.0, 180.0])),
        Step::Topo(Topo::Explode("a", "f")),
        Step::View(ViewStep::Donly(&["a", "a_2", "a_3"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "a_2", "a_3")),
        Step::Check(Check::Props(Prop::Length, "result", 30.0036, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_3,
    "tests/bugs/modalg_2/bug23367_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "pp",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("ff", "pp", false)),
        Step::Sweep(Sweep::Prism("a", "ff", [0.0, 0.0, 50.0])),
        Step::Topo(Topo::Explode("a", "f")),
        Step::View(ViewStep::Donly(&["a", "a_6", "a_7"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "a_6", "a_7")),
        Step::Check(Check::Props(Prop::Length, "result", 50.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_4,
    "tests/bugs/modalg_2/bug23367_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Prim(Prim::Torus("a", &[50.0, 10.0])),
        Step::Topo(Topo::Explode("a", "e")),
        Step::Profile(Profile::Wire("ww", &["a_2"])),
        Step::View(ViewStep::Donly(&["a", "ww"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "ww", "ww")),
        Step::Check(Check::Props(Prop::Length, "result", 314.159, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_5,
    "tests/bugs/modalg_2/bug23367_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Prim(Prim::Cylinder("a", &[10.0, 50.0])),
        Step::Topo(Topo::Explode("a", "f")),
        Step::View(ViewStep::Donly(&["a", "a_2", "a_3"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "a", "a_2", "a_3")),
        Step::Check(Check::Props(Prop::Length, "result", 50.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23367_9,
    "tests/bugs/modalg_2/bug23367_9",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23367"),
        Step::Puts("============"),
        Step::Prim(Prim::Torus("a", &[50.0, 10.0, 0.0, 180.0])),
        Step::Topo(Topo::Explode("a", "f")),
        Step::More(More::EmptyShape("b", "Sh")),
        Step::Topo(Topo::Add("a_1", "b")),
        Step::Topo(Topo::Explode("b", "e")),
        Step::Profile(Profile::Wire("ww", &["b_2"])),
        Step::View(ViewStep::Donly(&["b", "ww"])),
        Step::View(ViewStep::Fit),
        Step::More(More::Middle("result", "b", "ww", "ww")),
        Step::Check(Check::Props(Prop::Length, "result", 314.159, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug23436,
    "tests/bugs/modalg_2/bug23436",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("CR23436"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug23436_myloft.brep", "s")),
        Step::Topo(Topo::ExplodeChildren("s")),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["s_1", "s_2", "s_3"]
        }),
    ]
);

offset_case!(
    bugs_modalg_2_bug421,
    "tests/bugs/modalg_2/bug421",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 421 "),
        Step::Puts("========================"),
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
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug422_1,
    "tests/bugs/modalg_2/bug422_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 422 "),
        Step::Puts("(case 1)"),
        Step::Puts("========================"),
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
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug422_2,
    "tests/bugs/modalg_2/bug422_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 422 "),
        Step::Puts("(case 2)"),
        Step::Puts("========================"),
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
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug427_1,
    "tests/bugs/modalg_2/bug427_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========="),
        Step::Puts(" 427 "),
        Step::Puts("(case 1)"),
        Step::Puts("========="),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 671.382, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug427_2,
    "tests/bugs/modalg_2/bug427_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 427 "),
        Step::Puts("(case 2)"),
        Step::Puts("========================"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 858.167, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_2_bug427_3,
    "tests/bugs/modalg_2/bug427_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts(" 427 "),
        Step::Puts("(case 3)"),
        Step::Puts("========================"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Area, "result", 885.688, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);
