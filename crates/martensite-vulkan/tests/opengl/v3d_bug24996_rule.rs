//! Grade the bug24996 wireframe frame by the diamond-exit line rule (D2 analytic reference).
//!
//! Native OpenGL drivers and Vulkan drivers snap line endpoints differently, so the native image
//! is not a cross-driver reference. The expected fragments come from the frame's own camera:
//! every box edge is projected with the submitted projection and orientation matrices and
//! rasterized by `line_diamond_rule::diamond_exit`; the bound is stated there.
use super::source_output::SourceCompletion;
use crate::hatch_surface::scene::{NamedSourceScene, SourceViewOutput};
use crate::line_diamond_rule::{Pixel, diamond_exit, rule_violation};
use martensite_tkservice::graphic3d::Graphic3dCamera;
use martensite_vulkan::{FrameCapture, Graphic3dRenderView};
use std::{collections::BTreeSet, error::Error};

/// The original's three 1×2×3 boxes, by corner; see `v3d_bug24996::source_boxes`.
const BOX_CORNERS: [[f64; 3]; 3] = [[0., 0., 0.], [3., 2., 1.], [5., -4., 0.]];
const BOX_SIZE: [f64; 3] = [1., 2., 3.];

/// Run the original commands and grade the frame; `assert_bug24996_line_rule()?`.
pub(super) fn assert_bug24996_line_rule() -> Result<(), Box<dyn Error>> {
    let completion = super::v3d_bug24996::case_commands()?;
    let capture = frame_capture(&completion)?;
    let camera = completion.scene.source_view_handle().camera();
    let segments = box_edges();
    let rule = segments
        .iter()
        .flat_map(|[a, b]| {
            diamond_exit(window(&camera, &capture, *a), window(&camera, &capture, *b))
        })
        .collect::<BTreeSet<Pixel>>();
    let drawn = drawn_pixels(&capture);
    let violation = rule_violation(&drawn, &rule, segments.len());
    completion.finish_after_end()?;
    match violation {
        Some(message) => Err(std::io::Error::other(message).into()),
        None => Ok(()),
    }
}

fn frame_capture(
    completion: &SourceCompletion<NamedSourceScene>,
) -> Result<FrameCapture, Box<dyn Error>> {
    let view = completion.scene.source_view_handle().get().unwrap().view();
    let render: martensite_tkernel::standard::Handle<Graphic3dRenderView> = view.cast();
    Ok(render.capture()?)
}

/// The twelve edges of each box: corner pairs differing in exactly one coordinate.
fn box_edges() -> Vec<[[f64; 3]; 2]> {
    let corner = |origin: [f64; 3], bits: usize| {
        [0, 1, 2].map(|k| origin[k] + if bits >> k & 1 == 1 { BOX_SIZE[k] } else { 0. })
    };
    let mut edges = Vec::new();
    for origin in BOX_CORNERS {
        for bits in 0..8usize {
            for k in 0..3 {
                if bits >> k & 1 == 0 {
                    edges.push([corner(origin, bits), corner(origin, bits | 1 << k)]);
                }
            }
        }
    }
    edges
}

/// Top-left window coordinates of a world point; `window(&camera, &capture, [0., 0., 0.])`.
fn window(camera: &Graphic3dCamera, capture: &FrameCapture, point: [f64; 3]) -> [f64; 2] {
    let eye = transform(
        camera.orientation_matrix().data(),
        [point[0], point[1], point[2], 1.],
    );
    let clip = transform(camera.projection_matrix().data(), eye);
    let (width, height) = (f64::from(capture.width), f64::from(capture.height));
    [
        (clip[0] / clip[3] + 1.) * width / 2.,
        (1. - clip[1] / clip[3]) * height / 2.,
    ]
}

/// Column-major matrix times vector.
fn transform(matrix: [f64; 16], vector: [f64; 4]) -> [f64; 4] {
    [0, 1, 2, 3].map(|row| {
        (0..4)
            .map(|column| matrix[column * 4 + row] * vector[column])
            .sum()
    })
}

fn drawn_pixels(capture: &FrameCapture) -> BTreeSet<Pixel> {
    let pixels = capture.rgba.as_chunks::<4>().0;
    let background = pixels[0];
    let width = capture.width as usize;
    (0..pixels.len())
        .filter(|&index| pixels[index] != background)
        .map(|index| ((index % width) as i64, (index / width) as i64))
        .collect()
}
