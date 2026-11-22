//! Interface aliases for classes with both standard concrete and polymorphic forms.
macro_rules! canonical_object_model_type {
    ($interface:ty, $concrete:ty, [$($ancestor:ty),* $(,)?]) => {
        impl martensite_tkernel::standard::RuntimeTyped for $interface {
            const TYPE_NAME: &'static str = <$concrete as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
            fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
                <$concrete as martensite_tkernel::standard::RuntimeTyped>::static_type()
            }
        }
        martensite_tkernel::impl_transient_interface_target!($interface);

        $(impl martensite_tkernel::standard::HandleUpcast<$ancestor> for $interface {
            fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<$ancestor> {
                owner
            }
        })*
    };
}
pub(super) use canonical_object_model_type;
