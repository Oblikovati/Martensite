//! Context forwarding retains the same real composition collection without a renderer fork.
use crate::display::SensitiveDisplayDriver;
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::{
    ais::{AisInteractiveContext, AisKindOfInteractive, AisTypeFilter},
    v3d::V3dViewer,
};

#[test]
fn context_forwarding_retains_live_collection_after_return() {
    let (driver, _trace) = SensitiveDisplayDriver::new_handle(0);
    let viewer = V3dViewer::new(&driver).unwrap();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let collection = context.filters();
    let child = Handle::new(AisTypeFilter::new(AisKindOfInteractive::Datum)).cast();
    assert!(collection.is_empty());
    context.add_filter(&child);
    assert_eq!(collection.len(), 1);
    context.remove_filter(&child);
    assert!(collection.is_empty());
}
