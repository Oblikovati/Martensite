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
    offset_wire_closed_inside_0_005_h6,
    "tests/offset/wire_closed_inside_0_005/H6",
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
    offset_wire_closed_inside_0_005_h7,
    "tests/offset/wire_closed_inside_0_005/H7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(109.379),
        vertices: Some(14),
        edges: Some(14),
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
    offset_wire_closed_inside_0_005_h8,
    "tests/offset/wire_closed_inside_0_005/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(170.965),
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
    offset_wire_closed_inside_0_005_h9,
    "tests/offset/wire_closed_inside_0_005/H9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(231.555),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_inside_0_005_i1,
    "tests/offset/wire_closed_inside_0_005/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(2655.76),
        vertices: Some(10),
        edges: Some(10),
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
    offset_wire_closed_inside_0_005_i2,
    "tests/offset/wire_closed_inside_0_005/I2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1217.27),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_005_i3,
    "tests/offset/wire_closed_inside_0_005/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(218.917),
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
    offset_wire_closed_inside_0_005_i4,
    "tests/offset/wire_closed_inside_0_005/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(283.282),
        vertices: Some(32),
        edges: Some(32),
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
    offset_wire_closed_inside_0_005_i5,
    "tests/offset/wire_closed_inside_0_005/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(358.205),
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
    offset_wire_closed_inside_0_005_i6,
    "tests/offset/wire_closed_inside_0_005/I6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(191.974),
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
    offset_wire_closed_inside_0_005_i7,
    "tests/offset/wire_closed_inside_0_005/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(23.4597),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_inside_0_005_i8,
    "tests/offset/wire_closed_inside_0_005/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(316.751),
        vertices: Some(43),
        edges: Some(43),
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
    offset_wire_closed_inside_0_005_i9,
    "tests/offset/wire_closed_inside_0_005/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(770.627),
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
    offset_wire_closed_inside_0_005_j1,
    "tests/offset/wire_closed_inside_0_005/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(782.13),
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
    offset_wire_closed_inside_0_005_j2,
    "tests/offset/wire_closed_inside_0_005/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(827.017),
        vertices: Some(23),
        edges: Some(23),
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
    offset_wire_closed_inside_0_005_j3,
    "tests/offset/wire_closed_inside_0_005/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(314.489),
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
    offset_wire_closed_inside_0_005_j4,
    "tests/offset/wire_closed_inside_0_005/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(320.973),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_inside_0_005_j5,
    "tests/offset/wire_closed_inside_0_005/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(246.044),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_005_j6,
    "tests/offset/wire_closed_inside_0_005/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1239.19),
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
    offset_wire_closed_inside_0_005_j7,
    "tests/offset/wire_closed_inside_0_005/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(150.604),
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
    offset_wire_closed_inside_0_005_j9,
    "tests/offset/wire_closed_inside_0_005/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1125.12),
        vertices: Some(22),
        edges: Some(22),
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
    offset_wire_closed_inside_0_005_k1,
    "tests/offset/wire_closed_inside_0_005/K1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(357.293),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_inside_0_005_k2,
    "tests/offset/wire_closed_inside_0_005/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(246.976),
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
    offset_wire_closed_inside_0_025_a1,
    "tests/offset/wire_closed_inside_0_025/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(195.472),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_025_a4,
    "tests/offset/wire_closed_inside_0_025/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(166.151),
        vertices: Some(21),
        edges: Some(21),
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
    offset_wire_closed_inside_0_025_a5,
    "tests/offset/wire_closed_inside_0_025/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(110.349),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_inside_0_025_a6,
    "tests/offset/wire_closed_inside_0_025/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(206.188),
        vertices: Some(27),
        edges: Some(27),
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
    offset_wire_closed_inside_0_025_a7,
    "tests/offset/wire_closed_inside_0_025/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1296.92),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_008.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_a8,
    "tests/offset/wire_closed_inside_0_025/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(32.9112),
        vertices: Some(13),
        edges: Some(13),
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
    offset_wire_closed_inside_0_025_a9,
    "tests/offset/wire_closed_inside_0_025/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1384.33),
        vertices: Some(16),
        edges: Some(16),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_013.brep", "s")),
    ]
);
