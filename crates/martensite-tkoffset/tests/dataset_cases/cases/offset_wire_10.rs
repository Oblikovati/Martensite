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
    offset_wire_closed_outside_0_005_c5,
    "tests/offset/wire_closed_outside_0_005/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(4.60678),
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
    offset_wire_closed_outside_0_005_c6,
    "tests/offset/wire_closed_outside_0_005/C6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(985.83),
        vertices: Some(39),
        edges: Some(39),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Topo(Topo::Restore("brep/offset_wire_037.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_c7,
    "tests/offset/wire_closed_outside_0_005/C7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(448.052),
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
    offset_wire_closed_outside_0_005_c8,
    "tests/offset/wire_closed_outside_0_005/C8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(201.174),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_005_c9,
    "tests/offset/wire_closed_outside_0_005/C9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_040.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d1,
    "tests/offset/wire_closed_outside_0_005/D1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(3536.16),
        vertices: Some(621),
        edges: Some(621),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 31005 ALL: Error: Offset is not done"),
        Step::Puts("TODO 31005 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 31005 ALL: Error : The offset cannot be built"),
        Step::Topo(Topo::Restore("brep/offset_wire_041.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d2,
    "tests/offset/wire_closed_outside_0_005/D2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(273.521),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_042.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d3,
    "tests/offset/wire_closed_outside_0_005/D3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_043.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d4,
    "tests/offset/wire_closed_outside_0_005/D4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(33.0816),
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
    offset_wire_closed_outside_0_005_d5,
    "tests/offset/wire_closed_outside_0_005/D5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(64.8375),
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
    offset_wire_closed_outside_0_005_d6,
    "tests/offset/wire_closed_outside_0_005/D6",
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
        Step::Topo(Topo::Restore("brep/offset_wire_046.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d7,
    "tests/offset/wire_closed_outside_0_005/D7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(111.051),
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

offset_case!(
    offset_wire_closed_outside_0_005_d8,
    "tests/offset/wire_closed_outside_0_005/D8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(209.015),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_048.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_d9,
    "tests/offset/wire_closed_outside_0_005/D9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(32.5937),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_049.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e1,
    "tests/offset/wire_closed_outside_0_005/E1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(6.4907),
        vertices: Some(17),
        edges: Some(17),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_050.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e2,
    "tests/offset/wire_closed_outside_0_005/E2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(36.8298),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_051.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e3,
    "tests/offset/wire_closed_outside_0_005/E3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(74.8316),
        vertices: Some(18),
        edges: Some(18),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_052.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e4,
    "tests/offset/wire_closed_outside_0_005/E4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(202.645),
        vertices: Some(42),
        edges: Some(42),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_054.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e5,
    "tests/offset/wire_closed_outside_0_005/E5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(526.412),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_055.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e6,
    "tests/offset/wire_closed_outside_0_005/E6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1229.28),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : big tolerance of shape result"),
        Step::Topo(Topo::Restore("brep/offset_wire_056.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e7,
    "tests/offset/wire_closed_outside_0_005/E7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1033.04),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_057.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_e9,
    "tests/offset/wire_closed_outside_0_005/E9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(15.7022),
        vertices: Some(32),
        edges: Some(32),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_060.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f1,
    "tests/offset/wire_closed_outside_0_005/F1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(2197.63),
        vertices: Some(29),
        edges: Some(29),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_061.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f2,
    "tests/offset/wire_closed_outside_0_005/F2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(436.194),
        vertices: Some(8),
        edges: Some(8),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_062.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f3,
    "tests/offset/wire_closed_outside_0_005/F3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(573.077),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_063.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f4,
    "tests/offset/wire_closed_outside_0_005/F4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1005.58),
        vertices: Some(11),
        edges: Some(11),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_064.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f5,
    "tests/offset/wire_closed_outside_0_005/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(457.109),
        vertices: Some(45),
        edges: Some(45),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_065.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f6,
    "tests/offset/wire_closed_outside_0_005/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(94.2743),
        vertices: Some(80),
        edges: Some(80),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_066.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_f9,
    "tests/offset/wire_closed_outside_0_005/F9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_077.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_g1,
    "tests/offset/wire_closed_outside_0_005/G1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(488.408),
        vertices: Some(73),
        edges: Some(73),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_078.brep", "s")),
    ]
);
