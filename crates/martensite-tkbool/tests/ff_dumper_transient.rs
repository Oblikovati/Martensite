//! A real transient must preserve allocation and live operation identity through handle casts.
use martensite_tkbool::{top_ope_brep::*, top_ope_brep_ds::*};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient, Transient};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::sync::{Arc, RwLock};
type GenericHandle = Handle<StandardTransient>;

fn face(normal: Direction3) -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(Point3::default(), normal).unwrap(),
        (-2., 2., -2., 2.),
    )
    .unwrap()
    .face()
    .unwrap()
}
fn context() -> TopOpeBRepFacesFillerContext {
    let (first, second) = (face(Direction3::Z), face(Direction3::X));
    let mut pair = TopOpeBRepFacesIntersector::new();
    pair.perform(&first, &second).unwrap();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let mut filler = TopOpeBRepFacesFiller::new();
    filler
        .bind_owned_context(&first, &second, pair, &hds)
        .unwrap();
    Arc::new(RwLock::new(filler))
}
#[test]
fn generic_and_concrete_base_handles_round_trip_the_same_diagnostic_object() {
    let context = context();
    let original = Handle::new(TopOpeBRepFFDumper::new_debug(context.clone()).unwrap());
    let generic: GenericHandle = original.cast();
    let base: Handle<StandardTransient> = original.cast();
    assert!(!generic.is_null() && !base.is_null());
    let recovered = Handle::<TopOpeBRepFFDumper>::down_cast(&generic);
    let from_base = Handle::<TopOpeBRepFFDumper>::down_cast(&base);
    assert_eq!(original, recovered);
    assert_eq!(original, from_base);
    assert_eq!(original.as_ptr(), recovered.as_ptr());
    assert_eq!(generic.dynamic_type().unwrap().name(), "TopOpeBRepFFDumper");
    assert!(std::ptr::eq(
        generic.dynamic_type().unwrap(),
        TopOpeBRepFFDumper::get_type_descriptor()
    ));
    assert!(generic.is_kind(StandardTransient::static_type()));
    assert!(Arc::ptr_eq(
        &recovered.p_faces_filler_dummy().unwrap().unwrap(),
        &context
    ));
}
#[test]
fn failed_downcast_is_null_and_release_mode_retains_the_same_silent_contract() {
    let unrelated: GenericHandle = Handle::new(StandardTransient::default()).cast();
    assert!(Handle::<TopOpeBRepFFDumper>::down_cast(&unrelated).is_null());
    let release = Handle::new(TopOpeBRepFFDumper::new(Some(context())));
    let generic: GenericHandle = release.cast();
    let recovered = Handle::<TopOpeBRepFFDumper>::down_cast(&generic);
    let mut output = b"preserved".to_vec();
    recovered.dump_line_index(-1, &mut output).unwrap();
    recovered
        .dump_vp(&TopOpeBRepVPointInter::new(), &mut output)
        .unwrap();
    assert_eq!(output, b"preserved");
    assert!(recovered.p_faces_filler_dummy().unwrap().is_none());
}
#[test]
fn caller_mutation_of_actual_filler_storage_is_visible_after_handle_round_trip() {
    let context = context();
    let dumper = Handle::new(TopOpeBRepFFDumper::new_debug(context.clone()).unwrap());
    let generic: GenericHandle = dumper.cast();
    let recovered = Handle::<TopOpeBRepFFDumper>::down_cast(&generic);
    let hds = context.read().unwrap().hdata_structure();
    let index = hds
        .change_ds()
        .add_point(&TopOpeBRepDSPoint::new(Point3::default(), 0.125));
    let point = TopOpeBRepVPointInter::new();
    let mut before = Vec::new();
    recovered
        .dump_dsp(&point, TopOpeBRepDSKind::Point, index, true, &mut before)
        .unwrap();
    context
        .write()
        .unwrap()
        .change_data_structure()
        .unwrap()
        .change_point(index)
        .unwrap()
        .set_tolerance(0.25);
    let mut after = Vec::new();
    recovered
        .dump_dsp(&point, TopOpeBRepDSKind::Point, index, true, &mut after)
        .unwrap();
    assert_eq!(before, b"VP 0 on 0 gives new DSP 1 tol = 0.125\n");
    assert_eq!(after, b"VP 0 on 0 gives new DSP 1 tol = 0.25\n");
    assert!(Arc::ptr_eq(
        &dumper.p_faces_filler_dummy().unwrap().unwrap(),
        &context
    ));
}
#[test]
fn owned_binding_retains_shared_intersection_arrays_and_cache_across_calls() {
    let context = context();
    let lines = context
        .read()
        .unwrap()
        .p_faces_intersector_dummy()
        .unwrap()
        .lines();
    let dumper = Handle::new(TopOpeBRepFFDumper::new_debug(context.clone()).unwrap());
    lines.change_array1().change_value(1).unwrap().set_index(37);
    let mut output = Vec::new();
    dumper.dump_line_index(1, &mut output).unwrap();
    assert!(
        String::from_utf8(output)
            .unwrap()
            .contains("line 37/1 is a LINE")
    );
    let mut source = martensite_tkgeomalgo::int_patch::IntPatchPoint::new();
    source.set_value_tangent(Point3::default(), 1e-7, false);
    let mut point = TopOpeBRepVPointInter::new();
    point.set_owned_point(source);
    let mut output = Vec::new();
    dumper.dump_vp(&point, &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("dinp L37P0"));
    assert_eq!(
        context
            .read()
            .unwrap()
            .p_faces_intersector_dummy()
            .unwrap()
            .change_line(1)
            .unwrap()
            .index(),
        37
    );
}
#[test]
fn shared_context_rebinding_is_visible_from_every_handle_and_releases_old_owner() {
    let first = context();
    let second = context();
    let dumper = Handle::new(TopOpeBRepFFDumper::new_debug(first.clone()).unwrap());
    let generic: GenericHandle = dumper.cast();
    let recovered = Handle::<TopOpeBRepFFDumper>::down_cast(&generic);
    let first_weak = Arc::downgrade(&first);
    drop(first);
    assert!(first_weak.upgrade().is_some());
    recovered.init(Some(second.clone())).unwrap();
    assert!(first_weak.upgrade().is_none());
    assert!(Arc::ptr_eq(
        &dumper.p_faces_filler_dummy().unwrap().unwrap(),
        &second
    ));
}
#[test]
fn owned_diagnostics_and_context_are_send_sync_and_survive_case_owner_drop() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<TopOpeBRepFFDumper>();
    send_sync::<TopOpeBRepFacesFillerContext>();
    let context = context();
    let generic: GenericHandle =
        Handle::new(TopOpeBRepFFDumper::new_debug(context.clone()).unwrap()).cast();
    drop(context);
    let output = std::thread::spawn(move || {
        let recovered = Handle::<TopOpeBRepFFDumper>::down_cast(&generic);
        let mut output = Vec::new();
        recovered.dump_line_index(1, &mut output).unwrap();
        output
    })
    .join()
    .unwrap();
    assert!(!output.is_empty());
}

#[test]
fn concurrent_handle_readers_and_context_writer_preserve_complete_diagnostic_records() {
    let context = context();
    let hds = context.read().unwrap().hdata_structure();
    let index = hds
        .change_ds()
        .add_point(&TopOpeBRepDSPoint::new(Point3::default(), 0.125));
    let dumper = Handle::new(TopOpeBRepFFDumper::new_debug(context.clone()).unwrap());
    let mut workers = Vec::new();
    for _ in 0..4 {
        let alias = dumper.clone();
        workers.push(std::thread::spawn(move || {
            for _ in 0..100 {
                let mut output = Vec::new();
                alias
                    .dump_dsp(
                        &TopOpeBRepVPointInter::new(),
                        TopOpeBRepDSKind::Point,
                        index,
                        true,
                        &mut output,
                    )
                    .unwrap();
                assert!(
                    output == b"VP 0 on 0 gives new DSP 1 tol = 0.125\n"
                        || output == b"VP 0 on 0 gives new DSP 1 tol = 0.25\n"
                );
            }
        }));
    }
    for step in 0..100 {
        context
            .write()
            .unwrap()
            .change_data_structure()
            .unwrap()
            .change_point(index)
            .unwrap()
            .set_tolerance(if step % 2 == 0 { 0.125 } else { 0.25 });
    }
    for worker in workers {
        worker.join().unwrap();
    }
}

#[test]
fn transient_trait_object_projects_the_actual_derived_and_parent_subobjects() {
    let concrete = Arc::new(TopOpeBRepFFDumper::new_debug(context()).unwrap());
    let erased: Arc<dyn Transient> = concrete.clone();
    let projection = erased
        .project(std::any::TypeId::of::<TopOpeBRepFFDumper>())
        .unwrap();
    let recovered = projection.downcast_ref::<TopOpeBRepFFDumper>().unwrap();
    assert!(std::ptr::eq(recovered, concrete.as_ref()));
    let parent = erased
        .project(std::any::TypeId::of::<StandardTransient>())
        .unwrap();
    assert!(parent.downcast_ref::<StandardTransient>().is_some());
    assert_eq!(erased.dynamic_type().name(), "TopOpeBRepFFDumper");
    assert!(std::ptr::eq(erased.dynamic_type(), concrete.dynamic_type()));
}
