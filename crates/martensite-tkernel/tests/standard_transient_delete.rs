//! The transient delete hook as a virtual hook distinct from destruction, and the
//! static descriptor handles that the static and dynamic type queries return, against the
//! expected rows.
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardPersistent, StandardTransient, Transient,
    TransientStorage, project_concrete,
};
use std::any::{Any, TypeId};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicI32, Ordering},
};

const NATIVE: &str = include_str!("fixtures/tkernel_transient_delete.expected");

fn native(label: &str) -> String {
    let line = NATIVE.lines().find(|line| line.starts_with(label)).unwrap();
    line[label.len() + 1..].to_owned()
}

/// What the derived test classes record.
#[derive(Default)]
struct DeleteSignals {
    calls: AtomicI32,
    destroyed: AtomicI32,
    counter: AtomicI32,
    kept: Mutex<Option<TransientStorage>>,
}

/// A class whose delete override keeps the object (a retaining object);
/// `retain: false` gives the default delete (a counting object).
struct DeleteProbe {
    base: StandardTransient,
    signals: Arc<DeleteSignals>,
    retain: bool,
}

impl Drop for DeleteProbe {
    fn drop(&mut self) {
        self.signals.destroyed.fetch_add(1, Ordering::SeqCst);
    }
}

impl RuntimeTyped for DeleteProbe {
    const TYPE_NAME: &'static str = "DeleteProbe";
    fn static_type() -> &'static RuntimeType {
        StandardTransient::static_type()
    }
    fn project_from(object: &dyn Transient) -> Option<&Self> {
        project_concrete(object)
    }
}

impl Transient for DeleteProbe {
    fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
    fn project(&self, requested: TypeId) -> Option<&dyn Any> {
        if requested == TypeId::of::<Self>() {
            return Some(self);
        }
        self.base.project(requested)
    }
    fn transient_base(&self) -> &StandardTransient {
        &self.base
    }
    fn delete(&self, storage: TransientStorage) {
        if !self.retain {
            return drop(storage);
        }
        self.signals.calls.fetch_add(1, Ordering::SeqCst);
        let count = self.base.ref_count();
        self.signals.counter.store(count, Ordering::SeqCst);
        *self.signals.kept.lock().unwrap() = Some(storage);
    }
}

fn probe(retain: bool) -> (Handle<DeleteProbe>, Arc<DeleteSignals>) {
    let signals = Arc::new(DeleteSignals::default());
    let object = DeleteProbe {
        base: StandardTransient::default(),
        signals: Arc::clone(&signals),
        retain,
    };
    (Handle::new(object), signals)
}

#[test]
fn retaining_delete_keeps_the_object_until_explicit_destruction() {
    let (first, signals) = probe(true);
    let second = first.clone();
    drop(first);
    drop(second);
    let kept = signals
        .kept
        .lock()
        .unwrap()
        .take()
        .expect("retained storage");
    let observed = format!(
        "{} {} {} {}",
        signals.calls.load(Ordering::SeqCst),
        signals.counter.load(Ordering::SeqCst),
        signals.destroyed.load(Ordering::SeqCst),
        kept.object().transient_base().ref_count()
    );
    assert_eq!(observed, native("retained"));
    drop(kept);
    let destroyed = signals.destroyed.load(Ordering::SeqCst);
    assert_eq!(destroyed.to_string(), native("explicit"));
}

#[test]
fn default_delete_destroys_with_the_last_handle() {
    let (handle, signals) = probe(false);
    drop(handle);
    let destroyed = signals.destroyed.load(Ordering::SeqCst);
    assert_eq!(destroyed.to_string(), native("default"));
    assert_eq!(signals.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn a_non_final_release_does_not_delete() {
    let (first, signals) = probe(true);
    let second = first.clone();
    drop(first);
    assert_eq!(signals.calls.load(Ordering::SeqCst), 0);
    assert_eq!(second.ref_count(), 1);
}

fn flag(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

#[test]
fn descriptor_handles_are_the_one_static_slot() {
    let object = Handle::new(StandardTransient::default());
    let slot = StandardTransient::static_type_ref();
    let same = std::ptr::eq(slot, StandardTransient::static_type_ref());
    let dynamic = std::ptr::eq(slot, object.get().unwrap().dynamic_type_ref());
    assert_eq!(
        format!("{} {}", flag(same), flag(dynamic)),
        native("same_slot")
    );
    assert!(std::ptr::eq(
        slot.get().unwrap(),
        StandardTransient::static_type()
    ));
    assert!(std::ptr::eq(
        StandardTransient::static_type().class_handle_ref(),
        slot
    ));
    let type_slot = RuntimeType::static_type_ref();
    let type_dynamic = std::ptr::eq(type_slot, type_slot.get().unwrap().dynamic_type_ref());
    let persistent = StandardPersistent::static_type_ref()
        .get()
        .unwrap()
        .name()
        .to_owned();
    let observed = format!(
        "{} {} {persistent}",
        flag(type_dynamic),
        type_slot.get().unwrap().name()
    );
    assert_eq!(observed, native("type_slot"));
}

#[test]
fn handle_type_queries_match_native() {
    let object = Handle::new(StandardTransient::default());
    let object = object.get().unwrap();
    let null = Handle::<RuntimeType>::default();
    let nulls = format!(
        "{} {}",
        flag(object.is_instance_handle(&null)),
        flag(object.is_kind_handle(&null))
    );
    assert_eq!(nulls, native("null_queries"));
    let transient = StandardTransient::static_type_ref();
    let type_slot = RuntimeType::static_type_ref();
    let queries = format!(
        "{} {} {} {}",
        flag(object.is_instance_handle(transient)),
        flag(object.is_kind_handle(transient)),
        flag(object.is_instance_handle(type_slot)),
        flag(type_slot.get().unwrap().is_kind_handle(transient))
    );
    assert_eq!(queries, native("handle_queries"));
}

#[test]
fn generic_descriptor_handles_resolve_to_the_retained_class_slot() {
    let array = Handle::new(HArray1::<u16>::default());
    let slot = <HArray1<u16> as RuntimeTyped>::static_type_ref();
    assert!(std::ptr::eq(slot, array.get().unwrap().dynamic_type_ref()));
    assert_eq!(slot.get().unwrap().name(), "NCollection_HArray1");
}
