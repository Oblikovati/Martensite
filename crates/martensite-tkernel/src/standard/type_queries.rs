//! Free helpers over handles and runtime types.
//!
//! Each helper accepts a raw transient reference or a handle. A
//! raw pointer is an `Option<&dyn Transient>` here (`None` is `nullptr`); the handle
//! forms carry a `_handle` suffix because Rust has no overloading.
use super::{Handle, HandleView, RuntimeType, RuntimeTyped, Transient};

/// The object is not null and is a `T` or inherits from it;
/// `is_kind::<StandardTransient>(Some(&object))`.
pub fn is_kind<T: RuntimeTyped>(object: Option<&dyn Transient>) -> bool {
    object.is_some_and(|object| object.is_kind(T::static_type()))
}
/// The same for a handle; `is_kind_handle::<StandardTransient, _>(&handle)`.
pub fn is_kind_handle<T: RuntimeTyped, B: RuntimeTyped>(object: &Handle<B>) -> bool {
    !object.is_null() && object.is_kind(T::static_type())
}
/// The object is not null and exactly a `T`;
/// `is_instance::<StandardTransient>(Some(&object))`.
pub fn is_instance<T: RuntimeTyped>(object: Option<&dyn Transient>) -> bool {
    object.is_some_and(|object| object.is_instance(T::static_type()))
}
/// The same for a handle; `is_instance_handle::<StandardTransient, _>(&handle)`.
pub fn is_instance_handle<T: RuntimeTyped, B: RuntimeTyped>(object: &Handle<B>) -> bool {
    !object.is_null() && object.is_instance(T::static_type())
}
/// The descriptor of `T`;
/// `type_of::<StandardTransient>().name() == "Standard_Transient"`.
pub fn type_of<T: RuntimeTyped>() -> &'static RuntimeType {
    T::static_type()
}
/// Down-casts a handle; `down_cast::<Derived, _>(&base)`.
pub fn down_cast<D: RuntimeTyped, B: RuntimeTyped>(object: &Handle<B>) -> Handle<D> {
    Handle::<D>::down_cast(object)
}
/// Down-casts a reference to an object a handle owns (a view is the pointer);
/// `down_cast_view::<Derived, _>(&base.view())`.
pub fn down_cast_view<D: RuntimeTyped, B: RuntimeTyped>(object: &HandleView<'_, B>) -> Handle<D> {
    Handle::<D>::from_view(object)
}
