//! Handle identity and counter deltas are independent from runtime-specific absolute counts.
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped};

/// Owned metadata/counter observation, without any class-instance or mutable-state retention.
pub(super) struct StaticObservation {
    name: &'static str,
    parent: &'static str,
    chain: String,
    repeated_slot: bool,
    class_slot: bool,
    same_descriptor: bool,
    ownership: OwnershipDeltas,
}

struct OwnershipDeltas {
    borrowed: i32,
    copied: i32,
    dropped: i32,
}

fn ownership_deltas<T: RuntimeTyped>(slot: &'static Handle<RuntimeType>) -> OwnershipDeltas {
    let initial = slot.ref_count();
    let _borrowed: &'static Handle<RuntimeType> = T::static_type_ref();
    let borrowed = slot.ref_count() - initial;
    let owned = slot.clone();
    let copied = slot.ref_count() - initial;
    drop(owned);
    OwnershipDeltas {
        borrowed,
        copied,
        dropped: slot.ref_count() - initial,
    }
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

/// Observe the actual static slot and descriptor; `observe::<ConcreteOwner>()`.
pub(super) fn observe<T: RuntimeTyped>() -> StaticObservation {
    let slot: &'static Handle<RuntimeType> = T::static_type_ref();
    let descriptor = slot
        .get()
        .expect("static descriptor slot; expected non-null handle");
    StaticObservation {
        name: descriptor.name(),
        parent: descriptor.parent().map_or("<none>", RuntimeType::name),
        chain: descriptor_chain(Some(descriptor)),
        repeated_slot: std::ptr::eq(slot, T::static_type_ref()),
        class_slot: std::ptr::eq(slot, descriptor.class_handle_ref()),
        same_descriptor: std::ptr::eq(descriptor, T::static_type()),
        ownership: ownership_deltas::<T>(slot),
    }
}

impl OwnershipDeltas {
    fn verify(&self, name: &str) {
        assert_eq!(self.borrowed, 0, "owner={name}; expected borrowing delta0");
        assert_eq!(
            self.copied, 1,
            "owner={name}; expected one owned handle delta1"
        );
        assert_eq!(
            self.dropped, 0,
            "owner={name}; expected restored reference count"
        );
    }
}

impl StaticObservation {
    /// Assert the declared name/parent and handle contract; `observation.verify(name, parent)`.
    pub(super) fn verify(&self, name: &str, parent: &str) {
        assert_eq!(self.name, name, "expected declared descriptor name");
        assert_eq!(
            self.parent, parent,
            "owner={name}; expected declared descriptor parent"
        );
        assert!(
            self.repeated_slot && self.class_slot && self.same_descriptor,
            "owner={name}; expected canonical static handle/object identity"
        );
        self.ownership.verify(name);
    }
    /// Emit only comparable metadata and deltas; `observation.render(name)`.
    pub(super) fn render(&self, owner: &str) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            owner,
            self.name,
            self.chain,
            self.repeated_slot,
            self.same_descriptor,
            self.ownership.borrowed,
            self.ownership.copied,
            self.ownership.dropped,
        )
    }
}
