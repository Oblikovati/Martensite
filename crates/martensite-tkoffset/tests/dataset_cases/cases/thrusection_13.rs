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
    thrusection_specific_m8,
    "tests/thrusection/specific/M8",
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
            translate: true,
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
            translate: true,
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
    thrusection_specific_m9,
    "tests/thrusection/specific/M9",
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
            translate: true,
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
            translate: true,
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

offset_case!(
    thrusection_specific_n1,
    "tests/thrusection/specific/N1",
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
        Step::Profile(Profile::Vertex("v4", [40.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [40.0, 0.0, -100.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 186534.0, 0.01, None)),
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
    thrusection_specific_n2,
    "tests/thrusection/specific/N2",
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
        Step::Profile(Profile::Vertex("v4", [40.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [40.0, 0.0, -100.0])),
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
        Step::Check(Check::Props(Prop::Area, "result", 622222.0, 0.01, None)),
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
    thrusection_specific_n3,
    "tests/thrusection/specific/N3",
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
        Step::Profile(Profile::Vertex("v4", [40.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [40.0, 0.0, -100.0])),
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
            translate: true,
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
            translate: true,
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
    thrusection_specific_n4,
    "tests/thrusection/specific/N4",
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
        Step::Profile(Profile::Vertex("v4", [40.0, 0.0, 100.0])),
        Step::Profile(Profile::Vertex("v5", [40.0, 0.0, -100.0])),
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
            translate: true,
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
            translate: true,
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

offset_case!(
    thrusection_specific_n5,
    "tests/thrusection/specific/N5",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts(
            "28642: BRepOffsetAPIThruSections/BRepFillGenerator are modifying the input shape sections"
        ),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline("p1", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p2", &[2.0, 1.0, 0.0, 2.0, 2.0, 0.0])),
        Step::Profile(Profile::Polyline("p3", &[3.0, 0.0, 0.0, 4.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p4", &[2.0, -1.0, 0.0, 2.0, -2.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["p1", "p2", "p3", "p4", "p1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 10.1587, 0.01, None)),
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
    thrusection_specific_n6,
    "tests/thrusection/specific/N6",
    Shape,
    600,
    End::ResultView("Error : The command cannot be built.", false),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts(
            "28642: BRepOffsetAPIThruSections/BRepFillGenerator are modifying the input shape sections"
        ),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline("p1", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p2", &[2.0, 1.0, 0.0, 2.0, 2.0, 0.0])),
        Step::Profile(Profile::Polyline("p3", &[3.0, 0.0, 0.0, 4.0, 0.0, 0.0])),
        Step::Profile(Profile::Polyline("p4", &[2.0, -1.0, 0.0, 2.0, -2.0, 0.0])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: true,
            sections: &["p1", "p2", "p3", "p4", "p1"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 6.0, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 12),
                ("WIRE", 4),
                ("FACE", 4),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 29)
            ]
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a1,
    "tests/thrusection/specific_locked/A1",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 490.101, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 27)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a2,
    "tests/thrusection/specific_locked/A2",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 490.101, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 28)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a3,
    "tests/thrusection/specific_locked/A3",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 651.466, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 18)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a4,
    "tests/thrusection/specific_locked/A4",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [0.0, 10.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 651.466, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 19)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a6,
    "tests/thrusection/specific_locked/A6",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 485.502, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 9),
                ("WIRE", 6),
                ("FACE", 6),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 28)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a7,
    "tests/thrusection/specific_locked/A7",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 646.621, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 18)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_a8,
    "tests/thrusection/specific_locked/A8",
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
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [5.0, 5.0, 0.0])),
        Step::Profile(Profile::Vertex("v3", [10.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v4", [5.0, 2.5, 20.0])),
        Step::Profile(Profile::Vertex("v5", [5.0, 2.5, -20.0])),
        Step::Profile(Profile::EdgeOfVertices("e1", "v1", "v2")),
        Step::Profile(Profile::EdgeOfVertices("e2", "v2", "v3")),
        Step::Profile(Profile::EdgeOfVertices("e3", "v3", "v1")),
        Step::Profile(Profile::Wire("w1", &["e1", "e2", "e3"])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v4", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v5", &[("locked", true)])),
        Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v4", "w1", "v5"]
        }),
        Step::Check(Check::Props(Prop::Area, "result", 646.621, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 9),
                ("WIRE", 3),
                ("FACE", 3),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 19)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_b1,
    "tests/thrusection/specific_locked/B1",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1404.96, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_b2,
    "tests/thrusection/specific_locked/B2",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1829.23, 0.01, None)),
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
    thrusection_specific_locked_b3,
    "tests/thrusection/specific_locked/B3",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1829.23, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_b5,
    "tests/thrusection/specific_locked/B5",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2160.94, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_b6,
    "tests/thrusection/specific_locked/B6",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2556.25, 0.01, None)),
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
    thrusection_specific_locked_b7,
    "tests/thrusection/specific_locked/B7",
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
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 2556.25, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 2),
                ("EDGE", 3),
                ("WIRE", 1),
                ("FACE", 1),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_b9,
    "tests/thrusection/specific_locked/B9",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1021.13, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_c1,
    "tests/thrusection/specific_locked/C1",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
    thrusection_specific_locked_c2,
    "tests/thrusection/specific_locked/C2",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_c4,
    "tests/thrusection/specific_locked/C4",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 1353.78, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_c5,
    "tests/thrusection/specific_locked/C5",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
    thrusection_specific_locked_c6,
    "tests/thrusection/specific_locked/C6",
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
        Step::Curve(Curve::Ellipse("ellipse1", &[0.0, 0.0, 0.0, 10.0, 5.0])),
        Step::Profile(Profile::Edge("e1", "ellipse1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [40.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [40.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_c8,
    "tests/thrusection/specific_locked/C8",
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
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: true,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
        })),
        Step::Check(Check::Props(Prop::Area, "result", 449.295, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 3),
                ("EDGE", 5),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 14)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);

offset_case!(
    thrusection_specific_locked_c9,
    "tests/thrusection/specific_locked/C9",
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
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
    thrusection_specific_locked_d1,
    "tests/thrusection/specific_locked/D1",
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
        Step::Curve(Curve::Bezier(
            "bc1",
            &[
                5.0, 5.0, 5.0, 0.0, 5.0, -5.0, 0.0, -5.0, -5.0, 0.0, -5.0, 5.0, 0.0, 5.0, 5.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e1", "bc1", None)),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 20.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, 0.0, -20.0])),
        Step::Topo(Topo::SetFlags("w1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v1", &[("locked", true)])),
        Step::Topo(Topo::SetFlags("v2", &[("locked", true)])),
        Step::Catch(&Step::Loft(Loft {
            result: "result",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: false,
            sections: &["v1", "w1", "v2"]
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
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 9)
            ]
        )),
        Step::Check(Check::ClosedElse(
            "result",
            "Faulty 7166 : result is not Closed shape"
        )),
    ]
);
