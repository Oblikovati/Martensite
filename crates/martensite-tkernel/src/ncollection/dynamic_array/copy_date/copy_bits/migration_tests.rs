//! Copy-policy compatibility and workload baselines (#83).
use super::{AllocVec, BaseAllocator, copy_bitwise_values};
use std::cell::Cell;

#[derive(Copy)]
struct CloneProbe<'a>(&'a Cell<usize>, u64);
#[allow(clippy::non_canonical_clone_impl)]
impl Clone for CloneProbe<'_> {
    fn clone(&self) -> Self {
        self.0.set(self.0.get() + 1);
        *self
    }
}

#[test]
fn copy_bypasses_clone_and_keeps_reserved_pointer_and_capacity() {
    let calls = Cell::new(0);
    let source = [CloneProbe(&calls, 11), CloneProbe(&calls, 17)];
    let mut block = AllocVec::with_capacity_in(7, BaseAllocator::default());
    let pointer = block.as_ptr();
    let capacity = block.capacity();
    copy_bitwise_values(&source, &mut block);
    assert_eq!(calls.get(), 0);
    assert_eq!(block.as_ptr(), pointer);
    assert_eq!(block.capacity(), capacity);
    assert_eq!(block[1].1, 17);
    block[0].1 = 42;
    assert_eq!(source[0].1, 11);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(align(128))]
struct AlignedWord(u64);

#[test]
fn copying_empty_zero_sized_and_overaligned_values_preserves_storage() {
    let mut empty = AllocVec::<u64, _>::new_in(BaseAllocator::default());
    copy_bitwise_values(&[], &mut empty);
    assert_eq!(empty.capacity(), 0);
    let mut zero = AllocVec::new_in(BaseAllocator::default());
    copy_bitwise_values(&[(); 23], &mut zero);
    assert_eq!(zero.len(), 23);
    let mut aligned = AllocVec::with_capacity_in(2, BaseAllocator::default());
    let pointer = aligned.as_ptr();
    copy_bitwise_values(&[AlignedWord(7), AlignedWord(9)], &mut aligned);
    assert_eq!(aligned.as_ptr(), pointer);
    assert_eq!(pointer.addr() % 128, 0);
    assert_eq!(&aligned[..], &[AlignedWord(7), AlignedWord(9)]);
}

#[test]
#[should_panic(expected = "expected empty copy destination")]
fn copy_rejects_an_initialized_destination() {
    let mut block = AllocVec::with_capacity_in(2, BaseAllocator::default());
    block.push(1);
    copy_bitwise_values(&[2], &mut block);
}

#[test]
#[should_panic(expected = "expected at least 1 source values")]
fn copy_rejects_insufficient_capacity() {
    copy_bitwise_values(&[1], &mut AllocVec::new_in(BaseAllocator::default()));
}

#[test]
#[ignore = "manual release copy workload; compare identical inputs before and after #83"]
fn copy_storage_workload() {
    let source: Vec<u64> = (0..65_536).collect();
    let mut block = AllocVec::with_capacity_in(source.len(), BaseAllocator::default());
    let started = std::time::Instant::now();
    for _ in 0..1_000 {
        copy_bitwise_values(std::hint::black_box(&source), &mut block);
        assert_eq!(std::hint::black_box(&block).last(), Some(&65_535));
        block.clear();
    }
    println!(
        "copy_storage_workload elements=65536 rounds=1000 bytes=524288000 elapsed_ns={}",
        started.elapsed().as_nanos()
    );
}
