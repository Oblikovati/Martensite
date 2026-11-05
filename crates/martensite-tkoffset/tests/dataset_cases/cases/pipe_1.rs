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
    pipe_bugs_bug24049,
    "tests/pipe/bugs/bug24049",
    Shape,
    600,
    End::Nothing,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============="),
        Step::Puts("24049: Modeling Algorithms - cras-h in BRepOffsetAPIMakePipe"),
        Step::Puts("============="),
        Step::Topo(Topo::Restore("brep/bug24049_spine.brep", "spine")),
        Step::Profile(Profile::Edge("spine", "spine", None)),
        Step::Profile(Profile::Wire("spine", &["spine"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[
                27378.59961,
                -1139.028442,
                -621.0,
                0.0,
                0.0,
                -1.0,
                -1.0,
                0.0,
                -0.0,
                6.349999904633
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Sweep(Sweep::Pipe("pipe", "spine", "profile", 2, false)),
        Step::Check(Check::Shape("pipe")),
        Step::Puts("TEST COMPLETED"),
    ]
);

offset_case!(
    pipe_bugs_bug24909_1,
    "tests/pipe/bugs/bug24909_1",
    Shape,
    600,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO ?24909 ALL: An exception was caught"),
        Step::Puts("TODO ?24909 ALL: TEST INCOMPLETE"),
        Step::Puts("TODO ?24909 ALL: ScriptStep Exception: Buildsweep : Not Done"),
        Step::Puts("========"),
        Step::Puts("0024909: S I G S E G V in buildsweep"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug24909_sweep3-draw-Sweep-0-spine.brep",
            "Sweep-0-spine"
        )),
        Step::Shell(PipeShell::Start("Sweep-0-spine")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Topo(Topo::Restore(
            "brep/bug24909_sweep3-draw-Sweep-0-section-00-Sketch.brep",
            "Sweep-0-section-00-Sketch"
        )),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Sweep-0-section-00-Sketch",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Topo(Topo::Restore(
            "brep/bug24909_sweep3-draw-Sweep-0-section-01-Sketch001.brep",
            "Sweep-0-section-01-Sketch001"
        )),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Sweep-0-section-01-Sketch001",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep",
            mode: '-',
            solid: false
        })),
    ]
);

offset_case!(
    pipe_bugs_bug24909_3,
    "tests/pipe/bugs/bug24909_3",
    Shape,
    600,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 24909 ALL: Error: sweep failed"),
        Step::Puts("========"),
        Step::Puts("24909"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug24909_minihyd-draw-BezCurve.brep",
            "BezCurve"
        )),
        Step::Place(Place::Translate(&["BezCurve"], [0.0, -8.0, 0.0])),
        Step::Topo(Topo::Explode("BezCurve", "E")),
        Step::Profile(Profile::Wire("Sweep001-0-spine", &["BezCurve_1"])),
        Step::Shell(PipeShell::Start("Sweep001-0-spine")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Curve(Curve::Circle("Circle-curve", &[0.0, 0.0, 0.0, 18.0])),
        Step::Profile(Profile::Edge(
            "Circle-edge",
            "Circle-curve",
            Some([0.0, 0.0])
        )),
        Step::Profile(Profile::Wire("Circle", &["Circle-edge"])),
        Step::Place(Place::Rotate(
            &["Circle"],
            [
                0.0,
                0.0,
                0.0,
                0.5773502691896257,
                0.5773502691896257,
                0.5773502691896257,
                120.0
            ]
        )),
        Step::Place(Place::Translate(&["Circle"], [0.0, -71.0, 203.5])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Circle",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Curve(Curve::Circle("Circle001-curve", &[0.0, 0.0, 0.0, 16.0])),
        Step::Profile(Profile::Edge(
            "Circle001-edge",
            "Circle001-curve",
            Some([0.0, 0.0])
        )),
        Step::Profile(Profile::Wire("Circle001", &["Circle001-edge"])),
        Step::Place(Place::Rotate(
            &["Circle001"],
            [
                0.0,
                0.0,
                0.0,
                0.5773502691896257,
                0.5773502691896257,
                0.5773502691896257,
                120.0
            ]
        )),
        Step::Place(Place::Translate(&["Circle001"], [0.0, -71.0, 203.5])),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Circle001",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep001",
            mode: 'C',
            solid: true
        })),
        Step::Let("result", Value::SweepOutcome),
        Step::If(
            Cond::Holds("result", "Not Done"),
            &[Step::Puts("Error: sweep failed")],
            &[]
        ),
    ]
);

offset_case!(
    pipe_bugs_bug25110,
    "tests/pipe/bugs/bug25110",
    Shape,
    600,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("TODO 25110 ALL: An exception was caught"),
        Step::Puts("TODO 25110 ALL: TEST INCOMPLETE"),
        Step::Puts("========"),
        Step::Puts(
            "0025110: TCollection_IndexedDataMap::FindFromKey e r r o r when sweeping circle along BezierCurve"
        ),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug25110_unsweepable1-draw-Sweep-0-spine.brep",
            "Sweep-0-spine"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug25110_unsweepable1-draw-Sweep-0-section-00-Circle.brep",
            "Sweep-0-section-00-Circle"
        )),
        Step::Shell(PipeShell::Start("Sweep-0-spine")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "Sweep-0-section-00-Circle",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "Sweep",
            mode: 'C',
            solid: true
        })),
    ]
);

offset_case!(
    pipe_bugs_bug30148,
    "tests/pipe/bugs/bug30148",
    Shape,
    600,
    End::Nothing,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("============="),
        Step::Puts(
            "30148: Modeling Algorithms - exc-eption in BRepOffestAPI_MakePipe for some periodic BSpline spines"
        ),
        Step::Puts("============="),
        Step::Topo(Topo::Restore("brep/bug30148_1.brep", "profile")),
        Step::Topo(Topo::Restore("brep/bug30148_2.brep", "spine")),
        Step::Sweep(Sweep::Pipe("pipe", "spine", "profile", 0, false)),
        Step::Puts("TEST COMPLETED"),
    ]
);

offset_case!(
    pipe_bugs_bug32807,
    "tests/pipe/bugs/bug32807",
    Shape,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("bug32807: Sweep algorithm deal with EE more than one intersect points"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug32807_path.brep", "p")),
        Step::Topo(Topo::Restore("brep/bug32807_profile.brep", "pr")),
        Step::Shell(PipeShell::Start("p")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
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
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 34),
                ("EDGE", 68),
                ("WIRE", 34),
                ("FACE", 34),
                ("SHELL", 1)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    pipe_bugs_bug32809,
    "tests/pipe/bugs/bug32809",
    Shape,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("bug32809: Sweep algorithm with Round Corner does not work"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug32809_path.brep", "p")),
        Step::Topo(Topo::Restore("brep/bug32809_profile.brep", "pr")),
        Step::Shell(PipeShell::Start("p")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'R',
            solid: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 140),
                ("EDGE", 285),
                ("WIRE", 140),
                ("FACE", 140),
                ("SHELL", 1)
            ]
        )),
    ]
);

offset_case!(
    pipe_bugs_bug32818,
    "tests/pipe/bugs/bug32818",
    Shape,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("0032818: Modeling Algorithms - Result of sweep operation is invalid"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug32818.brep", "bug32818")),
        Step::Topo(Topo::ExplodeChildren("bug32818")),
        Step::Topo(Topo::Rename("bug32818_1", "p")),
        Step::Topo(Topo::Rename("bug32818_2", "pr")),
        Step::Shell(PipeShell::Start("p")),
        Step::Shell(PipeShell::CorrectedFrenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "pr",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: 'R',
            solid: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 80),
                ("EDGE", 160),
                ("WIRE", 80),
                ("FACE", 80),
                ("SHELL", 1),
                ("SOLID", 1)
            ]
        )),
    ]
);

offset_case!(
    pipe_specific_g7,
    "tests/pipe/specific/G7",
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
        Step::Check(Check::Props(Prop::Area, "result", 20489.7, 0.01, None)),
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
    pipe_specific_g8,
    "tests/pipe/specific/G8",
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
        Step::Check(Check::Props(Prop::Area, "result", 20709.4, 0.01, None)),
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
    pipe_specific_g9,
    "tests/pipe/specific/G9",
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
    pipe_specific_h1,
    "tests/pipe/specific/H1",
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
    pipe_specific_h2,
    "tests/pipe/specific/H2",
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
    pipe_specific_h3,
    "tests/pipe/specific/H3",
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
    pipe_specific_h4,
    "tests/pipe/specific/H4",
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
    pipe_specific_h5,
    "tests/pipe/specific/H5",
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
    pipe_specific_h6,
    "tests/pipe/specific/H6",
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
    pipe_specific_h7,
    "tests/pipe/specific/H7",
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
        Step::Check(Check::Props(Prop::Area, "result", 20489.7, 0.01, None)),
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
    pipe_specific_h8,
    "tests/pipe/specific/H8",
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
            Step::Check(Check::Props(Prop::Area, "result", 20752.1, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 48604.1, 0.01, None))
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
    pipe_specific_h9,
    "tests/pipe/specific/H9",
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
        Step::Check(Check::Props(Prop::Area, "result", 20709.4, 0.01, None)),
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
    pipe_specific_i1,
    "tests/pipe/specific/I1",
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
    pipe_specific_i2,
    "tests/pipe/specific/I2",
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
    pipe_specific_i3,
    "tests/pipe/specific/I3",
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
    pipe_specific_i4,
    "tests/pipe/specific/I4",
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
    pipe_specific_i5,
    "tests/pipe/specific/I5",
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
    pipe_specific_i6,
    "tests/pipe/specific/I6",
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
    pipe_specific_i7,
    "tests/pipe/specific/I7",
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
    pipe_specific_i8,
    "tests/pipe/specific/I8",
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
        Step::Check(Check::Props(Prop::Area, "result", 20489.7, 0.01, None)),
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
    pipe_specific_i9,
    "tests/pipe/specific/I9",
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
            Step::Check(Check::Props(Prop::Area, "result", 20752.1, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 48604.1, 0.01, None))
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
    pipe_specific_j1,
    "tests/pipe/specific/J1",
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
        Step::Check(Check::Props(Prop::Area, "result", 20709.4, 0.01, None)),
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
