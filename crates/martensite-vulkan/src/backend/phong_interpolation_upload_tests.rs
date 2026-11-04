use super::*;
fn phong_triangle() -> PrimitiveDraw {
    let vertices = [[-1., -1., 0.], [1., -1., 0.], [-1., 1., 0.]]
        .map(|p| crate::RenderVertex::new(p, [1.; 4]))
        .to_vec();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
    let lighting = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        crate::RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    draw.with_lighting(lighting).unwrap()
}
#[test]
fn selected_domain_is_admitted_without_mutating_input() {
    let draw = phong_triangle();
    let snapshot = draw.clone();
    assert!(admit(Behavior::UnsnappedV2, &draw, [0., 0., 409., 409.]).is_ok());
    assert_eq!(draw, snapshot);
    assert!(require_interface(Behavior::UnsnappedV2, &draw, true).is_ok());
    assert!(
        require_interface(Behavior::UnsnappedV2, &draw, false)
            .unwrap_err()
            .phong_interpolation_failure()
            .is_some()
    );
}
#[test]
fn v2_refuses_nonpositive_w_while_hardware_remains_replayable() {
    let mut draw = phong_triangle();
    draw.clip_transform[15] = -1.;
    assert!(admit(Behavior::HardwareV1, &draw, [0., 0., 4., 4.]).is_ok());
    assert!(matches!(
        admit(Behavior::UnsnappedV2, &draw, [0., 0., 4., 4.])
            .unwrap_err()
            .phong_interpolation_failure(),
        Some(Failure::InvalidTriangle { triangle: 0, .. })
    ));
    assert!(require_interface(Behavior::HardwareV1, &draw, false).is_ok());
}
#[test]
fn unsupported_selected_interfaces_stay_on_hardware_and_other_shading_is_unchanged() {
    let mut draw = phong_triangle();
    draw.fixed_function_lighting = true;
    assert!(admit(Behavior::UnsnappedV2, &draw, [0., 0., 4., 4.]).is_ok());
    assert!(require_interface(Behavior::UnsnappedV2, &draw, false).is_ok());
    draw.lighting.as_mut().unwrap().shading = RenderShading::Gouraud;
    assert!(admit(Behavior::UnsnappedV2, &draw, [0., 0., 4., 4.]).is_ok());
    assert!(require_interface(Behavior::UnsnappedV2, &draw, false).is_ok());
}
#[test]
fn actual_viewport_sign_and_ordered_transform_are_preserved() {
    let draw = phong_triangle();
    assert_eq!(
        clip_position(draw.clip_transform, draw.vertices[0]),
        [-1., -1., 0., 1.]
    );
    let coordinates = crate::RenderCoordinateConvention::default();
    let actual = viewport(
        coordinates,
        vk::Extent2D {
            width: 4,
            height: 5,
        },
    );
    assert_eq!(actual[2].abs(), 4.);
    assert_eq!(actual[3].abs(), 5.);
}

#[test]
fn v3_retains_original_degenerate_vertices_indices_material_and_draw_count() {
    let mut draw = phong_triangle();
    for vertex in &mut draw.vertices {
        vertex.position[0] = 0.;
    }
    draw.indices = vec![2, 0, 1, 1, 2, 0];
    let original = draw.clone();
    assert!(admit(Behavior::UnsnappedV3, &draw, [0., 0., 409., 409.]).is_ok());
    assert_eq!(draw, original);
    assert_eq!(draw.indices.len() / 3, 2);
    assert!(admit(Behavior::UnsnappedV2, &draw, [0., 0., 409., 409.]).is_err());
    assert!(admit(Behavior::HardwareV1, &draw, [0., 0., 409., 409.]).is_ok());
    assert!(require_interface(Behavior::UnsnappedV3, &draw, false).is_err());
}

#[test]
fn v3_retains_input_on_unsupported_interface_and_invalid_clip_refusals() {
    let mut draw = phong_triangle();
    draw.fixed_function_lighting = true;
    let original = draw.clone();
    assert!(admit(Behavior::UnsnappedV3, &draw, [0., 0., 4., 4.]).is_ok());
    assert_eq!(draw, original);
    draw.fixed_function_lighting = false;
    for bad in [0., -1., f32::INFINITY, f32::NAN] {
        draw.clip_transform[15] = bad;
        assert!(admit(Behavior::UnsnappedV3, &draw, [0., 0., 4., 4.]).is_err());
        assert_eq!(draw.clip_transform[15].to_bits(), bad.to_bits());
        assert_eq!(draw.vertices, original.vertices);
        assert_eq!(draw.indices, original.indices);
    }
}

#[test]
fn private_carrier_triangles_skip_the_numeric_domain_check() {
    let mut draw = phong_triangle();
    draw.indices = vec![0, 0, 0];
    draw.clip_transform[15] = 0.;
    assert!(admit(Behavior::UnsnappedV3, &draw, [0., 0., 4., 4.]).is_ok());
}

pub(super) fn phong_triangle_for_coverage() -> crate::PrimitiveDraw {
    phong_triangle()
}
