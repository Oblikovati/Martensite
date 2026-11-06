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
    offset_shape_type_i_c_multi_a1,
    "tests/offset/shape_type_i_c_multi/A1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 5.0, 5.0, 0.0, 5.0, 0.0, 0.0, 5.0, 0.0,
                0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[3, 4],
            face_values: &[1.0, 2.0],
            refs: &[470.0, 650.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[3, 4],
            face_values: &[2.0, 1.0],
            refs: &[470.0, 650.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[3, 4],
            face_values: &[2.0, 2.0],
            refs: &[480.0, 700.0, 7.0, 7.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[3, 4],
            face_values: &[2.0, 3.0],
            refs: &[708.0, 1224.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[3, 4],
            face_values: &[3.0, 2.0],
            refs: &[708.0, 1224.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_01",
            shape: "s",
            value: 2.0,
            faces: &[3, 4],
            face_values: &[4.0, 4.0],
            refs: &[1008.0, 2156.0, 7.0, 7.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_02",
            shape: "s",
            value: 2.0,
            faces: &[],
            face_values: &[],
            refs: &[896.0, 1764.0, 7.0, 7.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_03",
            shape: "s",
            value: 2.0,
            faces: &[3, 4],
            face_values: &[0.0, 1.0],
            refs: &[826.0, 1470.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_04",
            shape: "s",
            value: 2.0,
            faces: &[3, 4],
            face_values: &[1.0, 1.0],
            refs: &[840.0, 1568.0, 7.0, 7.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_01",
            shape: "s",
            value: 3.0,
            faces: &[4],
            face_values: &[5.0],
            refs: &[1312.0, 3072.0, 8.0, 8.0],
            unified: false
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_01_unif"),
            &[Step::CheckView("r2_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_02_unif"),
            &[Step::CheckView("r2_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_03_unif"),
            &[Step::CheckView("r2_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_04_unif"),
            &[Step::CheckView("r2_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_01_unif"),
            &[Step::CheckView("r3_01_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r3_01", "result")),
        Step::Topo(Topo::SharedCopy("r3_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a2,
    "tests/offset/shape_type_i_c_multi/A2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 5.0, 0.0, 5.0, 5.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0,
                0.0, 5.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[2, 3, 5, 6],
            face_values: &[1.0, 2.0, 3.0, 4.0],
            refs: &[620.0, 1000.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[3, 5],
            face_values: &[2.0, 3.0],
            refs: &[550.0, 750.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[3, 6],
            face_values: &[2.0, 3.0],
            refs: &[550.0, 750.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[2, 6],
            face_values: &[2.0, 3.0],
            refs: &[550.0, 750.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[2, 5],
            face_values: &[2.0, 3.0],
            refs: &[550.0, 750.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[2, 3, 5, 6],
            face_values: &[2.0, 2.0, 3.0, 3.0],
            refs: &[600.0, 1000.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[2, 3, 5, 6],
            face_values: &[3.0, 3.0, 2.0, 2.0],
            refs: &[600.0, 1000.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[2, 3, 5, 6],
            face_values: &[0.0, 1.0, 2.0, 3.0],
            refs: &[696.0, 1152.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[2, 3, 5, 6],
            face_values: &[0.0, 2.0, 0.0, 2.0],
            refs: &[672.0, 1008.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[2, 3, 5, 6],
            face_values: &[2.0, 0.0, 2.0, 0.0],
            refs: &[672.0, 1008.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[3, 5],
            face_values: &[2.0, 2.0],
            refs: &[696.0, 1152.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[],
            face_values: &[],
            refs: &[624.0, 1008.0, 8.0, 8.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[2, 3, 5, 6],
            face_values: &[3.0, 3.0, 2.0, 2.0],
            refs: &[768.0, 1440.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_01",
            shape: "s",
            value: 3.0,
            faces: &[2, 6],
            face_values: &[0.0, 0.0],
            refs: &[1120.0, 2048.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_02",
            shape: "s",
            value: 3.0,
            faces: &[3, 5],
            face_values: &[1.0, 1.0],
            refs: &[1152.0, 2304.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_03",
            shape: "s",
            value: 3.0,
            faces: &[2, 5],
            face_values: &[2.0, 2.0],
            refs: &[1184.0, 2560.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_04",
            shape: "s",
            value: 3.0,
            faces: &[3, 6],
            face_values: &[0.0, 0.0],
            refs: &[1120.0, 2048.0, 10.0, 10.0],
            unified: false
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_01_unif"),
            &[Step::CheckView("r3_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_02_unif"),
            &[Step::CheckView("r3_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_03_unif"),
            &[Step::CheckView("r3_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_04_unif"),
            &[Step::CheckView("r3_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a3,
    "tests/offset/shape_type_i_c_multi/A3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 2.0, 8.0, 0.0, 2.0, 6.0, 0.0, 2.0, 6.0,
                0.0, 5.0, 6.0, 0.0, 8.0, 4.0, 0.0, 8.0, 4.0, 0.0, 5.0, 4.0, 0.0, 2.0, 2.0, 0.0,
                2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[3, 4, 5, 6, 8, 9, 10, 11],
            face_values: &[1.0, 2.0, 1.0, 2.0, 2.0, 1.0, 2.0, 1.0],
            refs: &[516.0, 580.0, 18.0, 18.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 3.0, 3.0, 4.0],
            refs: &[688.0, 640.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 5.0, 5.0, 4.0],
            refs: &[748.0, 740.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[10.0, 5.0, 6.0, 8.0],
            refs: &[918.0, 890.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[2.0, 4.0, 4.0, 2.0],
            refs: &[728.0, 640.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[],
            face_values: &[],
            refs: &[672.0, 864.0, 14.0, 14.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[2.0, 2.0, 2.0, 2.0],
            refs: &[796.0, 1032.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 2.0, 2.0, 3.0],
            refs: &[856.0, 1104.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 3.0, 3.0, 4.0],
            refs: &[880.0, 1248.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 5.0, 5.0, 4.0],
            refs: &[904.0, 1392.0, 18.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 6.0, 6.0, 4.0],
            refs: &[968.0, 1488.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[6.0, 6.0, 6.0, 6.0],
            refs: &[968.0, 1488.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[8.0, 6.0, 6.0, 10.0],
            refs: &[1088.0, 1632.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_01",
            shape: "s",
            value: 2.0,
            faces: &[],
            face_values: &[],
            refs: &[968.0, 1680.0, 12.0, 12.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_02",
            shape: "s",
            value: 2.0,
            faces: &[6, 8],
            face_values: &[4.0, 4.0],
            refs: &[1120.0, 1960.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_03",
            shape: "s",
            value: 2.0,
            faces: &[3, 11],
            face_values: &[4.0, 4.0],
            refs: &[1000.0, 1904.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_04",
            shape: "s",
            value: 2.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 7.0, 7.0, 4.0],
            refs: &[1140.0, 2492.0, 10.0, 10.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_01",
            shape: "s",
            value: 3.0,
            faces: &[],
            face_values: &[],
            refs: &[1312.0, 2816.0, 10.0, 10.0],
            unified: false
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r4_01",
            shape: "s",
            value: 4.0,
            faces: &[],
            face_values: &[],
            refs: &[1704.0, 4320.0, 10.0, 10.0],
            unified: false
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_01_unif"),
            &[Step::CheckView("r2_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_02_unif"),
            &[Step::CheckView("r2_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_03_unif"),
            &[Step::CheckView("r2_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_04_unif"),
            &[Step::CheckView("r2_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_01_unif"),
            &[Step::CheckView("r3_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r4_01_unif"),
            &[Step::CheckView("r4_01_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a4,
    "tests/offset/shape_type_i_c_multi/A4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 2.0, 8.0, 0.0, 2.0, 6.0, 0.0, 2.0, 6.0,
                0.0, 5.0, 6.0, 0.0, 8.0, 4.0, 0.0, 8.0, 4.0, 0.0, 5.0, 4.0, 0.0, 2.0, 2.0, 0.0,
                2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 2.0, 2.0, 3.0],
            refs: &[672.0, 560.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 3.0, 3.0, 3.0],
            refs: &[684.0, 620.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 2.0, 2.0, 4.0],
            refs: &[680.0, 600.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 4.0, 4.0, 3.0],
            refs: &[696.0, 680.0, 18.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 5.0, 5.0, 3.0],
            refs: &[748.0, 740.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 2.0, 2.0, 3.0],
            refs: &[856.0, 1104.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[4.0, 2.0, 2.0, 4.0],
            refs: &[868.0, 1176.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 3.0, 3.0, 3.0],
            refs: &[872.0, 1200.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 4.0, 4.0, 3.0],
            refs: &[888.0, 1296.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[3.0, 5.0, 5.0, 3.0],
            refs: &[904.0, 1392.0, 18.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[3, 6, 8, 11],
            face_values: &[7.0, 2.0, 2.0, 7.0],
            refs: &[904.0, 1392.0, 18.0, 14.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r1_02", "result")),
        Step::Topo(Topo::SharedCopy("r1_02_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a5,
    "tests/offset/shape_type_i_c_multi/A5",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 1.0, 8.0, 0.0, 1.0, 8.0, 0.0, 4.0, 8.0,
                0.0, 7.0, 7.0, 0.0, 7.0, 7.0, 0.0, 4.0, 7.0, 0.0, 1.0, 3.0, 0.0, 1.0, 3.0, 0.0,
                4.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0, 2.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],
            refs: &[568.0, 340.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            refs: &[568.0, 340.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[8, 10],
            face_values: &[1.0, 1.0],
            refs: &[516.0, 280.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[8, 10],
            face_values: &[2.0, 2.0],
            refs: &[468.0, 340.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[8, 10],
            face_values: &[3.0, 3.0],
            refs: &[468.0, 340.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[8, 11],
            face_values: &[1.0, 1.0],
            refs: &[536.0, 280.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[8, 11],
            face_values: &[2.0, 2.0],
            refs: &[568.0, 340.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[8, 11],
            face_values: &[3.0, 3.0],
            refs: &[560.0, 400.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[1.0, 1.0],
            refs: &[536.0, 280.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_10",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[2.0, 2.0],
            refs: &[568.0, 340.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_11",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[3.0, 3.0],
            refs: &[560.0, 400.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_12",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[2.0, 2.0],
            refs: &[548.0, 340.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_13",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[3.0, 3.0],
            refs: &[548.0, 340.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],
            refs: &[816.0, 720.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            refs: &[816.0, 720.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[8, 10],
            face_values: &[1.5, 1.5],
            refs: &[796.0, 888.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[8, 10],
            face_values: &[2.0, 2.0],
            refs: &[752.0, 912.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[8, 10],
            face_values: &[3.0, 3.0],
            refs: &[752.0, 912.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[8, 11],
            face_values: &[1.5, 1.5],
            refs: &[810.0, 900.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[8, 11],
            face_values: &[2.0, 2.0],
            refs: &[828.0, 936.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[8, 11, 7, 10],
            face_values: &[3.0, 3.0, 0.0, 0.0],
            refs: &[828.0, 936.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_09",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[1.5, 1.5],
            refs: &[810.0, 900.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_10",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[2.0, 2.0],
            refs: &[828.0, 936.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 8, 11],
            face_values: &[3.0, 3.0, 0.0, 0.0],
            refs: &[828.0, 936.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_12",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[2.0, 2.0],
            refs: &[760.0, 960.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_13",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[3.0, 3.0],
            refs: &[760.0, 960.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_01",
            shape: "s",
            value: 2.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0],
            refs: &[1184.0, 1428.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_02",
            shape: "s",
            value: 2.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
            refs: &[1040.0, 1204.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_03",
            shape: "s",
            value: 2.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 2.0, 1.0, 1.0, 2.0, 0.0, 1.0],
            refs: &[996.0, 1484.0, 20.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_01",
            shape: "s",
            value: 3.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0],
            refs: &[1416.0, 2368.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_02",
            shape: "s",
            value: 3.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
            refs: &[1368.0, 1984.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_03",
            shape: "s",
            value: 3.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[1.0, 0.0, 2.0, 1.0, 1.0, 2.0, 0.0, 1.0],
            refs: &[1224.0, 2368.0, 10.0, 10.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_10_unif"),
            &[Step::CheckView("r0_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_11_unif"),
            &[Step::CheckView("r0_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_12_unif"),
            &[Step::CheckView("r0_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_13_unif"),
            &[Step::CheckView("r0_13_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_09_unif"),
            &[Step::CheckView("r1_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_10_unif"),
            &[Step::CheckView("r1_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_12_unif"),
            &[Step::CheckView("r1_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_13_unif"),
            &[Step::CheckView("r1_13_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_01_unif"),
            &[Step::CheckView("r2_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_02_unif"),
            &[Step::CheckView("r2_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_03_unif"),
            &[Step::CheckView("r2_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_01_unif"),
            &[Step::CheckView("r3_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_02_unif"),
            &[Step::CheckView("r3_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_03_unif"),
            &[Step::CheckView("r3_03_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r1_01", "result")),
        Step::Topo(Topo::SharedCopy("r1_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a6,
    "tests/offset/shape_type_i_c_multi/A6",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 1.0, 8.0, 0.0, 1.0, 8.0, 0.0, 4.0, 8.0,
                0.0, 7.0, 7.0, 0.0, 7.0, 7.0, 0.0, 5.0, 7.0, 0.0, 1.0, 3.0, 0.0, 1.0, 3.0, 0.0,
                3.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0, 2.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],
            refs: &[568.0, 360.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[2.0, 2.0],
            refs: &[560.0, 300.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[4.0, 2.0],
            refs: &[568.0, 340.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[4.0, 4.0],
            refs: &[536.0, 380.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[7, 10],
            face_values: &[5.0, 3.0],
            refs: &[572.0, 360.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[2.0, 2.0],
            refs: &[568.0, 340.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[3.0, 2.0],
            refs: &[568.0, 340.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[3.0, 3.0],
            refs: &[572.0, 360.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[0.0, 4.0],
            refs: &[536.0, 380.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 10, 11, 13, 14],
            face_values: &[0.0, 1.5, 0.0, 1.5, 0.0, 1.5, 0.0, 1.5],
            refs: &[858.0, 828.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[2.0, 2.0],
            refs: &[824.0, 912.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[4.0, 4.0],
            refs: &[760.0, 960.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[0.0, 4.0],
            refs: &[740.0, 984.0, 16.0, 14.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a7,
    "tests/offset/shape_type_i_c_multi/A7",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 1.0, 8.0, 0.0, 1.0, 8.0, 0.0, 4.0, 8.0,
                0.0, 7.0, 7.0, 0.0, 7.0, 7.0, 0.0, 5.0, 7.0, 0.0, 1.0, 3.0, 0.0, 1.0, 3.0, 0.0,
                3.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0, 2.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[4.0, 2.0],
            refs: &[782.0, 948.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[7, 10],
            face_values: &[5.0, 3.0],
            refs: &[760.0, 960.0, 16.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[2.0, 2.0],
            refs: &[784.0, 960.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[3.0, 2.0],
            refs: &[784.0, 960.0, 18.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[7, 11],
            face_values: &[3.0, 3.0],
            refs: &[740.0, 984.0, 16.0, 14.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r1_03", "result")),
        Step::Topo(Topo::SharedCopy("r1_03_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a8,
    "tests/offset/shape_type_i_c_multi/A8",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 11.0, 0.0, 0.0, 11.0, 0.0, 1.0, 9.0, 0.0, 1.0, 9.0, 0.0, 4.0, 9.0,
                0.0, 7.0, 8.0, 0.0, 7.0, 8.0, 0.0, 4.0, 8.0, 0.0, 1.0, 6.0, 0.0, 1.0, 5.0, 0.0,
                1.0, 3.0, 0.0, 1.0, 3.0, 0.0, 4.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0,
                2.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[686.0, 630.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[708.0, 640.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 4.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[690.0, 650.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 8.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[698.0, 690.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.1, 1.0, 1.0, 4.0, 1.0, 1.0, 2.1, 2.0, 1.0],
            refs: &[690.8, 654.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.5, 1.0, 1.0, 4.0, 1.0, 1.0, 2.5, 2.0, 1.0],
            refs: &[654.0, 670.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 3.0, 1.0, 1.0, 4.0, 1.0, 1.0, 3.0, 2.0, 1.0],
            refs: &[654.0, 670.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[2.0, 1.0, 2.0],
            refs: &[532.0, 360.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[2.2, 1.0, 2.2],
            refs: &[533.6, 368.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_10",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[2.5, 1.0, 2.5],
            refs: &[496.0, 380.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_11",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[3.0, 3.0, 3.0],
            refs: &[496.0, 380.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_12",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[2.0, 3.0, 2.0],
            refs: &[696.0, 380.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_13",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[2.0, 5.0, 2.0],
            refs: &[660.0, 400.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_14",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[2.5, 3.0, 2.5],
            refs: &[642.0, 410.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_15",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[2.5, 5.0, 2.5],
            refs: &[642.0, 410.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_16",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[3.0, 5.0, 3.0],
            refs: &[642.0, 410.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[976.0, 1104.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 3.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[1002.0, 1116.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.1, 1.0, 1.0, 4.0, 1.0, 1.0, 2.1, 2.0, 1.0],
            refs: &[981.2, 1135.2, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.5, 1.0, 1.0, 4.0, 1.0, 1.0, 2.5, 2.0, 1.0],
            refs: &[914.0, 1164.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 3.0, 1.0, 1.0, 4.0, 1.0, 1.0, 3.0, 2.0, 1.0],
            refs: &[914.0, 1164.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[2.0, 2.0, 2.0],
            refs: &[808.0, 960.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_09",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[2.2, 1.0, 2.2],
            refs: &[831.6, 957.6, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_10",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[2.5, 1.0, 2.5],
            refs: &[786.0, 972.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[3.0, 3.0, 3.0],
            refs: &[786.0, 972.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_12",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 13],
            face_values: &[2.0, 3.0, 2.0],
            refs: &[938.0, 1020.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_15",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 13],
            face_values: &[2.5, 5.0, 2.5],
            refs: &[850.0, 1068.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_16",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 13],
            face_values: &[3.0, 5.0, 3.0],
            refs: &[850.0, 1068.0, 22.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2_01",
            shape: "s",
            value: 2.0,
            faces: &[],
            face_values: &[],
            refs: &[1166.0, 1890.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r3_01",
            shape: "s",
            value: 3.0,
            faces: &[5, 15],
            face_values: &[6.0, 6.0],
            refs: &[1490.0, 3728.0, 10.0, 10.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_10_unif"),
            &[Step::CheckView("r0_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_11_unif"),
            &[Step::CheckView("r0_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_12_unif"),
            &[Step::CheckView("r0_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_13_unif"),
            &[Step::CheckView("r0_13_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_14_unif"),
            &[Step::CheckView("r0_14_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_15_unif"),
            &[Step::CheckView("r0_15_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_16_unif"),
            &[Step::CheckView("r0_16_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_09_unif"),
            &[Step::CheckView("r1_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_10_unif"),
            &[Step::CheckView("r1_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_12_unif"),
            &[Step::CheckView("r1_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_15_unif"),
            &[Step::CheckView("r1_15_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_16_unif"),
            &[Step::CheckView("r1_16_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r2_01_unif"),
            &[Step::CheckView("r2_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r3_01_unif"),
            &[Step::CheckView("r3_01_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_a9,
    "tests/offset/shape_type_i_c_multi/A9",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 11.0, 0.0, 0.0, 11.0, 0.0, 1.0, 9.0, 0.0, 1.0, 9.0, 0.0, 4.0, 9.0,
                0.0, 7.0, 8.0, 0.0, 7.0, 8.0, 0.0, 4.0, 8.0, 0.0, 1.0, 6.0, 0.0, 1.0, 5.0, 0.0,
                1.0, 3.0, 0.0, 1.0, 3.0, 0.0, 4.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0,
                2.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 4.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[980.0, 1128.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[4, 5, 7, 8, 9, 10, 11, 12, 13, 15, 16],
            face_values: &[1.0, 2.0, 2.0, 1.0, 1.0, 8.0, 1.0, 1.0, 2.0, 2.0, 1.0],
            refs: &[940.0, 1176.0, 30.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 13],
            face_values: &[2.0, 5.0, 2.0],
            refs: &[894.0, 1044.0, 26.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[7, 10, 13],
            face_values: &[2.5, 3.0, 2.5],
            refs: &[850.0, 1068.0, 22.0, 18.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r1_03", "result")),
        Step::Topo(Topo::SharedCopy("r1_03_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b1,
    "tests/offset/shape_type_i_c_multi/B1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_iso_face_top.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[7],
            face_values: &[2.0],
            refs: &[416.0, 508.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[6, 7],
            face_values: &[1.0, 2.0],
            refs: &[448.0, 604.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 7],
            face_values: &[2.0, 1.0],
            refs: &[488.0, 696.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[6],
            face_values: &[2.0],
            refs: &[496.0, 692.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[7],
            face_values: &[2.0],
            refs: &[632.0, 1012.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[6, 7],
            face_values: &[2.0, 3.0],
            refs: &[680.0, 1156.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[6, 7],
            face_values: &[3.0, 2.0],
            refs: &[728.0, 1292.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[6],
            face_values: &[3.0],
            refs: &[736.0, 1288.0, 12.0, 11.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b2,
    "tests/offset/shape_type_i_c_multi/B2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_top_split1.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8, 9, 10],
            face_values: &[1.0, 1.0, 1.0, 1.0, 1.0],
            refs: &[440.0, 600.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[6],
            face_values: &[1.0],
            refs: &[420.0, 524.0, 11.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 9],
            face_values: &[1.0, 2.0],
            refs: &[460.0, 572.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[6, 8],
            face_values: &[1.0, 1.0],
            refs: &[432.0, 548.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[6, 8],
            face_values: &[1.0, 2.0],
            refs: &[452.0, 572.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8],
            face_values: &[1.0, 2.0, 2.0],
            refs: &[484.0, 620.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9],
            face_values: &[1.0, 2.0, 3.0],
            refs: &[496.0, 644.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7],
            face_values: &[1.0, 2.0, 3.0, 4.0],
            refs: &[544.0, 740.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7],
            face_values: &[1.0, 2.0, 3.0, 2.0],
            refs: &[512.0, 692.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_10",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[1.0, 2.0, 3.0, 2.0, 5.0],
            refs: &[520.0, 712.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_11",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[1.0, 2.0, 3.0, 4.0, 5.0],
            refs: &[544.0, 760.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_12",
            shape: "s",
            value: 0.0,
            faces: &[10],
            face_values: &[2.0],
            refs: &[416.0, 508.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[6, 7, 8, 9, 10],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[672.0, 1152.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[6],
            face_values: &[2.0],
            refs: &[648.0, 1043.0, 11.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[6, 9],
            face_values: &[2.0, 3.0],
            refs: &[696.0, 1113.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[6, 8],
            face_values: &[2.0, 2.0],
            refs: &[662.0, 1078.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[6, 8],
            face_values: &[2.0, 3.0],
            refs: &[686.0, 1113.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[6, 7, 8],
            face_values: &[2.0, 3.0, 3.0],
            refs: &[724.0, 1183.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9],
            face_values: &[2.0, 3.0, 4.0],
            refs: &[738.0, 1218.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7],
            face_values: &[2.0, 3.0, 4.0, 5.0],
            refs: &[794.0, 1358.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_09",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7],
            face_values: &[2.0, 3.0, 4.0, 3.0],
            refs: &[756.0, 1288.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_10",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 3.0, 5.0],
            refs: &[756.0, 1304.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 5.0, 6.0],
            refs: &[794.0, 1378.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 5.0, 2.5],
            refs: &[784.0, 1364.0, 20.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_12",
            shape: "s",
            value: 1.0,
            faces: &[10],
            face_values: &[2.0],
            refs: &[632.0, 1012.0, 12.0, 11.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_10_unif"),
            &[Step::CheckView("r0_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_11_unif"),
            &[Step::CheckView("r0_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_12_unif"),
            &[Step::CheckView("r0_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_09_unif"),
            &[Step::CheckView("r1_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_10_unif"),
            &[Step::CheckView("r1_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_12_unif"),
            &[Step::CheckView("r1_12_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_11", "result")),
        Step::Topo(Topo::SharedCopy("r0_11_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b3,
    "tests/offset/shape_type_i_c_multi/B3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_top_split2.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8, 9, 10],
            face_values: &[1.0, 1.0, 1.0, 1.0, 1.0],
            refs: &[440.0, 600.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[6],
            face_values: &[1.0],
            refs: &[420.0, 524.0, 9.0, 9.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 9],
            face_values: &[1.0, 2.0],
            refs: &[460.0, 572.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[6, 8],
            face_values: &[1.0, 1.0],
            refs: &[432.0, 548.0, 10.0, 10.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[6, 8],
            face_values: &[1.0, 2.0],
            refs: &[452.0, 572.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8],
            face_values: &[1.0, 2.0, 2.0],
            refs: &[484.0, 620.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9],
            face_values: &[1.0, 2.0, 3.0],
            refs: &[496.0, 644.0, 15.0, 15.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7],
            face_values: &[1.0, 2.0, 3.0, 4.0],
            refs: &[544.0, 740.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7],
            face_values: &[1.0, 2.0, 3.0, 2.0],
            refs: &[512.0, 692.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_10",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[1.0, 2.0, 3.0, 2.0, 5.0],
            refs: &[520.0, 712.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_11",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[1.0, 2.0, 3.0, 4.0, 5.0],
            refs: &[544.0, 760.0, 15.0, 15.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_12",
            shape: "s",
            value: 0.0,
            faces: &[10],
            face_values: &[2.0],
            refs: &[416.0, 508.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[6, 7, 8, 9, 10],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[672.0, 1152.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[6],
            face_values: &[2.0],
            refs: &[648.0, 1043.0, 9.0, 9.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[6, 9],
            face_values: &[2.0, 3.0],
            refs: &[696.0, 1113.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[6, 8],
            face_values: &[2.0, 2.0],
            refs: &[662.0, 1078.0, 10.0, 10.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[6, 8],
            face_values: &[2.0, 3.0],
            refs: &[686.0, 1113.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[6, 7, 8],
            face_values: &[2.0, 3.0, 3.0],
            refs: &[724.0, 1183.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9],
            face_values: &[2.0, 3.0, 4.0],
            refs: &[738.0, 1218.0, 15.0, 15.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7],
            face_values: &[2.0, 3.0, 4.0, 5.0],
            refs: &[794.0, 1358.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_09",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7],
            face_values: &[2.0, 3.0, 4.0, 3.0],
            refs: &[756.0, 1288.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_10",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 3.0, 5.0],
            refs: &[756.0, 1304.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 5.0, 6.0],
            refs: &[794.0, 1378.0, 15.0, 15.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_12",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 9, 7, 10],
            face_values: &[2.0, 3.0, 4.0, 5.0, 2.5],
            refs: &[784.0, 1364.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_13",
            shape: "s",
            value: 1.0,
            faces: &[10],
            face_values: &[2.0],
            refs: &[632.0, 1012.0, 12.0, 11.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_10_unif"),
            &[Step::CheckView("r0_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_11_unif"),
            &[Step::CheckView("r0_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_12_unif"),
            &[Step::CheckView("r0_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_09_unif"),
            &[Step::CheckView("r1_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_10_unif"),
            &[Step::CheckView("r1_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_12_unif"),
            &[Step::CheckView("r1_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_13_unif"),
            &[Step::CheckView("r1_13_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_11", "result")),
        Step::Topo(Topo::SharedCopy("r0_11_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b4,
    "tests/offset/shape_type_i_c_multi/B4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_top_split3.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8, 9, 10, 11, 12, 13, 14],
            face_values: &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            refs: &[440.0, 600.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[7, 8, 11, 12],
            face_values: &[1.0, 1.0, 1.0, 1.0],
            refs: &[448.0, 532.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 9, 10, 13, 14],
            face_values: &[1.0, 1.0, 1.0, 1.0, 1.0],
            refs: &[472.0, 568.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[14],
            face_values: &[1.0],
            refs: &[408.0, 504.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 8, 9, 10, 11, 12, 13],
            face_values: &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            refs: &[448.0, 596.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[6, 7, 8, 9, 10, 11, 12, 13, 14],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[672.0, 1152.0, 6.0, 6.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[7, 8, 11, 12],
            face_values: &[2.0, 2.0, 2.0, 2.0],
            refs: &[680.0, 1048.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 10, 13, 14],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[712.0, 1112.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[14],
            face_values: &[2.0],
            refs: &[632.0, 1012.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[14],
            face_values: &[0.0],
            refs: &[632.0, 1004.0, 12.0, 11.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_02", "result")),
        Step::Topo(Topo::SharedCopy("r0_02_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b5,
    "tests/offset/shape_type_i_c_multi/B5",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_top_split4.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[2, 5, 6, 8],
            face_values: &[1.0, 2.0, 3.0, 4.0],
            refs: &[530.0, 750.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[5, 6],
            face_values: &[1.0, 1.0],
            refs: &[440.0, 550.0, 13.0, 13.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[5, 6],
            face_values: &[1.0, 2.0],
            refs: &[460.0, 575.0, 13.0, 13.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[2, 5, 6, 8],
            face_values: &[2.0, 3.0, 4.0, 5.0],
            refs: &[780.0, 1368.0, 12.0, 12.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[5, 6],
            face_values: &[2.0, 2.0],
            refs: &[672.0, 1080.0, 13.0, 13.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[5, 6],
            face_values: &[2.0, 3.0],
            refs: &[696.0, 1116.0, 13.0, 13.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b6,
    "tests/offset/shape_type_i_c_multi/B6",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_plate_split.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[10],
            face_values: &[3.0],
            refs: &[428.0, 204.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[10],
            face_values: &[4.0],
            refs: &[432.0, 212.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[10],
            face_values: &[5.0],
            refs: &[432.0, 212.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[6, 14],
            face_values: &[1.0, 1.0],
            refs: &[464.0, 220.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[6, 10, 14],
            face_values: &[1.0, 2.0, 1.0],
            refs: &[464.0, 236.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[6, 10, 14],
            face_values: &[1.0, 3.0, 1.0],
            refs: &[464.0, 240.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[6, 10, 14],
            face_values: &[1.0, 4.0, 1.0],
            refs: &[464.0, 244.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[9, 11],
            face_values: &[3.0, 3.0],
            refs: &[440.0, 228.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_09",
            shape: "s",
            value: 0.0,
            faces: &[9, 11],
            face_values: &[4.0, 4.0],
            refs: &[448.0, 244.0, 24.0, 23.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_10",
            shape: "s",
            value: 0.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.0, 2.0, 2.0, 1.0],
            refs: &[464.0, 252.0, 34.0, 34.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_11",
            shape: "s",
            value: 0.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.0, 3.0, 3.0, 1.0],
            refs: &[464.0, 260.0, 34.0, 34.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_12",
            shape: "s",
            value: 0.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.0, 4.0, 4.0, 1.0],
            refs: &[464.0, 268.0, 36.0, 35.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_13",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[3.0, 3.0, 3.0],
            refs: &[452.0, 252.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_14",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[4.0, 4.0, 4.0],
            refs: &[464.0, 276.0, 22.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_15",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.0, 3.0, 3.0, 3.0, 1.0],
            refs: &[464.0, 280.0, 30.0, 30.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_16",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.0, 4.0, 3.0, 4.0, 1.0],
            refs: &[464.0, 288.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_17",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.0, 3.0, 4.0, 3.0, 1.0],
            refs: &[464.0, 284.0, 30.0, 30.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_18",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.0, 4.0, 4.0, 4.0, 1.0],
            refs: &[464.0, 292.0, 30.0, 28.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_19",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[1.0, 2.0, 1.0],
            refs: &[424.0, 228.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_20",
            shape: "s",
            value: 0.0,
            faces: &[7, 10, 13],
            face_values: &[1.0, 3.0, 1.0],
            refs: &[424.0, 228.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_21",
            shape: "s",
            value: 0.0,
            faces: &[7, 9, 11, 13],
            face_values: &[1.0, 2.0, 2.0, 1.0],
            refs: &[424.0, 236.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_22",
            shape: "s",
            value: 0.0,
            faces: &[7, 8, 10, 12, 13],
            face_values: &[1.0, 2.0, 2.0, 2.0, 1.0],
            refs: &[424.0, 244.0, 26.0, 24.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_23",
            shape: "s",
            value: 0.0,
            faces: &[8, 9, 10, 11, 12],
            face_values: &[1.0, 2.0, 3.0, 4.0, 4.0],
            refs: &[392.0, 292.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[10],
            face_values: &[3.0],
            refs: &[672.0, 728.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[10],
            face_values: &[5.0],
            refs: &[672.0, 736.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[10],
            face_values: &[6.0],
            refs: &[672.0, 736.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[6, 14],
            face_values: &[1.5, 1.5],
            refs: &[702.0, 756.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[1.5, 2.0, 1.5],
            refs: &[698.0, 760.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[1.5, 3.0, 1.5],
            refs: &[696.0, 762.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_07",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[1.5, 5.0, 1.5],
            refs: &[692.0, 766.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_08",
            shape: "s",
            value: 1.0,
            faces: &[9, 11],
            face_values: &[3.0, 3.0],
            refs: &[672.0, 736.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_09",
            shape: "s",
            value: 1.0,
            faces: &[9, 11],
            face_values: &[5.0, 5.0],
            refs: &[672.0, 752.0, 24.0, 23.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_10",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.5, 2.0, 2.0, 1.5],
            refs: &[694.0, 764.0, 34.0, 34.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_11",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.5, 4.0, 4.0, 1.5],
            refs: &[686.0, 772.0, 34.0, 34.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_12",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.5, 5.0, 5.0, 1.5],
            refs: &[682.0, 776.0, 36.0, 35.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_13",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[3.0, 3.0, 3.0],
            refs: &[664.0, 752.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_14",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[5.0, 5.0, 5.0],
            refs: &[656.0, 784.0, 22.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_15",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.5, 3.0, 3.0, 3.0, 1.5],
            refs: &[672.0, 780.0, 30.0, 30.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_16",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.5, 5.0, 4.0, 5.0, 1.5],
            refs: &[654.0, 794.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_17",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.5, 4.0, 5.0, 4.0, 1.5],
            refs: &[660.0, 790.0, 30.0, 30.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_18",
            shape: "s",
            value: 1.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.5, 5.0, 5.0, 5.0, 1.5],
            refs: &[652.0, 796.0, 30.0, 28.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_19",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[0.0, 2.0, 0.0],
            refs: &[660.0, 652.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_20",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[0.0, 3.0, 0.0],
            refs: &[660.0, 652.0, 22.0, 22.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_21",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[0.0, 2.0, 2.0, 0.0],
            refs: &[660.0, 656.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_22",
            shape: "s",
            value: 1.0,
            faces: &[7, 8, 10, 12, 13],
            face_values: &[1.5, 2.0, 2.0, 2.0, 1.5],
            refs: &[664.0, 740.0, 26.0, 24.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_23",
            shape: "s",
            value: 1.0,
            faces: &[8, 9, 10, 11, 12],
            face_values: &[1.0, 2.0, 3.0, 4.0, 5.0],
            refs: &[640.0, 768.0, 20.0, 20.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_09_unif"),
            &[Step::CheckView("r0_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_10_unif"),
            &[Step::CheckView("r0_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_11_unif"),
            &[Step::CheckView("r0_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_12_unif"),
            &[Step::CheckView("r0_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_13_unif"),
            &[Step::CheckView("r0_13_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_14_unif"),
            &[Step::CheckView("r0_14_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_15_unif"),
            &[Step::CheckView("r0_15_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_16_unif"),
            &[Step::CheckView("r0_16_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_17_unif"),
            &[Step::CheckView("r0_17_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_18_unif"),
            &[Step::CheckView("r0_18_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_19_unif"),
            &[Step::CheckView("r0_19_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_20_unif"),
            &[Step::CheckView("r0_20_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_21_unif"),
            &[Step::CheckView("r0_21_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_22_unif"),
            &[Step::CheckView("r0_22_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_23_unif"),
            &[Step::CheckView("r0_23_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_07_unif"),
            &[Step::CheckView("r1_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_08_unif"),
            &[Step::CheckView("r1_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_09_unif"),
            &[Step::CheckView("r1_09_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_10_unif"),
            &[Step::CheckView("r1_10_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_11_unif"),
            &[Step::CheckView("r1_11_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_12_unif"),
            &[Step::CheckView("r1_12_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_13_unif"),
            &[Step::CheckView("r1_13_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_14_unif"),
            &[Step::CheckView("r1_14_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_15_unif"),
            &[Step::CheckView("r1_15_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_16_unif"),
            &[Step::CheckView("r1_16_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_17_unif"),
            &[Step::CheckView("r1_17_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_18_unif"),
            &[Step::CheckView("r1_18_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_19_unif"),
            &[Step::CheckView("r1_19_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_20_unif"),
            &[Step::CheckView("r1_20_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_21_unif"),
            &[Step::CheckView("r1_21_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_22_unif"),
            &[Step::CheckView("r1_22_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_23_unif"),
            &[Step::CheckView("r1_23_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b7,
    "tests/offset/shape_type_i_c_multi/B7",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 31200 All: Error: operation with offset value"),
        Step::Puts("TODO 31200 All: Error: The area of result shape is"),
        Step::Puts("TODO 31200 All: Error: The volume of result shape is"),
        Step::Puts("TODO 31200 All: Error: number of "),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_plate_split.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[6, 10, 14],
            face_values: &[1.0, 5.0, 1.0],
            refs: &[464.0, 244.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[9, 11],
            face_values: &[5.0, 5.0],
            refs: &[448.0, 244.0, 24.0, 23.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.0, 5.0, 5.0, 1.0],
            refs: &[464.0, 268.0, 36.0, 35.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[8, 10, 12],
            face_values: &[5.0, 5.0, 5.0],
            refs: &[464.0, 276.0, 22.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_05",
            shape: "s",
            value: 0.0,
            faces: &[6, 8, 10, 12, 14],
            face_values: &[1.0, 5.0, 5.0, 5.0, 1.0],
            refs: &[464.0, 292.0, 30.0, 28.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_06",
            shape: "s",
            value: 0.0,
            faces: &[7, 9, 11, 13],
            face_values: &[1.0, 3.0, 3.0, 1.0],
            refs: &[424.0, 236.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_07",
            shape: "s",
            value: 0.0,
            faces: &[7, 8, 10, 12, 13],
            face_values: &[1.0, 3.0, 3.0, 3.0, 1.0],
            refs: &[424.0, 244.0, 26.0, 24.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_08",
            shape: "s",
            value: 0.0,
            faces: &[8, 9, 10, 11, 12],
            face_values: &[1.0, 2.0, 3.0, 4.0, 5.0],
            refs: &[392.0, 292.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_01",
            shape: "s",
            value: 1.0,
            faces: &[6, 10, 14],
            face_values: &[1.5, 6.0, 1.5],
            refs: &[692.0, 766.0, 26.0, 26.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_02",
            shape: "s",
            value: 1.0,
            faces: &[9, 11],
            face_values: &[6.0, 6.0],
            refs: &[672.0, 752.0, 24.0, 23.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_03",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[1.5, 6.0, 6.0, 1.5],
            refs: &[682.0, 776.0, 36.0, 35.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_04",
            shape: "s",
            value: 1.0,
            faces: &[8, 10, 12],
            face_values: &[6.0, 6.0, 6.0],
            refs: &[656.0, 784.0, 22.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_05",
            shape: "s",
            value: 1.0,
            faces: &[6, 9, 11, 14],
            face_values: &[0.0, 3.0, 3.0, 0.0],
            refs: &[660.0, 656.0, 28.0, 27.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1_06",
            shape: "s",
            value: 1.0,
            faces: &[8, 9, 10, 11, 12],
            face_values: &[2.0, 3.0, 4.0, 5.0, 6.0],
            refs: &[622.0, 786.0, 18.0, 18.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_05_unif"),
            &[Step::CheckView("r0_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_06_unif"),
            &[Step::CheckView("r0_06_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_07_unif"),
            &[Step::CheckView("r0_07_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_08_unif"),
            &[Step::CheckView("r0_08_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_01_unif"),
            &[Step::CheckView("r1_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_02_unif"),
            &[Step::CheckView("r1_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_03_unif"),
            &[Step::CheckView("r1_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_04_unif"),
            &[Step::CheckView("r1_04_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_05_unif"),
            &[Step::CheckView("r1_05_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r1_06_unif"),
            &[Step::CheckView("r1_06_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_02", "result")),
        Step::Topo(Topo::SharedCopy("r0_02_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b8,
    "tests/offset/shape_type_i_c_multi/B8",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_U_shape_split1.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[6],
            face_values: &[3.0],
            refs: &[708.0, 340.0, 14.0, 14.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[6, 10],
            face_values: &[3.0, 3.0],
            refs: &[798.0, 418.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[9, 6, 11, 8, 1, 3, 30, 32, 27, 25, 22, 24, 19, 17, 14, 16],
            face_values: &[
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0
            ],
            refs: &[1132.0, 860.0, 58.0, 58.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_b9,
    "tests/offset/shape_type_i_c_multi/B9",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_U_shape_split2.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[8, 11],
            face_values: &[3.0, 3.0],
            refs: &[816.0, 499.0, 16.0, 16.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[7, 11],
            face_values: &[3.0, 3.0],
            refs: &[804.0, 448.0, 17.0, 17.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[2, 6, 10, 14, 18, 22, 26, 30],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[932.0, 544.0, 30.0, 28.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[2, 6, 10, 18, 22, 26],
            face_values: &[3.0, 3.0, 3.0, 2.0, 2.0, 2.0],
            refs: &[968.0, 584.0, 26.0, 26.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_c1,
    "tests/offset/shape_type_i_c_multi/C1",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_U_shape_split3.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_01",
            shape: "s",
            value: 0.0,
            faces: &[2, 6],
            face_values: &[3.0, 3.0],
            refs: &[810.0, 478.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_02",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 17],
            face_values: &[3.0, 3.0, 3.0],
            refs: &[816.0, 508.0, 18.0, 18.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_03",
            shape: "s",
            value: 0.0,
            faces: &[6, 7, 17],
            face_values: &[3.0, 3.0, 2.0],
            refs: &[812.0, 485.0, 20.0, 20.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r0_04",
            shape: "s",
            value: 0.0,
            faces: &[20, 21, 22, 29, 31],
            face_values: &[2.0, 2.0, 2.0, 2.0, 2.0],
            refs: &[712.0, 474.0, 20.0, 20.0],
            unified: true
        })),
        Step::If(
            Cond::Bound("r0_01_unif"),
            &[Step::CheckView("r0_01_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_02_unif"),
            &[Step::CheckView("r0_02_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_03_unif"),
            &[Step::CheckView("r0_03_unif")],
            &[]
        ),
        Step::If(
            Cond::Bound("r0_04_unif"),
            &[Step::CheckView("r0_04_unif")],
            &[]
        ),
        Step::Topo(Topo::SharedCopy("r0_01", "result")),
        Step::Topo(Topo::SharedCopy("r0_01_unif", "result_unif")),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_c2,
    "tests/offset/shape_type_i_c_multi/C2",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore(
            "brep/bug31148_plate_offset_parallel.brep",
            "s"
        )),
        Step::More(More::MultiOffset(MultiOffset {
            result: "result",
            shape: "s",
            value: 0.0,
            faces: &[22, 24],
            face_values: &[3.0, 3.0],
            refs: &[472.0, 260.0, 34.0, 34.0],
            unified: true
        })),
        Step::CheckView("result_unif"),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_c3,
    "tests/offset/shape_type_i_c_multi/C3",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_iso_face_top.brep", "s")),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r1",
            shape: "s",
            value: 0.0,
            faces: &[7],
            face_values: &[2.0],
            refs: &[416.0, 508.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "res1",
            shape: "r1_unif",
            value: 1.0,
            faces: &[],
            face_values: &[],
            refs: &[656.0, 1040.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "r2",
            shape: "s",
            value: 0.0,
            faces: &[6],
            face_values: &[2.0],
            refs: &[496.0, 692.0, 12.0, 11.0],
            unified: true
        })),
        Step::More(More::MultiOffset(MultiOffset {
            result: "result",
            shape: "r2_unif",
            value: 0.5,
            faces: &[],
            face_values: &[],
            refs: &[602.0, 966.0, 12.0, 11.0],
            unified: true
        })),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_c4,
    "tests/offset/shape_type_i_c_multi/C4",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Topo(Topo::Restore("brep/bug31148_box_iso_face_top.brep", "s")),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', true)),
        Step::Offset(Offset::Load("s", 0.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_6", 1.0)])),
        Step::Offset(Offset::OnFace(&[("s_7", 2.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::More(More::History(HistoryStep::Save("h"))),
        Step::More(More::History(HistoryStep::Generated("gf6", "h", "s_6"))),
        Step::Check(Check::PropsEqual("gf6", "s_6")),
        Step::More(More::History(HistoryStep::Generated("gf7", "h", "s_7"))),
        Step::Check(Check::PropsEqual("gf7", "s_7")),
        Step::Run(crate::one_off::c4_history),
    ]
);

offset_case!(
    offset_shape_type_i_c_multi_c5,
    "tests/offset/shape_type_i_c_multi/C5",
    Shape,
    900,
    End::OffsetShape(None),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("========"),
        Step::Puts("0031148: Offset adjacent co-planar faces with different offset values"),
        Step::Puts("========"),
        Step::Profile(Profile::Polyline(
            "p",
            &[
                0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, 0.0, 1.0, 8.0, 0.0, 1.0, 8.0, 0.0, 4.0, 8.0,
                0.0, 7.0, 7.0, 0.0, 7.0, 7.0, 0.0, 4.0, 7.0, 0.0, 1.0, 3.0, 0.0, 1.0, 3.0, 0.0,
                4.0, 3.0, 0.0, 7.0, 2.0, 0.0, 7.0, 2.0, 0.0, 4.0, 2.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0
            ]
        )),
        Step::Profile(Profile::Plane("f", "p", false)),
        Step::Sweep(Sweep::Prism("s", "f", [0.0, 10.0, 0.0])),
        Step::Topo(Topo::Explode("s", "f")),
        Step::Offset(Offset::Parameter(1e-07, true, 'i', true)),
        Step::Offset(Offset::Load("s", 0.0, &[])),
        Step::Offset(Offset::OnFace(&[("s_7", 3.0)])),
        Step::Offset(Offset::OnFace(&[("s_11", 3.0)])),
        Step::Offset(Offset::Perform("result")),
        Step::More(More::History(HistoryStep::Save("h"))),
        Step::Let("answer_1_isdeleted", Value::Deleted("h", "s_7")),
        Step::If(
            Cond::Holds("answer_1_isdeleted", "Not deleted"),
            &[Step::Puts("Error: History information is wrong")],
            &[]
        ),
        Step::Let("answer_1_isdeleted", Value::Deleted("h", "s_11")),
        Step::If(
            Cond::Holds("answer_1_isdeleted", "Not deleted"),
            &[Step::Puts("Error: History information is wrong")],
            &[]
        ),
        Step::Topo(Topo::Explode("s_7", "e")),
        Step::More(More::History(HistoryStep::Generated("ge1", "h", "s_7_2"))),
        Step::Check(Check::CountsNamed(
            "ge1",
            &[("FACE", 2)],
            "History information"
        )),
        Step::Topo(Topo::Explode("s_11", "e")),
        Step::More(More::History(HistoryStep::Generated("ge2", "h", "s_11_1"))),
        Step::Check(Check::CountsNamed(
            "ge2",
            &[("FACE", 2)],
            "History information"
        )),
        Step::More(More::Compound(&["ge1", "ge2"], "ge")),
        Step::Check(Check::CountsNamed(
            "ge",
            &[("FACE", 3)],
            "History information"
        )),
    ]
);
