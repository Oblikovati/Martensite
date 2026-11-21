//! Canonical class descriptors for runtime names and persisted class ancestry.
use super::{Handle, Transient};
use std::{
    any::{Any, TypeId},
    fmt,
    io::{self, Write},
    sync::Arc,
};
mod interface;
mod registry;
pub use interface::{
    InterfaceCaster, InterfaceEntry, TransientHandleTarget, own_interface, project_concrete,
    project_interface,
};

/// A process-lived class descriptor with its Rust type, recorded name and ancestry.
pub struct RuntimeType {
    system_name: &'static str,
    name: Box<str>,
    size: usize,
    rust_type: TypeId,
    parent: Option<&'static RuntimeType>,
    interfaces: Vec<InterfaceEntry>,
}

crate::impl_transient!(RuntimeType, "Standard_Type", dyn Transient);

/// The bytes of a C string argument, up to its first NUL.
pub(crate) fn c_prefix(text: &str) -> &str {
    text.split('\0').next().unwrap_or_default()
}

impl RuntimeType {
    /// Return or register `T`'s static descriptor. The first registration fixes its class metadata.
    /// `RuntimeType::register::<T>("T", size_of::<T>(), Some(parent))`.
    pub fn register<T: ?Sized + 'static>(
        name: &str,
        size: usize,
        parent: Option<&'static RuntimeType>,
    ) -> &'static RuntimeType {
        Self::register_with_interfaces::<T>(name, size, parent, Vec::new())
    }

    /// Register a class with its optional abstract object conversions.
    /// `RuntimeType::register_with_interfaces::<T>("T", size_of::<T>(), parent, entries)`.
    pub fn register_with_interfaces<T: ?Sized + 'static>(
        name: &str,
        size: usize,
        parent: Option<&'static RuntimeType>,
        interfaces: Vec<InterfaceEntry>,
    ) -> &'static RuntimeType {
        registry::register::<T>(name, size, parent, interfaces, true)
    }

    pub(crate) fn register_generic<T: 'static>(
        name: &str,
        size: usize,
        parent: &'static RuntimeType,
    ) -> &'static Self {
        registry::register::<T>(name, size, Some(parent), Vec::new(), false)
    }

    /// Find an already registered, unambiguous class name for persistence.
    ///
    /// Generic collection names require a concrete Rust instantiation.
    /// `RuntimeType::registered_by_name("Units_Token")` gives a descriptor
    /// when the name identifies one registered class.
    pub fn registered_by_name(name: &str) -> Option<&'static Self> {
        registry::by_name(c_prefix(name))
    }

    fn new<T: ?Sized + 'static>(
        name: &str,
        size: usize,
        parent: Option<&'static RuntimeType>,
        interfaces: Vec<InterfaceEntry>,
    ) -> Self {
        Self {
            system_name: std::any::type_name::<T>(),
            name: c_prefix(name).into(),
            size,
            rust_type: TypeId::of::<T>(),
            parent,
            interfaces,
        }
    }

    /// The descriptor of a registered class; `RuntimeType::instance::<dyn Transient>().name()`.
    pub fn instance<T: ?Sized + RuntimeTyped>() -> &'static RuntimeType {
        T::static_type()
    }

    /// The Rust type name; `descriptor.system_name()`.
    pub fn system_name(&self) -> &str {
        self.system_name
    }

    /// The class name used by persistence; `descriptor.name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The Rust object's size in bytes; `descriptor.size()`.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The Rust type identity associated with this descriptor; `descriptor.rust_type()`.
    pub fn rust_type(&self) -> TypeId {
        self.rust_type
    }

    /// The descriptor's direct parent; `descriptor.parent()`.
    pub fn parent(&self) -> Option<&'static Self> {
        self.parent
    }

    pub(super) fn interface_caster<I: ?Sized + 'static>(&self) -> Option<&InterfaceCaster<I>> {
        self.interfaces
            .iter()
            .find_map(|entry| entry.caster.downcast_ref())
    }

    /// Check class ancestry by descriptor identity; `class.is_subtype(base)`.
    pub fn is_subtype(&self, base: &Self) -> bool {
        self.ancestry().any(|ancestor| std::ptr::eq(ancestor, base))
    }

    /// Check class ancestry by C-string name; `class.is_subtype_name("Standard_Transient")`.
    pub fn is_subtype_name(&self, name: &str) -> bool {
        let name = c_prefix(name);
        self.ancestry().any(|ancestor| ancestor.name() == name)
    }

    fn ancestry(&self) -> impl Iterator<Item = &Self> {
        std::iter::successors(Some(self), |class| class.parent())
    }

    /// Write the descriptor address and name; `descriptor.print(&mut out)?`.
    pub fn print(&self, stream: &mut impl Write) -> io::Result<()> {
        write!(stream, "{self}")
    }
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:p} : {}", self, self.name)
    }
}

impl fmt::Debug for RuntimeType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

/// Declares one named Rust class or interface and its class descriptor.
pub trait RuntimeTyped: 'static {
    /// Class name without registering it first; `<dyn Transient>::TYPE_NAME`.
    const TYPE_NAME: &'static str;
    /// Return the class's canonical descriptor; `T::static_type()`.
    fn static_type() -> &'static RuntimeType;
    /// Select hashing by the static handle type; `T::handle_key_hash(&handle)`.
    fn handle_key_hash(handle: &Handle<Self>) -> usize {
        handle.hash_code()
    }
    /// Select map key equality independently of handle identity; `T::handle_key_equal(&a, &b)`.
    fn handle_key_equal(first: &Handle<Self>, second: &Handle<Self>) -> bool {
        first == second
    }
}
