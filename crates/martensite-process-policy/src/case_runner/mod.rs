//! Exact self-reexecution for isolated ordinary Rust tests.
//!
//! This optional module owns the Rust 1.99.0 pretty/show-output libtest profile.
//! Direct descriptor diagnostics may split its result row; reserved harness lines
//! in that span are rejected rather than searched past. Raw logs are never rewritten.
//! Framing is a sanity check, not a security boundary against arbitrary child bytes;
//! exact-source execution also requires private bound records and process success.
//! It is not a general log parser or an expression evaluator. Only the matching
//! child changes process policy; the parent retains child logs without forwarding
//! nested libtest output. Artifact roots and any graphics mutex belong to callers.
mod artifacts;
mod child_policy;
mod context;
mod failure;
mod harness;
mod launch;
mod lifecycle;
mod runner;
mod spec;

pub use failure::{CaseFailure, CaseRunError};
pub use runner::{CaseReceipt, CaseRunner};
pub use spec::{CaseSpec, FactorProfile};

#[cfg(test)]
mod fake_controls;
#[cfg(test)]
mod graphics_controls;
#[cfg(test)]
mod harness_tests;
#[cfg(test)]
mod live_controls;
#[cfg(test)]
mod policy_controls;

#[cfg(test)]
mod prepared_control_support;
#[cfg(test)]
mod prepared_controls;

#[cfg(test)]
mod prepared_live_controls;
#[cfg(test)]
mod prepared_parent_controls;
