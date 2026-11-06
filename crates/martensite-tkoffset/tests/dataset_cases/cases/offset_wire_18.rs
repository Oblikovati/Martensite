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
    offset_wire_unclosed_outside_0_005_b1,
    "tests/offset/wire_unclosed_outside_0_005/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_032.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b2,
    "tests/offset/wire_unclosed_outside_0_005/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_033.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b3,
    "tests/offset/wire_unclosed_outside_0_005/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(910.505),
        vertices: Some(14),
        edges: Some(14),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 25124068 ALL: Error : big tolerance of shape result"),
        Step::Topo(Topo::Restore("brep/offset_wire_034.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b4,
    "tests/offset/wire_unclosed_outside_0_005/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(910.505),
        vertices: Some(30),
        edges: Some(30),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/case_3_wire2.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b5,
    "tests/offset/wire_unclosed_outside_0_005/B5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(371.303),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_053.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b6,
    "tests/offset/wire_unclosed_outside_0_005/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(3316.27),
        vertices: Some(62),
        edges: Some(62),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_058.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_b7,
    "tests/offset/wire_unclosed_outside_0_005/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(888.512),
        vertices: Some(22),
        edges: Some(22),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_069.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_c4,
    "tests/offset/wire_unclosed_outside_0_005/C4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(308.604),
        vertices: Some(84),
        edges: Some(84),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_075.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_c5,
    "tests/offset/wire_unclosed_outside_0_005/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_076.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a2,
    "tests/offset/wire_unclosed_outside_0_025/A2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(554.733),
        vertices: Some(52),
        edges: Some(52),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_009.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a3,
    "tests/offset/wire_unclosed_outside_0_025/A3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(554.716),
        vertices: Some(34),
        edges: Some(34),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_010.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a4,
    "tests/offset/wire_unclosed_outside_0_025/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_011.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a5,
    "tests/offset/wire_unclosed_outside_0_025/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(31.6702),
        vertices: Some(14),
        edges: Some(14),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_016.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a6,
    "tests/offset/wire_unclosed_outside_0_025/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(224.208),
        vertices: Some(49),
        edges: Some(49),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_022.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_a9,
    "tests/offset/wire_unclosed_outside_0_025/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(67864.4),
        vertices: Some(228),
        edges: Some(228),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_027.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b1,
    "tests/offset/wire_unclosed_outside_0_025/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_032.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b2,
    "tests/offset/wire_unclosed_outside_0_025/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(68.7414),
        vertices: Some(61),
        edges: Some(61),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_033.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b3,
    "tests/offset/wire_unclosed_outside_0_025/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(738.873),
        vertices: Some(306),
        edges: Some(306),
        wires: Some(5)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_034.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b4,
    "tests/offset/wire_unclosed_outside_0_025/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(946.063),
        vertices: Some(79),
        edges: Some(79),
        wires: Some(2)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 31005 ALL: Faulty shapes in variables"),
        Step::Puts("TODO 31005 ALL: Error : The length of result shape is"),
        Step::Puts("TODO 31005 ALL: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/case_3_wire2.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b5,
    "tests/offset/wire_unclosed_outside_0_025/B5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(394.271),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_053.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b6,
    "tests/offset/wire_unclosed_outside_0_025/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(2582.56),
        vertices: Some(47),
        edges: Some(47),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_058.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_b7,
    "tests/offset/wire_unclosed_outside_0_025/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(943.475),
        vertices: Some(22),
        edges: Some(22),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_069.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_c4,
    "tests/offset/wire_unclosed_outside_0_025/C4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(326.251),
        vertices: Some(100),
        edges: Some(100),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_075.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_025_c5,
    "tests/offset/wire_unclosed_outside_0_025/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_076.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a1,
    "tests/offset/wire_unclosed_outside_0_075/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(11.6898),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Puts("TODO 24156 MacOS: Error : The length of result shape is"),
        Step::Puts("TODO 24156 MacOS: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_001.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a2,
    "tests/offset/wire_unclosed_outside_0_075/A2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(634.956),
        vertices: Some(49),
        edges: Some(49),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_009.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a3,
    "tests/offset/wire_unclosed_outside_0_075/A3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(634.927),
        vertices: Some(25),
        edges: Some(25),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_010.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a4,
    "tests/offset/wire_unclosed_outside_0_075/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_011.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a5,
    "tests/offset/wire_unclosed_outside_0_075/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(36.0211),
        vertices: Some(14),
        edges: Some(14),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_016.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_a6,
    "tests/offset/wire_unclosed_outside_0_075/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(215.745),
        vertices: Some(39),
        edges: Some(39),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_022.brep", "s")),
    ]
);
