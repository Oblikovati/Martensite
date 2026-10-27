//! Owner-aware descriptor borrows remain separate from projected-subobject queries.
use martensite_tkernel::standard::{
    Handle, RuntimeType, RuntimeTyped, StandardTransient, Transient,
};
use martensite_tkernel::{impl_transient, impl_transient_interface};

trait DescriptorProjectionView: Transient {
    fn marker(&self) -> u32;
}
impl_transient_interface!(
    dyn DescriptorProjectionView,
    "DescriptorProjectionView",
    StandardTransient
);

#[derive(Default)]
struct DescriptorProjectionBase {
    transient: StandardTransient,
}
impl_transient!(
    DescriptorProjectionBase,
    "DescriptorProjectionBase",
    StandardTransient,
    transient
);

struct DescriptorProjectionDerived {
    base: DescriptorProjectionBase,
    marker: u32,
}
impl_transient!(DescriptorProjectionDerived, "DescriptorProjectionDerived", DescriptorProjectionBase, base,
    interfaces: [dyn DescriptorProjectionView]);

impl DescriptorProjectionView for DescriptorProjectionDerived {
    fn marker(&self) -> u32 {
        // A usable interface projection verifies the actual owner, not only a nominal type name.
        self.marker
    }
}

fn descriptor_projection() -> Handle<DescriptorProjectionDerived> {
    let object = DescriptorProjectionDerived {
        base: DescriptorProjectionBase::default(),
        marker: 17,
    };
    Handle::new(object)
}

#[test]
fn concrete_and_interface_handles_borrow_the_most_derived_canonical_slot() {
    let derived = descriptor_projection();
    let base = derived.cast::<DescriptorProjectionBase>();
    let interface = derived.cast::<dyn DescriptorProjectionView>();
    let expected = DescriptorProjectionDerived::static_type_ref();
    assert!(std::ptr::eq(
        derived.dynamic_type_handle_ref().unwrap(),
        expected
    ));
    assert!(std::ptr::eq(
        base.dynamic_type_handle_ref().unwrap(),
        expected
    ));
    assert!(std::ptr::eq(
        interface.dynamic_type_handle_ref().unwrap(),
        expected
    ));
    assert_eq!(interface.marker(), 17);
    assert_eq!(base.object_address(), derived.object_address());
}

#[test]
fn explicit_and_implicit_subobject_queries_keep_their_existing_return_shape() {
    let derived = descriptor_projection();
    let base = derived.cast::<DescriptorProjectionBase>();
    let implicit: &'static Handle<RuntimeType> = base.dynamic_type_ref();
    let explicit = <DescriptorProjectionBase as Transient>::dynamic_type_ref(base.get().unwrap());
    assert!(std::ptr::eq(
        implicit,
        DescriptorProjectionBase::static_type_ref()
    ));
    assert!(std::ptr::eq(implicit, explicit));
    assert!(!std::ptr::eq(
        implicit,
        base.dynamic_type_handle_ref().unwrap()
    ));
    assert_eq!(
        base.dynamic_type().unwrap().name(),
        "DescriptorProjectionDerived"
    );
}

#[test]
fn null_concrete_and_interface_handles_have_no_borrowed_descriptor() {
    let concrete = Handle::<DescriptorProjectionBase>::default();
    let interface = Handle::<dyn DescriptorProjectionView>::default();
    assert!(concrete.dynamic_type_handle_ref().is_none());
    assert!(interface.dynamic_type_handle_ref().is_none());
    assert!(concrete.dynamic_type().is_none());
    assert!(concrete.dynamic_type_handle().is_none());
}

#[test]
fn borrowing_preserves_object_and_descriptor_counts_while_owned_query_adds_one() {
    // Only this case copies this private descriptor; other cases borrow its process-lived slot.
    let derived = descriptor_projection();
    let base = derived.cast::<DescriptorProjectionBase>();
    let slot = DescriptorProjectionDerived::static_type_ref();
    let descriptor_count = slot.ref_count();
    let object_count = derived.ref_count();
    assert!(std::ptr::eq(base.dynamic_type_handle_ref().unwrap(), slot));
    assert_eq!(slot.ref_count(), descriptor_count);
    assert_eq!(derived.ref_count(), object_count);
    let owned = base.dynamic_type_handle().unwrap();
    assert_eq!(owned.ref_count(), descriptor_count + 1);
    drop(owned);
    assert_eq!(slot.ref_count(), descriptor_count);
}

#[test]
fn borrowed_metadata_outlives_the_object_without_retaining_it() {
    let derived = descriptor_projection();
    let weak = derived.downgrade();
    let base = derived.cast::<DescriptorProjectionBase>();
    let metadata = base.dynamic_type_handle_ref().unwrap();
    drop(base);
    drop(derived);
    assert!(weak.upgrade().is_null());
    assert!(std::ptr::eq(
        metadata,
        DescriptorProjectionDerived::static_type_ref()
    ));
    assert_eq!(metadata.name(), "DescriptorProjectionDerived");
}
