use super::*;

#[test]
fn horizontal_edges_on_pixel_centres_are_ties() {
    let triangle = [[10., 187.5], [30., 187.5], [20., 214.5]];
    let edges = triangle_tie_edges(&triangle);
    let top = TieEdge {
        y: 187.5,
        x0: 10.,
        x1: 30.,
        interior: 1,
    };
    assert_eq!(edges, vec![top]);
}

#[test]
fn edges_between_pixel_centres_are_not_ties() {
    let triangle = [[10., 187.], [30., 187.], [20., 214.25]];
    assert!(triangle_tie_edges(&triangle).is_empty());
}

#[test]
fn sidecar_names_size_and_edges() {
    let edge = TieEdge {
        y: 204.5,
        x0: 1.,
        x1: 2.,
        interior: -1,
    };
    let line = ThinLine {
        from: [1., 2.5],
        to: [9., 2.5],
        depth: [0.25, 0.5],
    };
    let dash = DashedLine {
        segment: [line.from, line.to],
        pattern: 0xFFC0,
        factor: 1,
    };
    let snaps = std::collections::BTreeSet::from([(5, 6)]);
    let json = tie_edges_json(&[edge], &[line], &[dash], &[[3., 4.5]], &snaps, [409, 409]);
    assert!(json.contains("\"size\": [409, 409]"), "{json}");
    assert!(json.contains("\"thin_lines\": [{\"x0\": 1.0, \"y0\": 2.5, \"x1\": 9.0, \"y1\": 2.5, \"z0\": 0.25, \"z1\": 0.5}]"), "{json}");
    assert!(
        json.contains("\"y\": 204.5, \"x0\": 1.0, \"x1\": 2.0, \"interior\": -1"),
        "{json}"
    );
    assert!(
        json.contains("\"points\": [{\"x\": 3.0, \"y\": 4.5}]"),
        "{json}"
    );
    let dashed = "\"dashed_lines\": [{\"x0\": 1.0, \"y0\": 2.5, \"x1\": 9.0, \"y1\": 2.5, \"pattern\": 65472, \"factor\": 1}]";
    assert!(json.contains(dashed), "{json}");
    assert!(json.contains("\"snap_pixels\": [[5, 6]]"), "{json}");
}
