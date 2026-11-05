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
    bugs_modalg_1_buc60649,
    "tests/bugs/modalg_1/buc60649",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("=========="),
        Step::Puts("BUC60649"),
        Step::Puts("BUC60657"),
        Step::Puts("=========="),
        Step::Topo(Topo::Restore("brep/buc60649a.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::Offset(Offset::Planar(PlanarOffset {
                    result: "result",
                    base: "a",
                    count: 1,
                    step: 1.0,
                    approx: false,
                    join: 'a',
                    altitude: 0.0,
                    open: false
                })),
                Step::Topo(Topo::Rename("result_1", "result"))
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 271.473, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60714,
    "tests/bugs/modalg_1/buc60714",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("BUC60714"),
        Step::Puts("========================"),
        Step::More(More::Moves("f", &["x", "30.0", "y", "20.0", "x", "-30.0"])),
        Step::Topo(Topo::Explode("f", "w")),
        Step::Profile(Profile::Vertex("v1", [0.0, 0.0, 0.0])),
        Step::Profile(Profile::Vertex("v2", [0.0, -10.0, -10.0])),
        Step::Profile(Profile::EdgeOfVertices("e", "v1", "v2")),
        Step::Profile(Profile::Wire("w", &["e"])),
        Step::Shell(PipeShell::Start("f_1")),
        Step::Shell(PipeShell::Frenet),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w",
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
        Step::Check(Check::Props(Prop::Area, "result", 1979.9, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60777,
    "tests/bugs/modalg_1/buc60777",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("BUC60777"),
        Step::Puts("========================"),
        Step::More(More::Moves(
            "p",
            &[
                "x", "30.0", "y", "-10.0", "x", "10.0", "y", "10.0", "x", "10.0", "y", "30.0", "t",
                "-30.0", "-20.0", "t", "-30.0", "20.0"
            ]
        )),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "result",
            base: "p",
            count: 1,
            step: -1.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Explode("p", "w")),
        Step::Let("end", Value::Mass(Prop::Length, "result_1")),
        Step::Let("end1", Value::Mass(Prop::Length, "p_1")),
        Step::If(
            Cond::NotBelow("end", "end1"),
            &[Step::Puts(" Faulty Mkoffset operation was made  wrongly")],
            &[Step::Puts(" Mkoffset  operation was made successfully")]
        ),
        Step::Topo(Topo::Rename("result_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 191.044, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60782_1,
    "tests/bugs/modalg_1/buc60782_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("BUC60782"),
        Step::Puts("CASE 1"),
        Step::Puts("========================"),
        Step::Puts("Results of Geom2d_OffsetCurve are sometimes strange (not correct). "),
        Step::Topo(Topo::Restore("brep/buc60782a.brep", "a")),
        Step::Let("che", Value::CheckShape("a")),
        Step::If(
            Cond::Holds("che", "Faulty"),
            &[Step::Puts(
                "Faulty BUC60782 (case 1): Source shape is invalid"
            )],
            &[Step::Puts("BUC60782 OK (case 1): Sourse shape is correct")]
        ),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::If(
                    Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                        result: "result",
                        base: "a",
                        count: 1,
                        step: 10.0,
                        approx: false,
                        join: 'a',
                        altitude: 0.0,
                        open: false
                    }))),
                    &[
                        Step::Puts("Faulty BUC60782: mkoffset works wrongly"),
                        Step::Topo(Topo::Rename("a", "result"))
                    ],
                    &[
                        Step::Puts("OK BUC60782: offset is created"),
                        Step::Topo(Topo::Rename("result_1", "result"))
                    ]
                )
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 9026.65, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60782_2,
    "tests/bugs/modalg_1/buc60782_2",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Puts(
            "TODO 12345 ALL: Faulty : mistakes are found in SOURCE  shape by checkshape command"
        ),
        Step::Puts("TODO 12345 ALL: Faulty BUC60782"),
        Step::Puts("========================"),
        Step::Puts("BUC60782"),
        Step::Puts("CASE 2"),
        Step::Puts("========================"),
        Step::Puts("Results of Geom2d_OffsetCurve are sometimes strange (not correct). "),
        Step::Topo(Topo::Restore("brep/buc60782b.brep", "a")),
        Step::Let("che", Value::CheckShape("a")),
        Step::If(
            Cond::Holds("che", "Faulty"),
            &[Step::Puts(
                "Faulty BUC60782 (case 2): Source shape is invalid"
            )],
            &[Step::Puts("BUC60782 OK (case 2): Sourse shape is correct")]
        ),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::If(
                    Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                        result: "result",
                        base: "a",
                        count: 1,
                        step: 10.0,
                        approx: false,
                        join: 'a',
                        altitude: 0.0,
                        open: false
                    }))),
                    &[
                        Step::Puts("Faulty BUC60782: mkoffset works wrongly"),
                        Step::Topo(Topo::Rename("a", "result"))
                    ],
                    &[
                        Step::Puts("OK BUC60782: offset is created"),
                        Step::Stop("tpcompound result: the reference run does not reach it")
                    ]
                ),
                Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
                Step::Check(Check::Shape("result")),
                Step::CheckView("result")
            ]
        ),
    ]
);

offset_case!(
    bugs_modalg_1_buc60782_3,
    "tests/bugs/modalg_1/buc60782_3",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 25919 ALL: Error: Offset is not done."),
        Step::Puts("TODO 25919 ALL: Faulty BUC60782: mkoffset works wrongly"),
        Step::Puts("TODO 25919 ALL: Error : The area of result shape is"),
        Step::Puts("========================"),
        Step::Puts("BUC60782"),
        Step::Puts("CASE 3"),
        Step::Puts("========================"),
        Step::Puts("Results of Geom2d_OffsetCurve are sometimes strange (not correct). "),
        Step::Topo(Topo::Restore("brep/buc60782c.brep", "a")),
        Step::Let("che", Value::CheckShape("a")),
        Step::If(
            Cond::Holds("che", "Faulty"),
            &[Step::Puts(
                "Faulty BUC60782 (case 3): Source shape is invalid"
            )],
            &[Step::Puts("BUC60782 OK (case 3): Sourse shape is correct")]
        ),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::If(
                    Cond::Failed(&Step::Offset(Offset::Planar(PlanarOffset {
                        result: "result",
                        base: "a",
                        count: 1,
                        step: 10.0,
                        approx: false,
                        join: 'a',
                        altitude: 0.0,
                        open: false
                    }))),
                    &[
                        Step::Puts("Faulty BUC60782: mkoffset works wrongly"),
                        Step::Topo(Topo::Rename("a", "result"))
                    ],
                    &[
                        Step::Puts("OK BUC60782: offset is created"),
                        Step::Stop("tpcompound result: the reference run does not reach it")
                    ]
                )
            ]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60830_2,
    "tests/bugs/modalg_1/buc60830_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============="),
        Step::Puts("BUC60830"),
        Step::Puts("BUC60829"),
        Step::Puts("============="),
        Step::Extra(Extra::CoordLoad("res", "others/cont1.dat")),
        Step::Extra(Extra::TopoLoad("res")),
        Step::Extra(Extra::DrawContours),
        Step::Extra(Extra::Side(false)),
        Step::Extra(Extra::Mat),
        Step::Extra(Extra::MatResult),
        Step::Extra(Extra::Side(true)),
        Step::Extra(Extra::Mat),
        Step::Extra(Extra::MatResult),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::View(ViewStep::Xwd("photo")),
    ]
);

offset_case!(
    bugs_modalg_1_buc60830_3,
    "tests/bugs/modalg_1/buc60830_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============="),
        Step::Puts("BUC60830"),
        Step::Puts("BUC60829"),
        Step::Puts("============="),
        Step::Extra(Extra::CoordLoad("res", "others/cont2.dat")),
        Step::Extra(Extra::TopoLoad("res")),
        Step::Extra(Extra::DrawContours),
        Step::Extra(Extra::Side(false)),
        Step::Extra(Extra::Mat),
        Step::Extra(Extra::MatResult),
        Step::Extra(Extra::Side(true)),
        Step::Extra(Extra::Mat),
        Step::Extra(Extra::MatResult),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::View(ViewStep::Xwd("photo")),
    ]
);

offset_case!(
    bugs_modalg_1_buc60878_1,
    "tests/bugs/modalg_1/buc60878_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============="),
        Step::Puts("BUC60878"),
        Step::Puts("============="),
        Step::Topo(Topo::Restore("brep/BUC60878-wire1.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::Offset(Offset::Planar(PlanarOffset {
                    result: "result",
                    base: "a",
                    count: 1,
                    step: 0.5,
                    approx: false,
                    join: 'a',
                    altitude: 0.0,
                    open: false
                })),
                Step::Topo(Topo::Rename("result_1", "result"))
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 370.796, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60878_2,
    "tests/bugs/modalg_1/buc60878_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============="),
        Step::Puts("BUC60878"),
        Step::Puts("============="),
        Step::Topo(Topo::Restore("brep/BUC60878-wire3.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::Offset(Offset::Planar(PlanarOffset {
                    result: "result",
                    base: "a",
                    count: 1,
                    step: 0.25,
                    approx: false,
                    join: 'a',
                    altitude: 0.0,
                    open: false
                })),
                Step::Topo(Topo::Rename("result_1", "result"))
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 11.2406, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_buc60878_3,
    "tests/bugs/modalg_1/buc60878_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============="),
        Step::Puts("BUC60878"),
        Step::Puts("============="),
        Step::Topo(Topo::Restore("brep/BUC60878-wire5.brep", "a")),
        Step::Check(Check::Shape("a")),
        Step::Let("che", Value::CheckShapeNamed("a")),
        Step::If(
            Cond::Lacks("che", "OK"),
            &[
                Step::Puts("Faulty : mistakes are found in SOURCE  shape by checkshape command"),
                Step::Puts("Body of the script was not executed")
            ],
            &[
                Step::Puts("Checking of SOURCE shape by checkshape - OK"),
                Step::Offset(Offset::Planar(PlanarOffset {
                    result: "result",
                    base: "a",
                    count: 1,
                    step: 4.0,
                    approx: false,
                    join: 'a',
                    altitude: 0.0,
                    open: false
                })),
                Step::Topo(Topo::Rename("result_1", "result"))
            ]
        ),
        Step::Check(Check::Props(Prop::Length, "result", 107.351, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug110_1,
    "tests/bugs/modalg_1/bug110_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("  BUC60706 "),
        Step::Puts(" 110 "),
        Step::Puts("(cases 1-3)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706a.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706c.brep", "s")),
        Step::Check(Check::Shape("s")),
        Step::Bool(Bool::Cut("res_1", "s", "t")),
        Step::Check(Check::Shape("res_1")),
        Step::Topo(Topo::Restore("brep/buc60706d.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("res_2", "res_1", "t")),
        Step::Check(Check::Shape("res_2")),
        Step::Topo(Topo::Restore("brep/buc60706g.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("result", "res_2", "t")),
        Step::Check(Check::Props(Prop::Area, "result", 865172.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug110_2,
    "tests/bugs/modalg_1/bug110_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("  BUC60706 "),
        Step::Puts(" 110 "),
        Step::Puts("(case 4-6)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706j.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706l.brep", "s")),
        Step::Check(Check::Shape("s")),
        Step::Bool(Bool::Cut("res_1", "s", "t")),
        Step::Check(Check::Shape("res_1")),
        Step::Topo(Topo::Restore("brep/buc60706m.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("res_2", "res_1", "t")),
        Step::Check(Check::Shape("res_2")),
        Step::Topo(Topo::Restore("brep/buc60706p.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("result", "res_2", "t")),
        Step::Check(Check::Props(Prop::Area, "result", 860752.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug110_3,
    "tests/bugs/modalg_1/bug110_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========================"),
        Step::Puts("  BUC60706 "),
        Step::Puts(" 110 "),
        Step::Puts("(case 7-9)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706s.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706l.brep", "s")),
        Step::Check(Check::Shape("s")),
        Step::Bool(Bool::Cut("res_1", "s", "t")),
        Step::Check(Check::Shape("res_1")),
        Step::Topo(Topo::Restore("brep/buc60706v.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("res_2", "res_1", "t")),
        Step::Check(Check::Shape("res_2")),
        Step::Topo(Topo::Restore("brep/buc60706x.brep", "f")),
        Step::Check(Check::Shape("f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Check(Check::Shape("w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Bool(Bool::Cut("result", "res_2", "t")),
        Step::Check(Check::Props(Prop::Area, "result", 861220.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug12213,
    "tests/bugs/modalg_1/bug12213",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("12213"),
        Step::Puts("============"),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 100.0])),
        Step::More(More::TrimCurve("cc1", "c1", [0.0, 3.14])),
        Step::Profile(Profile::Edge("e2", "cc1", None)),
        Step::Profile(Profile::Wire("w2", &["e2"])),
        Step::Prim(Prim::Box("a", &[95.0, -10.0, -5.0, 10.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("a", "w")),
        Step::Topo(Topo::Explode("a_4", "e")),
        Step::Profile(Profile::Wire("w21", &["a_4_1", "a_4_2", "a_4_3", "a_4_4"])),
        Step::Prim(Prim::Box("b", &[-110.0, -10.0, -5.0, 20.0, 10.0, 10.0])),
        Step::Topo(Topo::Explode("b", "w")),
        Step::Topo(Topo::Explode("b_4", "e")),
        Step::Profile(Profile::Wire("w22", &["b_4_1", "b_4_2", "b_4_3", "b_4_4"])),
        Step::Topo(Topo::Explode("w2", "v")),
        Step::Shell(PipeShell::Start("w2")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w21",
            vertex: Some("w2_1"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "w22",
            vertex: Some("w2_2"),
            translate: false,
            rotate: false,
            law: &[]
        })),
        Step::Shell(PipeShell::Build(SweepBuild {
            result: "result",
            mode: '-',
            solid: false
        })),
        Step::Check(Check::Props(Prop::Area, "result", 15708.8, 0.01, None)),
        Step::Check(Check::Shape("result")),
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
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug13140,
    "tests/bugs/modalg_1/bug13140",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("13140"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC13140_Spine.brep", "Spine")),
        Step::Topo(Topo::Restore("brep/OCC13140_Profile.brep", "Profile")),
        Step::Sweep(Sweep::Pipe("result", "Spine", "Profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 3.68518, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Let("explode_v_length", Value::ExplodeCount("result", "v")),
        Step::Let("explode_e_length", Value::ExplodeCount("result", "e")),
        Step::Let("explode_w_length", Value::ExplodeCount("result", "w")),
        Step::Let("explode_f_length", Value::ExplodeCount("result", "f")),
        Step::If(
            Cond::Differs("explode_v_length", 2.0),
            &[Step::Puts("vertex: Faulty")],
            &[Step::Puts("vertex: OK")]
        ),
        Step::If(
            Cond::Differs("explode_e_length", 3.0),
            &[Step::Puts("edge: Faulty")],
            &[Step::Puts("edge: OK")]
        ),
        Step::If(
            Cond::Differs("explode_w_length", 1.0),
            &[Step::Puts("wire: Faulty")],
            &[Step::Puts("wire: OK")]
        ),
        Step::If(
            Cond::Differs("explode_f_length", 1.0),
            &[Step::Puts("face: Faulty")],
            &[Step::Puts("face: OK")]
        ),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug13395,
    "tests/bugs/modalg_1/bug13395",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("13395"),
        Step::Puts("============"),
        Step::Curve(Curve::Line("line1", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])),
        Step::Profile(Profile::Edge("edge1", "line1", Some([0.0, 10.0]))),
        Step::Sweep(Sweep::Revol(
            "revol1",
            "edge1",
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 180.0]
        )),
        Step::Curve(Curve::Line("line2", &[0.0, 0.0, 0.0, 0.0, 1.0, 0.0])),
        Step::Profile(Profile::Edge("edge2", "line2", Some([0.0, 10.0]))),
        Step::Profile(Profile::Wire("wire_spine", &["edge2"])),
        Step::Sweep(Sweep::Pipe("result", "wire_spine", "revol1", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 828.319, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Let("explode_v_length", Value::ExplodeCount("result", "v")),
        Step::Let("explode_e_length", Value::ExplodeCount("result", "e")),
        Step::Let("explode_w_length", Value::ExplodeCount("result", "w")),
        Step::Let("explode_f_length", Value::ExplodeCount("result", "f")),
        Step::If(
            Cond::Differs("explode_v_length", 6.0),
            &[Step::Puts("vertex: Faulty")],
            &[Step::Puts("vertex: OK")]
        ),
        Step::If(
            Cond::Differs("explode_e_length", 9.0),
            &[Step::Puts("edge: Faulty")],
            &[Step::Puts("edge: OK")]
        ),
        Step::If(
            Cond::Differs("explode_w_length", 5.0),
            &[Step::Puts("wire: Faulty")],
            &[Step::Puts("wire: OK")]
        ),
        Step::If(
            Cond::Differs("explode_f_length", 5.0),
            &[Step::Puts("face: Faulty")],
            &[Step::Puts("face: OK")]
        ),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_1,
    "tests/bugs/modalg_1/bug1477_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 913238.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_10,
    "tests/bugs/modalg_1/bug1477_10",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 11111 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 11111 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_11,
    "tests/bugs/modalg_1/bug1477_11",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Puts("TODO 29663 ALL: Faulty shapes in variables faulty_1 to faulty_4"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1816470.0,
            "1.81647e+006",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_12,
    "tests/bugs/modalg_1/bug1477_12",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1990190.0,
            "1.99019e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_2,
    "tests/bugs/modalg_1/bug1477_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 913238.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_3,
    "tests/bugs/modalg_1/bug1477_3",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1026480.0,
            "1.02648e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_4,
    "tests/bugs/modalg_1/bug1477_4",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1117070.0,
            "1.11707e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_5,
    "tests/bugs/modalg_1/bug1477_5",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1023420.0,
            "1.02342e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_6,
    "tests/bugs/modalg_1/bug1477_6",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
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
                mode: 'R',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1108910.0,
            "1.10891e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_7,
    "tests/bugs/modalg_1/bug1477_7",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1499070.0,
            "1.49907e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_8,
    "tests/bugs/modalg_1/bug1477_8",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: true,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: '-',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1567860.0,
            "1.56786e+06",
            0.01,
            None
        )),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug1477_9,
    "tests/bugs/modalg_1/bug1477_9",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 11111 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 11111 ALL: Error : The area of result shape is"),
        Step::Puts("============"),
        Step::Puts("1477"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC1477-1.brep", "s1")),
        Step::Topo(Topo::Restore("brep/OCC1477-2.brep", "s2")),
        Step::Check(Check::Shape("s1")),
        Step::Check(Check::Shape("s2")),
        Step::Shell(PipeShell::Start("s1")),
        Step::Shell(PipeShell::Add(SweepSection {
            section: "s2",
            vertex: None,
            translate: false,
            rotate: false,
            law: &[0.0, 1.0, 1.0, 2.0, 2.0, 0.5]
        })),
        Step::If(
            Cond::Failed(&Step::Shell(PipeShell::Build(SweepBuild {
                result: "result",
                mode: 'C',
                solid: false
            }))),
            &[Step::Puts("Faulty 1477")],
            &[]
        ),
        Step::Check(Check::Props(Prop::Area, "result", 0.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug15968_1,
    "tests/bugs/modalg_1/bug15968_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("15968"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC15968.brep", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::More(More::EmptyShape("s1", "sh")),
        Step::Topo(Topo::Add("s_2", "s1")),
        Step::Topo(Topo::Add("s_1", "s1")),
        Step::Topo(Topo::Rename("s1", "result")),
        Step::Check(Check::Props(Prop::Area, "result", 164.0, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 10),
                ("EDGE", 11),
                ("WIRE", 3),
                ("FACE", 2),
                ("SHELL", 1),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 27)
            ]
        )),
        Step::CheckView("result"),
    ]
);
