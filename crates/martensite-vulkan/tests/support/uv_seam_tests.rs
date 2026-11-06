//! The UV-seam grader passes only controlled seam pixels with unchanged coverage, and the seam
//! comes from shared positions with different texture coordinates.
use super::*;

/// Vertices at `positions` (z = 0 unless given), projected to ten pixels per unit.
fn mesh(positions: &[[f64; 3]], uvs: &[[f64; 2]], triangles: &[[usize; 3]]) -> SeamMesh {
    SeamMesh {
        pixels: positions.iter().map(|p| [p[0] * 10., p[1] * 10.]).collect(),
        positions: positions.to_vec(),
        uvs: uvs.to_vec(),
        triangles: triangles.to_vec(),
        eye: [0.5, 0.5, 5.],
    }
}

#[test]
fn front_facing_edges_between_seam_vertices_form_the_seam() {
    let positions = [
        [0., 0., 0.],
        [1., 0., 0.],
        [1., 1., 0.],
        [1., 0., 0.],
        [2., 0., 0.],
    ];
    let uvs = [[0., 0.], [1., 0.], [1., 1.], [0., 0.], [1., 0.]];
    // Vertex 1 and 3 share a position with different UVs; vertex 2 has no twin.
    let seam = seam_segments(&mesh(&positions, &uvs, &[[0, 1, 2], [3, 4, 2]]));
    assert!(
        seam.is_empty(),
        "one seam vertex makes no seam edge: {seam:?}"
    );
    let positions = [positions.as_slice(), &[[1., 1., 0.]]].concat();
    let uvs = [uvs.as_slice(), &[[0., 1.]]].concat();
    let seam = seam_segments(&mesh(&positions, &uvs, &[[0, 1, 2], [3, 4, 5]]));
    // Each side of the seam contributes its own edge at the same pixels.
    assert_eq!(seam, vec![[[10., 0.], [10., 10.]]; 2]);
}

#[test]
fn back_facing_seam_triangles_add_only_their_silhouette_edges() {
    let positions = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [1., 1., 0.],
        [1., 1., 0.],
    ];
    let uvs = [[0., 0.], [1., 0.], [0., 1.], [1., 1.], [0., 1.]];
    // [0, 1, 2] faces the eye; [1, 2, 3] is wound away and touches seam vertex 3.
    let seam = seam_segments(&mesh(&positions, &uvs, &[[0, 1, 2], [1, 2, 3]]));
    assert_eq!(seam, vec![[[10., 0.], [0., 10.]]]);
    let seam = seam_segments(&mesh(&positions, &uvs, &[[0, 1, 2], [1, 3, 2]]));
    assert!(seam.is_empty(), "both triangles face the eye: {seam:?}");
}

#[test]
fn outline_edges_need_a_triangle_touching_a_seam_vertex() {
    // The approved scope (2026-10-29) takes outline edges only from triangles that share a
    // seam vertex: the same back-facing triangle without one adds nothing.
    let positions = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [1., 1., 0.],
        [1., 1., 0.],
    ];
    let uvs = [[0., 0.], [1., 0.], [0., 1.], [1., 1.], [1., 1.]];
    let seam = seam_segments(&mesh(&positions, &uvs, &[[0, 1, 2], [1, 2, 3]]));
    assert!(seam.is_empty(), "equal UVs make no seam vertex: {seam:?}");
}

#[test]
fn an_outline_pixel_passes_only_with_the_control_proof() {
    // The outline edge (1,0)-(0,1), projected to (2,0)-(0,1), passes 0.22 px from the centre
    // of pixel (1, 0), the differing pixel of the 4x1 strip.
    let positions = [
        [0., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [1., 1., 0.],
        [1., 1., 0.],
    ];
    let uvs = [[0., 0.], [1., 0.], [0., 1.], [1., 1.], [0., 1.]];
    let mut outline_mesh = mesh(&positions, &uvs, &[[0, 1, 2], [1, 2, 3]]);
    outline_mesh.pixels = positions.iter().map(|p| [p[0] * 2., p[1]]).collect();
    let outline = seam_segments(&outline_mesh);
    assert_eq!(outline, vec![[[2., 0.], [0., 1.]]]);
    let proven = seam_case([10, 83, 96], [0; 3]);
    assert_eq!(seam_grade(&proven, true, &outline), (0, 1));
    let unproven = seam_case([11, 83, 96], [0; 3]);
    assert_eq!(seam_grade(&unproven, true, &outline), (1, 0));
    let coverage = seam_case([10, 83, 96], [1, 0, 0]);
    assert_eq!(seam_grade(&coverage, true, &outline), (1, 0));
    assert_eq!(seam_grade(&proven, false, &outline), (1, 0));
}

#[test]
fn the_seam_reach_is_one_pixel() {
    let seam = [[[10., 0.], [10., 10.]]];
    assert!(on_seam(&seam, 9, 4));
    assert!(on_seam(&seam, 10, 4));
    assert!(!on_seam(&seam, 11, 4));
    assert!(!on_seam(&seam, 8, 4));
}

/// A 4x1 strip: background, two covered pixels, background; pixel 1 differs by 95 levels.
fn seam_case(control_pixel: [u8; 3], rust_last: [u8; 3]) -> [Vec<u8>; 3] {
    let native = [[0, 0, 0], [10, 83, 96], [20, 80, 96], [0, 0, 0]].concat();
    let rust = [[0, 0, 0], [105, 84, 96], [20, 80, 96], rust_last].concat();
    let control = [[0, 0, 0], control_pixel, [20, 80, 96], [0, 0, 0]].concat();
    [rust, native, control]
}

fn seam_grade(case: &[Vec<u8>; 3], control: bool, segments: &[SeamSegment]) -> (usize, usize) {
    let [rust, native, control_rgb] = case;
    let control = control.then_some(control_rgb.as_slice());
    let index = 3;
    let pixel = |bytes: &[u8]| bytes[index..index + 3].to_vec();
    let pixels = vec![(1, 0, pixel(rust), pixel(native), control.map(pixel))];
    let (left, graded) = grade_uv_seam(pixels, rust, native, control, 4, segments);
    (left.len(), graded)
}

const THROUGH_PIXEL_1: [SeamSegment; 1] = [[[1.5, 0.], [1.5, 1.]]];

#[test]
fn a_controlled_seam_pixel_passes() {
    let case = seam_case([10, 83, 96], [0; 3]);
    assert_eq!(seam_grade(&case, true, &THROUGH_PIXEL_1), (0, 1));
}

#[test]
fn a_seam_pixel_the_control_does_not_reproduce_is_rejected() {
    let case = seam_case([11, 83, 96], [0; 3]);
    assert_eq!(seam_grade(&case, true, &THROUGH_PIXEL_1), (1, 0));
}

#[test]
fn a_coverage_change_rejects_the_seam_pixels() {
    let case = seam_case([10, 83, 96], [1, 0, 0]);
    assert_eq!(seam_grade(&case, true, &THROUGH_PIXEL_1), (1, 0));
}

#[test]
fn pixels_off_the_seam_and_hardware_are_rejected() {
    let case = seam_case([10, 83, 96], [0; 3]);
    let far: [SeamSegment; 1] = [[[3.6, 0.], [3.6, 1.]]];
    assert_eq!(seam_grade(&case, true, &far), (1, 0));
    assert_eq!(seam_grade(&case, false, &THROUGH_PIXEL_1), (1, 0));
}
