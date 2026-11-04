//! Process-isolated baseline/candidate workloads for live composition storage.
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::{select_mgr::*, std_select::StdSelectShapeTypeFilter};
use std::{hint::black_box, time::Instant};

fn measured_collection(count: usize) -> (Handle<SelectMgrAndFilter>, Handle<dyn SelectMgrFilter>) {
    let parent = Handle::new(SelectMgrAndFilter::default());
    let child: Handle<dyn SelectMgrFilter> =
        Handle::new(StdSelectShapeTypeFilter::new(ShapeEnum::Face)).cast();
    for _ in 0..count {
        parent.add(&child);
    }
    (parent, child)
}

fn getter_step(parent: &SelectMgrAndFilter, child: &Handle<dyn SelectMgrFilter>) -> i32 {
    let alias = black_box(parent.stored_filters());
    black_box(alias.len());
    child.ref_count()
}

fn collection_step(
    mode: &str,
    parent: &SelectMgrAndFilter,
    child: &Handle<dyn SelectMgrFilter>,
) -> i32 {
    match mode {
        "getter" => return getter_step(parent, child),
        "all" => {
            black_box(parent.acts_on(ShapeEnum::Edge));
        }
        "first" => {
            black_box(parent.acts_on(ShapeEnum::Face));
        }
        "noop" => {
            black_box(parent);
        }
        other => panic!("mode {other:?}; expected getter/all/first/noop"),
    }
    child.ref_count()
}

fn collection_workload(mode: &str, count: usize, iterations: usize) -> (u128, i32) {
    let (parent, child) = measured_collection(count);
    let started = Instant::now();
    let mut references = child.ref_count();
    for _ in 0..iterations {
        references = collection_step(mode, &parent, &child);
    }
    (started.elapsed().as_nanos(), references)
}

fn lock_workload(mode: &str, iterations: usize) -> u128 {
    let state = SharedState::new(0_u64);
    let started = Instant::now();
    for _ in 0..iterations {
        if mode == "read" {
            black_box(*state.read());
        } else {
            *state.write() += 1;
        }
    }
    started.elapsed().as_nanos()
}

fn argument_usize(arguments: &[String], index: usize) -> usize {
    let text = arguments
        .get(index)
        .unwrap_or_else(|| panic!("missing argument {index}; expected usize"));
    text.parse()
        .unwrap_or_else(|error| panic!("argument {index}={text:?}; expected usize: {error}"))
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let mode = arguments
        .get(1)
        .expect("missing mode; expected workload name");
    let count = argument_usize(&arguments, 2);
    let iterations = argument_usize(&arguments, 3);
    let (elapsed, references) = if mode == "read" || mode == "write" {
        (lock_workload(mode, iterations), 0)
    } else {
        collection_workload(mode, count, iterations)
    };
    println!(
        "{{\"mode\":{mode:?},\"count\":{count},\"iterations\":{iterations},\"elapsed_ns\":{elapsed},\"observed_intrusive_count\":{references}}}"
    );
}
