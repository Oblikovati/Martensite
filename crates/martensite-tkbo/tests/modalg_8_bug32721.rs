//! Original geometry assertions under inherited bugs-suite CPU policy.
#![allow(non_snake_case)]
#[path = "support/draw_cpu_limit.rs"]
mod draw_cpu_limit;
#[allow(unused_imports)]
mod modalg8_cases;

#[test]
fn Modalg8__Bug32721() {
    draw_cpu_limit::run_bug_case("Modalg8__Bug32721", || {
        modalg8_cases::checked_cone_prism();
    });
}
