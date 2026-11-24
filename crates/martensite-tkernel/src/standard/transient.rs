//! Minimal root interface for dynamically named Rust objects.
use super::{RuntimeType, RuntimeTyped, runtime_type::c_prefix};
use std::{any::Any, sync::OnceLock};

/// Runtime class identity for objects that can cross a kernel interface.
pub trait Transient: Any + Send + Sync {
    /// Dispatch the object's runtime descriptor; `object.dynamic_type()`.
    fn dynamic_type(&self) -> &'static RuntimeType;

    /// Select the descriptor holding this Rust object's interface conversions.
    /// By default it is the dynamic descriptor; `object.rust_interface_descriptor()`.
    fn rust_interface_descriptor(&self) -> &'static RuntimeType {
        self.dynamic_type()
    }

    /// Check exact class identity by descriptor; `object.is_instance(Class::static_type())`.
    fn is_instance(&self, exact: &RuntimeType) -> bool {
        std::ptr::eq(self.dynamic_type(), exact)
    }

    /// Check the exact class name, through the first NUL; `object.is_instance_name("Thing")`.
    fn is_instance_name(&self, name: &str) -> bool {
        self.dynamic_type().name() == c_prefix(name)
    }

    /// Check this class and its ancestors; `object.is_kind(<dyn Transient>::static_type())`.
    fn is_kind(&self, base: &RuntimeType) -> bool {
        self.dynamic_type().is_subtype(base)
    }

    /// Check class ancestry by name; `object.is_kind_name("Standard_Transient")`.
    fn is_kind_name(&self, name: &str) -> bool {
        self.dynamic_type().is_subtype_name(name)
    }
}

/// A root-class instance without embedded bookkeeping; `Handle::new(StandardTransient)`.
#[derive(Clone, Copy, Default)]
pub struct StandardTransient;

impl RuntimeTyped for dyn Transient {
    const TYPE_NAME: &'static str = "Standard_Transient";
    fn static_type() -> &'static RuntimeType {
        static DESCRIPTOR: OnceLock<&'static RuntimeType> = OnceLock::new();
        DESCRIPTOR
            .get_or_init(|| RuntimeType::register::<StandardTransient>(Self::TYPE_NAME, 0, None))
    }
}

impl RuntimeTyped for StandardTransient {
    const TYPE_NAME: &'static str = <dyn Transient>::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        <dyn Transient>::static_type()
    }
}

impl Transient for StandardTransient {
    fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
}

/// Declare a named Rust object and its actual trait conversions.
/// `impl_transient!(Line, "Geom_Line", dyn Curve, interfaces: [dyn Curve])`.
#[macro_export]
macro_rules! impl_transient {
    ($class:ty, $name:literal, $parent:ty) => {
        $crate::impl_transient!($class, $name, $parent, interfaces: []);
    };
    ($class:ty, $name:literal, $parent:ty, handle_key_policy: $policy:ty) => {
        $crate::impl_transient!($class, $name, $parent, interfaces: [], handle_key_policy: $policy);
    };
    ($class:ty, $name:literal, $parent:ty, interfaces: [$($interface:ty),* $(,)?] $(, handle_key_policy: $policy:ty)?) => {
        impl $crate::standard::RuntimeTyped for $class {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                static DESCRIPTOR: std::sync::OnceLock<&'static $crate::standard::RuntimeType> =
                    std::sync::OnceLock::new();
                DESCRIPTOR.get_or_init(|| {
                    $crate::standard::RuntimeType::register_with_interfaces::<Self>(
                        $name,
                        std::mem::size_of::<Self>(),
                        Some(<$parent as $crate::standard::RuntimeTyped>::static_type()),
                        vec![$($crate::standard::InterfaceEntry::new(
                            $crate::standard::InterfaceCaster::<$interface>::new(
                                |object| $crate::standard::project_concrete::<$class>(object)
                                    .map(|concrete| concrete as &$interface),
                                |owner| <$class as $crate::standard::TransientHandleTarget>::from_transient(owner)
                                    .map(|concrete| concrete as std::sync::Arc<$interface>),
                            )
                        )),*],
                    )
                })
            }
            $(fn handle_key_hash(handle: &$crate::standard::Handle<Self>) -> usize {
                $crate::ncollection::HashPolicy::hash(&<$policy>::default(), handle)
            }
            fn handle_key_equal(first: &$crate::standard::Handle<Self>, second: &$crate::standard::Handle<Self>) -> bool {
                $crate::ncollection::HashPolicy::equal(&<$policy>::default(), first, second)
            })?
        }

        impl $crate::standard::Transient for $class {
            fn dynamic_type(&self) -> &'static $crate::standard::RuntimeType {
                <Self as $crate::standard::RuntimeTyped>::static_type()
            }
        }

        $(impl $crate::standard::HandleUpcast<$interface> for $class {
            fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<$interface> {
                owner
            }
        })*
    };
}

/// Give a transient trait a class descriptor and its Rust ownership conversions.
/// `impl_transient_interface!(dyn Curve, "Geom_Curve", dyn Geometry)`.
#[macro_export]
macro_rules! impl_transient_interface {
    ($interface:ty, $name:literal, $parent:ty) => {
        $crate::impl_transient_interface!($interface, $name, $parent, upcasts: []);
    };
    ($interface:ty, $name:literal, $parent:ty, upcasts: [$($ancestor:ty),* $(,)?]) => {
        impl $crate::standard::RuntimeTyped for $interface {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                static DESCRIPTOR: std::sync::OnceLock<&'static $crate::standard::RuntimeType> =
                    std::sync::OnceLock::new();
                DESCRIPTOR.get_or_init(|| {
                    $crate::standard::RuntimeType::register::<Self>(
                        $name,
                        0,
                        Some(<$parent as $crate::standard::RuntimeTyped>::static_type()),
                    )
                })
            }
        }
        $crate::impl_transient_interface_target!($interface);

        $(impl $crate::standard::HandleUpcast<$ancestor> for $interface {
            fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<$ancestor> {
                owner
            }
        })*
    };
}

/// Connect a transient trait that shares an existing class descriptor to handle conversions.
/// `impl_transient_interface_target!(dyn AllocatorSource)`.
#[macro_export]
macro_rules! impl_transient_interface_target {
    ($interface:ty) => {
        impl $crate::standard::TransientHandleTarget for $interface {
            fn as_transient(&self) -> &dyn $crate::standard::Transient {
                self
            }
            fn into_transient(
                owner: std::sync::Arc<Self>,
            ) -> std::sync::Arc<dyn $crate::standard::Transient> {
                owner
            }
            fn borrow_from_transient(object: &dyn $crate::standard::Transient) -> Option<&Self> {
                $crate::standard::project_interface(object)
            }
            fn from_transient(
                owner: std::sync::Arc<dyn $crate::standard::Transient>,
            ) -> Option<std::sync::Arc<Self>> {
                $crate::standard::own_interface(owner)
            }
        }

        impl $crate::standard::HandleUpcast<dyn $crate::standard::Transient> for $interface {
            fn upcast_arc(
                owner: std::sync::Arc<Self>,
            ) -> std::sync::Arc<dyn $crate::standard::Transient> {
                owner
            }
        }
    };
}
