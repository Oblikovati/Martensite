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
    offset_wire_closed_inside_0_005_a4,
    "tests/offset/wire_closed_inside_0_005/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(222.808),
        vertices: Some(26),
        edges: Some(26),
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
    offset_wire_closed_inside_0_005_a5,
    "tests/offset/wire_closed_inside_0_005/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(150.854),
        vertices: Some(22),
        edges: Some(22),
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
    offset_wire_closed_inside_0_005_a6,
    "tests/offset/wire_closed_inside_0_005/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(242.115),
        vertices: Some(30),
        edges: Some(30),
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
    offset_wire_closed_inside_0_005_a7,
    "tests/offset/wire_closed_inside_0_005/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1967.47),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_005_a8,
    "tests/offset/wire_closed_inside_0_005/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(48.826),
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
    offset_wire_closed_inside_0_005_a9,
    "tests/offset/wire_closed_inside_0_005/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1873.7),
        vertices: Some(25),
        edges: Some(25),
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
    offset_wire_closed_inside_0_005_b1,
    "tests/offset/wire_closed_inside_0_005/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(776.496),
        vertices: Some(12),
        edges: Some(12),
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
    offset_wire_closed_inside_0_005_b2,
    "tests/offset/wire_closed_inside_0_005/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(768.162),
        vertices: Some(116),
        edges: Some(116),
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
    offset_wire_closed_inside_0_005_b3,
    "tests/offset/wire_closed_inside_0_005/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(122.21),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_005_b4,
    "tests/offset/wire_closed_inside_0_005/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(24.8301),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error: Offset is not done"),
        Step::Puts("TODO 23068 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23068 ALL: Error : The offset cannot be built"),
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_b6,
    "tests/offset/wire_closed_inside_0_005/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(190.807),
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
    offset_wire_closed_inside_0_005_b7,
    "tests/offset/wire_closed_inside_0_005/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(182.229),
        vertices: Some(44),
        edges: Some(44),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_2"),
        Step::Topo(Topo::Restore("brep/offset_wire_021.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_b8,
    "tests/offset/wire_closed_inside_0_005/B8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(62.3443),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_inside_0_005_b9,
    "tests/offset/wire_closed_inside_0_005/B9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(2209.15),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_inside_0_005_c1,
    "tests/offset/wire_closed_inside_0_005/C1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1657.26),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_005_c2,
    "tests/offset/wire_closed_inside_0_005/C2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1100.32),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_005_c3,
    "tests/offset/wire_closed_inside_0_005/C3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
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
    offset_wire_closed_inside_0_005_c4,
    "tests/offset/wire_closed_inside_0_005/C4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Topo(Topo::Restore("brep/offset_wire_031.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_c5,
    "tests/offset/wire_closed_inside_0_005/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(4.32621),
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
    offset_wire_closed_inside_0_005_c6,
    "tests/offset/wire_closed_inside_0_005/C6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(925.775),
        vertices: Some(38),
        edges: Some(38),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_037.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_c7,
    "tests/offset/wire_closed_inside_0_005/C7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(420.587),
        vertices: Some(63),
        edges: Some(63),
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
    offset_wire_closed_inside_0_005_c8,
    "tests/offset/wire_closed_inside_0_005/C8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(188.897),
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
    offset_wire_closed_inside_0_005_c9,
    "tests/offset/wire_closed_inside_0_005/C9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
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
    offset_wire_closed_inside_0_005_d1,
    "tests/offset/wire_closed_inside_0_005/D1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(3319.85),
        vertices: Some(618),
        edges: Some(618),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24682 ALL: Error: Offset is not done."),
        Step::Puts("TODO 24682 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 24682 ALL: Error : The offset cannot be built."),
        Step::Topo(Topo::Restore("brep/offset_wire_041.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_d2,
    "tests/offset/wire_closed_inside_0_005/D2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(256.859),
        vertices: Some(31),
        edges: Some(31),
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
    offset_wire_closed_inside_0_005_d3,
    "tests/offset/wire_closed_inside_0_005/D3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
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
    offset_wire_closed_inside_0_005_d4,
    "tests/offset/wire_closed_inside_0_005/D4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(30.9287),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_005_d5,
    "tests/offset/wire_closed_inside_0_005/D5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(60.3545),
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
    offset_wire_closed_inside_0_005_d6,
    "tests/offset/wire_closed_inside_0_005/D6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Topo(Topo::Restore("brep/offset_wire_046.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_d7,
    "tests/offset/wire_closed_inside_0_005/D7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(96.3379),
        vertices: Some(8),
        edges: Some(8),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_047.brep", "s")),
    ]
);
