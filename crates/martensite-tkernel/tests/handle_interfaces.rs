//! Handles of abstract classes written as traits (`Handle<dyn Trait>`): RTTI,
//! down casts, identity and shared mutation through interior mutability.
use martensite_tkernel::standard::{
    Handle, HandleFieldRef, HandleReference, ProjectedHandleReference, RuntimeType, RuntimeTyped,
    SharedState, StandardTransient, Transient,
};
use martensite_tkernel::{impl_transient, impl_transient_interface};
use std::sync::RwLock;

trait FakeShape: Transient {
    fn area(&self) -> f64;
    fn scale(&self, factor: f64);
}
impl_transient_interface!(dyn FakeShape, "Fake_Shape", StandardTransient);

trait FakeRound: FakeShape {
    fn radius(&self) -> f64;
}
impl_transient_interface!(dyn FakeRound, "Fake_Round", dyn FakeShape, upcasts: [dyn FakeShape]);

trait FakeUnrelated: Transient {}
impl_transient_interface!(dyn FakeUnrelated, "Fake_Unrelated", StandardTransient);

#[derive(Default)]
struct FakeDisk {
    radius: RwLock<f64>,
}
impl_transient!(FakeDisk, "Fake_Disk", dyn FakeRound, interfaces: [dyn FakeShape, dyn FakeRound]);
impl FakeShape for FakeDisk {
    fn area(&self) -> f64 {
        let radius: f64 = *self.radius.read().unwrap();
        radius * radius * 3.0
    }
    fn scale(&self, factor: f64) {
        *self.radius.write().unwrap() *= factor;
    }
}
impl FakeRound for FakeDisk {
    fn radius(&self) -> f64 {
        *self.radius.read().unwrap()
    }
}

#[derive(Default)]
struct FakeSquare {}
impl_transient!(FakeSquare, "Fake_Square", dyn FakeShape, interfaces: [dyn FakeShape]);
impl FakeShape for FakeSquare {
    fn area(&self) -> f64 {
        1.0
    }
    fn scale(&self, _factor: f64) {}
}

fn disk(radius: f64) -> Handle<FakeDisk> {
    Handle::new(FakeDisk {
        radius: RwLock::new(radius),
    })
}

#[test]
fn mutation_through_concrete_handle_is_visible_through_interface_handle() {
    let concrete: Handle<FakeDisk> = disk(2.0);
    let shape: Handle<dyn FakeShape> = concrete.upcast::<dyn FakeShape>();
    assert!(!shape.is_null());
    assert_eq!(shape.area(), 12.0);
    concrete.scale(0.5);
    assert_eq!(shape.area(), 3.0);
    shape.scale(4.0);
    assert_eq!(concrete.radius(), 4.0);
    assert_eq!(concrete.ref_count(), 2);
}

#[test]
fn interface_observer_uses_registered_projection_without_intrusive_ownership() {
    let concrete = disk(2.0);
    let projected = concrete.view::<dyn FakeShape>();
    assert_eq!(projected.object_address(), concrete.object_address());
    assert_eq!(projected.get().unwrap().area(), 12.0);
    projected.get().unwrap().scale(0.5);
    assert_eq!(concrete.radius(), 1.0);
    assert_eq!(concrete.ref_count(), 1);
    assert!(concrete.view::<dyn FakeUnrelated>().is_null());
}

fn projected_disk_slot(slot: &Handle<FakeDisk>) -> &Handle<FakeDisk> {
    // The selector must return the caller's actual storage, not a retained target snapshot.
    slot
}
#[test]
fn projected_reference_rereads_replacement_and_null_with_scoped_snapshots() {
    let first = disk(2.0);
    let second = disk(3.0);
    let fields = SharedState::new(first.clone());
    let alias = HandleFieldRef::new(&fields, projected_disk_slot);
    let projected = ProjectedHandleReference::<FakeDisk, dyn FakeShape>::new(&alias);
    assert_eq!(projected.reference_address(), alias.reference_address());
    assert_projected_disk_target(&projected, &first, 2);
    *fields.write() = second.clone();
    assert_projected_disk_target(&projected, &second, 2);
    assert_eq!(first.ref_count(), 1);
    *fields.write() = Handle::default();
    assert!(projected.observe().is_null());
    assert_eq!(second.ref_count(), 1);
}
fn assert_projected_disk_target(
    reference: &dyn HandleReference<dyn FakeShape>,
    expected: &Handle<FakeDisk>,
    owners: usize,
) {
    let observed = reference.observe();
    assert_eq!(observed.object_address(), expected.object_address());
    assert_eq!(observed.area(), expected.area());
    assert_eq!(observed.ref_count(), owners + 1);
}
#[test]
fn failed_projection_preserves_slot_identity_without_acquiring_a_target() {
    let concrete = disk(2.0);
    let projected = ProjectedHandleReference::<FakeDisk, dyn FakeUnrelated>::new(&concrete);
    assert!(projected.observe().is_null());
    assert_eq!(
        projected.reference_address(),
        std::ptr::from_ref(&concrete).cast()
    );
    assert_eq!(concrete.ref_count(), 1);
}

#[test]
fn interface_handles_down_cast_and_keep_identity() {
    let concrete: Handle<FakeDisk> = disk(1.0);
    let shape: Handle<dyn FakeShape> = concrete.cast();
    let round: Handle<dyn FakeRound> = shape.cast();
    let back: Handle<FakeDisk> = round.cast();
    assert!(!round.is_null() && !back.is_null());
    assert_eq!(round.upcast::<dyn FakeShape>(), shape);
    assert!(back == concrete && round == shape);
    assert_eq!(back.as_ptr(), concrete.as_ptr());
    assert_eq!(shape.object_address(), concrete.object_address());
    assert!(shape.cast::<FakeSquare>().is_null());
    assert!(shape.cast::<dyn FakeUnrelated>().is_null());
    assert!(shape.upcast::<dyn Transient>().is_some());
}

#[test]
fn interface_descriptors_form_the_native_hierarchy() {
    let square: Handle<dyn FakeShape> = Handle::new(FakeSquare::default()).cast();
    assert!(square.is_kind(<dyn FakeShape>::static_type()));
    assert!(!square.is_kind(<dyn FakeRound>::static_type()));
    assert!(square.cast::<dyn FakeRound>().is_null());
    let round: Handle<dyn FakeRound> = disk(1.0).cast();
    assert!(round.is_kind(<dyn FakeShape>::static_type()));
    assert!(round.is_kind_name("Standard_Transient"));
    assert!(round.is_instance_name("Fake_Disk"));
    assert_eq!(
        round.dynamic_type().unwrap().parent().unwrap().name(),
        "Fake_Round"
    );
}

#[test]
fn null_interface_handles_and_views() {
    let null: Handle<dyn FakeShape> = Handle::default();
    assert!(null.is_null() && null.get().is_none());
    assert!(null.cast::<FakeDisk>().is_null());
    assert!(null.object_address().is_null());
    let concrete: Handle<FakeDisk> = disk(3.0);
    let view = concrete.view::<dyn FakeRound>();
    assert!(!view.is_null());
    assert_eq!(view.get().unwrap().radius(), 3.0);
    assert!(concrete.view::<dyn FakeUnrelated>().is_null());
    assert_eq!(concrete.ref_count(), 1);
}

#[test]
fn interface_handles_share_mutation_across_threads() {
    let shape: Handle<dyn FakeShape> = disk(1.0).cast();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let local: Handle<dyn FakeShape> = shape.clone();
            scope.spawn(move || local.scale(2.0));
        }
    });
    assert_eq!(shape.cast::<dyn FakeRound>().radius(), 16.0);
    assert_eq!(shape.ref_count(), 1);
}

#[test]
fn abstract_descriptors_can_have_a_zero_size() {
    let shape: &RuntimeType = <dyn FakeShape>::static_type();
    let round: &RuntimeType = <dyn FakeRound>::static_type();
    assert_eq!(shape.size(), StandardTransient::static_type().size());
    assert_eq!(round.size(), shape.size());
    let disk_handle: Handle<dyn FakeRound> = disk(1.0).cast();
    assert!(disk_handle.is_kind(StandardTransient::static_type()));
    assert!(disk_handle.is_kind(round) && disk_handle.is_kind(shape));
    assert!(!disk_handle.is_kind(<dyn FakeUnrelated>::static_type()));
}

#[test]
fn interface_handles_order_and_hash_by_object_identity() {
    let concrete: Handle<FakeDisk> = disk(1.0);
    let shape: Handle<dyn FakeShape> = concrete.cast();
    let round: Handle<dyn FakeRound> = concrete.cast();
    assert_eq!(shape.hash_code(), concrete.hash_code());
    assert_eq!(shape.partial_cmp(&round), Some(std::cmp::Ordering::Equal));
    let null: Handle<dyn FakeShape> = Handle::default();
    assert!(null < shape && null.hash_code() == 0);
    let moved: Handle<dyn FakeRound> = shape.into_cast::<dyn FakeRound>();
    assert!(moved == round && concrete.ref_count() == 3);
}

#[test]
fn handle_and_weak_sizes_match_their_native_shared_pointers() {
    use martensite_tkernel::standard::WeakHandle;
    assert_eq!(size_of::<Handle<FakeDisk>>(), size_of::<usize>());
    assert_eq!(size_of::<Handle<dyn FakeShape>>(), 2 * size_of::<usize>());
    assert_eq!(size_of::<WeakHandle<FakeDisk>>(), size_of::<usize>());
    assert_eq!(
        size_of::<WeakHandle<dyn FakeShape>>(),
        2 * size_of::<usize>()
    );
}
