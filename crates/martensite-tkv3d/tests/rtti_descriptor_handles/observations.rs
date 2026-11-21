//! Check canonical borrowed descriptors and their class ancestry.
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped};
use std::any::TypeId;

/// Descriptor observation without constructing a class instance.
pub(super) struct StaticObservation {
    name: &'static str,
    parent: &'static str,
    chain: String,
    canonical_descriptor: bool,
    same_rust_type: bool,
}

fn descriptor_chain(mut descriptor: Option<&'static RuntimeType>) -> String {
    let mut names: Vec<&str> = Vec::new();
    let mut visited: Vec<*const RuntimeType> = Vec::new();
    while let Some(current) = descriptor {
        let address = std::ptr::from_ref(current);
        assert!(
            !visited.contains(&address),
            "descriptor={}; expected acyclic parents",
            current.name()
        );
        visited.push(address);
        names.push(current.name());
        descriptor = current.parent();
    }
    names.join(">")
}

/// Observe the process-lived descriptor; `observe::<ConcreteOwner>()`.
pub(super) fn observe<T: RuntimeTyped>() -> StaticObservation {
    let descriptor = T::static_type();
    StaticObservation {
        name: descriptor.name(),
        parent: descriptor.parent().map_or("<none>", RuntimeType::name),
        chain: descriptor_chain(Some(descriptor)),
        canonical_descriptor: std::ptr::eq(descriptor, RuntimeType::instance::<T>()),
        same_rust_type: descriptor.rust_type() == TypeId::of::<T>(),
    }
}

impl StaticObservation {
    /// Assert the declared name, parent and descriptor identity; `observation.verify(name, parent)`.
    pub(super) fn verify(&self, name: &str, parent: &str) {
        assert_eq!(self.name, name, "expected declared descriptor name");
        assert_eq!(
            self.parent, parent,
            "owner={name}; expected declared descriptor parent"
        );
        assert!(
            self.canonical_descriptor && self.same_rust_type,
            "owner={name}; expected canonical descriptor for the Rust type"
        );
    }
    /// Emit the declared class metadata; `observation.render(name)`.
    pub(super) fn render(&self, owner: &str) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            owner, self.name, self.chain, self.canonical_descriptor, self.same_rust_type,
        )
    }
}
