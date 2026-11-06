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
    offset_wire_closed_outside_0_075_d8,
    "tests/offset/wire_closed_outside_0_075/D8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(297.313),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_outside_0_075_d9,
    "tests/offset/wire_closed_outside_0_075/D9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(46.4925),
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
    offset_wire_closed_outside_0_075_e1,
    "tests/offset/wire_closed_outside_0_075/E1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(9.25335),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_e2,
    "tests/offset/wire_closed_outside_0_075/E2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(52.535),
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
    offset_wire_closed_outside_0_075_e3,
    "tests/offset/wire_closed_outside_0_075/E3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(95.4289),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_outside_0_075_e4,
    "tests/offset/wire_closed_outside_0_075/E4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(289.058),
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
    offset_wire_closed_outside_0_075_e5,
    "tests/offset/wire_closed_outside_0_075/E5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(750.888),
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
    offset_wire_closed_outside_0_075_e6,
    "tests/offset/wire_closed_outside_0_075/E6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1748.03),
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
    offset_wire_closed_outside_0_075_e7,
    "tests/offset/wire_closed_outside_0_075/E7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1202.36),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_outside_0_075_e9,
    "tests/offset/wire_closed_outside_0_075/E9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(22.2229),
        vertices: Some(9),
        edges: Some(9),
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
    offset_wire_closed_outside_0_075_f1,
    "tests/offset/wire_closed_outside_0_075/F1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(2663.0),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_f2,
    "tests/offset/wire_closed_outside_0_075/F2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(618.925),
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
    offset_wire_closed_outside_0_075_f3,
    "tests/offset/wire_closed_outside_0_075/F3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(816.771),
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
    offset_wire_closed_outside_0_075_f4,
    "tests/offset/wire_closed_outside_0_075/F4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1434.38),
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
    offset_wire_closed_outside_0_075_f5,
    "tests/offset/wire_closed_outside_0_075/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(635.295),
        vertices: Some(31),
        edges: Some(31),
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
    offset_wire_closed_outside_0_075_f6,
    "tests/offset/wire_closed_outside_0_075/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(133.596),
        vertices: Some(48),
        edges: Some(48),
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
    offset_wire_closed_outside_0_075_f9,
    "tests/offset/wire_closed_outside_0_075/F9",
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
    offset_wire_closed_outside_0_075_g1,
    "tests/offset/wire_closed_outside_0_075/G1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(593.849),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_outside_0_075_g2,
    "tests/offset/wire_closed_outside_0_075/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(152.526),
        vertices: Some(22),
        edges: Some(22),
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
    offset_wire_closed_outside_0_075_g3,
    "tests/offset/wire_closed_outside_0_075/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(202.475),
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
    offset_wire_closed_outside_0_075_g4,
    "tests/offset/wire_closed_outside_0_075/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(546.206),
        vertices: Some(34),
        edges: Some(34),
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
    offset_wire_closed_outside_0_075_g5,
    "tests/offset/wire_closed_outside_0_075/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1516.92),
        vertices: Some(25),
        edges: Some(25),
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
    offset_wire_closed_outside_0_075_g6,
    "tests/offset/wire_closed_outside_0_075/G6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1060.88),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_outside_0_075_g8,
    "tests/offset/wire_closed_outside_0_075/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(390.827),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_outside_0_075_g9,
    "tests/offset/wire_closed_outside_0_075/G9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(2606.92),
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
    offset_wire_closed_outside_0_075_h1,
    "tests/offset/wire_closed_outside_0_075/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(39.9553),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_outside_0_075_h2,
    "tests/offset/wire_closed_outside_0_075/H2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(100.243),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_outside_0_075_h3,
    "tests/offset/wire_closed_outside_0_075/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1367.49),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO ALL: Faulty shapes in variables faulty_1 to faulty_10"),
        Step::Topo(Topo::Restore("brep/offset_wire_089.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_075_h4,
    "tests/offset/wire_closed_outside_0_075/H4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(513.669),
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
    offset_wire_closed_outside_0_075_h5,
    "tests/offset/wire_closed_outside_0_075/H5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(31.8933),
        vertices: Some(16),
        edges: Some(16),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_091.brep", "s")),
    ]
);
