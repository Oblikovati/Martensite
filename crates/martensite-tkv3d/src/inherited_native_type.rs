//! Carries the declared native class identity separately from its Rust interface conversions.
macro_rules! inherited_native_type {
    ($class:ty, $native:ty, $metadata:literal, $identity:ident, [$($interface:ty),+ $(,)?]) => {
        impl $class {
            fn native_interface_metadata() -> &'static martensite_tkernel::standard::RuntimeType {
                use martensite_tkernel::standard::{
                    InterfaceCaster, InterfaceEntry, RuntimeType, RuntimeTyped, TransientHandleTarget,
                    project_concrete,
                };
                static DESCRIPTOR: std::sync::OnceLock<&'static RuntimeType> =
                    std::sync::OnceLock::new();
                DESCRIPTOR.get_or_init(|| {
                    RuntimeType::register_with_interfaces::<$class>(
                        $metadata,
                        std::mem::size_of::<$class>(),
                        Some(<$native as RuntimeTyped>::static_type()),
                        vec![$(InterfaceEntry::new(InterfaceCaster::<$interface>::new(
                            |object| project_concrete::<$class>(object).map(|owner| owner as &$interface),
                            |owner| <$class as TransientHandleTarget>::from_transient(owner)
                                .map(|owner| owner as std::sync::Arc<$interface>),
                        ))),+],
                    )
                })
            }
        }

        $crate::inherited_native_type::inherited_native_type!(@identity $class, $native, $metadata, $identity);

        impl martensite_tkernel::standard::Transient for $class {
            fn dynamic_type(&self) -> &'static martensite_tkernel::standard::RuntimeType {
                <$native as martensite_tkernel::standard::RuntimeTyped>::static_type()
            }
            fn rust_interface_descriptor(&self) -> &'static martensite_tkernel::standard::RuntimeType {
                Self::native_interface_metadata()
            }
        }

        $(impl martensite_tkernel::standard::HandleUpcast<$interface> for $class {
            fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<$interface> {
                owner
            }
        })+
    };
    (@identity $class:ty, $native:ty, $metadata:literal, inherited) => {
        impl martensite_tkernel::standard::RuntimeTyped for $class {
            const TYPE_NAME: &'static str =
                <$native as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
            fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
                <$native as martensite_tkernel::standard::RuntimeTyped>::static_type()
            }
        }
    };
    (@identity $class:ty, $native:ty, $metadata:literal, named) => {
        impl martensite_tkernel::standard::RuntimeTyped for $class {
            const TYPE_NAME: &'static str = $metadata;
            fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
                Self::native_interface_metadata()
            }
        }
    };
}
pub(crate) use inherited_native_type;
