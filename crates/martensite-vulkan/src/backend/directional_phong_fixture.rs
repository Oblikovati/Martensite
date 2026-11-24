//! The lit surface fixture that the directional Phong and batch tests share.
use crate::{
    PrimitiveDraw, RenderLightSet, RenderLighting, RenderMaterial, RenderShading, RenderVertex,
    identity_clip,
};
use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;

/// A lit Phong surface of `count` vertices under two directional lights.
pub(crate) fn lit_surface(count: usize) -> PrimitiveDraw {
    let positions = [
        [-0.9, -0.7, 0.2, 1.],
        [-0.2, -0.7, 0.5, 1.],
        [-1.1, 1.2, 0.7, 2.],
        [0.1, -0.7, 0.1, 1.],
        [0.6, 0.7, 0.4, 1.],
        [0.9, -0.7, 0.8, 1.],
    ];
    let vertices = positions
        .into_iter()
        .cycle()
        .take(count)
        .map(|point| RenderVertex::homogeneous(point, [0.7, 0.5, 0.3, 1.]))
        .collect();
    let normals = [[0.1, 0.3, 0.9], [0.3, -0.1, 0.9], [0., 0.5, 0.8]]
        .into_iter()
        .cycle()
        .take(count)
        .collect();
    let light = RenderLighting::new(
        normals,
        RenderMaterial::default(),
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(directional_light_set());
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 3, 4, 5], identity_clip())
        .unwrap()
        .with_lighting(light)
        .unwrap()
}

/// Two directional lights with an ambient term.
pub(crate) fn directional_light_set() -> RenderLightSet {
    let directions = [[0.2, 0.3, 0.9, 0.], [-0.2, 0.1, -0.9, 0.]];
    RenderLightSet {
        ambient: [0.1, 0.07, 0.06, 1.],
        sources: directions
            .into_iter()
            .map(|position| crate::light_snapshot::PackedRenderLight {
                kind: Graphic3dTypeOfLightSource::Directional as u32,
                cast_shadows: false,
                color: [0.7, 0.5, 0.4, 1.],
                position,
                direction: [0.; 4],
                params: [0.; 4],
            })
            .collect::<Vec<_>>()
            .into(),
    }
}
