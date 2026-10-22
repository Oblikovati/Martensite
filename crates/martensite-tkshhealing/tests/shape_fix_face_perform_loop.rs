//! Regression: ShapeFix_Face::Perform splits a wire through one vertex twice even when
//! ShapeFix_Wire::Perform changes nothing (natively FixLoopWire runs after every wire of
//! the second pass). The native comparison is the sh_fix_face requests of
//! tools/tkshhealing_corpus_fix_face.py.
use martensite_tkmath::gp::Point3;
use martensite_tkshhealing::shape_extend::ShapeExtendStatus;
use martensite_tkshhealing::shape_fix::ShapeFixFace;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeWire,
};

fn figure_eight_face() -> martensite_tkbrep::topods::Face {
    let p = |x: f64, y: f64| Point3::new(x, y, 0.);
    let points = [
        p(0., 0.),
        p(2., 0.),
        p(1., 1.),
        p(2., 2.),
        p(0., 2.),
        p(1., 1.),
        p(0., 0.),
    ];
    let mut wire = BRepBuilderAPIMakeWire::new();
    for pair in points.windows(2) {
        let edge = BRepBuilderAPIMakeEdge::from_points(pair[0], pair[1])
            .unwrap()
            .edge()
            .unwrap();
        wire.add_edge(&edge).unwrap();
    }
    BRepBuilderAPIMakeFace::from_wire(&wire.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn perform_splits_a_loop_wire_left_unchanged_by_the_wire_fix() {
    let fixer = ShapeFixFace::new(figure_eight_face().into_shape());
    assert!(fixer.perform().unwrap());
    assert!(fixer.status(ShapeExtendStatus::Done7));
    assert!(fixer.status(ShapeExtendStatus::Done1));
}

#[test]
fn fix_loop_wire_alone_reads_the_unloaded_wire_tool() {
    let fixer = ShapeFixFace::new(figure_eight_face().into_shape());
    let mut wires = martensite_tkernel::ncollection::Sequence::default();
    assert!(!fixer.fix_loop_wire(&mut wires).unwrap());
    assert_eq!(wires.len(), 0);
}
