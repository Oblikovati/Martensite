//! `bugs_fclasses` bodies written by hand: the scripts sample the heap in a loop. Every declared
//! problem is unstable (`?`), so a heap that does not grow and one that does both pass; the
//! script error's own prefix replaces the harness's in the declared pattern.
use crate::support::{
    commands::make_box,
    heap_trend::{checktrend, meminfo_heap},
    session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;

const NB_CYCLES: usize = 10;
const MESSAGE: &str = "Memory leak detected";

/// The result command each script repeats.
#[derive(Clone, Copy)]
enum Repeated {
    Result(Operation),
    Section,
}

/// When the script checks the samples.
#[derive(Clone, Copy, PartialEq)]
enum TrendCheck {
    EachCycle,
    AfterLoop,
}

struct HeapLoop {
    first_box_origin: [f64; 3],
    repeated: Repeated,
    tolerance: f64,
    check: TrendCheck,
    /// `bop` and the result once before the loop.
    warm_up: bool,
}

fn declarations(leading: Option<&str>) {
    if let Some(line) = leading {
        println!("{line}");
    }
    println!("TODO ?OCC7287 Linux: ScriptStep Exception: Memory leak detected");
    println!("TODO ?OCC7287 Linux: TEST INCOMPLETE");
    println!("TODO ?OCC7287 MacOS: ScriptStep Exception: Memory leak detected");
    println!("TODO ?OCC7287 MacOS: TEST INCOMPLETE");
    println!("============");
    println!("OCC7287");
    println!("============");
}

fn one_cycle(s: &mut dyn Session, two_step: &mut TwoStepBoolean, repeated: Repeated) {
    two_step.bop(s, "b1", "b2");
    match repeated {
        Repeated::Result(kind) => two_step.result(s, "r", kind),
        Repeated::Section => two_step.section(s, "r"),
    }
}

fn heap_loop(s: &mut dyn Session, case: &HeapLoop) {
    make_box(s, "b1", case.first_box_origin, [100., 100., 100.]);
    make_box(s, "b2", [0., 0., 0.], [50., 50., 50.]);
    let mut two_step = TwoStepBoolean::default();
    if case.warm_up {
        one_cycle(s, &mut two_step, case.repeated);
    }
    let mut samples = Vec::with_capacity(NB_CYCLES);
    for _ in 0..NB_CYCLES {
        one_cycle(s, &mut two_step, case.repeated);
        samples.push(meminfo_heap());
        if case.check == TrendCheck::EachCycle {
            checktrend(&samples, 0., case.tolerance, MESSAGE);
        }
    }
    if case.check == TrendCheck::AfterLoop {
        checktrend(&samples, 0., case.tolerance, MESSAGE);
    }
}

const ORIGIN: [f64; 3] = [0., 0., 0.];
const SHIFTED: [f64; 3] = [10., 10., 10.];

fn each_cycle(repeated: Repeated) -> HeapLoop {
    HeapLoop {
        first_box_origin: SHIFTED,
        repeated,
        tolerance: 100.,
        check: TrendCheck::EachCycle,
        warm_up: false,
    }
}

pub fn bug7287_1(s: &mut dyn Session) {
    declarations(Some("TODO ?СК33225 Linux: Checking trend failed "));
    let case = HeapLoop {
        first_box_origin: ORIGIN,
        repeated: Repeated::Result(Operation::Cut),
        tolerance: 10.,
        check: TrendCheck::EachCycle,
        warm_up: true,
    };
    heap_loop(s, &case);
}

pub fn bug7287_2(s: &mut dyn Session) {
    declarations(None);
    let case = HeapLoop {
        check: TrendCheck::AfterLoop,
        ..each_cycle(Repeated::Result(Operation::Cut))
    };
    heap_loop(s, &case);
}

pub fn bug7287_3(s: &mut dyn Session) {
    declarations(None);
    heap_loop(s, &each_cycle(Repeated::Result(Operation::Cut21)));
}

pub fn bug7287_4(s: &mut dyn Session) {
    declarations(Some("TODO ?OCC7287 Linux: Checking trend failed"));
    heap_loop(s, &each_cycle(Repeated::Result(Operation::Common)));
}

pub fn bug7287_5(s: &mut dyn Session) {
    declarations(Some(
        "TODO ?CR332257287 Linux: Checking trend failed: mean delta per step = 9432.0, sigma = 4220.013270121316, expected delta = 0",
    ));
    heap_loop(s, &each_cycle(Repeated::Result(Operation::Fuse)));
}

pub fn bug7287_6(s: &mut dyn Session) {
    declarations(Some(
        "TODO ?CR33225 Linux: Checking trend failed: mean delta per step =",
    ));
    heap_loop(s, &each_cycle(Repeated::Section));
}
