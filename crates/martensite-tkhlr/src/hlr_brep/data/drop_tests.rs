use super::Data;
use martensite_tkernel::standard::Handle;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;

#[test]
fn data_drop_releases_owned_classifier_projector_and_final_handle() {
    let owned = Handle::new(Data::new(0, 0, 0).unwrap());
    let source = owned.downgrade();
    let projector = std::sync::Arc::downgrade(&owned.walk.read().proj);
    let classifier = Handle::new(BRepTopAdaptorTopolTool::new());
    let weak = classifier.downgrade();
    owned.walk.write().classifier = classifier;
    assert!(!weak.upgrade().is_null());
    #[allow(
        drop_bounds,
        reason = "requires an explicit destructor body, not only owned fields"
    )]
    fn explicit_drop<T: Drop>() {}
    explicit_drop::<Data>();
    drop(owned);
    assert!(source.upgrade().is_null());
    assert!(weak.upgrade().is_null());
    assert!(projector.upgrade().is_none());
}
#[test]
fn data_destroy_only_releases_cache_and_final_drop_respects_external_owners() {
    let owned = Data::new(0, 0, 0).unwrap();
    let classifier = Handle::new(BRepTopAdaptorTopolTool::new());
    let weak = classifier.downgrade();
    owned.walk.write().classifier = classifier.clone();
    owned.destroy();
    owned.destroy();
    assert!(owned.walk.read().rejection_cache.is_none());
    assert!(!weak.upgrade().is_null());
    drop(owned);
    assert!(!weak.upgrade().is_null());
    drop(classifier);
    assert!(weak.upgrade().is_null());
}

use crate::hlr_algo::WiresBlock;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{TShape, Vertex},
};
use martensite_tkernel::{impl_transient, standard::WeakHandle};
use martensite_tkmath::gp::Point3;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

struct ObservingClassifierRelease {
    parent: BRepTopAdaptorTopolTool,
    wires: WeakHandle<WiresBlock>,
    mapped_shape: WeakHandle<dyn TShape>,
    observations: Arc<AtomicU8>,
}
impl_transient!(
    ObservingClassifierRelease,
    "HiddenLineObservingClassifierRelease",
    BRepTopAdaptorTopolTool,
    parent
);
impl Drop for ObservingClassifierRelease {
    fn drop(&mut self) {
        let flags = u8::from(!self.wires.upgrade().is_null())
            | (u8::from(!self.mapped_shape.upgrade().is_null()) << 1);
        self.observations.store(flags, Ordering::SeqCst);
    }
}
fn mapped_vertex(owned: &Data) -> WeakHandle<dyn TShape> {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::default(), 1e-7)
        .unwrap();
    let weak = vertex.tshape().downgrade();
    owned.edge_map().add(vertex.into_shape());
    weak
}
#[test]
fn classifier_releases_before_owned_face_records_and_shape_maps() {
    let owned = Data::new(0, 0, 0).unwrap();
    let wires = Handle::new(WiresBlock::new(1).unwrap());
    let weak_wires = wires.downgrade();
    *owned.f_data_array()[0].wires_mut() = wires;
    let mapped_shape = mapped_vertex(&owned);
    let observations = Arc::new(AtomicU8::new(0));
    let observer = ObservingClassifierRelease {
        parent: BRepTopAdaptorTopolTool::new(),
        wires: weak_wires.clone(),
        mapped_shape: mapped_shape.clone(),
        observations: observations.clone(),
    };
    owned.walk.write().classifier = Handle::new(observer).cast();
    drop(owned);
    assert_eq!(observations.load(Ordering::SeqCst), 3);
    assert!(weak_wires.upgrade().is_null());
    assert!(mapped_shape.upgrade().is_null());
}
