//! Typed allocator rebinding preserves a shared allocation resource.
use super::{AllocatorList, BaseAllocator};
use crate::standard::Failure;
use std::{marker::PhantomData, ptr::NonNull};
mod element;
pub use element::{AllocatorTypes, StandardAllocator};

/// Typed view of an optional allocator.
///
/// `None` is the null handle, which allocates through the common optimal allocation
/// and compares unequal to every non-null allocator, including the common one.
/// Example: `TypedAllocator::<u8>::default().rebind::<i32>()`.
pub struct TypedAllocator<T> {
    resource: Option<BaseAllocator>,
    element: PhantomData<fn() -> T>,
}

impl<T> AllocatorTypes for TypedAllocator<T> {
    type Value = T;
    type Pointer = *mut T;
    type ConstPointer = *const T;
    type Size = usize;
    type Difference = isize;
    type Reference<'a>
        = &'a mut T
    where
        T: 'a;
    type ConstReference<'a>
        = &'a T
    where
        T: 'a;
}

impl<T> TypedAllocator<T> {
    /// Binds an element type to a resource; `TypedAllocator::<i32>::new(arena)`.
    pub fn new(resource: BaseAllocator) -> Self {
        Self::with_allocator(Some(resource))
    }
    /// Binds an element type to a possibly null resource.
    ///
    /// Example: `TypedAllocator::<i32>::with_allocator(None) == TypedAllocator::default()`.
    pub fn with_allocator(resource: Option<BaseAllocator>) -> Self {
        Self {
            resource,
            element: PhantomData,
        }
    }
    /// Changes element type while retaining resource identity (converting constructor).
    ///
    /// Example: `allocator.rebind::<f64>()`.
    pub fn rebind<U>(&self) -> TypedAllocator<U> {
        TypedAllocator::with_allocator(self.resource.clone())
    }
    /// Copies another element type's resource; `a.assign(&b)`.
    pub fn assign<U>(&mut self, other: &TypedAllocator<U>) {
        self.assign_returning(other);
    }
    /// Replaces the resource; `allocator.set_allocator(None)`.
    pub fn set_allocator(&mut self, resource: Option<BaseAllocator>) {
        self.resource = resource;
    }
    /// The resource, `None` for the null handle; `allocator.allocator()`.
    pub fn allocator(&self) -> Option<&BaseAllocator> {
        self.resource.as_ref()
    }
    /// The resource used for storage: the null handle means the common Standard policy.
    fn effective(&self) -> BaseAllocator {
        self.resource.clone().unwrap_or_else(BaseAllocator::common)
    }

    /// Raw storage for `count` elements: the resource's optimal allocation, or
    /// the common optimal allocation for the null handle. `None` where the allocator returns null
    /// or throw, including a byte count that overflows. Example: `allocator.allocate(4)`.
    pub fn allocate(&self, count: usize) -> Option<NonNull<T>> {
        let bytes = count.checked_mul(size_of::<T>())?;
        Some(self.effective().allocate_optimal_raw(bytes)?.cast())
    }

    /// Releases storage from `allocate` (`deallocate`; the templated overload is `cast` away).
    ///
    /// # Safety
    /// As `BaseAllocator::free_raw`: `pointer` must come from `allocate` on an equal allocator.
    pub unsafe fn deallocate(&self, pointer: NonNull<T>, _count: usize) -> Result<(), Failure> {
        // SAFETY: forwarded from this function's contract.
        unsafe { self.effective().free_raw(pointer.cast()) }
    }

    /// Estimated maximum element count (`max_size`); zero-sized types report `usize::MAX`.
    ///
    /// Example: `TypedAllocator::<u64>::default().max_size() == usize::MAX / 8`.
    pub fn max_size(&self) -> usize {
        usize::MAX / size_of::<T>().max(1)
    }

    /// Creates contiguous storage using this resource; `allocator.vector().push(value)`.
    pub fn vector(&self) -> AllocatorVector<T> {
        AllocatorVector {
            elements: allocator_api2::vec::Vec::new_in(self.effective()),
        }
    }
    /// Creates linked storage using this resource for each node; `allocator.list().push_back(value)`.
    pub fn list(&self) -> AllocatorList<T> {
        AllocatorList::with_allocator(self.effective())
    }
}

impl<T> Default for TypedAllocator<T> {
    /// The null-handle allocator.
    fn default() -> Self {
        Self::with_allocator(None)
    }
}
impl<T> Clone for TypedAllocator<T> {
    fn clone(&self) -> Self {
        self.rebind()
    }
}
impl<T, U> PartialEq<TypedAllocator<U>> for TypedAllocator<T> {
    fn eq(&self, other: &TypedAllocator<U>) -> bool {
        self.resource == other.resource
    }
}
impl<T> Eq for TypedAllocator<T> {}
impl<T> std::fmt::Debug for TypedAllocator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypedAllocator")
            .field(&self.resource)
            .finish()
    }
}

/// Contiguous allocator-owned elements; `allocator.vector()`.
pub struct AllocatorVector<T> {
    elements: allocator_api2::vec::Vec<T, BaseAllocator>,
}
impl<T> AllocatorVector<T> {
    /// Appends an element, growing through the selected allocator; `vector.push(42)`.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }
}
impl<T: Clone> AllocatorVector<T> {
    /// Extends by cloning a fill value; `vector.resize(10, 0)`.
    pub fn resize(&mut self, length: usize, value: T) {
        self.elements.resize(length, value);
    }
}
impl<T> std::ops::Deref for AllocatorVector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.elements
    }
}
impl<T> std::ops::DerefMut for AllocatorVector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }
}

mod method_contracts;
