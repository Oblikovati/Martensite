//! Handle and allocator cases; twins in `occt/objects.cpp`.
use super::{Case, Sample};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::standard::{Handle, StandardTransient};
use std::hint::black_box;
use std::time::Instant;

pub const CASES: &[Case] = &[
    Case {
        name: "handle_create",
        run: handle_create,
    },
    Case {
        name: "handle_clone",
        run: handle_clone,
    },
    Case {
        name: "inc_allocator_value",
        run: inc_allocator_value,
    },
];

fn handle_create(n: usize) -> Sample {
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..n {
        let handle = Handle::new(StandardTransient::default());
        count += i64::from(black_box(&handle).ref_count());
    }
    Sample::since(start, n, count)
}

fn handle_clone(n: usize) -> Sample {
    let handle = Handle::new(StandardTransient::default());
    let clones = 10 * n;
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..clones {
        let copy = black_box(&handle).clone();
        count += i64::from(copy.ref_count());
    }
    Sample::since(start, clones, count)
}

/// 32-byte values from an incremental arena, kept until the arena is reset.
fn inc_allocator_value(n: usize) -> Sample {
    let allocator = BaseAllocator::incremental(12 * 1024);
    let mut kept = Vec::with_capacity(n);
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..n {
        let value = allocator.allocate_value([(i & 1) as u64, 0, 0, 0]);
        sum += black_box(value[0]) as i64;
        kept.push(value);
    }
    kept.clear();
    let reset = allocator.reset(false);
    let sample = Sample::since(start, n, sum);
    reset
        .map(|()| sample)
        .unwrap_or_else(|failure| panic!("{failure:?}"))
}
