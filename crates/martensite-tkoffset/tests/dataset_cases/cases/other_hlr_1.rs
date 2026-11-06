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
    hlr_exact_hlr_plate,
    "tests/hlr/exact_hlr/Plate",
    Shape,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Profile(Profile::Polyline(
            "f1",
            &[
                0.0, 0.0, 0.0, 0.0, -10.0, 0.0, 100.0, -10.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f2",
            &[
                100.0, 0.0, 0.0, 110.0, 0.0, 0.0, 110.0, 100.0, 0.0, 100.0, 100.0, 0.0, 100.0, 0.0,
                0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f3",
            &[
                100.0, 100.0, 0.0, 100.0, 110.0, 0.0, 0.0, 110.0, 0.0, 0.0, 100.0, 0.0, 100.0,
                100.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f4",
            &[
                0.0, 100.0, 0.0, -10.0, 100.0, 0.0, -10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f1", "f1", false)),
        Step::Profile(Profile::Plane("f2", "f2", false)),
        Step::Profile(Profile::Plane("f3", "f3", false)),
        Step::Profile(Profile::Plane("f4", "f4", false)),
        Step::Topo(Topo::Explode("f1", "e")),
        Step::Topo(Topo::Explode("f2", "e")),
        Step::Topo(Topo::Explode("f3", "e")),
        Step::Topo(Topo::Explode("f4", "e")),
        Step::Extra(Extra::Point("pp", [50.0, 50.0, 100.0])),
        Step::Extra(Extra::Filling(Filling {
            result: "a",
            bounds: &[
                ("f1_4", "f1", 1),
                ("f2_4", "f2", 1),
                ("f3_4", "f3", 1),
                ("f4_4", "f4", 1)
            ],
            points: &["pp"]
        })),
        Step::Extra(Extra::Hlr(false)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Check(Check::Props(Prop::Length, "result", 404.283, 0.01, None)),
        Step::Screenshot("photo"),
    ]
);

offset_case!(
    hlr_poly_hlr_plate,
    "tests/hlr/poly_hlr/Plate",
    Shape,
    1000,
    End::Completed,
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts(
            "TODO 30286 Windows: Error : The length of result shape is 404.492, expected 404.386"
        ),
        Step::Puts(
            "TODO CR33225 Linux: Error : The length of result shape is 404.492, expected 404.386"
        ),
        Step::Profile(Profile::Polyline(
            "f1",
            &[
                0.0, 0.0, 0.0, 0.0, -10.0, 0.0, 100.0, -10.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f2",
            &[
                100.0, 0.0, 0.0, 110.0, 0.0, 0.0, 110.0, 100.0, 0.0, 100.0, 100.0, 0.0, 100.0, 0.0,
                0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f3",
            &[
                100.0, 100.0, 0.0, 100.0, 110.0, 0.0, 0.0, 110.0, 0.0, 0.0, 100.0, 0.0, 100.0,
                100.0, 0.0
            ]
        )),
        Step::Profile(Profile::Polyline(
            "f4",
            &[
                0.0, 100.0, 0.0, -10.0, 100.0, 0.0, -10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 100.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f1", "f1", false)),
        Step::Profile(Profile::Plane("f2", "f2", false)),
        Step::Profile(Profile::Plane("f3", "f3", false)),
        Step::Profile(Profile::Plane("f4", "f4", false)),
        Step::Topo(Topo::Explode("f1", "e")),
        Step::Topo(Topo::Explode("f2", "e")),
        Step::Topo(Topo::Explode("f3", "e")),
        Step::Topo(Topo::Explode("f4", "e")),
        Step::Extra(Extra::Point("pp", [50.0, 50.0, 100.0])),
        Step::Extra(Extra::Filling(Filling {
            result: "a",
            bounds: &[
                ("f1_4", "f1", 1),
                ("f2_4", "f2", 1),
                ("f3_4", "f3", 1),
                ("f4_4", "f4", 1)
            ],
            points: &["pp"]
        })),
        Step::Extra(Extra::Hlr(true)),
        Step::View(ViewStep::SmallView),
        Step::View(ViewStep::Fit),
        Step::Check(Check::Props(Prop::Length, "result", 404.386, 1e-07, None)),
        Step::Screenshot("photo"),
    ]
);
