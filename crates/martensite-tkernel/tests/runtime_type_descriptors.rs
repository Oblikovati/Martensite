//! Concurrent registration fixes one descriptor per class and retains its metadata for persistence.
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};

#[derive(Default)]
struct CanonicalParent;
impl_transient!(CanonicalParent, "CanonicalParent", dyn Transient);
struct CanonicalChild;
impl_transient!(CanonicalChild, "CanonicalChild", CanonicalParent);

#[test]
fn concurrent_class_registration_borrows_one_canonical_parent_and_child() {
    let descriptors = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(CanonicalChild::static_type))
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    let child = CanonicalChild::static_type();
    assert!(
        descriptors
            .into_iter()
            .all(|other| std::ptr::eq(other, child))
    );
    assert!(std::ptr::eq(
        child.parent().unwrap(),
        CanonicalParent::static_type()
    ));
    assert!(std::ptr::eq(
        RuntimeType::registered_by_name("CanonicalChild").unwrap(),
        child
    ));
}

#[test]
fn registration_detaches_name_storage_and_retains_the_first_class_metadata() {
    struct RuntimeNamedProbe;
    let name = String::from("RuntimeNamedProbe\0ignored");
    let class =
        RuntimeType::register::<RuntimeNamedProbe>(&name, 24, Some(CanonicalParent::static_type()));
    drop(name);
    let repeated = RuntimeType::register::<RuntimeNamedProbe>("Ignored", 0, None);
    assert!(std::ptr::eq(class, repeated));
    assert_eq!(class.size(), 24);
    assert_eq!(class.parent().unwrap().name(), "CanonicalParent");
    assert!(std::ptr::eq(
        RuntimeType::registered_by_name("RuntimeNamedProbe\0ignore").unwrap(),
        class
    ));
    assert!(RuntimeType::registered_by_name("Ignored").is_none());
}

#[test]
fn generic_instantiations_keep_identity_even_when_their_public_names_are_ambiguous() {
    use martensite_tkernel::ncollection::HArray1;
    let integers = HArray1::<i32>::static_type();
    assert!(RuntimeType::registered_by_name(integers.name()).is_none());
    let reals = HArray1::<f64>::static_type();
    assert!(!std::ptr::eq(integers, reals));
    assert_eq!(integers.name(), reals.name());
    assert!(RuntimeType::registered_by_name(integers.name()).is_none());
    assert!(integers.is_subtype_name("NCollection_HArray1"));
    assert!(reals.is_subtype_name("NCollection_HArray1"));
}
