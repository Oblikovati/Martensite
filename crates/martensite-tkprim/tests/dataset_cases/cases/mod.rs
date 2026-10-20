//! The case bodies, one module per test group, at
//!. Each body follows its script line by line,
//! logs what the script and its commands print and reports every variable and view
//! step to its [`crate::support::session::Session`].
pub mod bugs_fclasses;
pub mod bugs_modalg_1;
pub mod bugs_modalg_2;
pub mod bugs_modalg_4;
pub mod bugs_modalg_5;
pub mod bugs_modalg_6;
pub mod bugs_modalg_7;
pub mod bugs_modalg_8;
pub mod bugs_moddata_1;
pub mod bugs_moddata_2;
pub mod bugs_moddata_3;
pub mod demo_session;
pub mod geometry_preview;
pub mod perf_session;
pub mod prism_seminf;
