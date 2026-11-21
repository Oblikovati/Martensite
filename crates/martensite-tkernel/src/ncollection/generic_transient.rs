//! Descriptor registration for handle-shared generic collections.

/// Implement named object identity for each instantiation of a collection.
/// A Rust generic cannot own a static per instantiation, so the descriptor
/// registry selects the canonical value by `TypeId`.
macro_rules! impl_generic_transient {
    ($class:ident, $name:literal, $parent:ty) => {
        impl<T: Send + Sync + 'static> $crate::standard::RuntimeTyped for $class<T> {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                $crate::standard::RuntimeType::register_generic::<Self>(
                    $name,
                    std::mem::size_of::<Self>(),
                    <$parent as $crate::standard::RuntimeTyped>::static_type(),
                )
            }
        }
        impl<T: Send + Sync + 'static> $crate::standard::Transient for $class<T> {
            fn dynamic_type(&self) -> &'static $crate::standard::RuntimeType {
                <Self as $crate::standard::RuntimeTyped>::static_type()
            }
        }
    };
}
pub(crate) use impl_generic_transient;
