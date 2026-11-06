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
    bugs_moddata_2_bug712_2,
    "tests/bugs/moddata_2/bug712_2",
    Bugs,
    30,
    End::Completed,
    Some(crate::runner::STOPS_AT_CPU_LIMIT),
    false,
    &[
        Step::Puts("CPU and elapsed time limit set to 1000 seconds"),
        Step::Puts("========"),
        Step::Puts("712"),
        Step::Puts("========"),
        Step::Puts("CPU and elapsed time limit set to 30 seconds"),
        Step::If(
            Cond::Failed(&Step::Extra(Extra::SlabDraft(15.0, 30.0))),
            &[Step::Puts("Faulty 712")],
            &[Step::Puts("712 OK")]
        ),
        Step::CheckView("result"),
    ]
);
