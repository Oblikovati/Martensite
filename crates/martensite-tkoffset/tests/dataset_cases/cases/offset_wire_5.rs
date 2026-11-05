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
    offset_wire_closed_inside_0_025_e5,
    "tests/offset/wire_closed_inside_0_025/E5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(415.78),
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
    offset_wire_closed_inside_0_025_e6,
    "tests/offset/wire_closed_inside_0_025/E6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(802.683),
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
    offset_wire_closed_inside_0_025_e7,
    "tests/offset/wire_closed_inside_0_025/E7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(536.062),
        vertices: Some(11),
        edges: Some(11),
        wires: Some(3)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_057.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_e9,
    "tests/offset/wire_closed_inside_0_025/E9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(11.4888),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_inside_0_025_f1,
    "tests/offset/wire_closed_inside_0_025/F1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1189.78),
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
    offset_wire_closed_inside_0_025_f2,
    "tests/offset/wire_closed_inside_0_025/F2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(357.203),
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
    offset_wire_closed_inside_0_025_f3,
    "tests/offset/wire_closed_inside_0_025/F3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(467.995),
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
    offset_wire_closed_inside_0_025_f4,
    "tests/offset/wire_closed_inside_0_025/F4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(385.545),
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
    offset_wire_closed_inside_0_025_f5,
    "tests/offset/wire_closed_inside_0_025/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(357.708),
        vertices: Some(37),
        edges: Some(37),
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
    offset_wire_closed_inside_0_025_f6,
    "tests/offset/wire_closed_inside_0_025/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(77.0453),
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
    offset_wire_closed_inside_0_025_f9,
    "tests/offset/wire_closed_inside_0_025/F9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_077.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_g1,
    "tests/offset/wire_closed_inside_0_025/G1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(240.105),
        vertices: Some(38),
        edges: Some(38),
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
    offset_wire_closed_inside_0_025_g2,
    "tests/offset/wire_closed_inside_0_025/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(82.3154),
        vertices: Some(18),
        edges: Some(18),
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
    offset_wire_closed_inside_0_025_g3,
    "tests/offset/wire_closed_inside_0_025/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(116.004),
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
    offset_wire_closed_inside_0_025_g4,
    "tests/offset/wire_closed_inside_0_025/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(291.991),
        vertices: Some(20),
        edges: Some(20),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 24255 ALL: Faulty shapes in variables faulty_1 to faulty_2 "),
        Step::Topo(Topo::Restore("brep/offset_wire_081.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_g5,
    "tests/offset/wire_closed_inside_0_025/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(845.75),
        vertices: Some(19),
        edges: Some(19),
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
    offset_wire_closed_inside_0_025_g6,
    "tests/offset/wire_closed_inside_0_025/G6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(336.833),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_083.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_g8,
    "tests/offset/wire_closed_inside_0_025/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(188.241),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_025_g9,
    "tests/offset/wire_closed_inside_0_025/G9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_086.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_h1,
    "tests/offset/wire_closed_inside_0_025/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(22.1665),
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
    offset_wire_closed_inside_0_025_h2,
    "tests/offset/wire_closed_inside_0_025/H2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(29.3518),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_088.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_025_h3,
    "tests/offset/wire_closed_inside_0_025/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(746.196),
        vertices: Some(17),
        edges: Some(17),
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
    offset_wire_closed_inside_0_025_h4,
    "tests/offset/wire_closed_inside_0_025/H4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(123.945),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_025_h5,
    "tests/offset/wire_closed_inside_0_025/H5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(17.3562),
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

offset_case!(
    offset_wire_closed_inside_0_025_h6,
    "tests/offset/wire_closed_inside_0_025/H6",
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
    offset_wire_closed_inside_0_025_h7,
    "tests/offset/wire_closed_inside_0_025/H7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(91.5865),
        vertices: Some(10),
        edges: Some(10),
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
    offset_wire_closed_inside_0_025_h8,
    "tests/offset/wire_closed_inside_0_025/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(146.772),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_025_h9,
    "tests/offset/wire_closed_inside_0_025/H9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(22.0754),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_025_i1,
    "tests/offset/wire_closed_inside_0_025/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(2247.72),
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
    offset_wire_closed_inside_0_025_i2,
    "tests/offset/wire_closed_inside_0_025/I2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.025,
        length: Some(1014.6),
        vertices: Some(6),
        edges: Some(6),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_097.brep", "s")),
    ]
);
