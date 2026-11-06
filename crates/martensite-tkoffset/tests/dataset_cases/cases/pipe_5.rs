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
    pipe_standard_c4,
    "tests/pipe/standard/C4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[0.0, -52.3, 30.3, 0.0, 1.0, -1.0, 15.0, 15.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Parabola("w", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 12028.2, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c5,
    "tests/pipe/standard/C5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[0.0, -110.0, -90.0, 0.0, 1.0, 1.0, 15.0, 15.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1000.0, 1000.0, 1000.0,
                1.0, 1000.0, 0.0, 0.0, 1.0, -100.0, -100.0, -100.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 156102.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c6,
    "tests/pipe/standard/C6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Hyperbola(
            "profile",
            &[0.0, 130.0, 20.0, 0.0, 1.0, 1.0, 15.0, 15.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 150.0, 0.0, 0.0, 2000.0, 50.0, 0.0, 1700.0, 700.0, 0.0, 400.0, 300.0,
                0.0, 0.0, 0.0, 0.0, 900.0, 400.0, 0.0, 140.0, 100.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 187196.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c7,
    "tests/pipe/standard/C7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 10.0, 10.0, 0.0]
        )),
        Step::Curve(Curve::Circle("w", &[0.0, 20.0, 0.0, 1.0, 0.0, 0.0, 20.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 1332.86, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c8,
    "tests/pipe/standard/C8",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 3.0, 3.0, 0.0]
        )),
        Step::Curve(Curve::Ellipse(
            "w",
            &[10.0, 10.0, 0.0, 1.0, 0.0, 0.0, 30.0, 10.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 527.039, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_c9,
    "tests/pipe/standard/C9",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, -115.064945, 75.458923, 20.0, -115.064945, 75.458923]
        )),
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
        Step::Check(Check::Props(Prop::Area, "result", 5608.5, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d1,
    "tests/pipe/standard/D1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 20.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Polyline(
            "w",
            &[10.0, 0.0, 0.0, 0.0, 30.0, 30.0, 0.0, 70.0, 70.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 1979.9, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d2,
    "tests/pipe/standard/D2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, -31.415926, 4.934802, 20.0, -31.415926, 4.934802]
        )),
        Step::Curve(Curve::Parabola("w", &[10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 2670.02, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d3,
    "tests/pipe/standard/D3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 0.0, 0.0, 0.0, -20.0, 20.0]
        )),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 100.0, 100.0, 100.0,
                1.0, 100.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 8049.06, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d4,
    "tests/pipe/standard/D4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Profile(Profile::Polyline(
            "profile",
            &[0.0, 150.0, 0.0, 10.0, 150.0, 0.0]
        )),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 150.0, 0.0, 0.0, 200.0, 50.0, 0.0, 170.0, 70.0, 0.0, 40.0, 30.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 10.0, 0.0, 150.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 2669.45, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d5,
    "tests/pipe/standard/D5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Circle("w", &[0.0, 100.0, 0.0, 1.0, 0.0, 0.0, 100.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 77372.9, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d6,
    "tests/pipe/standard/D6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Ellipse(
            "w",
            &[0.0, 80.0, 0.0, 1.0, 0.0, 0.0, 200.0, 80.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 115411.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d7,
    "tests/pipe/standard/D7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[10.0, -115.064945, 75.458923, 0.0, 1.0, -1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
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
        Step::Check(Check::Props(Prop::Area, "result", 38722.8, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d8,
    "tests/pipe/standard/D8",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[10.0, 0.0, 0.0, 0.0, 1.0, -1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Polyline(
            "w",
            &[10.0, 0.0, 0.0, 10.0, -30.0, 30.0, 10.0, -70.0, 90.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 15270.1, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_d9,
    "tests/pipe/standard/D9",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[0.0, -62.83185, 19.739209, 0.0, 1.0, -1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Parabola("w", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 18784.6, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e1,
    "tests/pipe/standard/E1",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[-100.0, -110.0, -100.0, 1.0, 1.0, 0.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1000.0, 1000.0, 1000.0,
                1.0, 1000.0, 0.0, 0.0, 1.0, -100.0, -100.0, -100.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 391718.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e2,
    "tests/pipe/standard/E2",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::Parabola(
            "profile",
            &[0.0, 140.0, 100.0, 0.0, 1.0, 1.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "profile",
            "profile",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Curve(Curve::Bezier(
            "w",
            &[
                7.0, 0.0, 150.0, 0.0, 0.0, 2000.0, 50.0, 0.0, 1700.0, 700.0, 0.0, 400.0, 300.0,
                0.0, 0.0, 0.0, 0.0, 900.0, 400.0, 0.0, 140.0, 100.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 318347.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e3,
    "tests/pipe/standard/E3",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, -1.0, -1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, -1.0, -1.0, 0.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Circle("w", &[0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 10.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 329.199, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e4,
    "tests/pipe/standard/E4",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, -1.0, -1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 0.0, 1.0, -1.0, -1.0, 0.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Ellipse(
            "w",
            &[0.0, 8.0, 0.0, 1.0, 0.0, 0.0, 20.0, 8.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 482.227, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e5,
    "tests/pipe/standard/E5",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0,
                3.0,
                0.0,
                3.0,
                1.0,
                1.0,
                2.0,
                3.0,
                -25.0,
                -127.064945,
                75.0,
                5.0,
                25.0,
                -87.064945,
                75.0,
                5.0,
                25.0,
                -127.064945,
                77.0,
                5.0,
                -25.0,
                -127.064945,
                75.0,
                5.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Hyperbola(
            "w",
            &[0.0, 0.0, -50.0, 1.0, 0.0, 0.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 32658.6, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e6,
    "tests/pipe/standard/E6",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 10.0, 0.0, 5.0, -10.0, -10.0, 0.0,
                5.0, 10.0, -10.0, 5.0, 5.0, 0.0, 10.0, 0.0, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Profile(Profile::Polyline(
            "w",
            &[0.0, 0.0, 0.0, 10.0, -30.0, 30.0, 0.0, -70.0, 90.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 4352.33, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e7,
    "tests/pipe/standard/E7",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, -45.0, 20.74, 5.0, -10.0, -75.0,
                18.74, 5.0, 10.0, -75.0, 18.74, 5.0, 0.0, -45.0, 20.74, 5.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Parabola("w", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 6104.74, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e8,
    "tests/pipe/standard/E8",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 10.0, -7.0, 5.0, -10.0, -10.0, 8.0,
                5.0, 10.0, -10.0, 8.0, 5.0, 0.0, 10.0, -7.0, 5.0
            ]
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
        Step::Check(Check::Props(Prop::Area, "result", 16793.8, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_e9,
    "tests/pipe/standard/E9",
    Shape,
    600,
    End::ResultView("Error : The pipe cannot be built.", true),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 600 seconds"),
        Step::Curve(Curve::BSpline(
            "profile",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 30.0, -21.0, 5.0, -30.0, -30.0, 24.0,
                5.0, 30.0, -30.0, 24.0, 5.0, 0.0, 30.0, -21.0, 5.0
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
        Step::Check(Check::Props(Prop::Area, "result", 304960.0, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f1,
    "tests/pipe/standard/F1",
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
                6.0, 0.0, 10.0, 0.0, -5.0, 0.0, 0.0, -5.0, -20.0, 0.0, 5.0, -20.0, 0.0, 5.0, 0.0,
                0.0, 0.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Circle("w", &[0.0, 100.0, 0.0, 1.0, 0.0, 0.0, 100.0])),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 29584.3, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f2,
    "tests/pipe/standard/F2",
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
                6.0, 0.0, 10.0, 0.0, -5.0, 0.0, 0.0, -5.0, -20.0, 0.0, 5.0, -20.0, 0.0, 5.0, 0.0,
                0.0, 0.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Ellipse(
            "w",
            &[0.0, 80.0, 0.0, 1.0, 0.0, 0.0, 200.0, 80.0]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 43171.5, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f3,
    "tests/pipe/standard/F3",
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
                6.0, 0.0, -105.0, 75.5, -5.0, -115.0, 75.5, -5.0, -135.0, 75.5, 5.0, -135.0, 75.5,
                5.0, -115.0, 75.5, 0.0, -105.0, 75.5
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Hyperbola(
            "w",
            &[0.0, 0.0, -50.0, 1.0, 0.0, 0.0, 50.0, 50.0]
        )),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-1.5707963267948966, 1.5707963267948966])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 9533.54, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f4,
    "tests/pipe/standard/F4",
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
                6.0, 0.0, 10.0, 0.0, -5.0, 0.0, 0.0, -5.0, -20.0, 0.0, 5.0, -20.0, 0.0, 5.0, 0.0,
                0.0, 0.0, 10.0, 0.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Profile(Profile::Polyline(
            "w",
            &[0.0, 0.0, 0.0, 10.0, -30.0, 30.0, 0.0, -70.0, 90.0]
        )),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 4702.76, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f5,
    "tests/pipe/standard/F5",
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
                6.0, 0.0, -53.0, 19.8, -5.0, -63.0, 19.8, -5.0, -83.0, 19.8, 5.0, -83.0, 19.8, 5.0,
                -63.0, 19.8, 0.0, -53.0, 19.8
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::Parabola("w", &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 50.0])),
        Step::Profile(Profile::Edge(
            "w",
            "w",
            Some([-62.83185307179586, 62.83185307179586])
        )),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 3895.63, 0.01, None)),
    ]
);

offset_case!(
    pipe_standard_f6,
    "tests/pipe/standard/F6",
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
                6.0, 0.0, 0.0, 40.0, 0.0, -20.0, 0.0, 0.0, -20.0, -80.0, 0.0, 20.0, -80.0, 0.0,
                20.0, 0.0, 0.0, 0.0, 40.0
            ]
        )),
        Step::Profile(Profile::Edge("profile", "profile", None)),
        Step::Profile(Profile::Wire("profile", &["profile"])),
        Step::Profile(Profile::Plane("profile", "profile", false)),
        Step::Curve(Curve::BSpline(
            "w",
            &[
                2.0, 3.0, 0.0, 3.0, 1.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0, 1000.0, 1000.0, 1000.0,
                1.0, 1000.0, 0.0, 0.0, 1.0, -100.0, -100.0, -100.0, 2.0
            ]
        )),
        Step::Profile(Profile::Edge("w", "w", None)),
        Step::Profile(Profile::Wire("w", &["w"])),
        Step::Sweep(Sweep::Pipe("result", "w", "profile", 0, false)),
        Step::Check(Check::Props(Prop::Area, "result", 450587.0, 0.01, None)),
    ]
);
