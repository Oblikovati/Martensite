//! `bugs_modalg_6` (bugs group, a 1000 s budget); bodies in `cases/bugs_modalg_6.rs`.
use crate::support::process::{Group, run_case, run_script};
use crate::support::session::NoView;

#[test]
fn bug25214() {
    let identity = "bugs_modalg_6::bug25214";
    run_case(identity, Group::Bugs, || {
        run_script(identity, Group::Bugs, |draw| {
            crate::cases::bugs_modalg_6::bug25214(draw, &mut NoView)
        });
    });
}

#[test]
fn bug25908() {
    let identity = "bugs_modalg_6::bug25908";
    run_case(identity, Group::Bugs, || {
        run_script(identity, Group::Bugs, |draw| {
            crate::cases::bugs_modalg_6::bug25908(draw, &mut NoView)
        });
    });
}
