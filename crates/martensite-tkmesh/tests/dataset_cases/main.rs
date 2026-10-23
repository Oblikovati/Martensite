//! Dataset-backed mesh regression cases retain their recorded inputs, assertions and tolerances.
//! Case logs are checked against fixture rules. Cases with a CPU limit run in a
//! child process under that limit.

mod cases;
mod support;

mod bugs_moddata_1;
mod demo_draw;
