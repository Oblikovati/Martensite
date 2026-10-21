//! Finite midpoint action for all four adjacent edge orientation pairs.
mod fixture;
use fixture::{INITIAL, gapped_face, wire_edges};
use martensite_tkbrep::{topexp::Explorer, topods};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point2;
use martensite_tkshhealing::{
    shape_analysis::ShapeAnalysisEdge, shape_upgrade::ShapeUpgradeShapeConvertToBezier,
};

const JOINED: [[(f64, f64); 2]; 4] = [
    [(0.04, 0.), (2.015, 0.01)],
    [(2.015, 0.01), (2., 3.05)],
    [(0.025, 3.04), (2., 3.05)],
    [(0.04, 0.), (0.025, 3.04)],
];
#[test]
fn public_operation_joins_all_four_orientation_pairs_at_closed_form_midpoints() {
    let (face, original) = gapped_face();
    let converter = ShapeUpgradeShapeConvertToBezier::new(face.clone().into_shape());
    converter.set_2d_conversion(true);
    assert!(converter.perform(true).unwrap());
    let faces = Explorer::new(&converter.result(), ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let result = topods::face(faces.current()).unwrap();
    let wires = Explorer::new(&result, ShapeEnum::Wire, ShapeEnum::Shape).unwrap();
    let edges = wire_edges(&topods::wire(wires.current()).unwrap());
    assert_eq!(edges.len(), 4);
    assert_orientations(&edges);
    assert_points(&result, &edges, JOINED);
    assert_points(&face, &original, INITIAL);
}
fn assert_orientations(edges: &[martensite_tkbrep::topods::Edge]) {
    assert_eq!(
        edges
            .iter()
            .map(|edge| edge.orientation())
            .collect::<Vec<_>>(),
        [
            Orientation::Forward,
            Orientation::Forward,
            Orientation::Reversed,
            Orientation::Reversed
        ]
    );
}
fn assert_points(
    face: &martensite_tkbrep::topods::Face,
    edges: &[martensite_tkbrep::topods::Edge],
    expected: [[(f64, f64); 2]; 4],
) {
    for (edge, points) in edges.iter().zip(expected) {
        let (curve, first, last) = ShapeAnalysisEdge::new()
            .pcurve_on_face(edge, face, false)
            .unwrap();
        assert_eq!((first, last), (0., 1.));
        for (parameter, point) in [(0., points[0]), (1., points[1])] {
            let expected = Point2::new(point.0, point.1);
            let distance = curve.value(parameter).unwrap().distance(expected);
            assert!(distance <= 1e-14, "pcurve endpoint error {distance}");
        }
    }
}
