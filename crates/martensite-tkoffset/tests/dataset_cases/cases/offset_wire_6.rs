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
    offset_wire_closed_inside_0_025_i3,
    "tests/offset/wire_closed_inside_0_025/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(190.049),
        vertices: Some(14),
        edges: Some(14),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_098.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i4,
    "tests/offset/wire_closed_inside_0_025/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(211.408),
        vertices: Some(18),
        edges: Some(18),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_099.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i5,
    "tests/offset/wire_closed_inside_0_025/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(311.731),
        vertices: Some(16),
        edges: Some(16),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_100.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i6,
    "tests/offset/wire_closed_inside_0_025/I6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(159.896),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_101.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i7,
    "tests/offset/wire_closed_inside_0_025/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(20.1722),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_102.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i8,
    "tests/offset/wire_closed_inside_0_025/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(245.13),
        vertices: Some(27),
        edges: Some(27),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24156 MacOS: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_103.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_i9,
    "tests/offset/wire_closed_inside_0_025/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(670.646),
        vertices: Some(8),
        edges: Some(8),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_104.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j1,
    "tests/offset/wire_closed_inside_0_025/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(663.429),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_105.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j2,
    "tests/offset/wire_closed_inside_0_025/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(662.458),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_106.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j3,
    "tests/offset/wire_closed_inside_0_025/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(271.951),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_107.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j4,
    "tests/offset/wire_closed_inside_0_025/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(269.063),
        vertices: Some(10),
        edges: Some(10),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_108.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j5,
    "tests/offset/wire_closed_inside_0_025/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(209.053),
        vertices: Some(5),
        edges: Some(5),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_109.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_j6,
    "tests/offset/wire_closed_inside_0_025/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1078.42),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_025_j7,
    "tests/offset/wire_closed_inside_0_025/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(131.065),
        vertices: Some(25),
        edges: Some(25),
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
    offset_wire_closed_inside_0_025_j9,
    "tests/offset/wire_closed_inside_0_025/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(792.426),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_inside_0_025_k1,
    "tests/offset/wire_closed_inside_0_025/K1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(285.872),
        vertices: Some(11),
        edges: Some(11),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_114.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_k2,
    "tests/offset/wire_closed_inside_0_025/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(212.374),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_075_a1,
    "tests/offset/wire_closed_inside_0_075/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(94.8515),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_075_a4,
    "tests/offset/wire_closed_inside_0_075/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: None,
        vertices: Some(0),
        edges: Some(0),
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
    offset_wire_closed_inside_0_075_a5,
    "tests/offset/wire_closed_inside_0_075/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(16.307),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_075_a6,
    "tests/offset/wire_closed_inside_0_075/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(100.492),
        vertices: Some(10),
        edges: Some(10),
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
    offset_wire_closed_inside_0_075_a7,
    "tests/offset/wire_closed_inside_0_075/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: None,
        vertices: Some(0),
        edges: Some(0),
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
    offset_wire_closed_inside_0_075_a8,
    "tests/offset/wire_closed_inside_0_075/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: None,
        vertices: Some(0),
        edges: Some(0),
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
    offset_wire_closed_inside_0_075_a9,
    "tests/offset/wire_closed_inside_0_075/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(561.731),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_inside_0_075_b1,
    "tests/offset/wire_closed_inside_0_075/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(342.991),
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
    offset_wire_closed_inside_0_075_b2,
    "tests/offset/wire_closed_inside_0_075/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(156.199),
        vertices: Some(21),
        edges: Some(21),
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
    offset_wire_closed_inside_0_075_b3,
    "tests/offset/wire_closed_inside_0_075/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(24.4417),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_075_b4,
    "tests/offset/wire_closed_inside_0_075/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(0.0),
        vertices: Some(0),
        edges: Some(0),
        wires: Some(0)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : result is NOT a closed wire"),
        Step::Puts("TODO 23748 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Puts("TODO 23748 ALL: Error : The length of result shape is"),
        Step::Puts("TODO 23748 ALL: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_b6,
    "tests/offset/wire_closed_inside_0_075/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: None,
        vertices: Some(0),
        edges: Some(0),
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
    offset_wire_closed_inside_0_075_b7,
    "tests/offset/wire_closed_inside_0_075/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
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
