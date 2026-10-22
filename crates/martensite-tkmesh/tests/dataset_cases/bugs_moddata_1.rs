//! tests/bugs/moddata_1 (bugs group, `cpulimit 1000`); bodies in `cases/bugs_moddata_1.rs`.
use crate::support::process::{Group, draw_cases, run_case_exact_heap};

draw_cases!(run_case_exact_heap, Group::Bugs, bugs_moddata_1; bug20627);
