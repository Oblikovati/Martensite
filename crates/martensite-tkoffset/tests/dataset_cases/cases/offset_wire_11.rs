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
    offset_wire_closed_outside_0_005_g2,
    "tests/offset/wire_closed_outside_0_005/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(111.739),
        vertices: Some(37),
        edges: Some(37),
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
    offset_wire_closed_outside_0_005_g3,
    "tests/offset/wire_closed_outside_0_005/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(141.945),
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
    offset_wire_closed_outside_0_005_g4,
    "tests/offset/wire_closed_outside_0_005/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(401.356),
        vertices: Some(76),
        edges: Some(76),
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
    offset_wire_closed_outside_0_005_g5,
    "tests/offset/wire_closed_outside_0_005/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1077.36),
        vertices: Some(35),
        edges: Some(35),
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
    offset_wire_closed_outside_0_005_g6,
    "tests/offset/wire_closed_outside_0_005/G6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(882.11),
        vertices: Some(28),
        edges: Some(28),
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
    offset_wire_closed_outside_0_005_g8,
    "tests/offset/wire_closed_outside_0_005/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(283.791),
        vertices: Some(23),
        edges: Some(23),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_085.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_g9,
    "tests/offset/wire_closed_outside_0_005/G9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1150.26),
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
    offset_wire_closed_outside_0_005_h1,
    "tests/offset/wire_closed_outside_0_005/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(28.9362),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_outside_0_005_h2,
    "tests/offset/wire_closed_outside_0_005/H2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(79.7107),
        vertices: Some(32),
        edges: Some(32),
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
    offset_wire_closed_outside_0_005_h3,
    "tests/offset/wire_closed_outside_0_005/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(957.767),
        vertices: Some(30),
        edges: Some(30),
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
    offset_wire_closed_outside_0_005_h4,
    "tests/offset/wire_closed_outside_0_005/H4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(395.14),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_090.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_h5,
    "tests/offset/wire_closed_outside_0_005/H5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(22.6445),
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
    offset_wire_closed_outside_0_005_h6,
    "tests/offset/wire_closed_outside_0_005/H6",
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
    offset_wire_closed_outside_0_005_h7,
    "tests/offset/wire_closed_outside_0_005/H7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(117.29),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_outside_0_005_h8,
    "tests/offset/wire_closed_outside_0_005/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(182.056),
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
    offset_wire_closed_outside_0_005_h9,
    "tests/offset/wire_closed_outside_0_005/H9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(249.418),
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
    offset_wire_closed_outside_0_005_i1,
    "tests/offset/wire_closed_outside_0_005/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(2829.9),
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
    offset_wire_closed_outside_0_005_i2,
    "tests/offset/wire_closed_outside_0_005/I2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1307.78),
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
    offset_wire_closed_outside_0_005_i3,
    "tests/offset/wire_closed_outside_0_005/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(233.219),
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
    offset_wire_closed_outside_0_005_i4,
    "tests/offset/wire_closed_outside_0_005/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(302.636),
        vertices: Some(28),
        edges: Some(28),
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
    offset_wire_closed_outside_0_005_i5,
    "tests/offset/wire_closed_outside_0_005/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(381.441),
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
    offset_wire_closed_outside_0_005_i6,
    "tests/offset/wire_closed_outside_0_005/I6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(206.278),
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
    offset_wire_closed_outside_0_005_i7,
    "tests/offset/wire_closed_outside_0_005/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(24.9815),
        vertices: Some(18),
        edges: Some(18),
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
    offset_wire_closed_outside_0_005_i8,
    "tests/offset/wire_closed_outside_0_005/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(338.282),
        vertices: Some(53),
        edges: Some(53),
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
    offset_wire_closed_outside_0_005_i9,
    "tests/offset/wire_closed_outside_0_005/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(819.595),
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
    offset_wire_closed_outside_0_005_j1,
    "tests/offset/wire_closed_outside_0_005/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(837.134),
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
    offset_wire_closed_outside_0_005_j2,
    "tests/offset/wire_closed_outside_0_005/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(881.066),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_outside_0_005_j3,
    "tests/offset/wire_closed_outside_0_005/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(334.889),
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
    offset_wire_closed_outside_0_005_j4,
    "tests/offset/wire_closed_outside_0_005/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(343.316),
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
    offset_wire_closed_outside_0_005_j5,
    "tests/offset/wire_closed_outside_0_005/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(263.292),
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
