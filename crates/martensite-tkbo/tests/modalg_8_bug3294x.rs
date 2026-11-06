//! Original geometry assertions under inherited bugs-suite CPU policy.
#![allow(non_snake_case)]
#[path = "support/draw_cpu_limit.rs"]
mod draw_cpu_limit;
#[allow(unused_imports)]
mod modalg8_cases;

#[test]
fn Modalg8__Bug32942() {
    draw_cpu_limit::run_bug_case("Modalg8__Bug32942", || {
        modalg8_cases::checked_32942();
    });
}

#[test]
fn Modalg8__Bug32943() {
    draw_cpu_limit::run_bug_case("Modalg8__Bug32943", || {
        modalg8_cases::checked_32943();
    });
}

#[test]
fn Modalg8__Bug32944() {
    draw_cpu_limit::run_bug_case("Modalg8__Bug32944", || {
        modalg8_cases::checked_32944();
    });
}

#[test]
fn Modalg8__Bug32945() {
    draw_cpu_limit::run_bug_case("Modalg8__Bug32945", || {
        modalg8_cases::checked_32945();
    });
}
