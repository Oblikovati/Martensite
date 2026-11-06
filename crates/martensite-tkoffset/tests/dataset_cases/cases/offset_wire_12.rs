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
    offset_wire_closed_outside_0_005_j6,
    "tests/offset/wire_closed_outside_0_005/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1319.29),
        vertices: Some(3),
        edges: Some(3),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_110.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_j7,
    "tests/offset/wire_closed_outside_0_005/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(159.034),
        vertices: Some(17),
        edges: Some(17),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_111.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_j9,
    "tests/offset/wire_closed_outside_0_005/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1198.11),
        vertices: Some(21),
        edges: Some(21),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_113.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_k1,
    "tests/offset/wire_closed_outside_0_005/K1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(380.616),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_114.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_k2,
    "tests/offset/wire_closed_outside_0_005/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(264.29),
        vertices: Some(8),
        edges: Some(8),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_115.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_a1,
    "tests/offset/wire_closed_outside_0_025/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(280.35),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_002.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_a4,
    "tests/offset/wire_closed_outside_0_025/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(273.613),
        vertices: Some(35),
        edges: Some(35),
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
    offset_wire_closed_outside_0_025_a5,
    "tests/offset/wire_closed_outside_0_025/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(189.027),
        vertices: Some(33),
        edges: Some(33),
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
    offset_wire_closed_outside_0_025_a6,
    "tests/offset/wire_closed_outside_0_025/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(290.521),
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
    offset_wire_closed_outside_0_025_a7,
    "tests/offset/wire_closed_outside_0_025/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(2348.66),
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
    offset_wire_closed_outside_0_025_a8,
    "tests/offset/wire_closed_outside_0_025/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(65.4952),
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
    offset_wire_closed_outside_0_025_a9,
    "tests/offset/wire_closed_outside_0_025/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(2218.86),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_outside_0_025_b1,
    "tests/offset/wire_closed_outside_0_025/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(927.609),
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
    offset_wire_closed_outside_0_025_b2,
    "tests/offset/wire_closed_outside_0_025/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(838.161),
        vertices: Some(55),
        edges: Some(55),
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
    offset_wire_closed_outside_0_025_b3,
    "tests/offset/wire_closed_outside_0_025/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(148.835),
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
    offset_wire_closed_outside_0_025_b4,
    "tests/offset/wire_closed_outside_0_025/B4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_b6,
    "tests/offset/wire_closed_outside_0_025/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(230.563),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_outside_0_025_b7,
    "tests/offset/wire_closed_outside_0_025/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(164.679),
        vertices: Some(30),
        edges: Some(30),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_021.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_b8,
    "tests/offset/wire_closed_outside_0_025/B8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(74.7711),
        vertices: Some(13),
        edges: Some(13),
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
    offset_wire_closed_outside_0_025_b9,
    "tests/offset/wire_closed_outside_0_025/B9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(2565.1),
        vertices: Some(13),
        edges: Some(13),
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
    offset_wire_closed_outside_0_025_c1,
    "tests/offset/wire_closed_outside_0_025/C1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1991.59),
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
    offset_wire_closed_outside_0_025_c2,
    "tests/offset/wire_closed_outside_0_025/C2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1252.05),
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
    offset_wire_closed_outside_0_025_c3,
    "tests/offset/wire_closed_outside_0_025/C3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_030.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_c4,
    "tests/offset/wire_closed_outside_0_025/C4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_031.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_c5,
    "tests/offset/wire_closed_outside_0_025/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(5.16793),
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
    offset_wire_closed_outside_0_025_c6,
    "tests/offset/wire_closed_outside_0_025/C6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1105.94),
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
    offset_wire_closed_outside_0_025_c7,
    "tests/offset/wire_closed_outside_0_025/C7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(502.641),
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
    offset_wire_closed_outside_0_025_c8,
    "tests/offset/wire_closed_outside_0_025/C8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(221.429),
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
    offset_wire_closed_outside_0_025_c9,
    "tests/offset/wire_closed_outside_0_025/C9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_040.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_d1,
    "tests/offset/wire_closed_outside_0_025/D1",
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
        Step::Topo(Topo::Restore("brep/offset_wire_041.brep", "s")),
    ]
);
