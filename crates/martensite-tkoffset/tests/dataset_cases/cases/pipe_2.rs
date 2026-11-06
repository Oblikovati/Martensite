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
    pipe_specific_j2,
    "tests/pipe/specific/J2",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
            Step::Check(Check::Props(Prop::Area, "result", 20971.7, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 49125.1, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 20),
                ("EDGE", 36),
                ("WIRE", 18),
                ("FACE", 18),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 94)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j3,
    "tests/pipe/specific/J3",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 24005.2, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 20),
                ("EDGE", 36),
                ("WIRE", 16),
                ("FACE", 16),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 89)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j4,
    "tests/pipe/specific/J4",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
            Step::Check(Check::Props(Prop::Area, "result", 24267.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 68736.5, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 20),
                ("EDGE", 36),
                ("WIRE", 18),
                ("FACE", 18),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 94)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j5,
    "tests/pipe/specific/J5",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 24371.2, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 20),
                ("EDGE", 36),
                ("WIRE", 16),
                ("FACE", 16),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 89)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j6,
    "tests/pipe/specific/J6",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
        Step::Check(Check::Props(Prop::Area, "result", 23772.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 32),
                ("EDGE", 61),
                ("WIRE", 25),
                ("FACE", 25),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 144)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j7,
    "tests/pipe/specific/J7",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
            Step::Check(Check::Props(Prop::Area, "result", 24633.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 69784.6, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 20),
                ("EDGE", 36),
                ("WIRE", 18),
                ("FACE", 18),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 94)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j8,
    "tests/pipe/specific/J8",
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
        Step::Topo(Topo::Restore("brep/OCC1477_3dPolyline_c0.brep", "s1")),
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
            Step::Check(Check::Props(Prop::Area, "result", 24035.3, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 68299.1, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 32),
                ("EDGE", 61),
                ("WIRE", 27),
                ("FACE", 27),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 149)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_j9,
    "tests/pipe/specific/J9",
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
        Step::Curve(Curve::Bezier(
            "c11",
            &[
                5.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 20.0, 10.0, 12.0, 25.0, 30.0, 20.0, 50.0, 40.0,
                50.0
            ]
        )),
        Step::Curve(Curve::Bezier(
            "c12",
            &[
                4.0, 50.0, 40.0, 50.0, 70.0, 30.0, 30.0, 90.0, 20.0, 20.0, 100.0, 50.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("e11", "c11", None)),
        Step::Profile(Profile::Edge("e12", "c12", None)),
        Step::Profile(Profile::Wire("s1", &["e11", "e12"])),
        Step::Curve(Curve::Bezier(
            "c21",
            &[
                4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 0.0, 10.0, 20.0
            ]
        )),
        Step::Profile(Profile::Edge("e21", "c21", None)),
        Step::Profile(Profile::Wire("s2", &["e21"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 2570.32, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 6),
                ("EDGE", 7),
                ("WIRE", 2),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 18)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_k1,
    "tests/pipe/specific/K1",
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
        Step::Profile(Profile::Polyline(
            "s1",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 10.0, 10.0, 0.0, 10.0, 10.0,
                0.0, 10.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "s2",
            &[
                0.0, -2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0, 2.0, 0.0, -2.0, 2.0, 0.0, -2.0, -2.0
            ]
        )),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-/-R/)"),
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
            rotate: false,
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
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 929.097, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 939.398, 0.01, None))
        ]),
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
    pipe_specific_k2,
    "tests/pipe/specific/K2",
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
        Step::Profile(Profile::Polyline(
            "s1",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0]
        )),
        Step::Profile(Profile::Polyline(
            "w2",
            &[0.0, -10.0, 0.0, 20.0, -10.0, 10.0, 20.0, 10.0, 0.0]
        )),
        Step::Curve(Curve::PeriodicBSpline(
            "c3",
            &[
                3.0, 9.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0, 1.0, 4.0, 1.0, 5.0, 1.0, 6.0, 1.0,
                7.0, 1.0, 8.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -2.0, 2.0, 1.0, 0.0, 0.0, 1.0, 1.0,
                0.0, 2.0, 2.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 2.0, -2.0, 1.0, 0.0, 0.0, -1.0, 1.0,
                0.0, -2.0, -2.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("e3", "c3", None)),
        Step::Profile(Profile::Wire("s2", &["e3"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-G/-/-R/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Guide("w2", false, 0)),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
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
        Step::Check(Check::Props(Prop::Area, "result", 242.434, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 7),
                ("EDGE", 13),
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
    pipe_specific_q3,
    "tests/pipe/specific/Q3",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 27135.6, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q4,
    "tests/pipe/specific/Q4",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 27397.8, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 68648.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q5,
    "tests/pipe/specific/Q5",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-T/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 28434.6, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q6,
    "tests/pipe/specific/Q6",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-T/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 28696.8, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 72386.2, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q7,
    "tests/pipe/specific/Q7",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-R/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 27289.6, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q8,
    "tests/pipe/specific/Q8",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-R/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 27552.3, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 65400.7, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_q9,
    "tests/pipe/specific/Q9",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-T -R/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 28489.1, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r1,
    "tests/pipe/specific/R1",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-/-T -R/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 28751.0, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 69136.9, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r2,
    "tests/pipe/specific/R2",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
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
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 25522.2, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r3,
    "tests/pipe/specific/R3",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 25784.5, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 50977.6, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r4,
    "tests/pipe/specific/R4",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-T/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 27210.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r5,
    "tests/pipe/specific/R5",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-T/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 27472.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 55282.1, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r6,
    "tests/pipe/specific/R6",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-R/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 27294.9, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r7,
    "tests/pipe/specific/R7",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-R/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 27557.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 65439.6, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r8,
    "tests/pipe/specific/R8",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-T -R/-/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 28939.0, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_r9,
    "tests/pipe/specific/R9",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-FR/-T -R/-S/)"),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Frenet),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 29200.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 69371.6, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_s1,
    "tests/pipe/specific/S1",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-/-/)"),
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
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 27135.6, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_s2,
    "tests/pipe/specific/S2",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-/-S/)"),
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
            rotate: false,
            law: &[]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 27397.8, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 68648.0, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_s3,
    "tests/pipe/specific/S3",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T/-/)"),
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
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 28434.6, 0.01, None)),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 8),
                ("FACE", 8),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 49)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_s4,
    "tests/pipe/specific/S4",
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
        Step::Topo(Topo::Restore("brep/OCC1477_bsplinewire_c0.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477_profile1_rectangle.brep", "e1")),
        Step::Profile(Profile::Wire("s2", &["e1"])),
        Step::Puts(
            "###############################################################################"
        ),
        Step::Puts("# (case /-CF/-T/-S/)"),
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
                mode: 'M',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 28696.8, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 72386.2, 0.01, None))
        ]),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 12),
                ("EDGE", 20),
                ("WIRE", 10),
                ("FACE", 10),
                ("SHELL", 1),
                ("SOLID", 1),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 54)
            ]
        )),
    ]
);
