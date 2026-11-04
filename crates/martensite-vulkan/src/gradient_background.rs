//! Background draws retain their viewport chart and never participate in model traversal.
use crate::background_chart::{BackgroundChart, BackgroundGeometry};
use crate::colored_quad_background::{ColoredQuadBackground, colored_quad, corner_index};
use crate::{PrimitiveDraw, RenderError, RenderLayer, RenderVertex, error::rejected};
use martensite_tkservice::{aspect::AspectGradientFillMethod, graphic3d::*};

/// Prepend the unlit backgrounds after model extraction, the gradient before the image
/// (GlView::drawBackground), and return the corner gradient that the frame draws as its own
/// background; `prepend_background(view, [64,64], &mut passes)?`.
pub(crate) fn prepend_background(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    passes: &mut Vec<RenderLayer>,
) -> Result<Option<ColoredQuadBackground>, RenderError> {
    if !draws_flat_background(view.cview_base().fields.read().background_type) {
        return Ok(None);
    }
    // Native ffpEnable renders the backgrounds without sRGB (crate::fixed_background).
    let fixed = crate::fixed_view_lighting::uses_fixed_view_lighting(view);
    if let Some(mut image) = crate::image_background::image_background_layer(view, extent)? {
        if fixed {
            crate::fixed_background::encode_fixed_image(&mut image);
        }
        passes.insert(0, image);
    }
    if !crate::image_background::gradient_under_image(view) {
        return Ok(None);
    }
    prepend_gradient(view, (extent, fixed), passes)
}
fn prepend_gradient(
    view: &dyn Graphic3dCView,
    (extent, fixed): ([u32; 2], bool),
    passes: &mut Vec<RenderLayer>,
) -> Result<Option<ColoredQuadBackground>, RenderError> {
    let camera = crate::view_render_camera::owned_render_camera(view);
    let chart = BackgroundChart::new(&camera, extent, BackgroundGeometry::Gradient)?;
    let gradient = view.gradient_background();
    let method = gradient.bg_gradient_fill_method();
    if corner_index(method).is_some() {
        // GlView::drawBackground binds myColoredQuadParams for Corner1..4.
        let size = chart.pixel_size();
        let quad = colored_quad(method, gradient.colors(), chart.world_view(), size);
        return Ok(quad.map(|quad| ColoredQuadBackground {
            encoded: fixed,
            ..quad
        }));
    }
    let colors = gradient.colors();
    let layer = chart_gradient_layer(method, [rgba(colors.0), rgba(colors.1)], &chart)?;
    if let Some(mut layer) = layer {
        if fixed {
            crate::fixed_background::encode_fixed_gradient(&mut layer, method);
        }
        passes.insert(0, layer);
    }
    Ok(None)
}
/// Whether drawBackground draws the flat backgrounds (gradient and image) for `kind`; every
/// background kind is supported. Example: `draws_flat_background(view.background_type())`.
pub(crate) fn draws_flat_background(kind: Graphic3dTypeOfBackground) -> bool {
    match kind {
        Graphic3dTypeOfBackground::None => false,
        // drawBackground draws the gradient and the image for both kinds (image_background.rs).
        Graphic3dTypeOfBackground::Gradient | Graphic3dTypeOfBackground::Texture => true,
        // GlView::drawBackground draws only the cube map (environment_frame.rs).
        Graphic3dTypeOfBackground::Cubemap => false,
    }
}
fn rgba(color: martensite_tkernel::quantity::Color) -> [f32; 4] {
    [
        color.red() as f32,
        color.green() as f32,
        color.blue() as f32,
        1.,
    ]
}
/// The gradient layer of `method` on an untiled `extent` viewport;
/// `gradient_layer(AspectGradientFillMethod::Vertical, colors, [64, 64])?`.
#[cfg(test)]
fn gradient_layer(
    method: AspectGradientFillMethod,
    colors: [[f32; 4]; 2],
    extent: [u32; 2],
) -> Result<Option<RenderLayer>, RenderError> {
    let camera = Graphic3dCamera::default();
    let chart = BackgroundChart::new(&camera, extent, BackgroundGeometry::Gradient)?;
    chart_gradient_layer(method, colors, &chart)
}
/// The gradient layer of `method` on `chart`: geometry over the chart's view size (the camera
/// tile's total size when tiled), drawn through its world-view matrix.
fn chart_gradient_layer(
    method: AspectGradientFillMethod,
    colors: [[f32; 4]; 2],
    chart: &BackgroundChart,
) -> Result<Option<RenderLayer>, RenderError> {
    if method == AspectGradientFillMethod::Elliptical {
        return elliptical::layer(colors, chart).map(Some);
    }
    gradient_quad_layer(method, colors, chart)
}
fn gradient_quad_layer(
    method: AspectGradientFillMethod,
    colors: [[f32; 4]; 2],
    chart: &BackgroundChart,
) -> Result<Option<RenderLayer>, RenderError> {
    let Some(order) = gradient_corner_order(method)? else {
        return Ok(None);
    };
    let transform = chart.world_view();
    let palette = [colors[0], colors[1], diagonal_midpoint(colors)];
    let vertices = gradient_pixel_vertices(chart.pixel_size(), palette, order);
    Ok(Some(unlit_gradient_layer(
        vertices,
        vec![0, 1, 3, 1, 2, 3],
        transform,
    )?))
}
fn unlit_gradient_layer(
    vertices: Vec<RenderVertex>,
    indices: Vec<u32>,
    transform: [f32; 16],
) -> Result<RenderLayer, RenderError> {
    let draw = PrimitiveDraw::new(vertices, indices, transform)?;
    // GlView renders its background before scene layers, with depth testing/writes disabled.
    Ok(RenderLayer::new(vec![draw], false, false, false))
}

fn diagonal_midpoint(colors: [[f32; 4]; 2]) -> [f32; 4] {
    // Native BackgroundArray averages the authored linear f32 colors before raster interpolation.
    let [first, second] = colors;
    std::array::from_fn(|channel| 0.5_f32 * (first[channel] + second[channel]))
}

fn gradient_pixel_vertices(
    extent: [u32; 2],
    colors: [[f32; 4]; 3],
    order: [usize; 4],
) -> Vec<RenderVertex> {
    let [width, height] = extent.map(|value| value as f32);
    let points = [
        [width, 0., 0.],
        [width, height, 0.],
        [0., height, 0.],
        [0., 0., 0.],
    ];
    points
        .into_iter()
        .zip(order)
        .map(|(point, index)| RenderVertex::new(point, colors[index]))
        .collect()
}
fn gradient_corner_order(
    method: AspectGradientFillMethod,
) -> Result<Option<[usize; 4]>, RenderError> {
    use AspectGradientFillMethod::{Diagonal1, Diagonal2, Horizontal, Vertical};
    match method {
        AspectGradientFillMethod::None => Ok(None),
        Horizontal => Ok(Some([1, 1, 0, 0])),
        Vertical => Ok(Some([1, 0, 0, 1])),
        Diagonal1 => Ok(Some([1, 2, 0, 2])),
        Diagonal2 => Ok(Some([2, 0, 2, 1])),
        // Corner1..4 are colored quads (crate::colored_quad_background), not vertex colours.
        _ => Err(rejected(
            method,
            "None, Horizontal, Vertical, Diagonal1 or Diagonal2 gradient fill",
        )),
    }
}

#[cfg(test)]
#[path = "gradient_background_device_tests.rs"]
mod device_tests;
#[cfg(test)]
#[path = "gradient_background_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "gradient_diagonal_tests.rs"]
mod diagonal_tests;

#[cfg(test)]
#[path = "../tests/support/elliptical_gradient_reference.rs"]
mod elliptical_reference;
#[cfg(test)]
#[path = "gradient_elliptical_reference_tests.rs"]
mod elliptical_reference_tests;

#[path = "gradient_elliptical.rs"]
mod elliptical;
