//! The descriptor parents of the classes whose registration uses a macro-built name or that split into a trait and a concrete struct:
//! `DynamicType()->Parent()` names the `DEFINE_STANDARD_RTTIEXT` base, as natively.
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped, Transient};
use martensite_tklcaf::tdata_std::{
    TDataStdExtStringList, TDataStdGenericEmpty, TDataStdIntegerList, TDataStdRealList,
    TDataStdReferenceList, TDataStdTick,
};
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdStandardApplication};

fn parent_name(descriptor: &RuntimeType) -> &str {
    descriptor.parent().map_or("", RuntimeType::name)
}

#[test]
fn list_attributes_derive_from_tdf_attribute() {
    let descriptors = [
        (
            TDataStdExtStringList::static_type(),
            "TDataStd_ExtStringList",
        ),
        (TDataStdIntegerList::static_type(), "TDataStd_IntegerList"),
        (TDataStdRealList::static_type(), "TDataStd_RealList"),
        (
            TDataStdReferenceList::static_type(),
            "TDataStd_ReferenceList",
        ),
    ];
    for (descriptor, name) in descriptors {
        assert_eq!(descriptor.name(), name);
        assert_eq!(parent_name(descriptor), "TDF_Attribute", "{name}");
    }
}

/// `TDocStd_Application` derives from `CDF_Application`; the concrete application and the
/// trait every derived application names both report that parent.
#[test]
fn standard_application_derives_from_cdf_application() {
    let concrete = TDocStdStandardApplication::static_type();
    let interface = <dyn TDocStdApplication as RuntimeTyped>::static_type();
    for descriptor in [concrete, interface] {
        assert_eq!(descriptor.name(), "TDocStd_Application");
        assert_eq!(parent_name(descriptor), "CDF_Application");
    }
    let application = TDocStdStandardApplication::new();
    let dynamic = application.get().expect("application").dynamic_type();
    assert!(std::ptr::eq(dynamic, concrete));
    assert!(dynamic.is_subtype_name("CDM_Application"));
}

/// `TDataStd_GenericEmpty` derives from `TDF_Attribute`, and `TDataStd_Tick` from it.
#[test]
fn generic_empty_is_the_parent_of_tick() {
    let generic = <dyn TDataStdGenericEmpty as RuntimeTyped>::static_type();
    assert_eq!(
        (generic.name(), parent_name(generic)),
        ("TDataStd_GenericEmpty", "TDF_Attribute")
    );
    assert_eq!(
        parent_name(TDataStdTick::static_type()),
        "TDataStd_GenericEmpty"
    );
}
