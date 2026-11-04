//! A closed-form Lambert plane makes derivative orientation observable without a native runtime.
use super::*;
use crate::{
    PrimitiveDraw, RenderLightSet, RenderLighting, RenderMaterial, RenderMaterialCommon,
    RenderShading, RenderVertex, identity_clip,
};
fn lambert_material() -> RenderMaterial {
    let face = RenderMaterialCommon {
        diffuse: [1., 0., 0., 1.],
        emission: [0.; 4],
        specular_shininess: [0.; 4],
        ambient: [0.; 4],
    };
    RenderMaterial {
        common: [face; 2],
        ..Default::default()
    }
}
fn normal_light() -> RenderLightSet {
    let light = crate::light_snapshot::PackedRenderLight {
        kind: 1,
        cast_shadows: false,
        color: [1.; 4],
        position: [0., 0., 1., 0.],
        direction: [0.; 4],
        params: [0.; 4],
    };
    RenderLightSet {
        ambient: [0.; 4],
        sources: vec![light],
    }
}
fn facet_state() -> RenderLighting {
    RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        lambert_material(),
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::PhongFacet,
    )
    .unwrap()
    .with_lights(normal_light())
}
fn facet_frame() -> RenderFrame {
    let points = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = points.map(|p| RenderVertex::new(p, [1.; 4])).to_vec();
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip()).unwrap();
    let draw = draw.with_lighting(facet_state()).unwrap();
    RenderFrame::new(23, 17, [0., 0., 1., 1.], vec![draw]).unwrap()
}
#[test]
fn facet_derivatives_keep_positive_z_lambert_illumination() {
    let mut renderer = VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let capture = renderer.render(&facet_frame()).unwrap();
    assert_eq!(
        &capture.rgba[(8 * 23 + 11) * 4..(8 * 23 + 12) * 4],
        &[255, 0, 0, 255]
    );
    assert_eq!(capture.depth[8 * 23 + 11], renderer.stored_depth(0.5));
    assert!(renderer.validation_errors().unwrap().is_empty());
}
