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
    offset_wire_unclosed_outside_0_075_a9,
    "tests/offset/wire_unclosed_outside_0_075/A9",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(77744.9),
        vertices: Some(204),
        edges: Some(204),
        wires: Some(1)
    }),
    None,
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Puts("TODO 23748 ALL: Faulty shapes in variables faulty_1 to faulty_"),
        Step::Topo(Topo::Restore("brep/offset_wire_027.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b1,
    "tests/offset/wire_unclosed_outside_0_075/B1",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(92.0671),
        vertices: Some(41),
        edges: Some(41),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_032.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b2,
    "tests/offset/wire_unclosed_outside_0_075/B2",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(78.0343),
        vertices: Some(60),
        edges: Some(60),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_033.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b3,
    "tests/offset/wire_unclosed_outside_0_075/B3",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(702.216),
        vertices: Some(60),
        edges: Some(60),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_034.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b4,
    "tests/offset/wire_unclosed_outside_0_075/B4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(702.225),
        vertices: Some(55),
        edges: Some(55),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/case_3_wire2.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b5,
    "tests/offset/wire_unclosed_outside_0_075/B5",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(450.841),
        vertices: Some(44),
        edges: Some(44),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_053.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b6,
    "tests/offset/wire_unclosed_outside_0_075/B6",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: -0.075,
        length: Some(3127.75),
        vertices: Some(39),
        edges: Some(39),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_058.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_b7,
    "tests/offset/wire_unclosed_outside_0_075/B7",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(1080.88),
        vertices: Some(22),
        edges: Some(22),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_069.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_c4,
    "tests/offset/wire_unclosed_outside_0_075/C4",
    Shape,
    900,
    End::OffsetWire(WireEnd {
        off_param: 0.075,
        length: Some(368.064),
        vertices: Some(72),
        edges: Some(72),
        wires: Some(1)
    }),
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 900 seconds"),
        Step::Topo(Topo::Restore("brep/offset_wire_075.brep", "s")),
    ]
);

offset_case!(
    offset_wire_unclosed_outside_0_075_c5,
    "tests/offset/wire_unclosed_outside_0_075/C5",
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
        Step::Topo(Topo::Restore("brep/offset_wire_076.brep", "s")),
    ]
);
