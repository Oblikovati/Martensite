//! Concrete exchange descriptors retain their declared native ancestry.
use martensite_tkernel::standard::{RuntimeTyped, Transient};
use martensite_tkxsbase::{if_select::*, transfer::*};

fn assert_descriptor<T: Transient + RuntimeTyped>(object: T, name: &str, parent: &str) {
    assert_eq!(T::TYPE_NAME, name);
    assert_eq!(object.dynamic_type().name(), name);
    assert!(std::ptr::eq(object.dynamic_type(), T::static_type()));
    assert_eq!(object.dynamic_type().parent().unwrap().name(), parent);
    assert!(object.is_kind_name(parent));
    assert!(object.is_kind_name("Standard_Transient"));
    assert!(!object.is_kind_name("InterfaceCheck"));
}

#[test]
fn ifselect_dispglobal_descriptor_has_its_declared_parent() {
    assert_descriptor(
        IFSelectDispGlobal::new(),
        "IFSelectDispGlobal",
        "IFSelectDispatch",
    );
}

#[test]
fn ifselect_dispperone_descriptor_has_its_declared_parent() {
    assert_descriptor(
        IFSelectDispPerOne::new(),
        "IFSelectDispPerOne",
        "IFSelectDispatch",
    );
}

#[test]
fn ifselect_disppercount_descriptor_has_its_declared_parent() {
    assert_descriptor(
        IFSelectDispPerCount::new(),
        "IFSelectDispPerCount",
        "IFSelectDispatch",
    );
}

#[test]
fn ifselect_dispperfiles_descriptor_has_its_declared_parent() {
    assert_descriptor(
        IFSelectDispPerFiles::new(),
        "IFSelectDispPerFiles",
        "IFSelectDispatch",
    );
}

#[test]
fn ifselect_disppersignature_descriptor_has_its_declared_parent() {
    assert_descriptor(
        IFSelectDispPerSignature::new(),
        "IFSelectDispPerSignature",
        "IFSelectDispatch",
    );
}

#[test]
fn transfer_actoroftransientprocess_descriptor_has_its_declared_parent() {
    assert_descriptor(
        TransferActorOfTransientProcess::new(),
        "TransferActorOfTransientProcess",
        "TransferActorOfProcessForTransient",
    );
}

#[test]
fn transfer_actoroffinderprocess_descriptor_has_its_declared_parent() {
    assert_descriptor(
        TransferActorOfFinderProcess::new(),
        "TransferActorOfFinderProcess",
        "TransferActorOfProcessForFinder",
    );
}
