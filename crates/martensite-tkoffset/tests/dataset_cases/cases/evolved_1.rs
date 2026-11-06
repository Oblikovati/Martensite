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
    evolved_evolved_bug26470_1,
    "tests/evolved/evolved/bug26470_1",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof1.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: false,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 558.33, 0.01, None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "a", "b"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_evolved_bug26470_2,
    "tests/evolved/evolved/bug26470_2",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof2.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: false,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 572.14, 0.01, None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "a", "b"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt001,
    "tests/evolved/voluved/AGT001",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &[
                "F", "-1.0", "10.0", "TT", "-1.0", "-1.0", "C", "1.0", "90.0", "TT", "10.0",
                "-1.0", "W"
            ]
        )),
        Step::Profile(Profile::Polyline("t", &[-1.0, -1.0, 0.0, 0.0, -1.5, 5.0])),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 268.774, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 245.265, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("1.5e-7"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt002,
    "tests/evolved/voluved/AGT002",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 20.0, 10.0, 0.0, 20.0, 0.0, 0.0, 13.0, 0.0, 0.0,
                13.0, 5.0, 0.0, 6.0, 5.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "t",
            &[
                -2.0, 2.0, 0.0, 0.0, 2.0, 5.0, 0.0, 2.0, 10.0, 1.0, 2.0, 10.0, 1.0, 2.0, 0.0, -2.0,
                2.0, 0.0
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 1649.58, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 1023.79, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("1.5e-7"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt003,
    "tests/evolved/voluved/AGT003",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 20.0, 10.0, 0.0, 20.0, 0.0, 0.0, 13.0, 0.0, 0.0,
                13.0, 5.0, 0.0, 6.0, 5.0, 0.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "t",
            &[
                -1.0, 2.0, 0.0, 0.0, 1.0, 5.0, 0.0, 1.0, 10.0, 0.5, 0.5, 10.0, 0.5, 0.5, 0.0
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 1573.58, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 532.942, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("5.0e-6"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt004,
    "tests/evolved/voluved/AGT004",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &["X", "10.0", "Y", "2.0", "X", "-10.0", "W"]
        )),
        Step::Profile(Profile::Polyline("t", &[-1.0, 1.0, 0.0, 1.0, 1.0, 10.0])),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 287.517, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 211.903, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Check(Check::MaxTolerance("result", Some("1.5e-007"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt005,
    "tests/evolved/voluved/AGT005",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &["X", "10.0", "Y", "2.0", "X", "-10.0", "W"]
        )),
        Step::Profile(Profile::Polyline("t", &[-1.0, 1.0, 0.0, 1.2, 1.0, 10.0])),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 266.546, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 192.639, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("1.5e-7"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt006,
    "tests/evolved/voluved/AGT006",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &["X", "10.0", "Y", "2.0", "X", "-10.0", "W"]
        )),
        Step::More(More::Moves(
            "t",
            &[
                "O", "0.0", "1.0", "0.0", "P", "0.0", "1.0", "0.0", "0.0", "0.0", "1.0", "X",
                "5.0", "T", "-1.0", "2.0", "WW"
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 162.361, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 95.3333, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance(
            "result",
            Some("1.5000000368888203e-007"),
            None
        )),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "p", "t"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt007,
    "tests/evolved/voluved/AGT007",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &[
                "X", "15.0", "C", "1.0", "90.0", "Y", "5.0", "X", "-7.0", "Y", "-2.0", "C", "-1.0",
                "90.0", "X", "-8.0", "W"
            ]
        )),
        Step::More(More::Moves(
            "t",
            &[
                "O", "0.0", "1.0", "0.0", "P", "0.0", "1.0", "0.0", "0.0", "0.0", "1.0", "X",
                "5.0", "T", "-1.0", "2.0", "WW"
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 361.087, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 312.848, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("1.5e-7"), None)),
        Step::Run(crate::one_off::agt007_tolerance),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt008,
    "tests/evolved/voluved/AGT008",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &["X", "10.0", "Y", "2.0", "X", "-10.0", "W"]
        )),
        Step::More(More::Moves(
            "t",
            &[
                "O", "0.0", "1.0", "0.0", "P", "0.0", "1.0", "0.0", "0.0", "0.0", "1.0", "X",
                "2.0", "RR", "-45.0", "C", "1.0", "90.0", "T", "-1.0", "2.0", "WW"
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 129.967, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 70.6801, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 1), ("SHELL", 1)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("3.25e-007"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_agt009,
    "tests/evolved/voluved/AGT009",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("=========="),
        Step::Puts("29523"),
        Step::Puts("=========="),
        Step::More(More::Moves(
            "p",
            &["X", "10.0", "Y", "2.0", "X", "-10.0", "W"]
        )),
        Step::More(More::Moves(
            "t",
            &[
                "O", "0.0", "1.0", "0.0", "P", "0.0", "1.0", "0.0", "0.0", "0.0", "1.0", "RR",
                "30.0", "C", "-10.0", "60.0", "WW"
            ]
        )),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "p",
            profile: "t",
            solid: true,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Block(&[
            Step::Check(Check::Props(Prop::Area, "result", 142.657, 0.01, None)),
            Step::Check(Check::Props(Prop::Volume, "result", 39.1789, 0.01, None))
        ]),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Counts("result", &[("SOLID", 2), ("SHELL", 2)])),
        Step::Let("answer_6_bopargcheck", Value::ArgumentCheck("result")),
        Step::If(
            Cond::Holds("answer_6_bopargcheck", "Faulties"),
            &[Step::Puts(
                "Error: bopargcheck has found some faulties in res2"
            )],
            &[]
        ),
        Step::Check(Check::MaxTolerance("result", Some("1.0e-7"), None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "sw", "tw"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_bug26470_1,
    "tests/evolved/voluved/bug26470_1",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof1.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 558.33, 0.01, None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "a", "b"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    evolved_voluved_bug26470_2,
    "tests/evolved/voluved/bug26470_2",
    Evolved,
    600,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Puts("========"),
        Step::Puts("26470"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/OCC26470-ClosedWire.brep", "a")),
        Step::Topo(Topo::Restore("brep/OCC26470-wprof2.brep", "b")),
        Step::More(More::Evolved(Evolved {
            result: "result",
            spine: "a",
            profile: "b",
            solid: false,
            compute_axes: true,
            volume: true,
            tolerance: 0.0,
            parallel: true
        })),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Props(Prop::Area, "result", 572.14, 0.01, None)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Donly(&["result", "a", "b"])),
        Step::View(ViewStep::Fit),
        Step::Screenshot("photo"),
    ]
);
