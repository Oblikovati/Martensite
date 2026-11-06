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
    pipe_specific_zb9,
    "tests/pipe/specific/ZB9",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T/-C -S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 51424.7, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 141046.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 48),
                ("WIRE", 24),
                ("FACE", 24),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 122)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc1,
    "tests/pipe/specific/ZC1",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-R/-C/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 51595.5, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 48),
                ("WIRE", 24),
                ("FACE", 24),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 121)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc2,
    "tests/pipe/specific/ZC2",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-R/-R/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 51376.4, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 48),
                ("EDGE", 90),
                ("WIRE", 42),
                ("FACE", 42),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 223)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc3,
    "tests/pipe/specific/ZC3",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-R/-C -S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 51595.5, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 147739.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 48),
                ("WIRE", 24),
                ("FACE", 24),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 122)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc4,
    "tests/pipe/specific/ZC4",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-R/-R -S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 51376.4, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 147316.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 48),
                ("EDGE", 90),
                ("WIRE", 42),
                ("FACE", 42),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 224)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc5,
    "tests/pipe/specific/ZC5",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T -R/-C/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 53770.3, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 48),
                ("WIRE", 24),
                ("FACE", 24),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 121)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc6,
    "tests/pipe/specific/ZC6",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T -R/-R/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 53202.2, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 34),
                ("EDGE", 73),
                ("WIRE", 39),
                ("FACE", 39),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 186)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc7,
    "tests/pipe/specific/ZC7",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T -R/-C -S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 53770.3, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 153966.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 24),
                ("EDGE", 48),
                ("WIRE", 24),
                ("FACE", 24),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 122)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_zc8,
    "tests/pipe/specific/ZC8",
    Shape,
    600,
    End::ViewIfBound,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477_closedPolyline.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T -R/-R -S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: true,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 53202.2, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 152339.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 34),
                ("EDGE", 73),
                ("WIRE", 39),
                ("FACE", 39),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 187)
            ]
        )),
    ]
);

offset_case!(
    pipe_standard_a1,
    "tests/pipe/standard/A1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/3_base.rle", "base")),
        Step::Profile(Profile::Plane("fbase", "base", false)),
        Step::Topo(Topo::Restore("brep/3_A1.rle", "profile")),
        Step::Sweep(Sweep::Pipe("result", "profile", "fbase", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 38226.4, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a2,
    "tests/pipe/standard/A2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 150.0, 5.0, 0.0, 1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 150.0, 0.0, 0.0, 200.0, 50.0, 0.0, 170.0, 70.0, 0.0, 40.0, 30.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 10.0, 0.0, 140.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 7833.49, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a3,
    "tests/pipe/standard/A3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[0.0, 35.0, 35.0, 0.0, 1.0, -1.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Ellipse(
            "w",
            &[0.0, 40.0, 0.0, 1.0, 0.0, 0.0, 100.0, 40.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 168406.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a4,
    "tests/pipe/standard/A4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Circle("w", &[0.0, 50.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 8497.19, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a5,
    "tests/pipe/standard/A5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Ellipse(
            "w",
            &[20.0, 20.0, 0.0, 1.0, 0.0, 0.0, 60.0, 20.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 7262.6, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a6,
    "tests/pipe/standard/A6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, -115.064945, 75.458923, 1.0, -1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Hyperbola(
            "w",
            &[10.0, 0.0, -50.0, 1.0, 0.0, 0.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 8185.6, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a7,
    "tests/pipe/standard/A7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Profile(Profile::Polyline(
            "w",
            &[10.0, 0.0, 0.0, 0.0, 30.0, 30.0, 0.0, 70.0, 70.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 2925.22, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a8,
    "tests/pipe/standard/A8",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[10.0, -62.831853, 19.7392088, 1.0, -1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Parabola("w", &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 3935.74, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_a9,
    "tests/pipe/standard/A9",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 100.0, 100.0, 100.0,
                1.0, 100.0, 0.0, 0.0, 1.0, -10.0, -10.0, -10.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 8684.67, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b1,
    "tests/pipe/standard/B1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 22741 All: Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Topo(Topo::Restore("brep/3_base.rle", "base")),
        Step::Profile(Profile::Plane("fbase", "base", false)),
        Step::Topo(Topo::Restore("brep/3_B1.rle", "profile")),
        Step::Sweep(Sweep::Pipe("result", "profile", "fbase", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 43132.1, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b2,
    "tests/pipe/standard/B2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 10.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Circle("w", &[0.0, 50.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 12045.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b3,
    "tests/pipe/standard/B3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[20.0, 0.0, 0.0, 0.0, 1.0, 1.0, 5.0, 2.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Ellipse(
            "w",
            &[20.0, 20.0, 0.0, 1.0, 0.0, 0.0, 60.0, 20.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 4755.96, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b4,
    "tests/pipe/standard/B4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[10.0, -115.064945, 75.458923, 1.0, -1.0, 1.0, 5.0, 2.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Hyperbola(
            "w",
            &[10.0, 0.0, -50.0, 1.0, 0.0, 0.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 5810.34, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b5,
    "tests/pipe/standard/B5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[10.0, 0.0, 0.0, 1.0, 0.0, 1.0, 10.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Profile(Profile::Polyline(
            "w",
            &[10.0, 0.0, 0.0, 0.0, 30.0, 30.0, 0.0, 70.0, 70.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 4272.58, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b6,
    "tests/pipe/standard/B6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[10.0, -62.831853, 19.7392088, 0.0, 1.0, 0.0, 10.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Parabola("w", &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 6058.02, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b7,
    "tests/pipe/standard/B7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 10.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 100.0, 100.0, 100.0,
                1.0, 100.0, 0.0, 0.0, 1.0, -10.0, -10.0, -10.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 14092.7, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b8,
    "tests/pipe/standard/B8",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Ellipse(
            "profile",
            &[0.0, 154.0, -1.5, 0.0, 1.0, 1.0, 4.0, 2.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 150.0, 0.0, 0.0, 200.0, 50.0, 0.0, 170.0, 70.0, 0.0, 40.0, 30.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 10.0, 0.0, 140.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 5448.91, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_b9,
    "tests/pipe/standard/B9",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[0.0, 35.0, 35.0, 0.0, 1.0, -1.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Circle("w", &[0.0, 100.0, 0.0, 1.0, 0.0, 0.0, 100.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 210218.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c1,
    "tests/pipe/standard/C1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/3_base.rle", "base")),
        Step::Profile(Profile::Plane("fbase", "base", false)),
        Step::Topo(Topo::Restore("brep/3_C1.rle", "profile")),
        Step::Sweep(Sweep::Pipe("result", "profile", "fbase", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 180694.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c2,
    "tests/pipe/standard/C2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[10.0, -105.0, 85.0, 0.0, 1.0, -1.0, 15.0, 15.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Hyperbola(
            "w",
            &[10.0, 0.0, -50.0, 1.0, 0.0, 0.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 24825.3, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c3,
    "tests/pipe/standard/C3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[10.0, 10.6, 10.6, 0.0, 1.0, -1.0, 15.0, 15.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Polyline(
            "w",
            &[10.0, 0.0, 0.0, 10.0, -30.0, 30.0, 10.0, -70.0, 90.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 9588.68, 0.01, None)),
    ]
);
