//! Executable native group elements remain separate from retained style.
use crate::{PrimitiveDraw, RenderError, primitive::PrimitiveSubmission};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::graphic3d::*;
#[derive(Clone)]
pub(super) enum GroupCommand {
    Aspects(Handle<dyn Graphic3dAspectsSource>),
    Flipping(Option<Axis2>),
    Stencil(bool),
    Geometry(GroupGeometry),
    Element(std::sync::Arc<dyn crate::RenderElement>),
}
#[derive(Clone)]
pub(super) enum GroupGeometry {
    Primitives(PrimitiveSubmission),
    Text(
        Handle<Graphic3dText>,
        crate::element_upload::ElementUploadSlot,
        crate::render_element::TextPlacement,
    ),
}
impl GroupCommand {
    pub(super) fn advance(
        self,
        aspects: &mut Handle<dyn Graphic3dAspectsSource>,
        flips: &mut crate::group_flipping::GroupFlipStack,
        program: &mut crate::stencil_program::StencilProgram,
    ) -> Option<GroupGeometry> {
        match self {
            Self::Aspects(style) => advance_aspect(aspects, style),
            Self::Flipping(plane) => advance_flipping(flips, plane),
            Self::Stencil(enabled) => advance_stencil(program, enabled),
            Self::Geometry(geometry) => Some(geometry),
            // Elements are called by the extraction loop before advancing.
            Self::Element(_) => None,
        }
    }
}
impl GroupGeometry {
    /// Native IsFillDrawMode of the built-in element; texts never fill.
    /// Whether ray tracing renders this geometry (GlRaytrace::IsRaytracedElement).
    pub(super) fn is_raytraced(&self) -> bool {
        match self {
            Self::Primitives(primitive) => {
                !primitive.attributes.is_null()
                    && crate::raytrace::is_raytraced_primitive(primitive.kind)
            }
            Self::Text(..) => false,
        }
    }
    pub(super) fn is_fill(&self) -> bool {
        match self {
            Self::Primitives(primitive) => crate::render_element_pass::is_fill_primitive(
                primitive.kind,
                !primitive.attributes.is_null(),
            ),
            Self::Text(..) => false,
        }
    }
    pub(super) fn lower(
        self,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        closed: bool,
        matrix: [f32; 16],
        highlighted: bool,
        context: &crate::text_submission::TextSceneContext<'_>,
    ) -> Result<Vec<PrimitiveDraw>, RenderError> {
        if empty_line_geometry(&self, aspects) {
            return Ok(Vec::new());
        }
        match self {
            Self::Text(text, upload, placement) => {
                let draws =
                    lower_ordered_text(context, (&text, placement), &upload, aspects, closed)?;
                #[cfg(feature = "vertex-output-diagnostics")]
                return context.group.tag_vertex_output_text(&text, draws);
                #[cfg(not(feature = "vertex-output-diagnostics"))]
                Ok(draws)
            }
            Self::Primitives(primitive) => {
                let draws = super::lower_hatched_surface(
                    &primitive,
                    aspects,
                    closed,
                    matrix,
                    highlighted,
                    context,
                )?;
                #[cfg(feature = "vertex-output-diagnostics")]
                let draws = context
                    .group
                    .tag_vertex_output_primitive(&primitive, draws)?;
                Ok(draws)
            }
        }
    }
}

fn empty_line_geometry(
    geometry: &GroupGeometry,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> bool {
    let GroupGeometry::Primitives(primitive) = geometry else {
        return false;
    };
    // GlPrimitiveArray::Render returns before VBOs, shaders and line state.
    crate::element_stats::is_line_array(primitive.kind)
        && aspects.get().is_some_and(|style| {
            style.line_type() == martensite_tkservice::aspect::AspectTypeOfLine::Empty
        })
}

fn lower_ordered_text(
    context: &crate::text_submission::TextSceneContext<'_>,
    (text, placement): (&Graphic3dText, crate::render_element::TextPlacement),
    upload: &crate::element_upload::ElementUploadSlot,
    aspects: &Handle<dyn Graphic3dAspectsSource>,
    closed: bool,
) -> Result<Vec<PrimitiveDraw>, RenderError> {
    let draws = match placement {
        crate::render_element::TextPlacement::Scene => context.draws(text, aspects, Some(upload)),
        crate::render_element::TextPlacement::Pixel => context.pixel_draws(text, aspects, upload),
    };
    draws.map(|mut draws| {
        crate::text_culling::apply_text_face_culling(&mut draws, &**aspects, closed);
        draws
    })
}

fn advance_aspect(
    aspects: &mut Handle<dyn Graphic3dAspectsSource>,
    style: Handle<dyn Graphic3dAspectsSource>,
) -> Option<GroupGeometry> {
    *aspects = style;
    None
}
fn advance_flipping(
    flips: &mut crate::group_flipping::GroupFlipStack,
    plane: Option<Axis2>,
) -> Option<GroupGeometry> {
    flips.apply(plane);
    None
}
fn advance_stencil(
    program: &mut crate::stencil_program::StencilProgram,
    enabled: bool,
) -> Option<GroupGeometry> {
    program.command(crate::stencil_program::StencilCommand::Options(enabled));
    None
}

#[cfg(test)]
#[path = "graphic_group_empty_tests.rs"]
mod empty_tests;
