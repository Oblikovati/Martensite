use super::{InterfaceEntry, RuntimeType};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Mutex, OnceLock, PoisonError},
};

#[derive(Default)]
struct DescriptorRegistry {
    types: HashMap<TypeId, &'static RuntimeType>,
    names: HashMap<&'static str, Option<&'static RuntimeType>>,
}

fn registry() -> &'static Mutex<DescriptorRegistry> {
    static DESCRIPTORS: OnceLock<Mutex<DescriptorRegistry>> = OnceLock::new();
    DESCRIPTORS.get_or_init(Mutex::default)
}

pub(super) fn register<T: ?Sized + 'static>(
    name: &str,
    size: usize,
    parent: Option<&'static RuntimeType>,
    interfaces: Vec<InterfaceEntry>,
    resolve_name: bool,
) -> &'static RuntimeType {
    let mut registry = registry().lock().unwrap_or_else(PoisonError::into_inner);
    let identity = TypeId::of::<T>();
    if let Some(descriptor) = registry.types.get(&identity) {
        return descriptor;
    }
    // A process-lived descriptor has one address independent of any owned objects.
    let descriptor = Box::leak(Box::new(RuntimeType::new::<T>(
        name, size, parent, interfaces,
    )));
    registry.types.insert(identity, descriptor);
    registry
        .names
        .entry(descriptor.name())
        .and_modify(|first| *first = None)
        .or_insert(resolve_name.then_some(descriptor));
    descriptor
}

pub(super) fn by_name(name: &str) -> Option<&'static RuntimeType> {
    registry()
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .names
        .get(name)
        .copied()
        .flatten()
}
