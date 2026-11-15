use super::*;

#[test]
fn a_centre_a_ten_thousandth_from_an_edge_snaps() {
    // The edge passes 0.0002 px left of the centre (10.5, 20.5), as in bug29807_b5a (162,74).
    let triangle = [[10.4998, 0.], [10.4998, 40.], [0., 20.]];
    assert!(snap_pixels(&[triangle], [64, 64]).contains(&(10, 20)));
}

#[test]
fn a_centre_a_hundredth_from_an_edge_does_not_snap() {
    let triangle = [[10.49, 0.], [10.49, 40.], [0., 20.]];
    assert!(!snap_pixels(&[triangle], [64, 64]).contains(&(10, 20)));
    let diagonal = [[0.01, 0.], [40.01, 40.], [0., 40.]];
    assert!(
        snap_pixels(&[diagonal], [64, 64])
            .iter()
            .all(|&(x, y)| x != y)
    );
}

#[test]
fn diagonal_edges_and_segment_ends_are_measured_exactly() {
    let diagonal = [[0., 0.], [40., 40.], [0., 40.]];
    let pixels = snap_pixels(&[diagonal], [64, 64]);
    assert!((0..40).all(|i| pixels.contains(&(i, i))));
    assert!(!pixels.contains(&(41, 41)), "beyond the end of the segment");
    assert_eq!(
        snap_pixels_json(&BTreeSet::from([(1, 2)])),
        "\"snap_pixels\": [[1, 2]]"
    );
}
