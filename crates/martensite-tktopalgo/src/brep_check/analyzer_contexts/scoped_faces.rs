//! Schedule only distinct face contexts within one analyzer.
use super::{
    CheckerMap, CylindricalCurveBounds, MapOfShape, Shape, ShapeEnum, check_shape, sub_shapes,
};
use std::cmp::Reverse;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::thread::{self, Scope, ScopedJoinHandle};

pub(in super::super) fn run(
    map: &CheckerMap,
    bounds: &CylindricalCurveBounds,
    worker_limit: NonZeroUsize,
) -> Option<()> {
    let available = thread::available_parallelism()
        .ok()?
        .min(worker_limit)
        .get();
    if available < 2 {
        return None;
    }
    let mut plan = Plan::capture(map)?;
    let workers = available.min(plan.faces.len());
    if workers < 2 {
        return None;
    }
    // Edge polygon status, edge vertex contexts and solid shell contexts finish
    // before any worker can record a face context.
    for shape in plan.serial {
        check_shape(map, shape, Some(bounds)).ok()?;
    }
    plan.faces
        .sort_by_key(|job| (Reverse(job.edges), job.ordinal));
    let functions = FaceChecks {
        map,
        bounds,
        faces: &plan.faces,
        next: AtomicUsize::new(0),
        failed: AtomicBool::new(false),
    };
    functions.run(workers).then_some(())
}

struct FaceJob<'a> {
    shape: &'a Shape,
    edges: usize,
    ordinal: usize,
}

#[derive(Default)]
struct Plan<'a> {
    serial: Vec<&'a Shape>,
    faces: Vec<FaceJob<'a>>,
}

impl<'a> Plan<'a> {
    fn capture(map: &'a CheckerMap) -> Option<Self> {
        let mut plan = Self::default();
        let mut faces = MapOfShape::default();
        let mut wires = MapOfShape::default();
        for ordinal in 1..=map.extent() {
            let shape = map.find_key(ordinal).ok()?;
            if shape.shape_type().ok()? == ShapeEnum::Face {
                if !admit_face(shape, &mut faces, &mut wires)? {
                    return None;
                }
                let edges = sub_shapes(shape, ShapeEnum::Edge).ok()?.len();
                plan.faces.push(FaceJob {
                    shape,
                    edges,
                    ordinal,
                });
            } else {
                plan.serial.push(shape);
            }
        }
        Some(plan)
    }
}

fn admit_face(shape: &Shape, faces: &mut MapOfShape, wires: &mut MapOfShape) -> Option<bool> {
    // Context status keys use IsSame. A repeated face can alias a status slot;
    // a repeated wire can share its cached face classification.
    if !faces.add(shape.clone()) {
        return Some(false);
    }
    Some(
        sub_shapes(shape, ShapeEnum::Wire)
            .ok()?
            .into_iter()
            .all(|wire| wires.add(wire)),
    )
}

struct FaceChecks<'a> {
    map: &'a CheckerMap,
    bounds: &'a CylindricalCurveBounds,
    faces: &'a [FaceJob<'a>],
    next: AtomicUsize,
    failed: AtomicBool,
}

impl FaceChecks<'_> {
    fn run(&self, worker_limit: usize) -> bool {
        thread::scope(|scope| {
            let workers = self.start(scope, worker_limit);
            for worker in workers {
                if worker.join().is_err() {
                    self.failed.store(true, Relaxed);
                }
            }
        });
        !self.failed.load(Relaxed)
    }

    fn start<'scope>(
        &'scope self,
        scope: &'scope Scope<'scope, '_>,
        worker_limit: usize,
    ) -> Vec<ScopedJoinHandle<'scope, ()>> {
        let mut workers = Vec::with_capacity(worker_limit);
        for _ in 0..worker_limit {
            match thread::Builder::new().spawn_scoped(scope, || self.check_faces()) {
                Ok(worker) => workers.push(worker),
                Err(_) => {
                    self.failed.store(true, Relaxed);
                    break;
                }
            }
        }
        workers
    }

    fn check_faces(&self) {
        while !self.failed.load(Relaxed) {
            let index = self.next.fetch_add(1, Relaxed);
            let Some(job) = self.faces.get(index) else {
                break;
            };
            if check_shape(self.map, job.shape, Some(self.bounds)).is_err() {
                self.failed.store(true, Relaxed);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_check::analyzer_contexts::tests::triangle;
    use martensite_tkbrep::topods::Builder;

    #[test]
    fn scheduler_rejects_a_repeated_face_or_a_wire_shared_by_distinct_faces() {
        let original = triangle();
        let unrelated = triangle();
        let original_wire = &sub_shapes(&original, ShapeEnum::Wire).unwrap()[0];
        let mut duplicate_wire_use = original.empty_copied().unwrap();
        Builder.add(&mut duplicate_wire_use, original_wire).unwrap();
        assert!(!original.is_same(&duplicate_wire_use));

        let mut faces = MapOfShape::default();
        let mut wires = MapOfShape::default();
        assert_eq!(admit_face(&original, &mut faces, &mut wires), Some(true));
        assert_eq!(admit_face(&unrelated, &mut faces, &mut wires), Some(true));
        assert_eq!(
            admit_face(&original.reversed(), &mut faces, &mut wires),
            Some(false)
        );
        assert_eq!(
            admit_face(&duplicate_wire_use, &mut faces, &mut wires),
            Some(false)
        );
    }
}
