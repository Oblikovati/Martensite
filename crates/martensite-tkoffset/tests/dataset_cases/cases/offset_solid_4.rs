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
    offset_faces_type_i_i6,
    "tests/offset/faces_type_i/I6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 287.456, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i7,
    "tests/offset/faces_type_i/I7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 10.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 301.216, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i8,
    "tests/offset/faces_type_i/I8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 403.355, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_i9,
    "tests/offset/faces_type_i/I9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 273.873, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j1,
    "tests/offset/faces_type_i/J1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 275.348, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j2,
    "tests/offset/faces_type_i/J2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 203.5, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j3,
    "tests/offset/faces_type_i/J3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 415.433, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j4,
    "tests/offset/faces_type_i/J4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 295.724, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j5,
    "tests/offset/faces_type_i/J5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1566.87, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j6,
    "tests/offset/faces_type_i/J6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 610.501, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j7,
    "tests/offset/faces_type_i/J7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: ERROR. offsetperform operation not done."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 26556 ALL: Error : The offset cannot be built."),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j8,
    "tests/offset/faces_type_i/J8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[9.0, 4.0, 15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 622.419, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_j9,
    "tests/offset/faces_type_i/J9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1019.69, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k1,
    "tests/offset/faces_type_i/K1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 810.256, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k2,
    "tests/offset/faces_type_i/K2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 838.074, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k3,
    "tests/offset/faces_type_i/K3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 718.42, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k4,
    "tests/offset/faces_type_i/K4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 530.971, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k5,
    "tests/offset/faces_type_i/K5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 477.688, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k6,
    "tests/offset/faces_type_i/K6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 6083.13, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k7,
    "tests/offset/faces_type_i/K7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2155.26, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k8,
    "tests/offset/faces_type_i/K8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 993.896, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_k9,
    "tests/offset/faces_type_i/K9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 953.616, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l1,
    "tests/offset/faces_type_i/L1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 412.385, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l2,
    "tests/offset/faces_type_i/L2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 290.969, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l3,
    "tests/offset/faces_type_i/L3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 340.505, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l4,
    "tests/offset/faces_type_i/L4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 246.795, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l5,
    "tests/offset/faces_type_i/L5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 352.331, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l6,
    "tests/offset/faces_type_i/L6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::BSpline(
            "w1",
            &[
                3.0, 2.0, -1.0, 4.0, 1.0, 4.0, 0.0, 0.0, 0.0, 1.0, 2.0, 10.0, 0.0, 1.0, 4.0, 6.0,
                0.0, 1.0, 10.0, 0.0, 0.0, 1.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 0.0, 0.0, 3.0, -5.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [0.0, 0.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 272.867, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l7,
    "tests/offset/faces_type_i/L7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Bezier(
            "w1",
            &[
                5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 5.0, 0.0, 25.0, 10.0, 0.0, 10.0, 20.0,
                0.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 20.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [-2.0, 7.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1327.05, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_l8,
    "tests/offset/faces_type_i/L8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Bezier(
            "w1",
            &[
                5.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 20.0, 5.0, 0.0, 25.0, 10.0, 0.0, 10.0, 20.0,
                0.0
            ]
        )),
        Step::Profile(Profile::Edge("w1", "w1", None)),
        Step::Profile(Profile::Polyline(
            "w2",
            &[10.0, 20.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Prism("s", "w", [-2.0, 7.0, 10.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1040.6, 0.01, None)),
    ]
);
