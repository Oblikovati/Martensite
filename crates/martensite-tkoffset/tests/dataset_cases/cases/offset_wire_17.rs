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
    offset_wire_closed_outside_0_075_h6,
    "tests/offset/wire_closed_outside_0_075/H6",
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
    offset_wire_closed_outside_0_075_h7,
    "tests/offset/wire_closed_outside_0_075/H7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(162.676),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_outside_0_075_h8,
    "tests/offset/wire_closed_outside_0_075/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(259.689),
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
    offset_wire_closed_outside_0_075_h9,
    "tests/offset/wire_closed_outside_0_075/H9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(288.192),
        vertices: Some(14),
        edges: Some(14),
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
    offset_wire_closed_outside_0_075_i1,
    "tests/offset/wire_closed_outside_0_075/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(3858.73),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_outside_0_075_i2,
    "tests/offset/wire_closed_outside_0_075/I2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1831.24),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_i3,
    "tests/offset/wire_closed_outside_0_075/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(332.67),
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
    offset_wire_closed_outside_0_075_i4,
    "tests/offset/wire_closed_outside_0_075/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(406.896),
        vertices: Some(14),
        edges: Some(14),
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
    offset_wire_closed_outside_0_075_i5,
    "tests/offset/wire_closed_outside_0_075/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(537.618),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_outside_0_075_i6,
    "tests/offset/wire_closed_outside_0_075/I6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(294.241),
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
    offset_wire_closed_outside_0_075_i7,
    "tests/offset/wire_closed_outside_0_075/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(35.2234),
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
    offset_wire_closed_outside_0_075_i8,
    "tests/offset/wire_closed_outside_0_075/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(459.629),
        vertices: Some(18),
        edges: Some(18),
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
    offset_wire_closed_outside_0_075_i9,
    "tests/offset/wire_closed_outside_0_075/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1163.54),
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
    offset_wire_closed_outside_0_075_j1,
    "tests/offset/wire_closed_outside_0_075/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1191.76),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_outside_0_075_j2,
    "tests/offset/wire_closed_outside_0_075/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1232.45),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_outside_0_075_j3,
    "tests/offset/wire_closed_outside_0_075/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(475.251),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_j4,
    "tests/offset/wire_closed_outside_0_075/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(487.907),
        vertices: Some(12),
        edges: Some(12),
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
    offset_wire_closed_outside_0_075_j5,
    "tests/offset/wire_closed_outside_0_075/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(375.566),
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
    offset_wire_closed_outside_0_075_j6,
    "tests/offset/wire_closed_outside_0_075/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1879.92),
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
    offset_wire_closed_outside_0_075_j7,
    "tests/offset/wire_closed_outside_0_075/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(217.043),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_outside_0_075_j9,
    "tests/offset/wire_closed_outside_0_075/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1502.84),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_k1,
    "tests/offset/wire_closed_outside_0_075/K1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(530.167),
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
    offset_wire_closed_outside_0_075_k2,
    "tests/offset/wire_closed_outside_0_075/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(373.535),
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
    offset_wire_unclosed_outside_0_005_a1,
    "tests/offset/wire_unclosed_outside_0_005/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(8.78837),
        vertices: Some(37),
        edges: Some(37),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23068 ALL: Error : The resulting shape is WRONG"),
        Step::Topo(Topo::Restore("brep/offset_wire_001.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_a2,
    "tests/offset/wire_unclosed_outside_0_005/A2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(522.697),
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
    offset_wire_unclosed_outside_0_005_a3,
    "tests/offset/wire_unclosed_outside_0_005/A3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(522.697),
        vertices: Some(22),
        edges: Some(22),
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
    offset_wire_unclosed_outside_0_005_a4,
    "tests/offset/wire_unclosed_outside_0_005/A4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_011.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_a5,
    "tests/offset/wire_unclosed_outside_0_005/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(30.144),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_unclosed_outside_0_005_a6,
    "tests/offset/wire_unclosed_outside_0_005/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(234.171),
        vertices: Some(52),
        edges: Some(52),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_022.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_005_a9,
    "tests/offset/wire_unclosed_outside_0_005/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(63842.4),
        vertices: Some(220),
        edges: Some(220),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_027.brep", "s")),
    ]
);
