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
    offset_wire_closed_outside_0_025_d2,
    "tests/offset/wire_closed_outside_0_025/D2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(306.846),
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
    offset_wire_closed_outside_0_025_d3,
    "tests/offset/wire_closed_outside_0_025/D3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_043.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_d4,
    "tests/offset/wire_closed_outside_0_025/D4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(37.1121),
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
    offset_wire_closed_outside_0_025_d5,
    "tests/offset/wire_closed_outside_0_025/D5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(72.7148),
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
    offset_wire_closed_outside_0_025_d6,
    "tests/offset/wire_closed_outside_0_025/D6",
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
        Step::Topo(Topo::Restore("brep/offset_wire_046.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_d7,
    "tests/offset/wire_closed_outside_0_025/D7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(124.581),
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
    offset_wire_closed_outside_0_025_d8,
    "tests/offset/wire_closed_outside_0_025/D8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(234.48),
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
    offset_wire_closed_outside_0_025_d9,
    "tests/offset/wire_closed_outside_0_025/D9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(36.5648),
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
    offset_wire_closed_outside_0_025_e1,
    "tests/offset/wire_closed_outside_0_025/E1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(7.28151),
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
    offset_wire_closed_outside_0_025_e2,
    "tests/offset/wire_closed_outside_0_025/E2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(41.317),
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
    offset_wire_closed_outside_0_025_e3,
    "tests/offset/wire_closed_outside_0_025/E3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(83.3437),
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
    offset_wire_closed_outside_0_025_e4,
    "tests/offset/wire_closed_outside_0_025/E4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(227.334),
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
    offset_wire_closed_outside_0_025_e5,
    "tests/offset/wire_closed_outside_0_025/E5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(590.548),
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
    offset_wire_closed_outside_0_025_e6,
    "tests/offset/wire_closed_outside_0_025/E6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1377.49),
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
    offset_wire_closed_outside_0_025_e7,
    "tests/offset/wire_closed_outside_0_025/E7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(979.315),
        vertices: Some(11),
        edges: Some(11),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_057.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_e9,
    "tests/offset/wire_closed_outside_0_025/E9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(17.6153),
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
    offset_wire_closed_outside_0_025_f1,
    "tests/offset/wire_closed_outside_0_025/F1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(2175.16),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_outside_0_025_f2,
    "tests/offset/wire_closed_outside_0_025/F2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(487.352),
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
    offset_wire_closed_outside_0_025_f3,
    "tests/offset/wire_closed_outside_0_025/F3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(642.737),
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
    offset_wire_closed_outside_0_025_f4,
    "tests/offset/wire_closed_outside_0_025/F4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1128.09),
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
    offset_wire_closed_outside_0_025_f5,
    "tests/offset/wire_closed_outside_0_025/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(505.248),
        vertices: Some(35),
        edges: Some(35),
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
    offset_wire_closed_outside_0_025_f6,
    "tests/offset/wire_closed_outside_0_025/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(105.76),
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
    offset_wire_closed_outside_0_025_f9,
    "tests/offset/wire_closed_outside_0_025/F9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_077.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g1,
    "tests/offset/wire_closed_outside_0_025/G1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(547.534),
        vertices: Some(53),
        edges: Some(53),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_078.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g2,
    "tests/offset/wire_closed_outside_0_025/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(124.878),
        vertices: Some(26),
        edges: Some(26),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_079.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g3,
    "tests/offset/wire_closed_outside_0_025/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(159.24),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_080.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g4,
    "tests/offset/wire_closed_outside_0_025/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(433.038),
        vertices: Some(42),
        edges: Some(42),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_081.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g5,
    "tests/offset/wire_closed_outside_0_025/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1201.05),
        vertices: Some(27),
        edges: Some(27),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_082.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g6,
    "tests/offset/wire_closed_outside_0_025/G6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(965.993),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_083.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_g8,
    "tests/offset/wire_closed_outside_0_025/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(309.271),
        vertices: Some(19),
        edges: Some(19),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_085.brep", "s")),
    ]
);
