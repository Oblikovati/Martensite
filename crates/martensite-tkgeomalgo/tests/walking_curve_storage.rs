//! Walking-curve reads preserve shared ownership and synchronized point updates (#294).
use martensite_tkernel::standard::Handle;
use martensite_tkgeomalgo::int_patch::{
    IntPatchPoint, IntPatchPointLine, IntPatchWLType, IntPatchWLine,
};
use martensite_tkgeomalgo::int_surf::{IntSurfLineOn2S, IntSurfPntOn2S};
use martensite_tkmath::gp::Point3;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Instant;

fn sample(index: i32) -> IntSurfPntOn2S {
    let x = f64::from(index);
    let mut point = IntSurfPntOn2S::new();
    point.set_value_all(Point3::new(x, 2. * x, -x), x, -x, 2. * x, 4. * x);
    point
}

fn shared_samples() -> Handle<IntSurfLineOn2S> {
    let points = Handle::new(IntSurfLineOn2S::new(None));
    points.add(&sample(1));
    points.add(&sample(2));
    points
}

fn assert_shared_curve(line: &IntPatchWLine, points: &Handle<IntSurfLineOn2S>) {
    let exposed = line.curve();
    assert!(std::ptr::eq(exposed.get().unwrap(), points.get().unwrap()));
    assert_eq!(points.ref_count(), 3);
    drop(exposed);
    assert_eq!(points.ref_count(), 2);
}

#[test]
fn public_curve_clone_keeps_identity_and_balances_ownership() {
    let points = shared_samples();
    let line = IntPatchWLine::new_undecided(&points, false);
    assert_shared_curve(&line, &points);
    for _ in 0..512 {
        assert_eq!(line.point(1).unwrap(), sample(1));
        assert_eq!(line.nb_pnts().unwrap(), 2);
    }
    assert_eq!(points.ref_count(), 2);
    drop(line);
    assert_eq!(points.ref_count(), 1);
    println!("{{\"identity\":true,\"balanced_ownership\":true,\"point_reads\":512}}");
}

#[test]
fn point_updates_remain_visible_through_every_shared_owner() {
    let points = shared_samples();
    let line = Handle::new(IntPatchWLine::new_undecided(&points, false));
    let alias = line.clone();
    points.set_value(2, &sample(7)).unwrap();
    assert_eq!(alias.point(2).unwrap(), sample(7));
    let mut vertex = IntPatchPoint::new();
    vertex.set_pnt_on_2s(&sample(9));
    line.set_point(1, &vertex).unwrap();
    assert_eq!(points.value(1).unwrap(), sample(9));
    assert_eq!(alias.point(1).unwrap(), sample(9));
    assert_shared_curve(&line, &points);
    println!("{{\"shared_mutations\":[9,7],\"same_curve\":true}}");
}

#[test]
fn absent_curve_and_invalid_point_indices_keep_their_failures() {
    let empty = IntPatchWLine::new_undecided(&Handle::default(), false);
    assert!(empty.curve().is_null());
    let failure = empty.point(1).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_NullObject");
    assert_eq!(
        failure.message(),
        "IntPatch_WLine::Point() - the IntSurf_LineOn2S handle is null; expected an object"
    );
    let line = IntPatchWLine::new_undecided(&shared_samples(), false);
    for index in [-1, 0, 3] {
        let failure = line.point(index).unwrap_err();
        assert_eq!(failure.exception_type(), "Standard_OutOfRange");
        assert_eq!(failure.message(), "NCollection_Sequence::Value");
    }
    println!("{{\"null_failure\":true,\"range_failures\":[-1,0,3]}}");
}

fn interrupt_vertex_edit(line: &IntPatchWLine) {
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        line.change_vertex(1, &mut |vertex| {
            vertex.set_tolerance(0.5);
            panic!("NamedVertexEditFault: vertex1 tolerance0.5; expected recovery");
        })
        .unwrap();
    }));
    assert!(
        outcome.is_err(),
        "expected the named interrupted vertex edit"
    );
}

#[test]
fn poisoned_vertex_state_recovers_without_changing_curve_reads() {
    let points = shared_samples();
    let line = IntPatchWLine::new_undecided(&points, false);
    line.add_vertex(&IntPatchPoint::new(), false);
    interrupt_vertex_edit(&line);
    assert_eq!(line.point(1).unwrap(), sample(1));
    assert_eq!(line.nb_pnts().unwrap(), 2);
    assert_eq!(line.nb_vertex(), 1);
    assert_eq!(line.vertex(1).unwrap().tolerance(), 0.5);
    line.set_creating_way_info(IntPatchWLType::PrmPrm);
    assert_eq!(line.get_creating_way(), IntPatchWLType::PrmPrm);
    assert_shared_curve(&line, &points);
    println!("{{\"poison_recovered\":true,\"vertex_tolerance\":0.5}}");
}

fn concurrent_reader(line: &IntPatchWLine) {
    for _ in 0..2048 {
        let observed = line.point(1).unwrap();
        assert!(
            observed == sample(1) || observed == sample(2),
            "reader got {observed:?}; expected one whole sample"
        );
    }
}

fn concurrent_writer(points: &IntSurfLineOn2S) {
    for iteration in 0..2048 {
        points.set_value(1, &sample(1 + iteration % 2)).unwrap();
    }
}

fn simultaneous_access(line: &IntPatchWLine, points: &IntSurfLineOn2S) {
    std::thread::scope(|scope| {
        scope.spawn(|| concurrent_writer(points));
        for _ in 0..4 {
            scope.spawn(|| concurrent_reader(line));
        }
    });
}

#[test]
fn concurrent_point_updates_keep_whole_snapshots_and_final_state() {
    let points = shared_samples();
    let line = Handle::new(IntPatchWLine::new_undecided(&points, false));
    let _: &(dyn Send + Sync) = line.get().unwrap();
    let _: &(dyn Send + Sync) = &line;
    simultaneous_access(&line, &points);
    assert_eq!(line.point(1).unwrap(), sample(2));
    let serial = shared_samples();
    concurrent_writer(&serial);
    assert_eq!(points.value(1).unwrap(), serial.value(1).unwrap());
    println!(
        "{{\"concurrent_readers\":4,\"whole_snapshots\":true,\"serial_final_state_equal\":true}}"
    );
}

#[test]
#[ignore = "Manual cost probe; elapsed time is an observation, not a unit-test threshold"]
fn repeated_walking_point_read_cost() {
    let points = shared_samples();
    let line = IntPatchWLine::new_undecided(&points, false);
    let start = Instant::now();
    for _ in 0..200_000 {
        std::hint::black_box(line.point(std::hint::black_box(1)).unwrap());
    }
    println!(
        "{{\"walking_point_reads\":200000,\"elapsed_ns\":{}}}",
        start.elapsed().as_nanos()
    );
    assert_eq!(points.ref_count(), 2);
}
