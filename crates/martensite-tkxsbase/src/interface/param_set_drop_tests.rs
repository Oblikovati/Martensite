//! A returned list remains observable after its last parameter-set owner drops.
use super::{InterfaceParamSet, InterfaceParamType};
use martensite_tkernel::standard::Handle;

#[test]
fn final_set_owner_clears_escaped_shared_parameter_list() {
    let set = InterfaceParamSet::new(2, 1);
    set.append(b"12", 2, InterfaceParamType::ParamInteger, 0)
        .unwrap();
    let list = set.params(0, 0).unwrap();
    assert_eq!(list.length(), 1);
    drop(set);
    assert_eq!(list.length(), 0);
    assert_eq!(
        list.value(1).err().unwrap().exception_type(),
        "Standard_OutOfRange"
    );
}

#[test]
fn retained_set_owner_delays_shared_list_cleanup_until_final_drop() {
    let set = Handle::new(InterfaceParamSet::new(2, 1));
    set.append(b"#7", 2, InterfaceParamType::ParamIdent, 7)
        .unwrap();
    let list = set.params(0, 0).unwrap();
    let retained = set.clone();
    drop(set);
    assert_eq!(list.length(), 1);
    assert_eq!(retained.param(1).unwrap().entity_number(), 7);
    drop(retained);
    assert_eq!(list.length(), 0);
}

#[test]
fn independent_parameter_list_copy_keeps_its_length_after_set_cleanup() {
    let set = InterfaceParamSet::new(2, 1);
    set.append(b"12", 2, InterfaceParamType::ParamInteger, 0)
        .unwrap();
    let shared = set.params(0, 0).unwrap();
    let independent = set.params(1, 1).unwrap();
    assert_ne!(shared, independent);
    drop(set);
    assert_eq!(shared.length(), 0);
    assert_eq!(independent.length(), 1);
    // Native copied parameter text points into freed set storage; do not read it.
}
