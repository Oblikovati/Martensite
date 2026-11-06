//! Copies retain shared geometry identity but own the enclosing record/list.
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSGeometryData, TopOpeBRepDSInterference, TopOpeBRepDSSurface,
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::gp::Axis3;

#[test]
fn surface_copy_and_assignment_retain_handle_and_copy_flags() {
    let geometry: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let mut original = TopOpeBRepDSSurface::new(&geometry, 0.125);
    original.change_keep(true);
    let copied = original.clone();
    let mut assigned = TopOpeBRepDSSurface::default();
    assigned.assign(&original);
    original.set_tolerance(0.25);
    original.change_keep(false);
    for record in [&copied, &assigned] {
        assert!(std::ptr::eq(
            record.surface().get().unwrap(),
            geometry.get().unwrap()
        ));
        assert_eq!(record.tolerance(), 0.125);
        assert!(record.keep());
    }
}

#[test]
fn geometry_copy_owns_list_nodes_but_shares_interference_handles() {
    let event = Handle::new(TopOpeBRepDSInterference::default());
    let mut original = TopOpeBRepDSGeometryData::default();
    original.add_interference(&event);
    original.add_interference(&event);
    let copied = original.clone();
    let mut assigned = TopOpeBRepDSGeometryData::default();
    assigned.add_interference(&Handle::new(TopOpeBRepDSInterference::default()));
    assigned.assign(&original);
    original.change_interferences().clear();
    event.get().unwrap().set_geometry_index(17);
    for record in [&copied, &assigned] {
        assert_interference_aliases(record, &event);
    }
}

fn assert_interference_aliases(
    record: &TopOpeBRepDSGeometryData,
    event: &Handle<TopOpeBRepDSInterference>,
) {
    let items: Vec<_> = record.interferences().iter().collect();
    assert_eq!(items.len(), 2);
    for item in items {
        assert!(std::ptr::eq(item.get().unwrap(), event.get().unwrap()));
        assert_eq!(item.get().unwrap().geometry(), 17);
    }
}
