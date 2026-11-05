//! External callers reach the ranged face classification; its native poll and scope matrix is
//! compared in the unit tests of `bop_algo`.
#[allow(unused_imports)]
use martensite_tkbo::bop_algo;
use martensite_tkbo::bop_algo::ProgressCompletion;
use martensite_tkbo::bop_algo::tools::{
    FaceClassificationJob, InParts, ShapeBoxMap, SolidInternalFaces, classify_faces,
    classify_faces_with_progress,
};
use martensite_tkbo::int_tools::Context;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
#[allow(dead_code)]
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;

/// A horizontal square of side one around the middle of the cube, at height `z`.
fn square(z: f64) -> Shape {
    let plane = Plane3::new(Point3::new(0., 0., z), Direction3::new(0., 0., 1.).unwrap()).unwrap();
    BRepBuilderAPIMakeFace::from_plane_bounds(plane, (0.5, 1.5, 0.5, 1.5))
        .unwrap()
        .face()
        .unwrap()
        .clone()
        .into_shape()
}

struct Scene {
    faces: List<Shape>,
    solids: List<Shape>,
    inside: Shape,
}

fn scene() -> Scene {
    let inside = square(1.);
    let solid = BRepPrimAPIMakeBox::from_sizes(2., 2., 2.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    Scene {
        faces: [inside.clone(), square(5.)].into_iter().collect(),
        solids: [solid].into_iter().collect(),
        inside,
    }
}

fn run_ranged(
    scene: &Scene,
    parallel: bool,
    range: &ProgressRange<'_>,
) -> (InParts, ProgressCompletion) {
    let context = Context::default();
    let mut parts = InParts::default();
    let (boxes, internal) = (ShapeBoxMap::default(), SolidInternalFaces::default());
    let job = FaceClassificationJob {
        faces: &scene.faces,
        solids: &scene.solids,
        context: &context,
        parts: &mut parts,
        boxes: &boxes,
        internal: &internal,
        parallel,
    };
    let completion = classify_faces_with_progress(job, range);
    (parts, completion)
}

#[test]
fn only_the_face_inside_the_solid_is_classified_in_it_serial_and_parallel() {
    let scene = scene();
    for parallel in [false, true] {
        let (parts, completion) = run_ranged(&scene, parallel, &ProgressRange::default());
        assert_eq!(
            completion,
            ProgressCompletion::Completed,
            "parallel={parallel}"
        );
        assert_eq!(parts.extent(), 1, "parallel={parallel}");
        let inside = parts.find_from_index(1).unwrap();
        assert_eq!(inside.len(), 1, "parallel={parallel}");
        assert!(inside.iter().all(|face| face.is_same(&scene.inside)));
    }
}

#[test]
fn the_ranged_entry_agrees_with_the_unranged_one() {
    let scene = scene();
    let context = Context::default();
    let mut parts = InParts::default();
    let (boxes, internal) = (ShapeBoxMap::default(), SolidInternalFaces::default());
    classify_faces(
        &scene.faces,
        &scene.solids,
        &context,
        &mut parts,
        &boxes,
        &internal,
    )
    .unwrap();
    let (ranged, _) = run_ranged(&scene, false, &ProgressRange::default());
    assert_eq!(parts.extent(), ranged.extent());
    assert_eq!(
        parts.find_from_index(1).unwrap().len(),
        ranged.find_from_index(1).unwrap().len()
    );
}

#[test]
fn a_requested_break_is_returned_as_cancellation() {
    let scene = scene();
    let (indicator, _trace) = recording::before_indicator();
    let (_, completion) = run_ranged(&scene, false, &indicator.start());
    assert!(
        matches!(completion, ProgressCompletion::Cancelled { .. }),
        "got {completion:?}"
    );
}
