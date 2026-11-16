//! Explicit allocator lifetime and policy must survive the default-storage optimization (#496).
use super::{IntSurfLineOn2S, pnt_on_2s};
use martensite_tkernel::ncollection::{BaseAllocator, IncrementalAllocatorObject};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point2, Point3};

#[test]
fn explicit_allocator_object_stays_owned_until_line_drop() {
    let owner = Handle::new(IncrementalAllocatorObject::new(4096));
    let resource = BaseAllocator::from_handle(&owner);
    let before = owner.ref_count();
    let line = IntSurfLineOn2S::new(Some(resource.clone()));
    assert_eq!(owner.ref_count(), before + 1);
    line.add(&pnt_on_2s(1., 1.));
    line.clear();
    assert_eq!(owner.ref_count(), before + 1);
    drop(line);
    assert_eq!(owner.ref_count(), before);
}

#[test]
fn supplied_policy_receives_nodes_and_refuses_reset_until_clear() {
    let resource = BaseAllocator::incremental(4096);
    let line = IntSurfLineOn2S::new(Some(resource.clone()));
    assert_eq!(block_count(&resource), 0);
    line.add(&pnt_on_2s(1., 1.));
    assert_eq!(block_count(&resource), 1);
    assert!(resource.reset(true).is_err());
    line.clear();
    resource.reset(true).unwrap();
    line.add(&pnt_on_2s(2., 2.));
    assert!(resource.reset(false).is_err());
    drop(line);
    resource.reset(true).unwrap();
}

#[test]
fn split_tail_uses_default_storage_and_releases_original_arena_nodes() {
    let resource = BaseAllocator::incremental(4096);
    let line = IntSurfLineOn2S::new(Some(resource.clone()));
    line.add(&pnt_on_2s(1., 1.));
    line.add(&pnt_on_2s(2., 2.));
    let tail = line.split(2).unwrap();
    assert_eq!(line.nb_points(), 1);
    assert_eq!(tail.nb_points(), 1);
    assert_eq!(tail.value(1).unwrap().value().x(), 2.);
    assert!(resource.reset(true).is_err());
    line.remove_point(1).unwrap();
    resource.reset(true).unwrap();
    assert_eq!(tail.value(1).unwrap().value().x(), 2.);
}

#[test]
fn both_storage_policies_preserve_edits_order_and_lazy_boxes() {
    for policy in [None, Some(BaseAllocator::incremental(4096))] {
        let line = IntSurfLineOn2S::new(policy);
        edit_points(&line);
        check_boxes_and_tail(&line);
    }
}

fn edit_points(line: &IntSurfLineOn2S) {
    line.add(&pnt_on_2s(1., 1.));
    line.add(&pnt_on_2s(3., 3.));
    line.insert_before(2, &pnt_on_2s(2., 2.)).unwrap();
    line.insert_before(9, &pnt_on_2s(4., 4.)).unwrap();
    line.remove_point(4).unwrap();
    line.reverse();
    line.set_value(1, &pnt_on_2s(9., 9.)).unwrap();
    line.set_point(2, Point3::new(2.5, 0., 0.)).unwrap();
    line.set_uv(2, true, 2.5, 2.5).unwrap();
    assert_eq!(line.value(1).unwrap().value().x(), 9.);
    assert!(line.value(0).is_err());
    assert!(line.insert_before(0, &pnt_on_2s(0., 0.)).is_err());
    assert!(line.remove_point(0).is_err());
    assert!(line.split(0).is_err());
}

fn check_boxes_and_tail(line: &IntSurfLineOn2S) {
    assert!(!line.is_out_box(Point3::new(2.5, 0., 0.)));
    assert!(line.is_out_box(Point3::new(100., 0., 0.)));
    assert!(!line.is_out_surf1_box(Point2::new(2.5, 2.5)));
    assert!(!line.is_out_surf2_box(Point2::new(-2., -2.)));
    let tail = line.split(2).unwrap();
    assert_eq!(line.nb_points(), 1);
    assert_eq!(tail.nb_points(), 2);
    assert_eq!(tail.value(1).unwrap().value().x(), 2.5);
    assert_eq!(tail.value(2).unwrap().value().x(), 1.);
    line.clear();
    assert!(!line.is_out_box(Point3::new(100., 0., 0.)));
}

fn block_count(resource: &BaseAllocator) -> u32 {
    resource
        .incremental_extension()
        .unwrap()
        .observation()
        .block_count
}

#[test]
fn explicit_accumulating_policy_releases_nodes_on_remove_clear_and_drop() {
    let resource = BaseAllocator::accumulating(4096).unwrap();
    let line = IntSurfLineOn2S::new(Some(resource.clone()));
    line.add(&pnt_on_2s(1., 1.));
    line.add(&pnt_on_2s(2., 2.));
    assert_eq!(live_allocations(&resource), 2);
    line.remove_point(1).unwrap();
    assert_eq!(live_allocations(&resource), 1);
    line.clear();
    assert_eq!(live_allocations(&resource), 0);
    line.add(&pnt_on_2s(3., 3.));
    assert_eq!(live_allocations(&resource), 1);
    drop(line);
    assert_eq!(live_allocations(&resource), 0);
}

fn live_allocations(resource: &BaseAllocator) -> i32 {
    resource
        .accumulating_chain()
        .iter()
        .map(|block| block.alloc_count)
        .sum()
}
