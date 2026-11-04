//! Closed-form full-operation fixture; run only with an explicit shared GPU lease.
use crate::*;
#[path = "../tests/support/hatch_capture.rs"]
mod capture_support;
pub(crate) use capture_support::record as record_point_capture;

pub(crate) fn fixed_grid_renderer_with(coordinates: RenderCoordinateConvention) -> VulkanRenderer {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE")
        .ok()
        .map(|v| v.parse().unwrap());
    let renderer = VulkanRenderer::new_with_point_raster(
        VulkanOptions {
            validation: true,
            device_index,
        },
        Default::default(),
        coordinates,
        Default::default(),
        RenderPointRasterBehavior::FixedGridAliasedV2,
    )
    .unwrap();
    capture_support::record_identity(&renderer.driver_identity());
    renderer
}
fn plain_draw(size: f32, window: [f32; 2], weight: f32) -> PrimitiveDraw {
    let mut vertex = RenderVertex::new(
        [window[0] / 32. - 1., window[1] / 32. - 1., 0.5],
        [0., 1., 0., 1.],
    );
    vertex.weight = weight;
    let mut style = PointStyle::new(size, [1, 1], vec![[1.; 4]]).unwrap();
    style.color_policy = crate::point_style::PointColorPolicy::Plain;
    PrimitiveDraw::points(vec![vertex], vec![0], identity_clip(), style).unwrap()
}
fn require_box(
    capture: &FrameCapture,
    x: std::ops::RangeInclusive<usize>,
    y: std::ops::RangeInclusive<usize>,
) {
    for row in 0..64 {
        for column in 0..64 {
            let covered = x.contains(&column) && y.contains(&(63 - row));
            let index = row * 64 + column;
            let expected = if covered {
                [0, 255, 0, 255]
            } else {
                [0, 0, 0, 255]
            };
            assert_eq!(
                &capture.rgba[index * 4..index * 4 + 4],
                expected,
                "({column},{row})"
            );
            assert_eq!(
                capture.depth[index],
                if covered { 0.75 } else { 1. },
                "depth({column},{row})"
            );
        }
    }
}

#[test]
#[ignore = "requires explicitly authorized GPU lease and pinned validation environment"]
fn fixed_grid_owned_plain_fractional_odd_even_and_center_clipping() {
    let mut renderer = fixed_grid_renderer_with(Default::default());
    println!("MARTENSITE_DEVICE={}", renderer.device_name());
    for (size, first, last) in [
        (1., 16, 16),
        (1.5, 16, 16),
        (2., 15, 16),
        (2.5, 15, 16),
        (3., 15, 17),
        (3.5, 15, 17),
        (7., 13, 19),
    ] {
        let frame = RenderFrame::new(
            64,
            64,
            [0., 0., 0., 1.],
            vec![plain_draw(size, [16.25, 16.25], 1.)],
        )
        .unwrap();
        let capture = renderer.render(&frame).unwrap();
        record_point_capture(&format!("plain-size-{:08x}", size.to_bits()), &capture);
        require_box(&capture, first..=last, first..=last);
    }
    let frame = RenderFrame::new(
        64,
        64,
        [0., 0., 0., 1.],
        vec![plain_draw(7., [63.25, 16.25], 1.)],
    )
    .unwrap();
    let capture = renderer.render(&frame).unwrap();
    record_point_capture("inside-edge", &capture);
    require_box(&capture, 60..=63, 13..=19);
    for (window, weight) in [
        ([64.25, 16.25], 1.),
        ([16.25, 16.25], 0.),
        ([16.25, 16.25], -1.),
    ] {
        let frame = RenderFrame::new(
            64,
            64,
            [0., 0., 0., 1.],
            vec![plain_draw(7., window, weight)],
        )
        .unwrap();
        let capture = renderer.render(&frame).unwrap();
        record_point_capture(
            &format!(
                "clip-x-{:08x}-w-{:08x}",
                window[0].to_bits(),
                weight.to_bits()
            ),
            &capture,
        );
        assert!(
            capture
                .rgba
                .as_chunks::<4>()
                .0
                .iter()
                .all(|p| *p == [0, 0, 0, 255])
        );
        assert!(capture.depth.iter().all(|d| *d == 1.));
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

pub(crate) fn face_emission(color: [f32; 4]) -> RenderMaterialCommon {
    RenderMaterialCommon {
        diffuse: [0., 0., 0., 1.],
        emission: color,
        ambient: [0.; 4],
        specular_shininess: [0.; 4],
    }
}
pub(crate) fn lit_front_point(shading: RenderShading, mirrored: bool) -> PrimitiveDraw {
    let mut draw = plain_draw(3., [16.25, 16.25], 1.);
    draw.mirrored = mirrored;
    if mirrored {
        draw.clip_transform[0] = -1.;
    }
    let material = RenderMaterial {
        common: [
            face_emission([0., 1., 0., 1.]),
            face_emission([1., 0., 0., 1.]),
        ],
        ..Default::default()
    };
    let lighting = RenderLighting::new(
        vec![[0., 0., 1.]],
        material,
        identity_clip(),
        identity_clip(),
        false,
        shading,
    )
    .unwrap()
    .with_lights(RenderLightSet {
        ambient: [0.; 4],
        sources: vec![],
    });
    draw.with_lighting(lighting).unwrap()
}
#[test]
#[ignore = "requires explicitly authorized GPU lease and pinned validation environment"]
fn fixed_grid_owned_lit_front_material_survives_coordinate_and_model_mirrors() {
    for coordinates in [
        RenderCoordinateConvention::LegacyBottomUpV1,
        RenderCoordinateConvention::TopLeftLogicalV2,
    ] {
        let mut renderer = fixed_grid_renderer_with(coordinates);
        for (shading, mirrored) in [
            (RenderShading::Gouraud, false),
            (RenderShading::Gouraud, true),
            (RenderShading::Phong, false),
            (RenderShading::Phong, true),
        ] {
            let frame = RenderFrame::new(
                64,
                64,
                [0., 0., 0., 1.],
                vec![lit_front_point(shading, mirrored)],
            )
            .unwrap();
            let capture = renderer.render(&frame).unwrap();
            record_point_capture(
                &format!(
                    "lit-coords-{}-model-{}-mirror-{mirrored}",
                    coordinates.behavior_version(),
                    shading as u32
                ),
                &capture,
            );
            require_box(&capture, if mirrored { 46..=48 } else { 15..=17 }, 15..=17);
        }
        assert!(renderer.validation_errors().unwrap().is_empty());
    }
}
