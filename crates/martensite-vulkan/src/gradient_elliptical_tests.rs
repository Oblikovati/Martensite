//! Fan topology and authored color checks are independent of raster-device sampling.
use super::*;
const ELLIPTICAL_COLORS: [[f32; 4]; 2] = [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]];

/// The fan of an untiled `extent` viewport.
fn extent_layer(colors: [[f32; 4]; 2], extent: [u32; 2]) -> Result<RenderLayer, RenderError> {
    use crate::background_chart::BackgroundGeometry;
    let camera = martensite_tkservice::graphic3d::Graphic3dCamera::default();
    layer(
        colors,
        &BackgroundChart::new(&camera, extent, BackgroundGeometry::Gradient)?,
    )
}

#[test]
fn elliptical_fan_keeps_center_rim_colors_and_all_sixty_four_triangles() {
    let layer = extent_layer(ELLIPTICAL_COLORS, [97, 43]).unwrap();
    let draw = &layer.draws[0];
    assert_eq!(draw.vertices.len(), 66);
    assert_eq!(draw.vertices[0].position, [48.5, 21.5, 0.]);
    assert_eq!(draw.vertices[0].color, ELLIPTICAL_COLORS[0]);
    assert!(
        draw.vertices[1..]
            .iter()
            .all(|vertex| vertex.color == ELLIPTICAL_COLORS[1])
    );
    assert_eq!(draw.indices.len(), 192);
    assert_elliptical_fan_indices(&draw.indices);
    assert!(!layer.depth_test && !layer.depth_write && !layer.clear_depth);
}

fn assert_elliptical_fan_indices(indices: &[u32]) {
    for (triangle, indices) in indices.as_chunks::<3>().0.iter().enumerate() {
        assert_eq!(*indices, [0, triangle as u32 + 1, triangle as u32 + 2]);
    }
}

#[test]
fn elliptical_pixel_chart_preserves_odd_asymmetric_and_single_pixel_extents() {
    for extent in [[97, 43], [43, 97], [1, 1], [409, 409]] {
        let layer = extent_layer(ELLIPTICAL_COLORS, extent).unwrap();
        let mut expected = crate::identity_clip();
        expected[0] = 2. / extent[0] as f32;
        expected[5] = 2. / extent[1] as f32;
        expected[12] = -1.;
        expected[13] = -1.;
        assert_eq!(layer.draws[0].clip_transform(), &expected);
    }
}

#[test]
fn elliptical_zero_extent_is_rejected_before_geometry_construction() {
    for extent in [[0, 43], [97, 0], [0, 0]] {
        let failure = extent_layer(ELLIPTICAL_COLORS, extent).unwrap_err();
        assert!(
            failure
                .to_string()
                .contains("positive background viewport dimensions")
        );
        assert!(failure.to_string().contains(&format!("{extent:?}")));
    }
}

#[test]
fn elliptical_axes_follow_the_circumscribed_ellipse() {
    let vertices = elliptical_vertices(ELLIPTICAL_COLORS, [2, 2]);
    let radius = std::f64::consts::SQRT_2;
    for (index, expected) in [
        (1, [1. + radius, 1.]),
        (17, [1., 1. + radius]),
        (33, [1. - radius, 1.]),
        (49, [1., 1. - radius]),
    ] {
        for (actual, coordinate) in vertices[index].position[..2].iter().zip(expected) {
            assert!((f64::from(*actual) - coordinate).abs() < 2. * f64::from(f32::EPSILON));
        }
    }
}

#[test]
fn elliptical_generated_draw_owns_authored_colors_after_source_mutation() {
    let mut colors = ELLIPTICAL_COLORS;
    let layer = extent_layer(colors, [97, 43]).unwrap();
    colors[0] = [1.; 4];
    colors[1] = [0.; 4];
    assert_ne!(colors, ELLIPTICAL_COLORS);
    assert_eq!(layer.draws[0].vertices[0].color, ELLIPTICAL_COLORS[0]);
    assert_eq!(layer.draws[0].vertices[65].color, ELLIPTICAL_COLORS[1]);
}

#[test]
fn elliptical_all_vertex_bits_match_retained_actual_native_arrays() {
    let fixtures = [
        ([64, 64], include_str!("fixtures/elliptical/64x64.txt")),
        ([97, 43], include_str!("fixtures/elliptical/97x43.txt")),
        ([43, 97], include_str!("fixtures/elliptical/43x97.txt")),
        ([1, 1], include_str!("fixtures/elliptical/1x1.txt")),
        ([409, 409], include_str!("fixtures/elliptical/409x409.txt")),
    ];
    for (extent, fixture) in fixtures {
        assert_native_elliptical_vertices(extent, fixture);
    }
}

fn assert_native_elliptical_vertices(extent: [u32; 2], fixture: &str) {
    let vertices = elliptical_vertices(ELLIPTICAL_COLORS, extent);
    assert_eq!(fixture.lines().count(), vertices.len());
    for (index, (vertex, line)) in vertices.iter().zip(fixture.lines()).enumerate() {
        let expected: Vec<u32> = line
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        let actual = elliptical_vertex_bits(vertex);
        assert_eq!(
            actual.as_slice(),
            expected,
            "extent {extent:?}, vertex {index}"
        );
    }
}

fn elliptical_vertex_bits(vertex: &RenderVertex) -> [u32; 5] {
    [
        vertex.position[0],
        vertex.position[1],
        vertex.color[0],
        vertex.color[1],
        vertex.color[2],
    ]
    .map(f32::to_bits)
}
