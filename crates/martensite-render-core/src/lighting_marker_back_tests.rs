//! Actual aspect snapshots distinguish unused back opacity from contributing front opacity.
use super::*;
use crate::{PointStyle, RenderVertex, identity_clip};
use martensite_tkservice::graphic3d::*;

fn actual_aspect_material(front: f32, back: f32) -> RenderMaterial {
    let aspect = Graphic3dAspects::default();
    aspect.set_distinguish(true);
    aspect.set_alpha_mode(Graphic3dAlphaMode::BlendAuto, 0.5);
    aspect.change_front_material(|material| material.set_alpha(front).unwrap());
    aspect.change_back_material(|material| material.set_alpha(back).unwrap());
    let snapshot = crate::retained_lighting::aspect_material(&aspect);
    assert_eq!(snapshot.common[0].diffuse[3], front);
    assert_eq!(snapshot.common[1].diffuse[3], back);
    snapshot
}

fn actual_marker() -> PrimitiveDraw {
    PrimitiveDraw::points(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        identity_clip(),
        PointStyle::new(3., [1, 1], vec![[1.; 4]]).unwrap(),
    )
    .unwrap()
}

fn nodal_snapshot(
    material: RenderMaterial,
    shading: RenderShading,
) -> Result<RenderLighting, RenderError> {
    RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        material,
        identity_clip(),
        identity_clip(),
        false,
        shading,
    )
}

#[test]
fn valid_back_material_opacity_does_not_reject_front_lit_markers() {
    for shading in [RenderShading::Gouraud, RenderShading::Phong] {
        for back in [0.5, 0., 0.25, 1.] {
            let material = actual_aspect_material(1., back);
            let lighting = nodal_snapshot(material, shading).unwrap();
            let result = actual_marker().with_lighting(lighting);
            assert!(
                result.is_ok(),
                "back opacity {back}: expected front-opaque marker: {result:?}"
            );
        }
    }
}

#[test]
fn contributing_translucent_front_marker_material_remains_explicitly_rejected() {
    for front in [0., 0.5] {
        let material = actual_aspect_material(front, 1.);
        let lighting = nodal_snapshot(material, RenderShading::Phong).unwrap();
        let error = actual_marker().with_lighting(lighting).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("opaque marker diffuse opacity equal to 1")
        );
    }
}

#[test]
fn invalid_marker_material_components_remain_rejected_before_publication() {
    for side in 0..2 {
        for invalid in [-0.25, 1.25, f32::NAN, f32::INFINITY] {
            let mut material = actual_aspect_material(1., 1.);
            material.common[side].diffuse[3] = invalid;
            let error = nodal_snapshot(material, RenderShading::Phong).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("finite diffuse opacity in [0,1]")
            );
        }
    }
}
