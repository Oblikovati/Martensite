//! Exact named aliases must resolve to actual base types and support real projections.
use crate::{if_select::*, interface::*, transfer::*};
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use martensite_tkernel::tcollection::HAsciiString;
use std::any::TypeId;

fn assert_base_alias<A: ?Sized + 'static, Expected: ?Sized + 'static>() {
    assert_eq!(TypeId::of::<A>(), TypeId::of::<Expected>());
}

fn assert_no_polymorphic_root_view<T: RuntimeTyped>(owner: Handle<T>) {
    let embedded: Handle<StandardTransient> = owner.cast();
    assert_eq!(embedded.dynamic_type().unwrap().name(), T::TYPE_NAME);
    assert_eq!(
        embedded.get().unwrap().dynamic_type().name(),
        "Standard_Transient"
    );
}

#[test]
fn embedded_node_root_does_not_supply_a_polymorphic_borrowed_parent_view() {
    assert_no_polymorphic_root_view(Handle::new(InterfaceGlobalNodeOfGeneralLib::new()));
    assert_no_polymorphic_root_view(Handle::new(InterfaceNodeOfGeneralLib::new()));
    assert_no_polymorphic_root_view(Handle::new(InterfaceGlobalNodeOfReaderLib::new()));
    assert_no_polymorphic_root_view(Handle::new(InterfaceNodeOfReaderLib::new()));
}

#[test]
fn five_dispatch_base_aliases_are_the_abstract_dispatch_interface() {
    assert_base_alias::<IFSelectDispGlobalBaseType, dyn IFSelectDispatch>();
    assert_base_alias::<IFSelectDispPerOneBaseType, dyn IFSelectDispatch>();
    assert_base_alias::<IFSelectDispPerCountBaseType, dyn IFSelectDispatch>();
    assert_base_alias::<IFSelectDispPerFilesBaseType, dyn IFSelectDispatch>();
    assert_base_alias::<IFSelectDispPerSignatureBaseType, dyn IFSelectDispatch>();
}

#[test]
fn five_dispatch_base_aliases_project_to_live_trait_views() {
    assert_dispatch_projection::<_, IFSelectDispGlobalBaseType>(Handle::new(
        IFSelectDispGlobal::new(),
    ));
    assert_dispatch_projection::<_, IFSelectDispPerOneBaseType>(Handle::new(
        IFSelectDispPerOne::new(),
    ));
    assert_dispatch_projection::<_, IFSelectDispPerCountBaseType>(Handle::new(
        IFSelectDispPerCount::new(),
    ));
    assert_dispatch_projection::<_, IFSelectDispPerFilesBaseType>(Handle::new(
        IFSelectDispPerFiles::new(),
    ));
    assert_dispatch_projection::<_, IFSelectDispPerSignatureBaseType>(Handle::new(
        IFSelectDispPerSignature::new(),
    ));
}

fn assert_dispatch_projection<
    T: RuntimeTyped + IFSelectDispatch,
    Base: ?Sized + RuntimeTyped + IFSelectDispatch,
>(
    owner: Handle<T>,
) {
    let base: Handle<Base> = owner.cast();
    let name = Handle::new(HAsciiString::new("owned root"));
    base.set_root_name(&name);
    assert_eq!(owner.root_name(), name);
    assert!(std::ptr::eq(
        Base::static_type(),
        owner.dynamic_type().unwrap().parent().unwrap()
    ));
    assert_eq!(base.get().unwrap().dynamic_type().name(), T::TYPE_NAME);
    assert_eq!(Handle::<T>::down_cast(&base), owner);
}

#[test]
fn finder_actor_virtual_parent_view_shares_its_static_parent_descriptor() {
    let finder = Handle::new(TransferActorOfFinderProcess::new());
    let base: Handle<dyn TransferActorOps<TransferFinderKind>> = finder.cast();
    base.set_last(true);
    assert!(finder.is_last());
    assert!(std::ptr::eq(
        <dyn TransferActorOps<TransferFinderKind> as RuntimeTyped>::static_type(),
        finder.dynamic_type().unwrap().parent().unwrap()
    ));
    assert_eq!(
        base.get().unwrap().dynamic_type().name(),
        "TransferActorOfFinderProcess"
    );
}

#[test]
fn transient_actor_virtual_parent_view_shares_its_static_parent_descriptor() {
    let transient = Handle::new(TransferActorOfTransientProcess::new());
    let base: Handle<dyn TransferActorOps<TransferTransientKind>> = transient.cast();
    base.set_last(true);
    assert!(transient.is_last());
    assert!(std::ptr::eq(
        <dyn TransferActorOps<TransferTransientKind> as RuntimeTyped>::static_type(),
        transient.dynamic_type().unwrap().parent().unwrap()
    ));
    assert_eq!(
        base.get().unwrap().dynamic_type().name(),
        "TransferActorOfTransientProcess"
    );
}
