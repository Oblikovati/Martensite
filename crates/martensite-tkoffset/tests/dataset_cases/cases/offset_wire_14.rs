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
    offset_wire_closed_outside_0_025_g9,
    "tests/offset/wire_closed_outside_0_025/G9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1554.92),
        vertices: Some(31),
        edges: Some(31),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_086.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h1,
    "tests/offset/wire_closed_outside_0_025/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(32.2304),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_087.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h2,
    "tests/offset/wire_closed_outside_0_025/H2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(78.848),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_088.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h3,
    "tests/offset/wire_closed_outside_0_025/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1072.94),
        vertices: Some(25),
        edges: Some(25),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_089.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h4,
    "tests/offset/wire_closed_outside_0_025/H4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(392.505),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_090.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h5,
    "tests/offset/wire_closed_outside_0_025/H5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(25.2204),
        vertices: Some(17),
        edges: Some(17),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_091.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h6,
    "tests/offset/wire_closed_outside_0_025/H6",
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
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts(
            "Tested shape has no Face boundaries (all Edges have TopAbs_INTERNAL flag), therefore offset will NOT be created"
        ),
        Step::Puts("REQUIRED ALL: Error : The offset cannot be built."),
        Step::Puts("REQUIRED ALL: Error: The command cannot be built"),
        Step::Puts("REQUIRED ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_092.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h7,
    "tests/offset/wire_closed_outside_0_025/H7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(130.857),
        vertices: Some(22),
        edges: Some(22),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_093.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h8,
    "tests/offset/wire_closed_outside_0_025/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(204.236),
        vertices: Some(8),
        edges: Some(8),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_094.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_h9,
    "tests/offset/wire_closed_outside_0_025/H9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(275.765),
        vertices: Some(21),
        edges: Some(21),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_095.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_i1,
    "tests/offset/wire_closed_outside_0_025/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(3079.65),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_096.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_i2,
    "tests/offset/wire_closed_outside_0_025/I2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1467.11),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_097.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_i3,
    "tests/offset/wire_closed_outside_0_025/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(261.634),
        vertices: Some(18),
        edges: Some(18),
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
    offset_wire_closed_outside_0_025_i4,
    "tests/offset/wire_closed_outside_0_025/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(325.844),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 MacOS: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_099.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_i5,
    "tests/offset/wire_closed_outside_0_025/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(427.915),
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
    offset_wire_closed_outside_0_025_i6,
    "tests/offset/wire_closed_outside_0_025/I6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(231.41),
        vertices: Some(12),
        edges: Some(12),
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
    offset_wire_closed_outside_0_025_i7,
    "tests/offset/wire_closed_outside_0_025/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(27.7817),
        vertices: Some(14),
        edges: Some(14),
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
    offset_wire_closed_outside_0_025_i8,
    "tests/offset/wire_closed_outside_0_025/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(372.476),
        vertices: Some(30),
        edges: Some(30),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_103.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_025_i9,
    "tests/offset/wire_closed_outside_0_025/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(916.739),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_outside_0_025_j1,
    "tests/offset/wire_closed_outside_0_025/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(938.45),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_outside_0_025_j2,
    "tests/offset/wire_closed_outside_0_025/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(977.041),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_outside_0_025_j3,
    "tests/offset/wire_closed_outside_0_025/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(375.691),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_outside_0_025_j4,
    "tests/offset/wire_closed_outside_0_025/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(385.144),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_outside_0_025_j5,
    "tests/offset/wire_closed_outside_0_025/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(295.37),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_outside_0_025_j6,
    "tests/offset/wire_closed_outside_0_025/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1479.16),
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
    offset_wire_closed_outside_0_025_j7,
    "tests/offset/wire_closed_outside_0_025/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(174.262),
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
    offset_wire_closed_outside_0_025_j9,
    "tests/offset/wire_closed_outside_0_025/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(1202.97),
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
    offset_wire_closed_outside_0_025_k1,
    "tests/offset/wire_closed_outside_0_025/K1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(426.989),
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
    offset_wire_closed_outside_0_025_k2,
    "tests/offset/wire_closed_outside_0_025/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.025,
        length: Some(296.49),
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
    offset_wire_closed_outside_0_075_a1,
    "tests/offset/wire_closed_outside_0_075/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(356.468),
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
