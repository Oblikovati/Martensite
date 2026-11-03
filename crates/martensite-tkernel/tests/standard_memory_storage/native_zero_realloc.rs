//! Query the host allocator independently of Martensite's owned storage adapter.
#![allow(unsafe_code)]

/// Observe the zero-size result; `existing_zero_reallocation_returns_null()`.
pub(super) fn existing_zero_reallocation_returns_null() -> bool {
    let moved = reallocate_runtime_zero();
    let refused = moved.is_null();
    if !refused {
        // SAFETY: a non-null result is a live C allocation, including a zero-size one.
        unsafe { libc::free(moved) };
    }
    refused
}

fn reallocate_runtime_zero() -> *mut libc::c_void {
    // Reallocation calls realloc directly and raises only on null.
    // SAFETY: the allocated pointer is passed once to realloc; only its result is freed.
    unsafe {
        let original = libc::malloc(8);
        assert!(
            !original.is_null(),
            "malloc(8) returned null; expected probe storage"
        );
        // #369: LLVM may replace realloc(p, 0) with a constant result. Query the
        // actual runtime provider through an opaque typed pointer instead.
        let resize = std::hint::black_box(
            libc::realloc as unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void,
        );
        resize(original, 0)
    }
}
