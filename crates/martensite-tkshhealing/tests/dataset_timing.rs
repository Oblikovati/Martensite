//! The TKShHealing-owned scenario case that sets its own CPU limit (bugs/modalg_6/bug26627:
//! `cpulimit 10` before `fixshape result a 1e-6`, the Shape Healing hang of OCC 6.8.0).
//! It runs in its own target so that no other test of the binary competes for the CPU
//! while the limit applies; the body is the one in dataset_cases/cases.
#![allow(clippy::excessive_precision)] // Expected values keep the case scripts' decimals verbatim.

#[allow(dead_code, unused_imports)] // Only the timed body runs here.
#[path = "dataset_cases/cases/mod.rs"]
mod cases;
#[path = "dataset_cases/support/mod.rs"]
mod support;

use support::process::{BUGS_SECONDS, dataset_cases, run_case};
use support::session::NoView;

#[path = "dataset_timing/bugs_modalg_6.rs"]
mod bugs_modalg_6;
