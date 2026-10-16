use super::{InterfaceEntry, RuntimeType, RuntimeTyped};
use crate::standard::{Handle, Transient, WeakHandle};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, PoisonError},
};

type Registry = Mutex<HashMap<TypeId, RegistryEntry>>;
enum RegistryEntry {
    Owned(WeakHandle<RuntimeType>),
    LegacyBorrowed(&'static RuntimeType),
}
type StaticOwners = Mutex<HashMap<TypeId, &'static StaticDescriptorOwner>>;
struct StaticDescriptorOwner {
    class_handle: OnceLock<StaticClassHandle>,
    storage: OnceLock<Arc<dyn Transient>>,
    /// The class's own static handle (the `static` of `impl_transient!`), once published.
    published: OnceLock<&'static Handle<RuntimeType>>,
    /// A process-lived handle for classes that publish none.
    fallback: OnceLock<Handle<RuntimeType>>,
}
enum StaticClassHandle {
    Owned(Handle<RuntimeType>),
}
impl StaticDescriptorOwner {
    fn new() -> Self {
        Self {
            class_handle: OnceLock::new(),
            storage: OnceLock::new(),
            published: OnceLock::new(),
            fallback: OnceLock::new(),
        }
    }
    fn descriptor(&'static self) -> &'static RuntimeType {
        if let Some(handle) = self.class_handle.get() {
            return handle.descriptor();
        }
        RuntimeType::project_from(self.storage.get().expect("pinned descriptor").as_ref())
            .expect("stored runtime descriptor")
    }
}
impl StaticClassHandle {
    fn descriptor(&'static self) -> &'static RuntimeType {
        match self {
            Self::Owned(handle) => handle.get().expect("owned class descriptor"),
        }
    }
}
fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(Registry::default)
}
fn static_owners() -> &'static StaticOwners {
    static OWNERS: OnceLock<StaticOwners> = OnceLock::new();
    OWNERS.get_or_init(StaticOwners::default)
}
fn static_owner<T: ?Sized + 'static>() -> &'static StaticDescriptorOwner {
    static_owner_of(TypeId::of::<T>())
}
fn static_owner_of(type_id: TypeId) -> &'static StaticDescriptorOwner {
    let mut owners = static_owners()
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if let Some(owner) = owners.get(&type_id) {
        return owner;
    }
    let owner = Box::leak(Box::new(StaticDescriptorOwner::new()));
    owners.insert(type_id, owner);
    owner
}
/// Records `slot` as the static handle of class `T`; the first publication wins.
pub(super) fn publish_class_slot<T: ?Sized + 'static>(slot: &'static Handle<RuntimeType>) {
    let _ = static_owner::<T>().published.set(slot);
}
/// The process-lived handle that owns `descriptor`: the class's published static handle,
/// its retained class handle, or a fallback handle made once.
pub(super) fn class_slot(descriptor: &'static RuntimeType) -> &'static Handle<RuntimeType> {
    let owner = static_owner_of(descriptor.rust_type);
    let holds = |slot: &Handle<RuntimeType>| {
        slot.get()
            .is_some_and(|held| std::ptr::eq(held, descriptor))
    };
    if let Some(slot) = owner.published.get().filter(|slot| holds(slot)) {
        return slot;
    }
    if let Some(StaticClassHandle::Owned(slot)) = owner.class_handle.get()
        && holds(slot)
    {
        return slot;
    }
    owner
        .fallback
        .get_or_init(|| Handle::from_static(descriptor))
}
pub(super) fn register_owned<T: ?Sized + 'static>(
    name: &str,
    size: usize,
    mut parent: Option<Handle<RuntimeType>>,
    interfaces: Vec<InterfaceEntry>,
) -> Handle<RuntimeType> {
    let type_id = TypeId::of::<T>();
    let mut entries = registry().lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(existing) = owned_or_legacy(type_id, &entries) {
        drop(entries);
        drop(parent);
        return existing;
    }
    let candidate = Handle::new(RuntimeType::new::<T>(name, size, parent.take(), interfaces));
    entries.insert(type_id, RegistryEntry::Owned(candidate.downgrade()));
    candidate
}
pub(super) fn register_legacy_borrowed<T: ?Sized + 'static>(
    name: &str,
    size: usize,
    parent: Option<&'static RuntimeType>,
    interfaces: Vec<InterfaceEntry>,
) -> &'static RuntimeType {
    let type_id = TypeId::of::<T>();
    let mut entries = registry().lock().unwrap_or_else(PoisonError::into_inner);
    let existing = legacy_or_owned(type_id, &entries);
    if let Some(existing) = existing {
        drop(entries);
        drop(interfaces);
        return resolve_legacy_existing::<T>(existing);
    }
    create_legacy::<T>(type_id, name, size, parent, interfaces, &mut entries)
}
fn resolve_legacy_existing<T: ?Sized + 'static>(existing: LegacyExisting) -> &'static RuntimeType {
    match existing {
        LegacyExisting::Borrowed(descriptor) => descriptor,
        LegacyExisting::Owned(owner) => retain_storage_owner::<T>(owner),
    }
}
enum LegacyExisting {
    Borrowed(&'static RuntimeType),
    Owned(Handle<RuntimeType>),
}
fn legacy_or_owned(
    type_id: TypeId,
    entries: &HashMap<TypeId, RegistryEntry>,
) -> Option<LegacyExisting> {
    match entries.get(&type_id)? {
        RegistryEntry::LegacyBorrowed(descriptor) => Some(LegacyExisting::Borrowed(descriptor)),
        RegistryEntry::Owned(weak) => {
            let owner = weak.upgrade();
            (!owner.is_null()).then_some(LegacyExisting::Owned(owner))
        }
    }
}
fn create_legacy<T: ?Sized + 'static>(
    type_id: TypeId,
    name: &str,
    size: usize,
    parent: Option<&'static RuntimeType>,
    interfaces: Vec<InterfaceEntry>,
    entries: &mut HashMap<TypeId, RegistryEntry>,
) -> &'static RuntimeType {
    let descriptor = Box::leak(Box::new(RuntimeType::new::<T>(
        name,
        size,
        parent.map(Handle::from_static),
        interfaces,
    )));
    entries.insert(type_id, RegistryEntry::LegacyBorrowed(descriptor));
    descriptor
}
fn owned_or_legacy(
    type_id: TypeId,
    entries: &HashMap<TypeId, RegistryEntry>,
) -> Option<Handle<RuntimeType>> {
    match entries.get(&type_id)? {
        RegistryEntry::LegacyBorrowed(descriptor) => Some(Handle::from_static(*descriptor)),
        RegistryEntry::Owned(weak) => {
            let handle = weak.upgrade();
            (!handle.is_null()).then_some(handle)
        }
    }
}
fn retain_storage_owner<T: ?Sized + 'static>(handle: Handle<RuntimeType>) -> &'static RuntimeType {
    let owner = static_owner::<T>();
    if owner.class_handle.get().is_none() {
        let storage = handle
            .clone_storage_owner()
            .expect("owned descriptor storage");
        let _ = owner.storage.set(storage);
    }
    drop(handle);
    owner.descriptor()
}
pub(super) fn retain_class_handle<T: ?Sized + 'static>(
    handle: Handle<RuntimeType>,
) -> &'static RuntimeType {
    let owner = static_owner::<T>();
    let _ = owner.class_handle.set(StaticClassHandle::Owned(handle));
    owner.descriptor()
}
pub(super) fn retain_class_descriptor<T: ?Sized + 'static>(
    descriptor: &'static RuntimeType,
) -> &'static RuntimeType {
    let handle = registered_handle::<T>()
        .unwrap_or_else(|| panic!("no registered descriptor for {}", descriptor.name()));
    assert!(std::ptr::eq(
        handle.get().expect("registered descriptor"),
        descriptor
    ));
    retain_class_handle::<T>(handle)
}
fn registered_handle<T: ?Sized + 'static>() -> Option<Handle<RuntimeType>> {
    let entries = registry().lock().unwrap_or_else(PoisonError::into_inner);
    match entries.get(&TypeId::of::<T>())? {
        RegistryEntry::LegacyBorrowed(descriptor) => Some(Handle::from_static(*descriptor)),
        RegistryEntry::Owned(weak) => {
            let handle = weak.upgrade();
            (!handle.is_null()).then_some(handle)
        }
    }
}
pub(super) fn remove_owned(type_id: TypeId, address: *const ()) {
    let mut entries = registry().lock().unwrap_or_else(PoisonError::into_inner);
    let matches = match entries.get(&type_id) {
        Some(RegistryEntry::Owned(weak)) => weak.object_address() == address,
        _ => false,
    };
    if matches {
        entries.remove(&type_id);
    }
}
