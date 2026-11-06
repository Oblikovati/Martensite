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
    offset_wire_closed_inside_0_005_d8,
    "tests/offset/wire_closed_inside_0_005/D8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(27.7312),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_048.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_d9,
    "tests/offset/wire_closed_inside_0_005/D9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(26.7702),
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
    offset_wire_closed_inside_0_005_e1,
    "tests/offset/wire_closed_inside_0_005/E1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(5.21422),
        vertices: Some(10),
        edges: Some(10),
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
    offset_wire_closed_inside_0_005_e2,
    "tests/offset/wire_closed_inside_0_005/E2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(24.8784),
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
    offset_wire_closed_inside_0_005_e3,
    "tests/offset/wire_closed_inside_0_005/E3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(68.541),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_005_e4,
    "tests/offset/wire_closed_inside_0_005/E4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(190.3),
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

offset_case!(
    offset_wire_closed_inside_0_005_e5,
    "tests/offset/wire_closed_inside_0_005/E5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(491.555),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_005_e6,
    "tests/offset/wire_closed_inside_0_005/E6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1109.94),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_056.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_e7,
    "tests/offset/wire_closed_inside_0_005/E7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(970.536),
        vertices: Some(10),
        edges: Some(10),
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
    offset_wire_closed_inside_0_005_e9,
    "tests/offset/wire_closed_inside_0_005/E9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(14.3355),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_inside_0_005_f1,
    "tests/offset/wire_closed_inside_0_005/F1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(2062.48),
        vertices: Some(28),
        edges: Some(28),
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
    offset_wire_closed_inside_0_005_f2,
    "tests/offset/wire_closed_inside_0_005/F2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(410.561),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_005_f3,
    "tests/offset/wire_closed_inside_0_005/F3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(538.146),
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
    offset_wire_closed_inside_0_005_f4,
    "tests/offset/wire_closed_inside_0_005/F4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(591.249),
        vertices: Some(5),
        edges: Some(5),
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
    offset_wire_closed_inside_0_005_f5,
    "tests/offset/wire_closed_inside_0_005/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(427.733),
        vertices: Some(40),
        edges: Some(40),
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
    offset_wire_closed_inside_0_005_f6,
    "tests/offset/wire_closed_inside_0_005/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(88.5313),
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
    offset_wire_closed_inside_0_005_f9,
    "tests/offset/wire_closed_inside_0_005/F9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_077.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_005_g1,
    "tests/offset/wire_closed_inside_0_005/G1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(452.32),
        vertices: Some(62),
        edges: Some(62),
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
    offset_wire_closed_inside_0_005_g2,
    "tests/offset/wire_closed_inside_0_005/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(103.961),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_inside_0_005_g3,
    "tests/offset/wire_closed_inside_0_005/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(133.298),
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
    offset_wire_closed_inside_0_005_g4,
    "tests/offset/wire_closed_inside_0_005/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(376.906),
        vertices: Some(65),
        edges: Some(65),
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
    offset_wire_closed_inside_0_005_g5,
    "tests/offset/wire_closed_inside_0_005/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(1011.74),
        vertices: Some(37),
        edges: Some(37),
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
    offset_wire_closed_inside_0_005_g6,
    "tests/offset/wire_closed_inside_0_005/G6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(799.636),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_inside_0_005_g8,
    "tests/offset/wire_closed_inside_0_005/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(263.304),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_inside_0_005_g9,
    "tests/offset/wire_closed_inside_0_005/G9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(587.462),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_inside_0_005_h1,
    "tests/offset/wire_closed_inside_0_005/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(26.9235),
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
    offset_wire_closed_inside_0_005_h2,
    "tests/offset/wire_closed_inside_0_005/H2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(74.0829),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_inside_0_005_h3,
    "tests/offset/wire_closed_inside_0_005/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(896.364),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_inside_0_005_h4,
    "tests/offset/wire_closed_inside_0_005/H4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(392.211),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_005_h5,
    "tests/offset/wire_closed_inside_0_005/H5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.005,
        length: Some(21.0717),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_091.brep", "s")),
    ]
);
