//! Stronger real-owner observations are separate from the original predicates.
use super::*;
use martensite_tkernel::ncollection::List;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::std_select::{StdSelectBRepOwner, StdSelectBRepOwnerMembers as _};
mod inputs;
mod removal;
mod selection;

fn cpu_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("owner lifecycle CPU scene; expected one real 409x409 null-driver context")
}

fn selected_brep_shape(context: &AisInteractiveContext) -> Shape {
    context.init_selected();
    let owner = context
        .selected_owner()
        .unwrap()
        .cast::<StdSelectBRepOwner>();
    assert!(
        !owner.is_null(),
        "selected owner; expected a real BRep owner"
    );
    assert!(owner.has_shape());
    owner.shape_ref().clone_shape()
}
