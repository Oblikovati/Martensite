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
    boolean_bcut_simple_l9,
    "tests/boolean/bcut_simple/L9",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Puts("JAP60271"),
        Step::Puts("Can not create a CompSolid using exploded faces of 2 solids in contact."),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[5.0, 5.0, 5.0])),
        Step::Place(Place::Translate(&["b2"], [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("b1", "F")),
        Step::Topo(Topo::Explode("b2", "F")),
        Step::Bool(Bool::Cut("res", "b1_6", "b2_5")),
        Step::Topo(Topo::Explode("res", "F")),
        Step::More(More::Sewing(
            "se",
            1.0e-6,
            &["b1_1", "b1_2", "b1_3", "b1_4", "b1_5", "res_1", "b2_5"]
        )),
        Step::Topo(Topo::Explode("se", "Sh")),
        Step::More(More::EmptyShape("Solid1", "So")),
        Step::Topo(Topo::Add("se", "Solid1")),
        Step::More(More::EmptyShape("result", "CS")),
        Step::Topo(Topo::Add("b2", "result")),
        Step::Topo(Topo::Add("Solid1", "result")),
        Step::Check(Check::Props(Prop::Area, "result", 750.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["b1_6", "b2_5"], Prop::Area),
    ]
);

offset_case!(
    boolean_bcut_simple_m1,
    "tests/boolean/bcut_simple/M1",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Puts("JAP60271"),
        Step::Puts("Can not create a CompSolid using exploded faces of 2 solids in contact."),
        Step::Prim(Prim::Box("b1", &[10.0, 10.0, 10.0])),
        Step::Prim(Prim::Box("b2", &[5.0, 5.0, 5.0])),
        Step::Place(Place::Translate(&["b2"], [2.0, 2.0, 10.0])),
        Step::Topo(Topo::Explode("b1", "F")),
        Step::Topo(Topo::Explode("b2", "F")),
        Step::Bool(Bool::Cut("res", "b1_6", "b2_6")),
        Step::Topo(Topo::Explode("res", "F")),
        Step::More(More::Sewing(
            "se",
            1.0e-6,
            &["b1_1", "b1_2", "b1_3", "b1_4", "b1_5", "b2_6", "res_1"]
        )),
        Step::Topo(Topo::Explode("se", "Sh")),
        Step::More(More::EmptyShape("Solid1", "So")),
        Step::Topo(Topo::Add("se_1", "Solid1")),
        Step::More(More::EmptyShape("result", "CS")),
        Step::Topo(Topo::Add("b2", "result")),
        Step::Topo(Topo::Add("Solid1", "result")),
        Step::Check(Check::Props(Prop::Area, "result", 750.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["b1_6", "b2_6"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopcut_complex_a6,
    "tests/boolean/bopcut_complex/A6",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Puts("========================"),
        Step::Puts("  110 "),
        Step::Puts("Case is duplicate of chl 902 I1, bug BUC60706 "),
        Step::Puts("(cases D1,D2,..D9)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706a.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706c.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706f.brep")),
        Step::Topo(Topo::Restore("brep/buc60706d.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706f.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706i.brep")),
        Step::Topo(Topo::Restore("brep/buc60706g.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706i.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 865172.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["s", "t"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopcut_complex_a7,
    "tests/boolean/bopcut_complex/A7",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Puts("========================"),
        Step::Puts("  110 "),
        Step::Puts("Case is duplicate of chl 902 I1, bug BUC60706 "),
        Step::Puts("(cases D1,D2,..D9)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706j.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706c.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706o.brep")),
        Step::Topo(Topo::Restore("brep/buc60706m.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706o.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706r.brep")),
        Step::Topo(Topo::Restore("brep/buc60706p.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706r.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 860752.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["s", "t"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopcut_complex_a8,
    "tests/boolean/bopcut_complex/A8",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Puts("========================"),
        Step::Puts("  110 "),
        Step::Puts("Case is duplicate of chl 902 I1, bug BUC60706 "),
        Step::Puts("(cases D1,D2,..D9)"),
        Step::Puts("========================"),
        Step::Topo(Topo::Restore("brep/buc60706s.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706b.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::Restore("brep/buc60706c.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706y.brep")),
        Step::Topo(Topo::Restore("brep/buc60706v.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706e.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706y.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("res", BopKind::Cut)),
        Step::Topo(Topo::GeometryCopy("res", "buc60706z1.brep")),
        Step::Topo(Topo::Restore("brep/buc60706x.brep", "f")),
        Step::Topo(Topo::Restore("brep/buc60706h.brep", "w")),
        Step::Sweep(Sweep::Pipe("t", "w", "f", 0, false)),
        Step::Topo(Topo::GeometryCopy("buc60706z1.brep", "s")),
        Step::Bool(Bool::Prepare("s", "t")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 861220.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["s", "t"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopcut_complex_l8,
    "tests/boolean/bopcut_complex/L8",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095a.rle", "a")),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095b.rle", "b")),
        Step::More(More::EmptyShape("fso", "so")),
        Step::Topo(Topo::Add("b", "fso")),
        Step::Bool(Bool::Prepare("a", "fso")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 254359.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["a", "fso"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopcut_complex_m9,
    "tests/boolean/bopcut_complex/M9",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095a.rle", "a")),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095b.rle", "b")),
        Step::More(More::EmptyShape("fso", "so")),
        Step::Topo(Topo::Add("b", "fso")),
        Step::Bool(Bool::Prepare("a", "fso")),
        Step::Bool(Bool::TwoStep("result", BopKind::Cut)),
        Step::Check(Check::Props(Prop::Area, "result", 254359.0, 0.01, None)),
        Step::CheckViewOtherwise("result", &["a", "fso"], Prop::Area),
    ]
);

offset_case!(
    boolean_bopsection_a3,
    "tests/boolean/bopsection/A3",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095a.rle", "a")),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095b.rle", "b")),
        Step::More(More::EmptyShape("fso", "so")),
        Step::Topo(Topo::Add("b", "fso")),
        Step::Bool(Bool::Prepare("a", "fso")),
        Step::Bool(Bool::TwoStep("result", BopKind::Section)),
        Step::Check(Check::Props(Prop::Length, "result", 1244.52, 0.01, None)),
        Step::Check(Check::Section("result")),
        Step::CheckViewOtherwise("result", &["a", "fso"], Prop::Length),
    ]
);

offset_case!(
    boolean_bopsection_c6,
    "tests/boolean/bopsection/C6",
    Shape,
    300,
    End::Boolean,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 300 seconds"),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095a.rle", "a")),
        Step::Topo(Topo::Restore("brep/CTO904_pro10095b.rle", "b")),
        Step::More(More::EmptyShape("fso", "so")),
        Step::Topo(Topo::Add("b", "fso")),
        Step::Bool(Bool::Prepare("a", "fso")),
        Step::Bool(Bool::TwoStep("result", BopKind::Section)),
        Step::Check(Check::Props(Prop::Length, "result", 1244.52, 0.01, None)),
        Step::Check(Check::Section("result")),
        Step::CheckViewOtherwise("result", &["a", "fso"], Prop::Length),
    ]
);
