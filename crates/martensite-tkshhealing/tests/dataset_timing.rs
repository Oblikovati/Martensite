//! Healing regression with a dedicated ten-second CPU budget.
//! Run it in its own target so other tests do not consume the process budget.
//! The case body is shared with the dataset regression modules.
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
