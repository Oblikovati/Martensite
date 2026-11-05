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
    pipe_specific_s5,
    "tests/pipe/specific/S5",
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
        Step::Puts("# (case /-CF/-R/-/)"),
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
    pipe_specific_s6,
    "tests/pipe/specific/S6",
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
        Step::Puts("# (case /-CF/-R/-S/)"),
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
    pipe_specific_s7,
    "tests/pipe/specific/S7",
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
        Step::Puts("# (case /-CF/-T -R/-S/)"),
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
    pipe_specific_y9,
    "tests/pipe/specific/Y9",
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
        Step::Puts("# (case /-/-/-C/)"),
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
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
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
    pipe_specific_z1,
    "tests/pipe/specific/Z1",
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
        Step::Puts("# (case /-/-/-C -S/)"),
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
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 135793.0, 0.01, None))
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
    pipe_specific_z2,
    "tests/pipe/specific/Z2",
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
        Step::Puts("# (case /-/-T/-C/)"),
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
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 51424.7, 0.01, None)),
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
    pipe_specific_z3,
    "tests/pipe/specific/Z3",
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
        Step::Puts("# (case /-/-T/-C -S/)"),
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
    pipe_specific_z4,
    "tests/pipe/specific/Z4",
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
        Step::Puts("# (case /-/-R/-C/)"),
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
    pipe_specific_z5,
    "tests/pipe/specific/Z5",
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
        Step::Puts("# (case /-/-R/-R/)"),
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
    pipe_specific_z6,
    "tests/pipe/specific/Z6",
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
        Step::Puts("# (case /-/-R/-C -S/)"),
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
    pipe_specific_z7,
    "tests/pipe/specific/Z7",
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
        Step::Puts("# (case /-/-R/-R -S/)"),
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
    pipe_specific_z8,
    "tests/pipe/specific/Z8",
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
        Step::Puts("# (case /-/-T -R/-C/)"),
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
    pipe_specific_z9,
    "tests/pipe/specific/Z9",
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
        Step::Puts("# (case /-/-T -R/-R/)"),
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
    pipe_specific_za1,
    "tests/pipe/specific/ZA1",
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
        Step::Puts("# (case /-/-T -R/-C -S/)"),
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
    pipe_specific_za2,
    "tests/pipe/specific/ZA2",
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
        Step::Puts("# (case /-/-T -R/-R -S/)"),
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
    pipe_specific_za3,
    "tests/pipe/specific/ZA3",
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
        Step::Puts("# (case /-FR/-/-C/)"),
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
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
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
    pipe_specific_za4,
    "tests/pipe/specific/ZA4",
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
        Step::Puts("# (case /-FR/-/-C -S/)"),
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
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 135793.0, 0.01, None))
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
    pipe_specific_za5,
    "tests/pipe/specific/ZA5",
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
        Step::Puts("# (case /-FR/-T/-C/)"),
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
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 51424.7, 0.01, None)),
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
    pipe_specific_za6,
    "tests/pipe/specific/ZA6",
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
        Step::Puts("# (case /-FR/-T/-C -S/)"),
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
    pipe_specific_za7,
    "tests/pipe/specific/ZA7",
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
        Step::Puts("# (case /-FR/-R/-C/)"),
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
    pipe_specific_za8,
    "tests/pipe/specific/ZA8",
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
        Step::Puts("# (case /-FR/-R/-R/)"),
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
    pipe_specific_za9,
    "tests/pipe/specific/ZA9",
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
        Step::Puts("# (case /-FR/-R/-C -S/)"),
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
    pipe_specific_zb1,
    "tests/pipe/specific/ZB1",
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
        Step::Puts("# (case /-FR/-R/-R -S/)"),
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
    pipe_specific_zb2,
    "tests/pipe/specific/ZB2",
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
        Step::Puts("# (case /-FR/-T -R/-C/)"),
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
    pipe_specific_zb3,
    "tests/pipe/specific/ZB3",
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
        Step::Puts("# (case /-FR/-T -R/-R/)"),
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
    pipe_specific_zb4,
    "tests/pipe/specific/ZB4",
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
        Step::Puts("# (case /-FR/-T -R/-C -S/)"),
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
    pipe_specific_zb5,
    "tests/pipe/specific/ZB5",
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
        Step::Puts("# (case /-FR/-T -R/-R -S/)"),
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
    pipe_specific_zb6,
    "tests/pipe/specific/ZB6",
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
        Step::Puts("# (case /-CF/-/-C/)"),
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
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
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
    pipe_specific_zb7,
    "tests/pipe/specific/ZB7",
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
        Step::Puts("# (case /-CF/-/-C -S/)"),
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
                mode: 'C',
                solid: true
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 49509.6, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 135793.0, 0.01, None))
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
    pipe_specific_zb8,
    "tests/pipe/specific/ZB8",
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
        Step::Puts("# (case /-CF/-T/-C/)"),
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
                solid: false
            }))),
            &[Step::Puts("Faulty 1477: buildsweep is wrong")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 51424.7, 0.01, None)),
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
