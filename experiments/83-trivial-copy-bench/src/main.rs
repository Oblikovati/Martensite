//! #83 trial: times each block-copy variant in one process, rotating the order every repetition.
//! Run: `cargo run --release -- 9`; prints one JSON line per block size and variant.
use allocator_api2::alloc::Global;
use allocator_api2::vec::Vec as AVec;
use std::hint::black_box;
use std::time::Instant;

type Block = AVec<u64, Global>;

#[inline(never)]
fn raw(values: &[u64], block: &mut Block) {
    unsafe {
        std::ptr::copy_nonoverlapping(values.as_ptr(), block.as_mut_ptr(), values.len());
        block.set_len(values.len());
    }
}
#[inline(never)]
fn extend_copied(values: &[u64], block: &mut Block) {
    block.extend(values.iter().copied());
}
#[inline(never)]
fn extend_copied_memcpy(values: &[u64], block: &mut Block) {
    block.extend(values.iter().copied());
    block.as_mut_slice().copy_from_slice(values);
}
#[inline(never)]
fn extend_ref(values: &[u64], block: &mut Block) {
    block.extend(values);
}
#[inline(never)]
fn resize_with_first_memcpy(values: &[u64], block: &mut Block) {
    let first = values.first().copied();
    if let Some(first) = first {
        block.resize_with(values.len(), || first);
    }
    block.as_mut_slice().copy_from_slice(values);
}
#[inline(never)]
fn resize_with_first(values: &[u64], block: &mut Block) {
    let first = values.first().copied();
    if let Some(first) = first {
        block.resize_with(values.len(), || first);
    }
}
#[inline(never)]
fn memcpy_only(values: &[u64], block: &mut [u64]) {
    block.copy_from_slice(values);
}

fn run(name: &str, n: usize, rounds: usize, f: fn(&[u64], &mut Block)) -> u128 {
    let source: std::vec::Vec<u64> = (0..n as u64).collect();
    let mut block = Block::with_capacity_in(n, Global);
    let started = Instant::now();
    for _ in 0..rounds {
        f(black_box(&source), &mut block);
        assert_eq!(black_box(&block).len(), n);
        block.clear();
    }
    let ns = started.elapsed().as_nanos();
    let _ = name;
    ns
}
fn run_memcpy_only(n: usize, rounds: usize) -> u128 {
    let source: std::vec::Vec<u64> = (0..n as u64).collect();
    let mut dest = vec![0u64; n];
    let started = Instant::now();
    for _ in 0..rounds {
        memcpy_only(black_box(&source), &mut dest);
        assert_eq!(black_box(&dest).last(), Some(&(n as u64 - 1)));
    }
    started.elapsed().as_nanos()
}

fn main() {
    let variants: [(&str, fn(&[u64], &mut Block)); 6] = [
        ("raw", raw),
        ("extend_copied", extend_copied),
        ("extend_copied_memcpy", extend_copied_memcpy),
        ("extend_ref", extend_ref),
        ("resize_with_first", resize_with_first),
        ("resize_with_first_memcpy", resize_with_first_memcpy),
    ];
    let reps: usize = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(7);
    for &(n, rounds) in &[(256usize, 256_000usize), (4096, 16_000), (65_536, 1_000)] {
        let mut samples: std::vec::Vec<(&str, std::vec::Vec<u128>)> =
            variants.iter().map(|v| (v.0, vec![])).collect();
        samples.push(("memcpy_only", vec![]));
        for rep in 0..reps {
            // rotate order each repetition so no variant always runs first
            for k in 0..variants.len() {
                let i = (k + rep) % variants.len();
                samples[i]
                    .1
                    .push(run(variants[i].0, n, rounds, variants[i].1));
            }
            let last = samples.len() - 1;
            samples[last].1.push(run_memcpy_only(n, rounds));
        }
        for (name, mut s) in samples {
            s.sort();
            println!(
                "{{\"elements\":{n},\"rounds\":{rounds},\"variant\":\"{name}\",\"median_ns\":{},\"min_ns\":{},\"max_ns\":{}}}",
                s[s.len() / 2],
                s[0],
                s[s.len() - 1]
            );
        }
    }
}
