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
    bugs_mesh_bug32241,
    "tests/bugs/mesh/bug32241",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("0032241: Mesh - wrong shading display of thrusections"),
        Step::Puts("========"),
        Step::Curve(Curve::Circle("c0", &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0])),
        Step::Curve(Curve::Circle("c1", &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0])),
        Step::Extra(Extra::CurveRotate(
            &["c1"],
            [2000.0, 0.0, 0.0, 0.0, 1.0, 0.0, 15.0]
        )),
        Step::Curve(Curve::Circle("c2", &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0])),
        Step::Extra(Extra::CurveRotate(
            &["c2"],
            [2000.0, 0.0, 0.0, 0.0, 1.0, 0.0, 30.0]
        )),
        Step::Curve(Curve::Circle("c3", &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0])),
        Step::Extra(Extra::CurveRotate(
            &["c3"],
            [2000.0, 0.0, 0.0, 0.0, 1.0, 0.0, 45.0]
        )),
        Step::Curve(Curve::Circle("c4", &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 30.0])),
        Step::Extra(Extra::CurveRotate(
            &["c4"],
            [2000.0, 0.0, 0.0, 0.0, 1.0, 0.0, 60.0]
        )),
        Step::Profile(Profile::Edge("e0", "c0", None)),
        Step::Profile(Profile::Edge("e1", "c1", None)),
        Step::Profile(Profile::Edge("e2", "c2", None)),
        Step::Profile(Profile::Edge("e3", "c3", None)),
        Step::Profile(Profile::Edge("e4", "c4", None)),
        Step::Profile(Profile::Wire("w0", &["e0"])),
        Step::Profile(Profile::Wire("w1", &["e1"])),
        Step::Profile(Profile::Wire("w2", &["e2"])),
        Step::Profile(Profile::Wire("w3", &["e3"])),
        Step::Profile(Profile::Wire("w4", &["e4"])),
        Step::Loft(Loft {
            result: "result",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w1", "w2", "w3", "w4"]
        }),
        Step::CheckView("result"),
        Step::Extra(Extra::MeshAsViewer("result")),
        Step::Extra(Extra::TriangleCounts("result", 2766, 1431)),
    ]
);
