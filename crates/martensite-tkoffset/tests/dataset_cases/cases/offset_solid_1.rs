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
    offset_bugs_bug26030,
    "tests/offset/bugs/bug26030",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 26030 ALL: Error : The offset cannot be built"),
        Step::Puts("========"),
        Step::Puts("26030"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug26030_thickness-crash1-tcl-Loft001-00-section.brep",
            "Loft001-00-section"
        )),
        Step::Topo(Topo::Restore(
            "brep/bug26030_thickness-crash1-tcl-Loft001-01-section.brep",
            "Loft001-01-section"
        )),
        Step::Loft(Loft {
            result: "Loft001",
            solid: true,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["Loft001-00-section", "Loft001-01-section"]
        }),
        Step::Topo(Topo::Explode("Loft001", "F")),
        Step::Offset(Offset::Parameter(1e-07, false, 'i', false)),
        Step::Offset(Offset::Load("Loft001", 1.0, &[])),
        Step::Offset(Offset::Perform("Offset")),
    ]
);

offset_case!(
    offset_compshape_a1,
    "tests/offset/compshape/A1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CCV_2_a1_gst.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -10.0,
            tolerance: None,
            complete: true,
            faces: &["s_1"]
        })),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            1010000.0,
            "1.01e+06",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_compshape_a2,
    "tests/offset/compshape/A2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CCV_2_b1_gsu.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -10.0,
            tolerance: None,
            complete: true,
            faces: &["s_10"]
        })),
        Step::Check(Check::Props(Prop::Volume, "result", 894477.0, 0.01, None)),
    ]
);

offset_case!(
    offset_compshape_a3,
    "tests/offset/compshape/A3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO ?23068 ALL:Error : The volume"),
        Step::Puts("TODO ?23068 ALL:Faulty shapes in variables faulty_1 to faulty_"),
        Step::Topo(Topo::Restore("brep/CCV_2_c1_gsv.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Catch(&Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -10.0,
            tolerance: None,
            complete: true,
            faces: &["s_8", "s_10"]
        }))),
        Step::Check(Check::PropsAs(
            Prop::Volume,
            "result",
            3076400.0,
            "3.0764e+06",
            0.01,
            None
        )),
    ]
);

offset_case!(
    offset_compshape_a4,
    "tests/offset/compshape/A4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CCV_2_d1_gsw.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Catch(&Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -2.0,
            tolerance: None,
            complete: true,
            faces: &["s_17"]
        }))),
        Step::Check(Check::Props(Prop::Volume, "result", 534256.0, 0.01, None)),
    ]
);

offset_case!(
    offset_compshape_a5,
    "tests/offset/compshape/A5",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/2_E1.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -3.0,
            tolerance: None,
            complete: true,
            faces: &["s_2"]
        })),
        Step::Check(Check::Props(Prop::Volume, "result", 142832.0, 0.01, None)),
    ]
);

offset_case!(
    offset_compshape_a6,
    "tests/offset/compshape/A6",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CCV_2_f1_gsy.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -5.0,
            tolerance: None,
            complete: true,
            faces: &["s_10"]
        })),
        Step::Check(Check::Props(Prop::Volume, "result", 231220.0, 0.01, None)),
    ]
);

offset_case!(
    offset_compshape_a7,
    "tests/offset/compshape/A7",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CCV_2_g1_gsz.rle", "s")),
        Step::Topo(Topo::Explode("s", "F")),
        Step::Offset(Offset::Shape(OffsetShape {
            result: "result",
            shape: "s",
            value: -5.0,
            tolerance: None,
            complete: true,
            faces: &["s_7"]
        })),
        Step::Check(Check::Props(Prop::Volume, "result", 237534.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a3,
    "tests/offset/faces_type_a/A3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb17.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load(
            "s",
            -0.04,
            &["s_4", "s_9", "s_3", "s_5", "s_2", "s_7", "s_11"]
        )),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.182288, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a5,
    "tests/offset/faces_type_a/A5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb26.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.391709, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a6,
    "tests/offset/faces_type_a/A6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_borne.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.47668, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a7,
    "tests/offset/faces_type_a/A7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb2.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &["s_8"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.982069, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a8,
    "tests/offset/faces_type_a/A8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb15.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &["s_8", "s_7"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 4.50601, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_a9,
    "tests/offset/faces_type_a/A9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_bb20.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load("s", -0.1, &["s_4", "s_7"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.491586, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_a_b1,
    "tests/offset/faces_type_a/B1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/CHE_vevopar.rle", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'a', false)),
        Step::Offset(Offset::Load(
            "s",
            -0.3,
            &["s_5", "s_11", "s_3", "s_7", "s_8", "s_9"]
        )),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.707259, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a3,
    "tests/offset/faces_type_i/A3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 15.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 518.363, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a4,
    "tests/offset/faces_type_i/A4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cylinder("s", &[5.0, 15.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 424.115, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a5,
    "tests/offset/faces_type_i/A5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1147.83, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a6,
    "tests/offset/faces_type_i/A6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 957.403, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a7,
    "tests/offset/faces_type_i/A7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 1038.24, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a8,
    "tests/offset/faces_type_i/A8",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[10.0, 5.0, 20.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_2", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 904.725, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_a9,
    "tests/offset/faces_type_i/A9",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 78.53988, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b1,
    "tests/offset/faces_type_i/B1",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 65.1914, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b2,
    "tests/offset/faces_type_i/B2",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The volume of result shape is"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4", "s_3"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b3,
    "tests/offset/faces_type_i/B3",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 26556 ALL: Error : The volume"),
        Step::Puts("TODO 26556 ALL: Faulty shapes"),
        Step::Puts("TODO 26556 ALL: Error : The area"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b4,
    "tests/offset/faces_type_i/B4",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 235.619, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b5,
    "tests/offset/faces_type_i/B5",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Cone("s", &[5.0, 0.0, 12.0, 270.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 235.619, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b6,
    "tests/offset/faces_type_i/B6",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The area of face "),
        Step::Puts("TODO CR23530 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", 1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 3488.84, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b7,
    "tests/offset/faces_type_i/B7",
    Shape,
    900,
    End::OffsetShape(Some(false)),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Prim(Prim::Sphere("s", &[15.0, -90.0, 60.0, 90.0])),
        Step::Place(Place::Rotate(&["s"], [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 90.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', false)),
        Step::Offset(Offset::Load("s", -1.0, &["s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 955.222, 0.01, None)),
    ]
);

offset_case!(
    offset_faces_type_i_b8,
    "tests/offset/faces_type_i/B8",
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
        Step::Offset(Offset::Load("s", 1.0, &["s_3", "s_4"])),
        Step::Offset(Offset::Perform("result")),
        Step::Check(Check::Props(Prop::Volume, "result", 0.0, 0.01, None)),
    ]
);
