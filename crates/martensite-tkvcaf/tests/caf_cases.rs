//! The document presentation and transaction scripts: each runs alone in a child process under its
//! group's CPU budget, on the real 3D viewer over a virtual window where it opens one, and is
//! graded on its whole output.
#![cfg(target_os = "linux")]

#[path = "caf_cases/session/mod.rs"]
mod session;
#[path = "caf_cases/support/mod.rs"]
mod support;

/// One `#[test]` per script, named as libtest names it, which is what its child process runs.
macro_rules! caf_case {
    ($name:ident, $source:literal, $group:ident, $image:expr, $body:expr) => {
        #[test]
        fn $name() {
            let module = module_path!().split_once("::").map_or("", |parts| parts.1);
            $crate::session::hooks::run_script(
                &format!("{module}::{}", stringify!($name)),
                $source,
                &$crate::session::hooks::$group,
                $image,
                $body,
            );
        }
    };
}

#[path = "caf_cases/beyond_scripts.rs"]
mod beyond_scripts;
#[path = "caf_cases/bug_cases.rs"]
mod bug_cases;
#[path = "caf_cases/heap_cases.rs"]
mod heap_cases;
#[path = "caf_cases/orientation_cases.rs"]
mod orientation_cases;
#[path = "caf_cases/presentation_cases.rs"]
mod presentation_cases;
#[path = "caf_cases/transaction_cases.rs"]
mod transaction_cases;
