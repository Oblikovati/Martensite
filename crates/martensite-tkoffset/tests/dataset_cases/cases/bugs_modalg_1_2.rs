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
    bugs_modalg_1_bug15968_2,
    "tests/bugs/modalg_1/bug15968_2",
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
        Step::More(More::EmptyShape("s2", "sh")),
        Step::Topo(Topo::Add("s_1", "s2")),
        Step::Topo(Topo::Add("s_2", "s2")),
        Step::Topo(Topo::Rename("s2", "result")),
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

offset_case!(
    bugs_modalg_1_bug16667_1,
    "tests/bugs/modalg_1/bug16667_1",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("16667"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC16667-face.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: 20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 1542.3, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 7),
                ("EDGE", 7),
                ("WIRE", 1),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 15)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug16667_2,
    "tests/bugs/modalg_1/bug16667_2",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("16667"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC16667-face.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: -20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 1188.62, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 5),
                ("EDGE", 5),
                ("WIRE", 1),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 11)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug16667_3,
    "tests/bugs/modalg_1/bug16667_3",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("16667"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC16667-wire.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: 20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 1451.49, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 8),
                ("WIRE", 1),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
        Step::CheckView("result"),
    ]
);

offset_case!(
    bugs_modalg_1_bug16667_4,
    "tests/bugs/modalg_1/bug16667_4",
    Bugs,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 12345 ALL: Error :  is WRONG because number of "),
        Step::Puts("============"),
        Step::Puts("16667"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/OCC16667-wire.brep", "a")),
        Step::Offset(Offset::Planar(PlanarOffset {
            result: "resoffset",
            base: "a",
            count: 1,
            step: -20.0,
            approx: false,
            join: 'a',
            altitude: 0.0,
            open: false
        })),
        Step::Topo(Topo::Rename("resoffset_1", "result")),
        Step::Check(Check::Props(Prop::Length, "result", 1200.16, 0.01, None)),
        Step::Check(Check::Shape("result")),
        Step::Check(Check::Section("result")),
        Step::Check(Check::Counts(
            "result",
            &[
                ("VERTEX", 8),
                ("EDGE", 8),
                ("WIRE", 1),
                ("FACE", 0),
                ("SHELL", 0),
                ("SOLID", 0),
                ("COMPSOLID", 0),
                ("COMPOUND", 0),
                ("SHAPE", 17)
            ]
        )),
        Step::CheckView("result"),
    ]
);
