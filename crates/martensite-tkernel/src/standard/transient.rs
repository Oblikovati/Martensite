//! Transient base with atomic reference-count ordering and safe base projections.
use super::{
    Failure, Handle, RuntimeType, RuntimeTyped, exceptions, project_concrete,
    runtime_type::c_prefix,
};
use std::{
    any::{Any, TypeId},
    sync::{
        Once, OnceLock,
        atomic::{AtomicI32, Ordering, fence},
    },
};
/// Runtime object interface retained by every owning handle.
pub trait Transient: Any + Send + Sync {
    /// Dispatch the most-derived runtime descriptor; `object.dynamic_type()`.
    fn dynamic_type(&self) -> &'static RuntimeType;
    /// Choose Rust trait casters independently of inherited runtime types.
    /// Example: `object.rust_interface_descriptor().name()`; ordinarily this is `dynamic_type()`.
    fn rust_interface_descriptor(&self) -> &RuntimeType {
        self.dynamic_type()
    }
    /// Own the dynamic class descriptor; `object.dynamic_type_handle()`.
    fn dynamic_type_handle(&self) -> Handle<RuntimeType> {
        Handle::from_static(self.dynamic_type())
    }
    /// The most-derived class's process-lived descriptor handle;
    /// `object.dynamic_type_ref().get()`.
    fn dynamic_type_ref(&self) -> &'static Handle<RuntimeType> {
        self.dynamic_type().class_handle_ref()
    }
    /// The handle names exactly the dynamic
    /// type (a null handle never does); `object.is_instance_handle(T::static_type_ref())`.
    fn is_instance_handle(&self, exact: &Handle<RuntimeType>) -> bool {
        exact
            .get()
            .is_some_and(|exact| std::ptr::eq(self.dynamic_type(), exact))
    }
    /// Whether the dynamic type is a subtype of `base`, false for
    /// a null handle; `object.is_kind_handle(Base::static_type_ref())`.
    fn is_kind_handle(&self, base: &Handle<RuntimeType>) -> bool {
        base.get().is_some_and(|base| self.is_kind(base))
    }
    /// Project an actual base subobject, without pointer reinterpreting; `object.project(TypeId::of::<Base>())`.
    fn project(&self, requested: TypeId) -> Option<&dyn Any>;
    /// Borrow the embedded reference-count base; `object.transient_base().ref_count()`.
    fn transient_base(&self) -> &StandardTransient;
    /// True when the dynamic type is exactly `exact` (descriptor identity,
    /// handle comparison); `object.is_instance(StandardTransient::static_type())`.
    fn is_instance(&self, exact: &RuntimeType) -> bool {
        std::ptr::eq(self.dynamic_type(), exact)
    }
    /// True when the dynamic type's name equals `name` compared as a C string (it ends
    /// at a NUL); `object.is_instance_name("Standard_Transient")`.
    fn is_instance_name(&self, name: &str) -> bool {
        self.dynamic_type().name() == c_prefix(name)
    }
    /// True when the dynamic type is `base` or inherits from it; `object.is_kind(base)`.
    fn is_kind(&self, base: &RuntimeType) -> bool {
        self.dynamic_type().is_subtype(base)
    }
    /// True when the dynamic type or an ancestor is named `name`; `object.is_kind_name("Standard_Transient")`.
    fn is_kind_name(&self, name: &str) -> bool {
        self.dynamic_type().is_subtype_name(name)
    }
    /// What the last owning handle does at count zero. The default drops
    /// `storage`, destroying the object (`delete this`); an override may keep `storage`,
    /// retaining the constructed object, and destroy it later by dropping it.
    /// `fn delete(&self, storage: TransientStorage) { self.keep.set(storage) }`.
    fn delete(&self, storage: TransientStorage) {
        drop(storage);
    }
}

/// The storage of an object whose last handle was released, lent to [`Transient::delete`].
///
/// It is not a counted owner (the reference count stays zero); dropping it destroys the
/// object once no other storage holder remains. `storage.object().transient_base().ref_count()`.
pub struct TransientStorage {
    object: std::sync::Arc<dyn Transient>,
}

impl TransientStorage {
    pub(super) fn new(object: std::sync::Arc<dyn Transient>) -> Self {
        Self { object }
    }
    /// Borrow the retained object; `storage.object().dynamic_type().name()`.
    pub fn object(&self) -> &dyn Transient {
        self.object.as_ref()
    }
}

impl std::fmt::Debug for TransientStorage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "TransientStorage({})",
            self.object.dynamic_type().name()
        )
    }
}
/// Root object with atomic reference counting; `StandardTransient::default()`.
#[repr(C)]
#[derive(Default)]
pub struct StandardTransient {
    references: AtomicI32,
}
impl StandardTransient {
    /// Read the intrusive count; a standalone object has zero, `object.ref_count()`.
    pub fn ref_count(&self) -> i32 {
        self.references.load(Ordering::Relaxed)
    }
    /// Atomically acquire one reference; `object.increment_ref_counter()`.
    pub fn increment_ref_counter(&self) {
        self.references.fetch_add(1, Ordering::Relaxed);
    }
    /// Release one reference using release/acquire ordering; `object.decrement_ref_counter()`.
    pub fn decrement_ref_counter(&self) -> i32 {
        let remaining: i32 = self
            .references
            .fetch_sub(1, Ordering::Release)
            .wrapping_sub(1);
        if remaining == 0 {
            fence(Ordering::Acquire);
        }
        remaining
    }
    /// Reject an object no handle owns; the message is fixed
    /// (the offending value is the reference count 0); `object.this()?`.
    pub fn this(&self) -> Result<&Self, Failure> {
        if self.ref_count() == 0 {
            return Err(exceptions::program_error(
                "Attempt to create handle to object created in stack, not yet constructed, or destroyed",
            ));
        }
        Ok(self)
    }
}
impl Clone for StandardTransient {
    fn clone(&self) -> Self {
        Self::default()
    }
    fn clone_from(&mut self, _: &Self) { /* Native assignment preserves the counter. */
    }
}
impl RuntimeTyped for StandardTransient {
    const TYPE_NAME: &'static str = "Standard_Transient";
    fn static_type() -> &'static RuntimeType {
        Self::static_type_ref()
            .get()
            .expect("registered descriptor")
    }
    fn static_type_ref() -> &'static Handle<RuntimeType> {
        static DESCRIPTOR: OnceLock<Handle<RuntimeType>> = OnceLock::new();
        static PUBLISHED: Once = Once::new();
        let slot = DESCRIPTOR.get_or_init(|| {
            RuntimeType::register_owned::<Self>(Self::TYPE_NAME, std::mem::size_of::<Self>(), None)
        });
        PUBLISHED.call_once(|| RuntimeType::publish_class_handle_ref::<Self>(slot));
        slot
    }
    fn project_from(object: &dyn Transient) -> Option<&Self> {
        project_concrete::<Self>(object)
    }
}
impl Transient for StandardTransient {
    fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
    fn dynamic_type_ref(&self) -> &'static Handle<RuntimeType> {
        Self::static_type_ref()
    }
    fn project(&self, requested: TypeId) -> Option<&dyn Any> {
        (requested == TypeId::of::<Self>()).then_some(self)
    }
    fn transient_base(&self) -> &StandardTransient {
        self
    }
}
/// Implement safe single inheritance through an actual embedded parent field, with a
/// registered runtime type descriptor.
///
/// `$parent` names the descriptor parent; `$field` is the embedded subobject that
/// base projections and the reference counter delegate to. The optional
/// `interfaces` list registers one caster per abstract ancestor trait, so that
/// `Handle<dyn Trait>` can view the object.
/// A `handle_key_policy: MyPolicy` clause selects default-map hashing and equality
/// by static handle type; ordinary handles keep identity and Rust operators are unchanged.
/// Example: `impl_transient!(Derived, "Derived", Base, parent)` or
/// `impl_transient!(Circle, "Circle", dyn Conic, parent, interfaces: [dyn Curve, dyn Conic])`.
#[macro_export]
macro_rules! impl_transient {
    ($class:ty, $name:literal, $parent:ty, $field:ident, handle_key_policy: $policy:ty) => {
        $crate::impl_transient!($class, $name, $parent, $field, interfaces: [], handle_key_policy: $policy);
    };
    ($class:ty, $name:literal, $parent:ty, $field:ident) => {
        $crate::impl_transient!($class, $name, $parent, $field, interfaces: []);
    };
    ($class:ty, $name:literal, $parent:ty, $field:ident, interfaces: [$($interface:ty),* $(,)?] $(, handle_key_policy: $policy:ty)?) => {
        impl $crate::standard::RuntimeTyped for $class {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                Self::static_type_ref().get().expect("registered descriptor")
            }
            fn static_type_ref() -> &'static $crate::standard::Handle<$crate::standard::RuntimeType> {
                static DESCRIPTOR: std::sync::OnceLock<$crate::standard::Handle<$crate::standard::RuntimeType>> =
                    std::sync::OnceLock::new();
                static PUBLISHED: std::sync::Once = std::sync::Once::new();
                let slot = DESCRIPTOR.get_or_init(|| {
                    $crate::standard::RuntimeType::register_owned_with_interfaces::<Self>(
                            $name,
                            std::mem::size_of::<Self>(),
                            Some(<$parent as $crate::standard::RuntimeTyped>::static_type_handle()),
                            vec![$($crate::standard::InterfaceEntry::new(
                                $crate::standard::InterfaceCaster::<$interface>::new(|object| {
                                    let concrete: &$class =
                                        $crate::standard::project_concrete::<$class>(object)?;
                                    Some(concrete as &$interface)
                                }),
                            )),*],
                        )
                });
                PUBLISHED.call_once(|| $crate::standard::RuntimeType::publish_class_handle_ref::<Self>(slot));
                slot
            }
            fn project_from(object: &dyn $crate::standard::Transient) -> Option<&Self> {
                $crate::standard::project_concrete::<Self>(object)
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
            fn dynamic_type_ref(&self) -> &'static $crate::standard::Handle<$crate::standard::RuntimeType> {
                <Self as $crate::standard::RuntimeTyped>::static_type_ref()
            }
            fn project(&self, requested: std::any::TypeId) -> Option<&dyn std::any::Any> {
                if requested == std::any::TypeId::of::<Self>() {
                    return Some(self);
                }
                $crate::standard::Transient::project(&self.$field, requested)
            }
            fn transient_base(&self) -> &$crate::standard::StandardTransient {
                $crate::standard::Transient::transient_base(&self.$field)
            }
        }
    };
}
/// Register an abstract class, written as a trait, in the runtime type system.
///
/// The trait must have `Transient` as a supertrait. Handles of the trait object
/// (`Handle<dyn Trait>`) then see every concrete class that lists the trait in
/// the `interfaces` of its `impl_transient!`. A trait object has no size, so the
/// descriptor takes its parent's size: `is_subtype` stops at the first ancestor
/// smaller than the base, and an abstract class is never smaller than its parent.
/// Example: `impl_transient_interface!(dyn Curve, "Curve", dyn Geometry)`.
#[macro_export]
macro_rules! impl_transient_interface {
    ($interface:ty, $name:literal, $parent:ty) => {
        impl $crate::standard::RuntimeTyped for $interface {
            const TYPE_NAME: &'static str = $name;
            fn static_type() -> &'static $crate::standard::RuntimeType {
                Self::static_type_ref()
                    .get()
                    .expect("registered descriptor")
            }
            fn static_type_ref() -> &'static $crate::standard::Handle<$crate::standard::RuntimeType>
            {
                static DESCRIPTOR: std::sync::OnceLock<
                    $crate::standard::Handle<$crate::standard::RuntimeType>,
                > = std::sync::OnceLock::new();
                static PUBLISHED: std::sync::Once = std::sync::Once::new();
                let slot = DESCRIPTOR.get_or_init(|| {
                    let parent = <$parent as $crate::standard::RuntimeTyped>::static_type_handle();
                    let size = parent.get().expect("parent descriptor").size();
                    $crate::standard::RuntimeType::register_owned::<Self>($name, size, Some(parent))
                });
                PUBLISHED.call_once(|| {
                    $crate::standard::RuntimeType::publish_class_handle_ref::<Self>(slot)
                });
                slot
            }
            fn project_from(object: &dyn $crate::standard::Transient) -> Option<&Self> {
                $crate::standard::project_interface::<Self>(object)
            }
        }
    };
}
