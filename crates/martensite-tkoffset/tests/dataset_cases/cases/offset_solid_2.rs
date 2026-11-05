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
    offset_faces_type_i_b9,
    "tests/offset/faces_type_i/B9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape is"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c1,
    "tests/offset/faces_type_i/C1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 10466.5, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c2,
    "tests/offset/faces_type_i/C2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 10466.5, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c3,
    "tests/offset/faces_type_i/C3",
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
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c4,
    "tests/offset/faces_type_i/C4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape is"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c5,
    "tests/offset/faces_type_i/C5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Prim(Prim::Sphere("s", &[15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3534.29, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c6,
    "tests/offset/faces_type_i/C6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 968.134, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c7,
    "tests/offset/faces_type_i/C7",
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
        Step::Prim(Prim::Sphere("s", &[15.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c8,
    "tests/offset/faces_type_i/C8",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 25406 ALL: Error : The volume of result shape is"),
        Step::Puts("TODO 25406 ALL: Error: bsection of the result and s is not equal to zero"),
        Step::Puts("TODO 25406 ALL: Faulty shapes"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_c9,
    "tests/offset/faces_type_i/C9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
    ]
);

offset_case!(
    offset_faces_type_i_d1,
    "tests/offset/faces_type_i/D1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 10602.9, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d2,
    "tests/offset/faces_type_i/D2",
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
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d3,
    "tests/offset/faces_type_i/D3",
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
        Step::Prim(Prim::Sphere("s", &[15.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d4,
    "tests/offset/faces_type_i/D4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: ERROR. offsetperform operation not done."),
        Step::Puts("TODO 23068 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23068 ALL: Error: The command cannot be built"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d5,
    "tests/offset/faces_type_i/D5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2055.47, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d6,
    "tests/offset/faces_type_i/D6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[10.0, 10.0, 0.0, 45.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 954.134, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d7,
    "tests/offset/faces_type_i/D7",
    Shape,
    900,
    End::OffsetShape(Some(true)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: An exception was caught"),
        Step::Puts("TODO 24156 MacOS: TEST INCOMPLETE"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 360.0]
        )),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &[])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13139.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d8,
    "tests/offset/faces_type_i/D8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1198.8, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_d9,
    "tests/offset/faces_type_i/D9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 938.552, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e1,
    "tests/offset/faces_type_i/E1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1085.66, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e2,
    "tests/offset/faces_type_i/E2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 888.264, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e3,
    "tests/offset/faces_type_i/E3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3370.13, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e4,
    "tests/offset/faces_type_i/E4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2715.08, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e5,
    "tests/offset/faces_type_i/E5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3256.97, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e6,
    "tests/offset/faces_type_i/E6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Torus("s", &[20.0, 5.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 2664.79, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e7,
    "tests/offset/faces_type_i/E7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4385.14, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e8,
    "tests/offset/faces_type_i/E8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4385.14, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_e9,
    "tests/offset/faces_type_i/E9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4385.14, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f1,
    "tests/offset/faces_type_i/F1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol("s", "w", [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4", "s_5"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4385.14, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_f2,
    "tests/offset/faces_type_i/F2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Curve(Curve::Ellipse("w1", &[0.0, 0.0, 0.0, 15.0, 10.0])),
        Step::Profile(Profile::Edge("w1", "w1", Some([0.0, 1.5707963267948966]))),
        Step::Place(Place::Rotate(&["w1"], [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 90.0])),
        Step::Profile(Profile::Polyline(
            "w2",
            &[
                0.0, 0.0, 10.0, 0.0, 0.0, -10.0, 25.0, 0.0, -10.0, 15.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Wire("w", &["w1", "w2"])),
        Step::Profile(Profile::Plane("w", "w", false)),
        Step::Sweep(Sweep::Revol(
            "s",
            "w",
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 270.0]
        )),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 13155.2, 0.01, None)),
    ]
);
