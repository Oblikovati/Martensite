//! Located vertices are read concurrently.
//!
//! The location case needs topological shapes, so it runs with the topology crates.
#![allow(non_snake_case)]
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{Shape, Vertex},
};
use martensite_tkernel::osd::Parallel;
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};
use std::sync::atomic::{AtomicI32, Ordering};
struct TopLocTransformFunctor<'a> {
    shapes: &'a [Shape],
    races: AtomicI32,
}
impl TopLocTransformFunctor<'_> {
    fn evaluate(&self, index: usize) {
        if self.races.load(Ordering::SeqCst) == 0 {
            let vertex = Vertex::unchecked(self.shapes[index].clone());
            let point = BRepTool::pnt(&vertex).unwrap();
            if point.x() != index as f64 {
                self.races.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}
#[test]
fn TopLoc_Location_Test__OCC25545_ConcurrentTransformationAccess() {
    let n = 1000;
    let mut shapes = vec![Shape::default(); n];
    let mut locations = vec![Location::default(); n];
    let mut maker = BRepBuilderAPIMakeVertex::new(Point3::default()).unwrap();
    let shape: Shape = maker.shape().unwrap().clone();
    shapes[0] = shape.clone();
    for index in 1..n {
        let transform = Transform3::translation([1.0, 0.0, 0.0]);
        locations[index] = locations[index - 1]
            .multiplied(&Location::new(transform))
            .unwrap();
        shapes[index] = shape.moved(&locations[index], false).unwrap();
    }
    let functor = TopLocTransformFunctor {
        shapes: &shapes,
        races: AtomicI32::new(0),
    };
    Parallel::for_range(0, n as i32, |index| functor.evaluate(index as usize), false).unwrap();
    assert_eq!(
        functor.races.load(Ordering::SeqCst),
        0,
        "Data race detected in concurrent TopLoc_Location::Transformation() access"
    );
}
