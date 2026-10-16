//! Inline layout covers ordinary T; over-aligned Rust values use an indirect slot.
use crate::ncollection::SparseArrayBlock;
use std::ffi::c_void;
fn sparse_indirect<T>() -> bool {
    align_of::<T>() > align_of::<usize>()
}
pub(in super::super) fn sparse_stride<T>() -> usize {
    if sparse_indirect::<T>() {
        size_of::<*mut T>()
    } else {
        // The slot stride is never zero, so a Rust ZST still gets an aligned address per slot.
        size_of::<T>().max(align_of::<T>())
    }
}
pub(in super::super) unsafe fn sparse_project<T>(slot: *mut c_void) -> *mut T {
    if sparse_indirect::<T>() {
        // SAFETY: the caller passes a live slot; for indirect T it holds the boxed pointer
        // sparse_construct wrote.
        unsafe { slot.cast::<*mut T>().read() }
    } else {
        slot.cast()
    }
}
pub(in super::super) unsafe fn sparse_construct<T>(slot: *mut c_void, value: T) {
    if sparse_indirect::<T>() {
        // SAFETY: the caller passes a writable slot of pointer size for an indirect T.
        unsafe { slot.cast::<*mut T>().write(Box::into_raw(Box::new(value))) };
    } else {
        // SAFETY: the caller passes a writable slot sized and aligned for a direct T.
        unsafe { slot.cast::<T>().write(value) };
    }
}
pub(in super::super) unsafe fn sparse_destroy<T>(slot: *mut c_void) {
    if sparse_indirect::<T>() {
        // SAFETY: the caller passes a live slot; for indirect T it owns the Box
        // sparse_construct created, freed once here.
        unsafe { drop(Box::from_raw(sparse_project::<T>(slot))) };
    } else {
        // SAFETY: the caller passes a live slot holding a direct T, dropped once here.
        unsafe { slot.cast::<T>().drop_in_place() };
    }
}
pub(in super::super) fn sparse_checked_bytes(items: usize, stride: usize) -> usize {
    SparseArrayBlock::checked_storage_size(items, stride).unwrap_or_else(|| {
        panic!("SparseArray block {items} items of stride {stride}; expected addressable layout")
    })
}
