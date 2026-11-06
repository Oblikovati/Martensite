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
    bugs_fclasses_bug25024,
    "tests/bugs/fclasses/bug25024",
    Bugs,
    1000,
    End::Completed,
    None,
    true,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("============"),
        Step::Puts("25024"),
        Step::Puts("============"),
        Step::Topo(Topo::Restore("brep/bug25024_wires.brep", "w")),
        Step::Topo(Topo::ExplodeChildren("w")),
        Step::Loft(Loft {
            result: "r",
            solid: false,
            ruled: false,
            check: true,
            mutable_input: true,
            sections: &["w_1", "w_2"]
        }),
    ]
);
