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
    offset_wire_closed_inside_0_075_f3,
    "tests/offset/wire_closed_inside_0_075/F3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_063.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_f4,
    "tests/offset/wire_closed_inside_0_075/F4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_064.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_f5,
    "tests/offset/wire_closed_inside_0_075/F5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(175.978),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_075_f6,
    "tests/offset/wire_closed_inside_0_075/F6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(45.4428),
        vertices: Some(52),
        edges: Some(52),
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
    offset_wire_closed_inside_0_075_f9,
    "tests/offset/wire_closed_inside_0_075/F9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_077.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_g1,
    "tests/offset/wire_closed_inside_0_075/G1",
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
        Step::Topo(Topo::Restore("brep/offset_wire_078.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_g2,
    "tests/offset/wire_closed_inside_0_075/G2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(6.87722),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_075_g3,
    "tests/offset/wire_closed_inside_0_075/G3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(9.29926),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_075_g4,
    "tests/offset/wire_closed_inside_0_075/G4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(428.734),
        vertices: Some(18),
        edges: Some(18),
        wires: Some(2)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Error : The offset cannot be built."),
        Step::Puts("TODO 23748 ALL: Error: The command cannot be built"),
        Step::Puts("TODO 23748 ALL: Error: Offset is not done."),
        Step::Topo(Topo::Restore("brep/offset_wire_081.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_g5,
    "tests/offset/wire_closed_inside_0_075/G5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(160.262),
        vertices: Some(6),
        edges: Some(6),
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
    offset_wire_closed_inside_0_075_g6,
    "tests/offset/wire_closed_inside_0_075/G6",
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
        Step::Topo(Topo::Restore("brep/offset_wire_083.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_g8,
    "tests/offset/wire_closed_inside_0_075/G8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(73.5727),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_075_g9,
    "tests/offset/wire_closed_inside_0_075/G9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_086.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_h1,
    "tests/offset/wire_closed_inside_0_075/H1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(8.76538),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_075_h2,
    "tests/offset/wire_closed_inside_0_075/H2",
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
        Step::Topo(Topo::Restore("brep/offset_wire_088.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_h3,
    "tests/offset/wire_closed_inside_0_075/H3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(383.716),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_075_h4,
    "tests/offset/wire_closed_inside_0_075/H4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_090.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_h5,
    "tests/offset/wire_closed_inside_0_075/H5",
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
        Step::Topo(Topo::Restore("brep/offset_wire_091.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_h6,
    "tests/offset/wire_closed_inside_0_075/H6",
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
    offset_wire_closed_inside_0_075_h7,
    "tests/offset/wire_closed_inside_0_075/H7",
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
        Step::Topo(Topo::Restore("brep/offset_wire_093.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_h8,
    "tests/offset/wire_closed_inside_0_075/H8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(60.9685),
        vertices: Some(3),
        edges: Some(3),
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
    offset_wire_closed_inside_0_075_h9,
    "tests/offset/wire_closed_inside_0_075/H9",
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
        Step::Topo(Topo::Restore("brep/offset_wire_095.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_i1,
    "tests/offset/wire_closed_inside_0_075/I1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(1163.23),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_inside_0_075_i2,
    "tests/offset/wire_closed_inside_0_075/I2",
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
        Step::Topo(Topo::Restore("brep/offset_wire_097.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_i3,
    "tests/offset/wire_closed_inside_0_075/I3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(115.556),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_075_i4,
    "tests/offset/wire_closed_inside_0_075/I4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(4.75963),
        vertices: Some(12),
        edges: Some(12),
        wires: Some(2)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_099.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_i5,
    "tests/offset/wire_closed_inside_0_075/I5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(142.667),
        vertices: Some(12),
        edges: Some(12),
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
    offset_wire_closed_inside_0_075_i6,
    "tests/offset/wire_closed_inside_0_075/I6",
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
        Step::Topo(Topo::Restore("brep/offset_wire_101.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_i7,
    "tests/offset/wire_closed_inside_0_075/I7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(12.3888),
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
    offset_wire_closed_inside_0_075_i8,
    "tests/offset/wire_closed_inside_0_075/I8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(94.1559),
        vertices: Some(18),
        edges: Some(18),
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
