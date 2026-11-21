//! Handle operators: `Deref`, equality, ordering and hashing.
use super::{Handle, RuntimeTyped};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::Deref,
};
impl<T: ?Sized> Deref for Handle<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
            .expect("Null handle; expected a live object for dereference")
    }
}
impl<T: ?Sized, U: ?Sized> PartialEq<Handle<U>> for Handle<T> {
    fn eq(&self, other: &Handle<U>) -> bool {
        self.object_address() == other.object_address()
    }
}
impl<T: ?Sized> Eq for Handle<T> {}
/// Formats handle identity without borrowing object contents; `format!("{handle:?}")`.
impl<T: ?Sized> std::fmt::Debug for Handle<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Handle")
            .field("type", &std::any::type_name::<T>())
            .field("identity", &self.object_address())
            .field("is_null", &self.is_null())
            .finish()
    }
}
impl<T: RuntimeTyped> PartialEq<*const T> for Handle<T> {
    fn eq(&self, other: &*const T) -> bool {
        self.as_ptr() == *other
    }
}
impl<T: RuntimeTyped> PartialEq<Handle<T>> for *const T {
    fn eq(&self, other: &Handle<T>) -> bool {
        *self == other.as_ptr()
    }
}
/// Orders by object address, null first.
impl<T: ?Sized, U: ?Sized> PartialOrd<Handle<U>> for Handle<T> {
    fn partial_cmp(&self, other: &Handle<U>) -> Option<Ordering> {
        Some(self.hash_code().cmp(&other.hash_code()))
    }
}
impl<T: ?Sized> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash_code().cmp(&other.hash_code())
    }
}
/// Hashes the object address that [`Handle::hash_code`] returns.
impl<T: ?Sized> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_code().hash(state);
    }
}
/// The default map policy hashes the object address; Rust operators retain identity.
impl<T: ?Sized + RuntimeTyped> crate::ncollection::HashKey for Handle<T> {
    fn kernel_hash(&self) -> usize {
        T::handle_key_hash(self)
    }
    fn kernel_equal(&self, other: &Self) -> bool {
        T::handle_key_equal(self, other)
    }
}
