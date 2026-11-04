//! Authored camera/triangle fixtures reuse the existing backend without a view-setter override.
use super::super::gradient_layer;
use crate::{PrimitiveDraw, RenderFrame, RenderLayer, RenderVertex};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkservice::{
    aspect::AspectGradientFillMethod,
    graphic3d::{Graphic3dCamera, Graphic3dCameraProjection},
};

#[derive(Clone, Copy)]
pub(super) struct GradientMatrixCase {
    pub label: &'static str,
    pub extent: [u32; 2],
    pub method: AspectGradientFillMethod,
    pub foreground: bool,
}
use AspectGradientFillMethod::{Horizontal, Vertical};
pub(super) const MATRIX_CASES: [GradientMatrixCase; 7] = [
    GradientMatrixCase {
        label: "horizontal-64x64-0",
        extent: [64, 64],
        method: Horizontal,
        foreground: false,
    },
    GradientMatrixCase {
        label: "vertical-64x64-0",
        extent: [64, 64],
        method: Vertical,
        foreground: false,
    },
    GradientMatrixCase {
        label: "horizontal-64x64-1",
        extent: [64, 64],
        method: Horizontal,
        foreground: true,
    },
    GradientMatrixCase {
        label: "vertical-64x64-1",
        extent: [64, 64],
        method: Vertical,
        foreground: true,
    },
    GradientMatrixCase {
        label: "horizontal-97x43-1",
        extent: [97, 43],
        method: Horizontal,
        foreground: true,
    },
    GradientMatrixCase {
        label: "vertical-43x97-1",
        extent: [43, 97],
        method: Vertical,
        foreground: true,
    },
    GradientMatrixCase {
        label: "return-horizontal-64x64-0",
        extent: [64, 64],
        method: Horizontal,
        foreground: false,
    },
];

/// Prepare immutable direct-frame inputs; `matrix_frame(case, true)` enables the authored gradient.
pub(super) fn matrix_frame(case: GradientMatrixCase, with_gradient: bool) -> RenderFrame {
    let colors = [[0.125, 0.25, 0.75, 1.], [0.75, 0.5, 0.125, 1.]];
    let mut layers: Vec<RenderLayer> = Vec::new();
    if with_gradient {
        layers.push(
            gradient_layer(case.method, colors, case.extent)
                .unwrap()
                .unwrap(),
        );
    }
    let model = if case.foreground {
        vec![matrix_foreground(case.extent)]
    } else {
        Vec::new()
    };
    layers.push(RenderLayer::new(model, true, true, true));
    RenderFrame::with_layers(case.extent[0], case.extent[1], [0., 0., 1., 1.], layers).unwrap()
}
fn matrix_foreground(extent: [u32; 2]) -> PrimitiveDraw {
    let vertices = [[-8., -8., 0.], [8., -8., 0.], [0., 8., 0.]]
        .into_iter()
        .map(|position| RenderVertex::new(position, [1., 0., 0., 1.]))
        .collect();
    PrimitiveDraw::new(vertices, vec![0, 1, 2], matrix_camera_clip(extent)).unwrap()
}
fn matrix_camera_clip(extent: [u32; 2]) -> [f32; 16] {
    let camera = Graphic3dCamera::default();
    camera.set_eye(&Point3::new(0., 0., 20.)).unwrap();
    camera.set_center(&Point3::default()).unwrap();
    camera.set_up(&Direction3::new(0., 1., 0.).unwrap());
    camera.set_scale(20.);
    camera.set_aspect(f64::from(extent[0]) / f64::from(extent[1]));
    camera.set_z_range(1., 100.).unwrap();
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    (camera.projection_matrix() * camera.orientation_matrix())
        .data()
        .map(|value| value as f32)
}
