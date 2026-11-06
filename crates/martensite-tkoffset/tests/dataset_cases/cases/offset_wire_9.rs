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
    offset_wire_closed_inside_0_075_i9,
    "tests/offset/wire_closed_inside_0_075/I9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(420.694),
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
    offset_wire_closed_inside_0_075_j1,
    "tests/offset/wire_closed_inside_0_075/J1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(358.56),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_075_j2,
    "tests/offset/wire_closed_inside_0_075/J2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(344.421),
        vertices: Some(11),
        edges: Some(11),
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
    offset_wire_closed_inside_0_075_j3,
    "tests/offset/wire_closed_inside_0_075/J3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(131.962),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_075_j4,
    "tests/offset/wire_closed_inside_0_075/J4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(131.212),
        vertices: Some(8),
        edges: Some(8),
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
    offset_wire_closed_inside_0_075_j5,
    "tests/offset/wire_closed_inside_0_075/J5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(115.836),
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
    offset_wire_closed_inside_0_075_j6,
    "tests/offset/wire_closed_inside_0_075/J6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(676.488),
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
    offset_wire_closed_inside_0_075_j7,
    "tests/offset/wire_closed_inside_0_075/J7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(24.4139),
        vertices: Some(13),
        edges: Some(13),
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
    offset_wire_closed_inside_0_075_j9,
    "tests/offset/wire_closed_inside_0_075/J9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(82.2146),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_inside_0_075_k1,
    "tests/offset/wire_closed_inside_0_075/K1",
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
        Step::Topo(Topo::Restore("brep/offset_wire_114.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_inside_0_075_k2,
    "tests/offset/wire_closed_inside_0_075/K2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(6.81895),
        vertices: Some(4),
        edges: Some(4),
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
    offset_wire_closed_outside_0_005_a1,
    "tests/offset/wire_closed_outside_0_005/A1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(249.903),
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

offset_case!(
    offset_wire_closed_outside_0_005_a4,
    "tests/offset/wire_closed_outside_0_005/A4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(243.898),
        vertices: Some(35),
        edges: Some(35),
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
    offset_wire_closed_outside_0_005_a5,
    "tests/offset/wire_closed_outside_0_005/A5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(168.498),
        vertices: Some(33),
        edges: Some(33),
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
    offset_wire_closed_outside_0_005_a6,
    "tests/offset/wire_closed_outside_0_005/A6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(258.969),
        vertices: Some(36),
        edges: Some(36),
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
    offset_wire_closed_outside_0_005_a7,
    "tests/offset/wire_closed_outside_0_005/A7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(2145.41),
        vertices: Some(18),
        edges: Some(18),
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
    offset_wire_closed_outside_0_005_a8,
    "tests/offset/wire_closed_outside_0_005/A8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(58.3821),
        vertices: Some(16),
        edges: Some(16),
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
    offset_wire_closed_outside_0_005_a9,
    "tests/offset/wire_closed_outside_0_005/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(2012.29),
        vertices: Some(24),
        edges: Some(24),
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
    offset_wire_closed_outside_0_005_b1,
    "tests/offset/wire_closed_outside_0_005/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(826.867),
        vertices: Some(20),
        edges: Some(20),
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
    offset_wire_closed_outside_0_005_b2,
    "tests/offset/wire_closed_outside_0_005/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(830.837),
        vertices: Some(115),
        edges: Some(115),
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
    offset_wire_closed_outside_0_005_b3,
    "tests/offset/wire_closed_outside_0_005/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(132.672),
        vertices: Some(7),
        edges: Some(7),
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
    offset_wire_closed_outside_0_005_b4,
    "tests/offset/wire_closed_outside_0_005/B4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_018.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_b6,
    "tests/offset/wire_closed_outside_0_005/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(209.1),
        vertices: Some(15),
        edges: Some(15),
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
    offset_wire_closed_outside_0_005_b7,
    "tests/offset/wire_closed_outside_0_005/B7",
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
        Step::Topo(Topo::Restore("brep/offset_wire_021.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_b8,
    "tests/offset/wire_closed_outside_0_005/B8",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(67.0006),
        vertices: Some(13),
        edges: Some(13),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_023.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_b9,
    "tests/offset/wire_closed_outside_0_005/B9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(2358.59),
        vertices: Some(15),
        edges: Some(15),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_025.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_c1,
    "tests/offset/wire_closed_outside_0_005/C1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1776.45),
        vertices: Some(9),
        edges: Some(9),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_028.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_c2,
    "tests/offset/wire_closed_outside_0_005/C2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.005,
        length: Some(1146.21),
        vertices: Some(3),
        edges: Some(3),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_029.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_c3,
    "tests/offset/wire_closed_outside_0_005/C3",
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
        Step::Topo(Topo::Restore("brep/offset_wire_030.brep", "s")),
    ]
);

offset_case!(
    offset_wire_closed_outside_0_005_c4,
    "tests/offset/wire_closed_outside_0_005/C4",
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
        Step::Topo(Topo::Restore("brep/offset_wire_031.brep", "s")),
    ]
);
