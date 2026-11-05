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
    blend_bfuseblend_a8,
    "tests/blend/bfuseblend/A8",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[-20.0, -20.0, -20.0, 40.0, 40.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge(
            "w1",
            "w1",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Wire("w", &["w1"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Bool(Bool::FuseBlend("result", "s", "b", 10.0)),
        Step::Check(Check::Props(Prop::Area, "result", 677145.0, 0.01, None)),
    ]
);

offset_case!(
    blend_bfuseblend_a9,
    "tests/blend/bfuseblend/A9",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[-30.0, -30.0, -30.0, 60.0, 60.0, 20.0])),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                15.0, 0.0, 0.0, 27.0, 0.0, -12.0, 0.0, 0.0, -12.0, 0.0, 0.0, 10.0
            ]
        )),
        Step::Topo(Topo::Orient("w2", 'R')),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Sweep(Sweep::Revol(
            "r",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::More(More::EmptyShape("s", "So")),
        Step::Topo(Topo::Add("r", "s")),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -40.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, -10.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Bool(Bool::FuseBlend("result", "s", "b", 10.0)),
        Step::Check(Check::PropsAs(
            Prop::Area,
            "result",
            1292620.0,
            "1.29262e+06",
            0.01,
            None
        )),
    ]
);

offset_case!(
    blend_bfuseblend_b1,
    "tests/blend/bfuseblend/B1",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[-15.0, -15.0, -15.0, 15.0, 30.0, 30.0])),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 10.0, 10.0, 0.0])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, -20.0]
        )),
        Step::Place(Place::Rotate(
            &["s", "b"],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 20.0]
        )),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Bool(Bool::FuseBlend("result", "s", "b", 10.0)),
        Step::Check(Check::Props(Prop::Area, "result", 395701.0, 0.01, None)),
    ]
);

offset_case!(
    blend_bfuseblend_b2,
    "tests/blend/bfuseblend/B2",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Prim(Prim::Box("b", &[-15.0, -15.0, -15.0, 15.0, 30.0, 30.0])),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                4.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 20.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 10.0])),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Bool(Bool::FuseBlend("result", "s", "b", 10.0)),
        Step::Check(Check::Props(Prop::Area, "result", 431283.0, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_d9,
    "tests/blend/buildevol/D9",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_4",
            &[[0.0, 10.0], [1.0, 20.0], [2.0, 10.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 87027.5, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_e1,
    "tests/blend/buildevol/E1",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_6",
            &[[0.0, 10.0], [1.0, 20.0], [2.0, 10.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 99053.9, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_e2,
    "tests/blend/buildevol/E2",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_6",
            &[[0.0, 10.0], [1.0, 20.0], [2.0, 10.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 81492.8, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_e3,
    "tests/blend/buildevol/E3",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_5",
            &[[0.0, 10.0], [1.0, 20.0], [2.0, 10.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 81299.7, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_f4,
    "tests/blend/buildevol/F4",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 10.0, 10.0, 0.0])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_1",
            &[[0.0, 20.0], [1.0, 10.0], [2.0, 20.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 50306.6, 0.01, None)),
    ]
);

offset_case!(
    blend_buildevol_f5,
    "tests/blend/buildevol/F5",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                4.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 20.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::MakeEvolved("result", "s")),
        Step::Fillet(Fillet::UpdateEvolved(
            "s_1",
            &[[0.0, 10.0], [1.0, 20.0], [2.0, 10.0]]
        )),
        Step::Fillet(Fillet::BuildEvolved),
        Step::Check(Check::Props(Prop::Area, "result", 86185.5, 0.01, None)),
    ]
);

offset_case!(
    blend_complex_c5,
    "tests/blend/complex/C5",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Topo(Topo::Restore("brep/CFI_pro6309a.rle", "a")),
        Step::Place(Place::Scale(&["a"], [0.0, 0.0, 0.0, 1000.0])),
        Step::Topo(Topo::Restore("brep/pro6309b.rle", "b")),
        Step::Place(Place::Scale(&["b"], [0.0, 0.0, 0.0, 1000.0])),
        Step::Topo(Topo::Invert("b")),
        Step::More(More::EmptyShape("bb", "So")),
        Step::Topo(Topo::Add("b", "bb")),
        Step::Bool(Bool::Cut("cab", "a", "bb")),
        Step::Topo(Topo::NumberedExplode("cab", "e")),
        Step::Fillet(Fillet::Blend(
            "result",
            "cab",
            &[
                (10.0, "cab_9"),
                (10.0, "cab_10"),
                (10.0, "cab_2"),
                (10.0, "cab_11")
            ]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 180881.0, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i1,
    "tests/blend/simple/I1",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_4")])),
        Step::Check(Check::Props(Prop::Area, "result", 91635.9, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i2,
    "tests/blend/simple/I2",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend(
            "result",
            "s",
            &[(10.0, "s_4"), (10.0, "s_2"), (10.0, "s_1")]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 90904.5, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i3,
    "tests/blend/simple/I3",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_6")])),
        Step::Check(Check::Props(Prop::Area, "result", 99301.8, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i4,
    "tests/blend/simple/I4",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Circle("w", &[-20.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", Some([0.0, 1.5707963267948966]))),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 5.0, 0.0, 5.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend(
            "result",
            "s",
            &[(10.0, "s_6"), (10.0, "s_9"), (10.0, "s_4")]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 90939.5, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i5,
    "tests/blend/simple/I5",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_6")])),
        Step::Check(Check::Props(Prop::Area, "result", 81863.7, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i6,
    "tests/blend/simple/I6",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend(
            "result",
            "s",
            &[(10.0, "s_5"), (10.0, "s_10"), (10.0, "s_6")]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 80209.3, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i7,
    "tests/blend/simple/I7",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_5")])),
        Step::Check(Check::Props(Prop::Area, "result", 81743.8, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_i8,
    "tests/blend/simple/I8",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[3.0, 0.0, 0.0, 0.0, 5.0, 10.0, 0.0, 10.0, 15.0, 0.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Profile(Profile::Polyline(
            "profile",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend(
            "result",
            "s",
            &[(10.0, "s_5"), (10.0, "s_8"), (10.0, "s_7")]
        )),
        Step::Check(Check::Props(Prop::Area, "result", 80586.1, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_j8,
    "tests/blend/simple/J8",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline("w", &[0.0, 0.0, 0.0, 10.0, 10.0, 0.0])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_1")])),
        Step::Check(Check::Props(Prop::Area, "result", 51928.1, 0.01, None)),
    ]
);

offset_case!(
    blend_simple_j9,
    "tests/blend/simple/J9",
    Blend,
    600,
    End::ResultView("Error : The blend cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                4.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 10.0, 0.0, 20.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Curve(Curve::Circle(
            "profile",
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 5.0]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Sweep(Sweep::Pipe("s", "w", "profile", 0, false)),
        Step::Place(Place::Scale(&["s"], [0.0, 0.0, 0.0, 10.0])),
        Step::Topo(Topo::ForceSameParameter("s", Some(1e-06))),
        Step::Topo(Topo::Explode("s", "E")),
        Step::Fillet(Fillet::Blend("result", "s", &[(10.0, "s_1")])),
        Step::Check(Check::Props(Prop::Area, "result", 86937.1, 0.01, None)),
    ]
);
