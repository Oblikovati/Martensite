//! Bounding-box highlighting (AspectTOHM_BOUNDBOX): GlStructure::renderBoundingBox draws
//! the structure's world-space box with getStdProgramBoundBox after its groups, which render
//! without the highlight colour.
use crate::{LineStyle, PrimitiveDraw, RenderError, RenderVertex, error::rejected};
use martensite_tkernel::ncollection::Mat4;
use martensite_tkservice::{aspect::AspectTypeOfHighlightMethod, graphic3d::*};

/// Whether the structure is highlighted with its bounding box; `is_bound_box_highlight(&fields)`.
pub(crate) fn is_bound_box_highlight(structure: &Graphic3dCStructureFields) -> bool {
    structure.highlight
        && !structure.highlight_style.is_null()
        && structure.highlight_style.method() == AspectTypeOfHighlightMethod::Boundbox
}

/// The box lines of a box-highlighted structure in an `extent` viewport, `None` otherwise or for
/// an invalid box; `bound_box_draw(&fields, &camera, Some([200, 200]))?`.
pub(crate) fn bound_box_draw(
    structure: &Graphic3dCStructureFields,
    camera: &Graphic3dCamera,
    extent: Option<[u32; 2]>,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if !is_bound_box_highlight(structure) || !structure.bnd_box.is_valid() {
        return Ok(None);
    }
    let color = interior_color(structure)?;
    let (center, size) = (structure.bnd_box.center(), structure.bnd_box.size());
    let center = [center[0], center[1], center[2]].map(|value| value as f32);
    let size = [size[0], size[1], size[2]].map(|value| value as f32);
    let vertices = unit_box_lines()
        .into_iter()
        .map(|corner| RenderVertex::new(box_position(corner, size, center), color))
        .collect();
    // ModelWorldState is popped before the box, but revertPersistence runs after it: the box
    // keeps the persistence world-view and projection (witness
    // the recorded boundbox_persistence capture).
    let (clip, depth_clamp) = match structure.trsf_pers.is_null() {
        true => (
            crate::persistent_transform::ordinary_clip(camera, Mat4::<f64>::identity()),
            false,
        ),
        false => crate::persistent_transform::structure_persistence_clip(
            camera,
            &structure.trsf_pers,
            extent,
        )?,
    };
    let style = LineStyle::new(1., 0xffff, 1)?;
    let mut draw = PrimitiveDraw::segments(vertices, (0..24).collect(), clip, style)?;
    draw.depth_clamp = depth_clamp;
    Ok(Some(draw))
}

/// GlWorkspace::InteriorColor with the highlight style set: its RGBA colour.
fn interior_color(structure: &Graphic3dCStructureFields) -> Result<[f32; 4], RenderError> {
    let color = structure.highlight_style.color_rgba();
    let rgba = [
        color.rgb().red() as f32,
        color.rgb().green() as f32,
        color.rgb().blue() as f32,
        color.alpha(),
    ];
    if rgba[3] != 1. {
        // The program writes occColor without enabling blending; Vulkan would need its own state.
        return Err(rejected(rgba, "an opaque bounding-box highlight colour"));
    }
    Ok(rgba)
}

/// `occVertex.xyz * occBBoxSize + occBBoxCenter` of the vertex stage, in float.
fn box_position(corner: [f32; 3], size: [f32; 3], center: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|axis| corner[axis] * size[axis] + center[axis])
}

/// The 24 line vertices of GlShaderManager::prepareStdProgramBoundBox, in its order.
fn unit_box_lines() -> [[f32; 3]; 24] {
    const LOOKUP1: [f32; 4] = [0., 1., 0., 1.];
    const LOOKUP2: [f32; 4] = [0., 0., 1., 1.];
    let shift = |axis: usize, scale: f32| {
        let mut vector = [0f32; 3];
        vector[axis % 3] = scale;
        vector
    };
    let add = |a: [f32; 3], b: [f32; 3]| std::array::from_fn(|i| a[i] + b[i]);
    let mut lines = [[0f32; 3]; 24];
    for axis in 0..3 {
        for component in 0..4 {
            let base = add(
                add([-0.5; 3], shift(axis + 1, LOOKUP1[component])),
                shift(axis + 2, LOOKUP2[component]),
            );
            let index = 8 * axis + 2 * component;
            lines[index] = base;
            lines[index + 1] = add(base, shift(axis, 1.));
        }
    }
    lines
}

#[cfg(test)]
#[path = "bound_box_highlight_tests.rs"]
mod tests;
