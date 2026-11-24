//! Static descriptor borrows follow the object across Rust interface conversions.
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped, Transient};
use martensite_tkernel::{impl_transient, impl_transient_interface};

trait DescriptorReading: Transient {
    fn marker(&self) -> u32;
}
impl_transient_interface!(dyn DescriptorReading, "DescriptorReading", dyn Transient);

struct DescriptorProbe(u32);
impl_transient!(DescriptorProbe, "DescriptorProbe", dyn DescriptorReading,
    interfaces: [dyn DescriptorReading]);

impl DescriptorReading for DescriptorProbe {
    fn marker(&self) -> u32 {
        self.0
    }
}

#[test]
fn all_views_borrow_the_same_descriptor_and_keep_ownership_unchanged() {
    let object = Handle::new(DescriptorProbe(17));
    let interface = object.upcast::<dyn DescriptorReading>();
    let root = object.upcast::<dyn Transient>();
    let expected = DescriptorProbe::static_type();
    let descriptors = [
        object.dynamic_type(),
        interface.dynamic_type(),
        root.dynamic_type(),
    ];
    assert!(
        descriptors
            .into_iter()
            .all(|kind| std::ptr::eq(kind.unwrap(), expected))
    );
    assert_eq!(interface.marker(), 17);
    assert_eq!(root.object_address(), object.object_address());
    assert_eq!(object.ref_count(), 3);
}

#[test]
fn borrowed_metadata_outlives_the_object_without_retaining_it() {
    let interface = Handle::new(DescriptorProbe(17)).into_upcast::<dyn DescriptorReading>();
    let weak = interface.downgrade();
    let metadata = interface.dynamic_type().unwrap();
    drop(interface);
    assert!(weak.upgrade().is_null());
    assert!(std::ptr::eq(metadata, DescriptorProbe::static_type()));
    assert!(std::ptr::eq(
        metadata,
        RuntimeType::registered_by_name("DescriptorProbe").unwrap()
    ));
}

#[test]
fn null_handles_have_no_descriptor() {
    assert!(
        Handle::<DescriptorProbe>::default()
            .dynamic_type()
            .is_none()
    );
    assert!(
        Handle::<dyn DescriptorReading>::default()
            .dynamic_type()
            .is_none()
    );
}
