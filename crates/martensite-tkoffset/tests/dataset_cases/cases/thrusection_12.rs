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
    thrusection_specific_j5,
    "tests/thrusection/specific/J5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 2042.02, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_j6,
    "tests/thrusection/specific/J6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2081.25, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_j7,
    "tests/thrusection/specific/J7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
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
        Step::Check(Check::Props(Prop::Area, "result", 1346.42, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_j8,
    "tests/thrusection/specific/J8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 2707.56, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_j9,
    "tests/thrusection/specific/J9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2736.65, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k1,
    "tests/thrusection/specific/K1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 2042.02, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k2,
    "tests/thrusection/specific/K2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2081.25, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k3,
    "tests/thrusection/specific/K3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
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
        Step::Check(Check::Props(Prop::Area, "result", 1683.87, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_k4,
    "tests/thrusection/specific/K4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k5,
    "tests/thrusection/specific/K5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 901.663, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k6,
    "tests/thrusection/specific/K6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k7,
    "tests/thrusection/specific/K7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 901.663, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_k8,
    "tests/thrusection/specific/K8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
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
        Step::Check(Check::Props(Prop::Area, "result", 597.473, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_k9,
    "tests/thrusection/specific/K9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 1480.16, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l1,
    "tests/thrusection/specific/L1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1465.92, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l2,
    "tests/thrusection/specific/L2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l3,
    "tests/thrusection/specific/L3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 901.663, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l4,
    "tests/thrusection/specific/L4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
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
        Step::Check(Check::Props(Prop::Area, "result", 881.045, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_l5,
    "tests/thrusection/specific/L5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l6,
    "tests/thrusection/specific/L6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l7,
    "tests/thrusection/specific/L7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l8,
    "tests/thrusection/specific/L8",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_l9,
    "tests/thrusection/specific/L9",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_m1,
    "tests/thrusection/specific/M1",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_m2,
    "tests/thrusection/specific/M2",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_m3,
    "tests/thrusection/specific/M3",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_m4,
    "tests/thrusection/specific/M4",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
    ]
);

offset_case!(
    thrusection_specific_m5,
    "tests/thrusection/specific/M5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 7166 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline("s1", &[0.0, 0.0, 20.0, 0.0, 0.0, -20.0])),
        Step::Curve(Curve::PeriodicBSpline(
            "pbs1",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, -1.0, 0.0, 0.0, 1.0, -2.0, 2.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                2.0, 2.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 2.0, -2.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0,
                -2.0, -2.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "pbs1", None)),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, -20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, 20.0])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 898.589, 0.01, None)),
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
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_m6,
    "tests/thrusection/specific/M6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "s1",
            &[0.0, 0.0, 100.0, 0.0, 0.0, -100.0]
        )),
        Step::Profile(Profile::Vertex("v4", [0.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [0.0, 0.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v4",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w1",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w11",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w21",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w111",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w101",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v5",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 185899.0, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 58)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_m7,
    "tests/thrusection/specific/M7",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("7166"),
        Step::Puts("============"),
        Step::Profile(Profile::Polyline(
            "s1",
            &[0.0, 0.0, 100.0, 0.0, 0.0, -100.0]
        )),
        Step::Profile(Profile::Vertex("v4", [0.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [0.0, 0.0, -100.0])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 40.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 40.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 40.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Profile(Profile::Vertex("v11", [15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v12", [-15.0, 15.0, 20.0])),
        Step::Profile(Profile::Vertex("v13", [-15.0, -15.0, 20.0])),
        Step::Profile(Profile::Vertex("v14", [15.0, -15.0, 20.0])),
        Step::Profile(Profile::EdgeOfVertices("e11", "v11", "v12")),
        Step::Profile(Profile::EdgeOfVertices("e12", "v12", "v13")),
        Step::Profile(Profile::EdgeOfVertices("e13", "v13", "v14")),
        Step::Profile(Profile::EdgeOfVertices("e14", "v14", "v11")),
        Step::Profile(Profile::Wire("w11", &["e11", "e12", "e13", "e14"])),
        Step::Profile(Profile::Vertex("v21", [25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v22", [-25.0, 25.0, 0.0])),
        Step::Profile(Profile::Vertex("v23", [-30.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v24", [-30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v25", [-25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v26", [25.0, -25.0, 0.0])),
        Step::Profile(Profile::Vertex("v27", [30.0, -10.0, 0.0])),
        Step::Profile(Profile::Vertex("v28", [30.0, 10.0, 0.0])),
        Step::Profile(Profile::EdgeOfVertices("e21", "v21", "v22")),
        Step::Profile(Profile::EdgeOfVertices("e22", "v22", "v23")),
        Step::Profile(Profile::EdgeOfVertices("e23", "v23", "v24")),
        Step::Profile(Profile::EdgeOfVertices("e24", "v24", "v25")),
        Step::Profile(Profile::EdgeOfVertices("e25", "v25", "v26")),
        Step::Profile(Profile::EdgeOfVertices("e26", "v26", "v27")),
        Step::Profile(Profile::EdgeOfVertices("e27", "v27", "v28")),
        Step::Profile(Profile::EdgeOfVertices("e28", "v28", "v21")),
        Step::Profile(Profile::Wire(
            "w21",
            &["e21", "e22", "e23", "e24", "e25", "e26", "e27", "e28"]
        )),
        Step::Profile(Profile::Vertex("v111", [15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v112", [-15.0, 15.0, -20.0])),
        Step::Profile(Profile::Vertex("v113", [-15.0, -15.0, -20.0])),
        Step::Profile(Profile::Vertex("v114", [15.0, -15.0, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e111", "v111", "v112")),
        Step::Profile(Profile::EdgeOfVertices("e112", "v112", "v113")),
        Step::Profile(Profile::EdgeOfVertices("e113", "v113", "v114")),
        Step::Profile(Profile::EdgeOfVertices("e114", "v114", "v111")),
        Step::Profile(Profile::Wire("w111", &["e111", "e112", "e113", "e114"])),
        Step::Profile(Profile::Vertex("v101", [0.0, 0.0, -40.0])),
        Step::Profile(Profile::Vertex("v102", [5.0, 5.0, -40.0])),
        Step::Profile(Profile::Vertex("v103", [0.0, 10.0, -40.0])),
        Step::Profile(Profile::EdgeOfVertices("e101", "v101", "v102")),
        Step::Profile(Profile::EdgeOfVertices("e102", "v102", "v103")),
        Step::Profile(Profile::EdgeOfVertices("e103", "v103", "v101")),
        Step::Profile(Profile::Wire("w101", &["e101", "e102", "e103"])),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v4",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w1",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w11",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w21",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w111",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w101",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "v5",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Catch(&Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        }))),
        Step::Check(Check::Props(Prop::Area, "result", 625461.0, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 33),
                ("WIRE", 11),
                ("FACE", 11),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 58)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);
