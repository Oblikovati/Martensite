//! The native release is safe to call at any time and leaves live data intact.
use super::{HeapRelease, NativeHeapRelease};

#[test]
fn native_release_keeps_live_allocations_and_can_repeat() {
    let live: Vec<Vec<u8>> = (0..64).map(|index| vec![index as u8; 4096]).collect();
    let freed: Vec<Vec<u8>> = (0..256).map(|_| vec![0; 64 * 1024]).collect();
    drop(freed);
    let first = NativeHeapRelease.release_free_pages();
    let _second = NativeHeapRelease.release_free_pages();
    for (index, block) in live.iter().enumerate() {
        assert!(block.iter().all(|byte| *byte == index as u8));
    }
    if !cfg!(all(target_os = "linux", target_env = "gnu")) {
        assert!(!first, "only glibc has pages to release on request");
    }
}
