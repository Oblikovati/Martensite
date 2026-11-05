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
    pipe_standard_f7,
    "tests/pipe/standard/F7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "profile",
            &[
                6.0, 0.0, 0.0, 20.0, -10.0, 0.0, 0.0, -10.0, 0.0, -40.0, 10.0, 0.0, -80.0, 10.0,
                0.0, 0.0, 0.0, 0.0, 20.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 0.0, 0.0, 0.0, 2000.0, 50.0, 0.0, 1700.0, 700.0, 0.0, 400.0, 300.0, 0.0,
                0.0, 0.0, 0.0, 900.0, 400.0, 0.0, 140.0, 100.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 301365.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_g1,
    "tests/pipe/standard/G1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, 9.0, 0.0, 9.1])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 32.412, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_g2,
    "tests/pipe/standard/G2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, 9.0, 0.0, 9.01])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
    ]
);

offset_case!(
    pipe_standard_g3,
    "tests/pipe/standard/G3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, 9.0, 0.0, 9.001])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
    ]
);

offset_case!(
    pipe_standard_g4,
    "tests/pipe/standard/G4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, -9.0, 0.0, 9.1])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 36.5019, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_g5,
    "tests/pipe/standard/G5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, -9.0, 0.0, 9.01])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 37.4358, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_g6,
    "tests/pipe/standard/G6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("c1", &[0.0, 10.0, 0.0, 10.0])),
        Step::Curve(Curve::Circle("c2", &[0.0, -9.0, 0.0, 9.001])),
        Step::Curve(Curve::Circle(
            "c3",
            &[10.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.05]
        )),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("profile", "c3", None)),
        Step::More(More::General(General::ClearObjects)),
        Step::More(More::General(General::AddObjects(&["e1", "e2"]))),
        Step::More(More::General(General::Fill)),
        Step::More(More::General(General::Build("result"))),
        Step::Topo(Topo::Explode("result", "e")),
        Step::Profile(Profile::Wire(
            "path",
            &["result_1", "result_3", "result_4", "result_6"]
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Shell(PipeShell::Start("path")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "profile",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'C',
            solid: false
        })),
    ]
);
