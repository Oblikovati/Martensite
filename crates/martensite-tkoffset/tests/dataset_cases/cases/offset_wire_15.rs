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
    offset_wire_closed_outside_0_075_a4,
    "tests/offset/wire_closed_outside_0_075/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(347.74),
        vertices: Some(31),
        edges: Some(31),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_005.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_a5,
    "tests/offset/wire_closed_outside_0_075/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(240.315),
        vertices: Some(32),
        edges: Some(32),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_006.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_a6,
    "tests/offset/wire_closed_outside_0_075/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(369.4),
        vertices: Some(36),
        edges: Some(36),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_007.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_a7,
    "tests/offset/wire_closed_outside_0_075/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(2936.76),
        vertices: Some(17),
        edges: Some(17),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_008.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_a8,
    "tests/offset/wire_closed_outside_0_075/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(83.2778),
        vertices: Some(16),
        edges: Some(16),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_012.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_a9,
    "tests/offset/wire_closed_outside_0_075/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(2701.11),
        vertices: Some(15),
        edges: Some(15),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_013.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b1,
    "tests/offset/wire_closed_outside_0_075/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1179.46),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_014.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b2,
    "tests/offset/wire_closed_outside_0_075/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1069.89),
        vertices: Some(41),
        edges: Some(41),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_015.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b3,
    "tests/offset/wire_closed_outside_0_075/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(189.244),
        vertices: Some(7),
        edges: Some(7),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_017.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b4,
    "tests/offset/wire_closed_outside_0_075/B4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b6,
    "tests/offset/wire_closed_outside_0_075/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(286.312),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_020.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b7,
    "tests/offset/wire_closed_outside_0_075/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(200.958),
        vertices: Some(26),
        edges: Some(26),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_021.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b8,
    "tests/offset/wire_closed_outside_0_075/B8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(94.6562),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_023.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_b9,
    "tests/offset/wire_closed_outside_0_075/B9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(3285.91),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_025.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c1,
    "tests/offset/wire_closed_outside_0_075/C1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(2529.44),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_028.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c2,
    "tests/offset/wire_closed_outside_0_075/C2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1586.48),
        vertices: Some(3),
        edges: Some(3),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_029.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c3,
    "tests/offset/wire_closed_outside_0_075/C3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_030.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c4,
    "tests/offset/wire_closed_outside_0_075/C4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_031.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c5,
    "tests/offset/wire_closed_outside_0_075/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(6.57079),
        vertices: Some(5),
        edges: Some(5),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_036.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c6,
    "tests/offset/wire_closed_outside_0_075/C6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1406.21),
        vertices: Some(38),
        edges: Some(38),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Puts("TODO 23068 ALL: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_037.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c7,
    "tests/offset/wire_closed_outside_0_075/C7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(639.113),
        vertices: Some(67),
        edges: Some(67),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_038.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c8,
    "tests/offset/wire_closed_outside_0_075/C8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(280.089),
        vertices: Some(7),
        edges: Some(7),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_039.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_c9,
    "tests/offset/wire_closed_outside_0_075/C9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_040.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d1,
    "tests/offset/wire_closed_outside_0_075/D1",
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
        Step::Topo(Topo::Restore("brep/offset_wire_041.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d2,
    "tests/offset/wire_closed_outside_0_075/D2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(390.158),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : big tolerance of shape result"),
        Step::Topo(Topo::Restore("brep/offset_wire_042.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d3,
    "tests/offset/wire_closed_outside_0_075/D3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_043.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d4,
    "tests/offset/wire_closed_outside_0_075/D4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(47.1885),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_044.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d5,
    "tests/offset/wire_closed_outside_0_075/D5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(92.4272),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_045.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d6,
    "tests/offset/wire_closed_outside_0_075/D6",
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
        Step::Topo(Topo::Restore("brep/offset_wire_046.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_d7,
    "tests/offset/wire_closed_outside_0_075/D7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(158.406),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_047.brep", "s")),
    ]
);
