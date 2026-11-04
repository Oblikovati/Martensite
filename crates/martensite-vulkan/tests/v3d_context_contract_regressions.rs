//! Complete context-mode, rectangle-selection, axial-view and explicit-highlight regressions.
#![allow(clippy::duplicate_mod)] // Shared case modules keep one identity per host path.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
#[path = "case_host/context_mode_transition.rs"]
mod context_mode_transition;
mod v3d_context_contract_cases;

macro_rules! context {
    ($name:ident, $budget:expr, $body:ident) => {
        #[test]
        fn $name() {
            case_v3d_process_case::run_case_within(
                stringify!($name),
                $budget,
                v3d_context_contract_cases::$body,
            );
        }
    };
}
context!(bug74, 300, mode);
context!(buc60774, 1000, rectangle);
context!(bug11615, 1000, axial);
context!(bug31965, 1000, highlight);
