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
    offset_wire_closed_inside_0_025_b1,
    "tests/offset/wire_closed_inside_0_025/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(663.664),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_025_b2,
    "tests/offset/wire_closed_inside_0_025/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(549.222),
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
    offset_wire_closed_inside_0_025_b3,
    "tests/offset/wire_closed_inside_0_025/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(93.864),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_025_b4,
    "tests/offset/wire_closed_inside_0_025/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(23.8673),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23068 ALL: Error : result_1 is NOT a closed wire"),
        Step::Puts("TODO 23068 ALL: Error : The length of result shape is"),
        Step::Puts("TODO 23068 ALL: Error : The resulting shape is WRONG because it must contain"),
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_b6,
    "tests/offset/wire_closed_inside_0_025/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(122.02),
        vertices: Some(11),
        edges: Some(11),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_020.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_b7,
    "tests/offset/wire_closed_inside_0_025/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
        Step::Topo(Topo::Restore("brep/offset_wire_021.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_b8,
    "tests/offset/wire_closed_inside_0_025/B8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(51.3716),
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
    offset_wire_closed_inside_0_025_b9,
    "tests/offset/wire_closed_inside_0_025/B9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1809.82),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_025_c1,
    "tests/offset/wire_closed_inside_0_025/C1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1395.61),
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
    offset_wire_closed_inside_0_025_c2,
    "tests/offset/wire_closed_inside_0_025/C2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(923.104),
        vertices: Some(7),
        edges: Some(7),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_029.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_c3,
    "tests/offset/wire_closed_inside_0_025/C3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
    offset_wire_closed_inside_0_025_c4,
    "tests/offset/wire_closed_inside_0_025/C4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
    offset_wire_closed_inside_0_025_c5,
    "tests/offset/wire_closed_inside_0_025/C5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(3.76506),
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
    offset_wire_closed_inside_0_025_c6,
    "tests/offset/wire_closed_inside_0_025/C6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(805.665),
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
    offset_wire_closed_inside_0_025_c7,
    "tests/offset/wire_closed_inside_0_025/C7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(365.164),
        vertices: Some(59),
        edges: Some(59),
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
    offset_wire_closed_inside_0_025_c8,
    "tests/offset/wire_closed_inside_0_025/C8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(158.841),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_inside_0_025_c9,
    "tests/offset/wire_closed_inside_0_025/C9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
        Step::Topo(Topo::Restore("brep/offset_wire_040.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_d1,
    "tests/offset/wire_closed_inside_0_025/D1",
    Shape,
    1000,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_041.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_d2,
    "tests/offset/wire_closed_inside_0_025/D2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(223.18),
        vertices: Some(27),
        edges: Some(27),
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
    offset_wire_closed_inside_0_025_d3,
    "tests/offset/wire_closed_inside_0_025/D3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
    offset_wire_closed_inside_0_025_d4,
    "tests/offset/wire_closed_inside_0_025/D4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(26.3475),
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
    offset_wire_closed_inside_0_025_d5,
    "tests/offset/wire_closed_inside_0_025/D5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(50.2954),
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
    offset_wire_closed_inside_0_025_d6,
    "tests/offset/wire_closed_inside_0_025/D6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
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
    offset_wire_closed_inside_0_025_d7,
    "tests/offset/wire_closed_inside_0_025/D7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(69.77),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_025_d8,
    "tests/offset/wire_closed_inside_0_025/D8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: None,
        vertices: Some(0),
        edges: Some(0),
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
    offset_wire_closed_inside_0_025_d9,
    "tests/offset/wire_closed_inside_0_025/D9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(14.5369),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_025_e1,
    "tests/offset/wire_closed_inside_0_025/E1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1.61237),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_025_e2,
    "tests/offset/wire_closed_inside_0_025/E2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(10.7233),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_025_e3,
    "tests/offset/wire_closed_inside_0_025/E3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(42.1732),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_052.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_e4,
    "tests/offset/wire_closed_inside_0_025/E4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(165.611),
        vertices: Some(41),
        edges: Some(41),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_054.brep", "s")),
    ]
);
