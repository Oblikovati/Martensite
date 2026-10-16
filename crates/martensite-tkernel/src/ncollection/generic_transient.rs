//! Runtime types for the handle-shared collection templates (shared arrays
//! and sequences), which give every
//! instantiation the template's runtime name and the transient base as parent.
use crate::standard::{Handle, RuntimeType};

/// The descriptor of the instantiation `C`, named `name` with `parent`, registered once.
///
/// A Rust generic cannot own a `static` per instantiation, so the TKernel type registry,
/// keyed by `TypeId`, keeps one descriptor per instantiation:
/// `generic_static_type::<HArray1<f64>>("NCollection_HArray1", parent_handle)`.
pub(crate) fn generic_static_type<C: 'static>(
    name: &'static str,
    parent: Handle<RuntimeType>,
) -> &'static RuntimeType {
    let size = std::mem::size_of::<C>();
    let descriptor = RuntimeType::register_owned::<C>(name, size, Some(parent));
    RuntimeType::retain_class_descriptor_handle::<C>(descriptor)
}

/// Implements `RuntimeTyped` and `Transient` for a generic handle-shared collection
/// `$class<T>` whose `$parent` subobject sits in a field named `base`;
/// `impl_generic_transient!(HArray1, "NCollection_HArray1", StandardTransient)`.
macro_rules! impl_generic_transient {
    ($class:ident, $name:literal, $parent:ty) => {
        impl<T: Send + Sync + 'static> $crate::standard::RuntimeTyped for $class<T> {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                let parent = <$parent as $crate::standard::RuntimeTyped>::static_type_handle();
                super::generic_transient::generic_static_type::<Self>($name, parent)
            }
            fn project_from(object: &dyn $crate::standard::Transient) -> Option<&Self> {
                $crate::standard::project_concrete::<Self>(object)
            }
        }
        impl<T: Send + Sync + 'static> $crate::standard::Transient for $class<T> {
            fn dynamic_type(&self) -> &'static $crate::standard::RuntimeType {
                <Self as $crate::standard::RuntimeTyped>::static_type()
            }
            fn project(&self, requested: std::any::TypeId) -> Option<&dyn std::any::Any> {
                if requested == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }
                $crate::standard::Transient::project(&self.base, requested)
            }
            fn transient_base(&self) -> &$crate::standard::StandardTransient {
                &self.base
            }
        }
    };
}
pub(crate) use impl_generic_transient;
