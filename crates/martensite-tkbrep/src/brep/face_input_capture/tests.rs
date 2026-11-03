//! Real cells and independently constructed geometry exercise capture, not face acceptance.
mod locations;
mod plane_spline;
mod replay;
mod representations;
mod surface_queries;
use super::*;
use crate::{
    brep::*,
    topods::{Face, Shape, TShape, TWire},
};
use martensite_tkernel::{
    ncollection::Array1,
    standard::{Handle, SharedReadFailure},
};
use martensite_tkg3d::{
    geom::{GeomBSplineCurve, GeomSurfaceOfLinearExtrusion},
    top_abs::Orientation,
};
use martensite_tkmath::{
    gp::{Direction3, Point3},
    toploc::Location,
};

fn support() -> Handle<GeomSurfaceOfLinearExtrusion> {
    let curve = GeomBSplineCurve::new(
        &Array1::from_shared(
            &[
                Point3::default(),
                Point3::new(1., 0., 0.),
                Point3::default(),
            ],
            1,
        ),
        &Array1::from_shared(&[0., 1.], 1),
        &Array1::from_shared(&[3, 3], 1),
        2,
        false,
    )
    .unwrap();
    Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&Handle::new(curve).cast(), Direction3::Z).unwrap(),
    )
}
fn face_fixture() -> (Face, Handle<BRepTFace>) {
    let fields = Handle::new(BRepTFace::new());
    fields.set_surface(&support().cast());
    let shape = Shape::from_parts(fields.cast(), Location::default(), Orientation::Forward);
    (crate::topods::face(&shape).unwrap(), fields)
}
fn captured(face: &Face) -> FaceInputSnapshot {
    capture_closed_extrusion_face(face, FaceCaptureLimits::bounded(), || false).unwrap()
}
#[test]
fn independent_source_values_are_copied_and_old_snapshot_is_isolated() {
    let (face, source) = face_fixture();
    let snapshot = captured_at_tolerance(&face, &source);
    source.set_tolerance(0.25);
    assert!(snapshot.cells().iter().any(|(_, cell)| matches!(
        cell,
        CapturedFaceCell::Face {
            tolerance: 0.125,
            ..
        }
    )));
    assert!(snapshot.cells().iter().any(|(_, cell)| matches!(cell, CapturedFaceCell::Spline(s) if s.poles()==[[0.,0.,0.],[1.,0.,0.],[0.,0.,0.]])));
    assert!(captured(&face).cells().iter().any(|(_, cell)| matches!(
        cell,
        CapturedFaceCell::Face {
            tolerance: 0.25,
            ..
        }
    )));
}
#[test]
fn actual_other_thread_writer_is_busy_without_waiting() {
    let (face, source) = face_fixture();
    std::thread::scope(|scope| {
        let (ready, waiting) = std::sync::mpsc::channel();
        let (release, resume) = std::sync::mpsc::channel();
        scope.spawn(move || {
            let _writer = source.fields.write();
            ready.send(()).unwrap();
            resume.recv().unwrap();
        });
        waiting.recv().unwrap();
        let result = capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false);
        release.send(()).unwrap();
        assert_busy(result);
    });
}
#[test]
fn actual_poisoned_face_cell_is_not_recovered_for_capture() {
    let (face, source) = face_fixture();
    let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _writer = source.fields.write();
        panic!("NamedPoisonedFaceCell writer failure");
    }));
    assert!(fault.is_err());
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::Read {
            cause: SharedReadFailure::Poisoned,
            ..
        })
    ));
}
#[test]
fn reference_change_after_discovery_refuses_the_simultaneous_view() {
    let (face, source) = face_fixture();
    let limits = FaceCaptureLimits::bounded();
    let inventory =
        inventory::Inventory::gather(&face, limits, FaceCaptureBehavior::StoredRepresentationsV1)
            .unwrap();
    source.set_surface(&support().cast());
    let cells = inventory.cells(limits).unwrap();
    let leases = cells
        .iter()
        .map(|(owner, cell)| (*owner, cell.acquire(*owner).unwrap()))
        .collect::<Vec<_>>();
    assert!(matches!(
        inventory.revalidate(&leases, limits),
        Err(FaceCaptureIncomplete::Changed { .. })
    ));
}
#[test]
fn cancellation_callback_can_mutate_after_every_lease_is_dropped() {
    let (face, source) = face_fixture();
    let mut polls = 0;
    let result = capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || {
        polls += 1;
        if polls == 2 {
            source.set_tolerance(0.5);
        }
        false
    });
    assert!(result.is_ok());
    assert_eq!(polls, 2);
    assert_eq!(source.tolerance(), 0.5);
}
#[test]
fn finite_entries_and_preacquisition_cancellation_remain_incomplete() {
    let (face, _) = face_fixture();
    let limits = FaceCaptureLimits {
        entries: 0,
        ..FaceCaptureLimits::bounded()
    };
    assert!(matches!(
        capture_closed_extrusion_face(&face, limits, || false),
        Err(FaceCaptureIncomplete::Limit { .. })
    ));
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || true),
        Err(FaceCaptureIncomplete::Cancelled)
    ));
}
#[test]
fn repeated_shared_child_is_one_owner_with_two_retained_occurrences() {
    let (face, _) = face_fixture();
    let wire = Handle::new(TWire::new());
    let occurrence = Shape::from_parts(wire.cast(), Location::default(), Orientation::Forward);
    let parent = face.tshape().get().unwrap().tshape_base();
    parent.append_child(occurrence.clone());
    parent.append_child(occurrence.reversed());
    let snapshot = captured(&face);
    let (_, CapturedFaceCell::Topology { children, .. }) = &snapshot.cells()[0] else {
        panic!("expected root topology")
    };
    assert_eq!(children.len(), 2);
    assert_eq!(children[0].owner, children[1].owner);
    assert_ne!(children[0].orientation, children[1].orientation);
}
#[test]
fn published_input_is_send_sync_and_multithread_identical() {
    fn require_shared<T: Send + Sync>() {}
    require_shared::<FaceInputSnapshot>();
    let (face, _) = face_fixture();
    let expected = format!("{:?}", captured(&face));
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| format!("{:?}", captured(&face))))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn assert_busy(result: Result<FaceInputSnapshot, FaceCaptureIncomplete>) {
    assert!(matches!(
        result,
        Err(FaceCaptureIncomplete::Read {
            cause: SharedReadFailure::Busy,
            ..
        })
    ));
}

fn captured_at_tolerance(face: &Face, source: &BRepTFace) -> FaceInputSnapshot {
    source.set_tolerance(0.125);
    captured(face)
}
