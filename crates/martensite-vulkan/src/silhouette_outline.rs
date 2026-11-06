//! Silhouette outlines (Graphic3dAspects::ToDrawSilhouette): GlPrimitiveArray::Render
//! redraws a back-culled closed triangulation front-culled with the outline variant of the
//! unlit program (getStdProgramUnlit, theIsOutline), which pushes every vertex along its normal
//! by `occSilhouetteThickness * (occOrthoScale > 0 ? occOrthoScale: gl_Position.w)` and
//! writes the edge colour. The displacement is evaluated here in the shader's float order; the
//! displaced triangles then use the ordinary unlit program.
use crate::{PrimitiveDraw, RenderError, RenderFaceCulling, error::rejected};
use martensite_tkservice::graphic3d::*;

/// The outline uniforms of one view: `occSilhouetteThickness` and `occOrthoScale`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SilhouetteParams {
    pub thickness: f32,
    pub ortho_scale: f32,
}

impl SilhouetteParams {
    /// EdgeWidth * LineWidthScale / min(viewport) and the orthographic camera scale (else -1);
    /// `SilhouetteParams::of(&*aspects, &camera, [w, h], line_scale)?`.
    pub fn of(
        aspects: &dyn Graphic3dAspectsSource,
        camera: &Graphic3dCamera,
        extent: Option<[u32; 2]>,
        line_scale: f32,
    ) -> Result<Self, RenderError> {
        let extent =
            extent.ok_or_else(|| rejected("no viewport", "a view extent for silhouettes"))?;
        let min = extent[0].min(extent[1]) as f32;
        Ok(Self {
            thickness: aspects.line_width() * line_scale / min,
            ortho_scale: match camera.is_orthographic() {
                true => camera.scale() as f32,
                false => -1.,
            },
        })
    }
}

/// The outline draw of a back-culled triangle draw, `None` when the native condition
/// (`ToDrawSilhouette && ToCullBackFaces`) does not hold; `outline_draw(&draw, &*aspects, p)?`.
pub(crate) fn outline_draw(
    draw: &PrimitiveDraw,
    aspects: &dyn Graphic3dAspectsSource,
    params: SilhouetteParams,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if !aspects.to_draw_silhouette()
        || draw.topology() != crate::PrimitiveTopology::Triangles
        || draw.face_culling != RenderFaceCulling::Back
    {
        return Ok(None);
    }
    let color = edge_color(aspects);
    let normals = draw.lighting.as_ref().map(|lighting| lighting.normals());
    let vertices = draw
        .vertices
        .iter()
        .enumerate()
        .map(|(index, vertex)| {
            // A missing normal attribute reads as zero: no displacement.
            let normal = normals
                .and_then(|n| n.get(index))
                .copied()
                .unwrap_or([0.; 3]);
            let [x, y, z] = displaced(&draw.clip_transform, vertex, normal, params);
            crate::RenderVertex::homogeneous([x, y, z, vertex.weight], color)
        })
        .collect();
    let mut outline = PrimitiveDraw::new(vertices, draw.indices.clone(), draw.clip_transform)?
        .with_face_culling(RenderFaceCulling::Front);
    outline.clipping = draw.clipping.clone();
    outline.polygon_offset = draw.polygon_offset;
    outline.depth_clamp = draw.depth_clamp;
    outline.mirrored = draw.mirrored;
    Ok(Some(outline))
}

fn edge_color(aspects: &dyn Graphic3dAspectsSource) -> [f32; 4] {
    let color = aspects.edge_color_rgba();
    [
        color.rgb().red() as f32,
        color.rgb().green() as f32,
        color.rgb().blue() as f32,
        color.alpha(),
    ]
}

/// `anOutlinePos = occVertex + vec4(occNormal * (occSilhouetteThickness * anOutlineDisp), 0)`,
/// with `gl_Position.w` summed as clip_position.glsl does.
fn displaced(
    clip: &[f32; 16],
    vertex: &crate::RenderVertex,
    normal: [f32; 3],
    params: SilhouetteParams,
) -> [f32; 3] {
    let ([x, y, z], weight) = (vertex.position, vertex.weight);
    let w = ((clip[3] * x + clip[7] * y) + clip[11] * z) + clip[15] * weight;
    let displacement = if params.ortho_scale > 0. {
        params.ortho_scale
    } else {
        w
    };
    let step = params.thickness * displacement;
    std::array::from_fn(|axis| vertex.position[axis] + normal[axis] * step)
}

#[cfg(test)]
#[path = "silhouette_outline_tests.rs"]
mod tests;
