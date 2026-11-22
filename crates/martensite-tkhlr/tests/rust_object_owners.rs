//! #592: descriptors describe native ancestry; Rust ownership retains actual objects.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient};
use martensite_tkhlr::{contap, hlr_algo, hlr_brep, hlr_topo_brep};

fn native_owner<T: RuntimeTyped + Transient + Send + Sync>(name: &str) {
    assert_eq!(T::static_type().name(), name);
    assert!(T::static_type().is_subtype_name("Standard_Transient"));
}

#[test]
fn native_owner_descriptors_and_send_sync_contracts_remain_complete() {
    native_owner::<contap::ContapTheIWLineOfTheIWalking>("Contap_TheIWLineOfTheIWalking");
    native_owner::<hlr_algo::EdgesBlock>("HLRAlgo_EdgesBlock");
    native_owner::<hlr_algo::WiresBlock>("HLRAlgo_WiresBlock");
    native_owner::<hlr_algo::PolyAlgo>("HLRAlgo_PolyAlgo");
    native_owner::<hlr_algo::PolyData>("HLRAlgo_PolyData");
    native_owner::<hlr_algo::PolyInternalData>("HLRAlgo_PolyInternalData");
    native_owner::<hlr_algo::PolyInternalNode>("HLRAlgo_PolyInternalNode");
    native_owner::<hlr_algo::PolyShellData>("HLRAlgo_PolyShellData");
    native_owner::<hlr_brep::Algo>("HLRBRep_Algo");
    native_owner::<hlr_brep::InternalAlgo>("HLRBRep_InternalAlgo");
    native_owner::<hlr_brep::AreaLimit>("HLRBRep_AreaLimit");
    native_owner::<hlr_brep::Data>("HLRBRep_Data");
    native_owner::<hlr_brep::PolyAlgo>("HLRBRep_PolyAlgo");
    native_owner::<hlr_topo_brep::OutLiner>("HLRTopoBRep_OutLiner");
}

#[test]
fn exact_algorithm_composes_the_same_borrowed_state_without_parent_projection() {
    let owner = Handle::new(hlr_brep::Algo::new().unwrap());
    let erased = owner.upcast::<dyn Transient>();
    assert_eq!(owner.object_address(), erased.object_address());
    assert!(erased.is_kind(hlr_brep::InternalAlgo::static_type()));
    assert!(erased.downcast_ref::<hlr_brep::InternalAlgo>().is_none());
    let borrowed = erased.downcast_ref::<hlr_brep::Algo>().unwrap();
    let internal: &hlr_brep::InternalAlgo = borrowed;
    owner.add(&Shape::default(), 0);
    assert_eq!(internal.nb_shapes(), 1);
    assert_eq!(borrowed.nb_shapes(), 1);
    assert_eq!(owner.ref_count(), 2);
    let copied = hlr_brep::Algo::from_algo(borrowed).unwrap();
    assert_eq!(copied.nb_shapes(), 1);
}

#[test]
fn arbitrary_user_payload_keeps_its_actual_allocation_and_type() {
    let algorithm = hlr_brep::Algo::new().unwrap();
    let payload = Handle::new(hlr_algo::WiresBlock::new(1).unwrap());
    let erased = payload.upcast::<dyn Transient>();
    algorithm.add_with_data(&Shape::default(), &erased, 0);
    let shapes = algorithm.seq_of_shape_bounds();
    let stored = shapes[0].shape_data();
    assert_eq!(stored.object_address(), payload.object_address());
    assert!(stored.downcast_ref::<hlr_algo::WiresBlock>().is_some());
    assert_eq!(payload.ref_count(), 3);
}
