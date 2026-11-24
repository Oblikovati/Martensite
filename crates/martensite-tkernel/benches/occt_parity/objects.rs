//! Handle and allocator cases; twins in `occt/objects.cpp`.
use super::{Case, Sample};
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use std::hint::black_box;
use std::time::Instant;

struct HandleProbe;
impl_transient!(HandleProbe, "HandleProbe", dyn Transient);

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
        name: "handle_upcast",
        run: handle_upcast,
    },
    Case {
        name: "handle_downcast",
        run: handle_downcast,
    },
    Case {
        name: "handle_downcast_ref",
        run: handle_downcast_ref,
    },
    Case {
        name: "handle_is_kind",
        run: handle_is_kind,
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
        let handle = Handle::new(StandardTransient);
        count += black_box(&handle).ref_count() as i64;
    }
    Sample::since(start, n, count)
}

fn handle_clone(n: usize) -> Sample {
    let handle = Handle::new(StandardTransient);
    let clones = 10 * n;
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..clones {
        let copy = black_box(&handle).clone();
        count += copy.ref_count() as i64;
    }
    Sample::since(start, clones, count)
}

fn handle_upcast(n: usize) -> Sample {
    let owner = Handle::new(HandleProbe);
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..n {
        let base = black_box(&owner).upcast::<dyn Transient>();
        count += black_box(&base).ref_count() as i64;
    }
    Sample::since(start, n, count)
}

fn handle_downcast(n: usize) -> Sample {
    let base = Handle::new(HandleProbe).into_upcast::<dyn Transient>();
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..n {
        let concrete = black_box(&base).downcast::<HandleProbe>();
        count += black_box(&concrete).ref_count() as i64;
    }
    Sample::since(start, n, count)
}

fn handle_downcast_ref(n: usize) -> Sample {
    let base = Handle::new(HandleProbe).into_upcast::<dyn Transient>();
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..n {
        let concrete = black_box(&base).downcast_ref::<HandleProbe>();
        count += i64::from(black_box(concrete).is_some());
    }
    Sample::since(start, n, count)
}

fn handle_is_kind(n: usize) -> Sample {
    let base = Handle::new(HandleProbe).into_upcast::<dyn Transient>();
    let kind = <dyn Transient>::static_type();
    base.dynamic_type();
    let start = Instant::now();
    let mut count = 0i64;
    for _ in 0..n {
        count += i64::from(black_box(&base).is_kind(black_box(kind)));
    }
    Sample::since(start, n, count)
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
