//! GlFrameStatsPrs: the on-screen counters text and timer chart. The content
//! is retained Graphic3d structures with the view's 2D persistence, lowered through the same
//! text and primitive paths as scene structures and drawn after every layer with depth writes
//! off, as `renderFrameStats` in the immediate pass.
use crate::Graphic3dRenderStructure as RenderStructure;
use crate::frame_stats_chart::ChartContent;
use crate::{RenderError, RenderFrameStats, RenderLayer, error::rejected};
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
use martensite_tkservice::graphic3d::*;

/// One retained overlay structure and its single group.
struct OverlayStructure {
    structure: Handle<Graphic3dStructure>,
    group: Handle<dyn Graphic3dGroup>,
}

impl OverlayStructure {
    fn new(manager: &Handle<Graphic3dStructureManager>) -> Result<Self, RenderError> {
        let structure = Graphic3dStructure::new(manager, &Handle::default())
            .map_err(|error| rejected(error, "an overlay structure of the view manager"))?;
        let group = structure
            .new_group()
            .map_err(|error| rejected(error, "an overlay group"))?;
        Ok(Self { structure, group })
    }
}

/// The text layout key whose change releases the counters text (native `Release`).
type TextKey = (
    i32,
    Graphic3dHorizontalTextAlignment,
    Graphic3dVerticalTextAlignment,
);

struct OverlayParts {
    previous: Handle<RenderFrameStats>,
    counters: OverlayStructure,
    chart: OverlayStructure,
    text_key: Option<TextKey>,
}

/// Per-view statistics overlay; `overlay.layer(&view, &stats, extent)?`.
#[derive(Default)]
pub(crate) struct FrameStatsOverlay {
    parts: SharedState<Option<OverlayParts>>,
}

impl FrameStatsOverlay {
    /// The overlay layer when `ToShowStats` and counters are requested, else none.
    pub fn layer(
        &self,
        view: &dyn Graphic3dCView,
        stats: &RenderFrameStats,
        extent: [u32; 2],
    ) -> Result<Option<RenderLayer>, RenderError> {
        let params = view.rendering_params();
        if !params.stats.to_show_stats
            || params.stats.collected_stats == Graphic3dRenderingParamsPerfCounters::NONE
        {
            return Ok(None);
        }
        let mut parts = self.parts.write();
        if parts.is_none() {
            *parts = Some(new_parts(&view.structure_manager())?);
        }
        let parts = parts.as_mut().expect("created above");
        update_parts(parts, stats, &params, extent)?;
        overlay_layer(parts, view, extent, &params).map(Some)
    }
}

fn new_parts(manager: &Handle<Graphic3dStructureManager>) -> Result<OverlayParts, RenderError> {
    Ok(OverlayParts {
        previous: Handle::default(),
        counters: OverlayStructure::new(manager)?,
        chart: OverlayStructure::new(manager)?,
        text_key: None,
    })
}

/// Native `Update`: refresh only when the major counters changed or the text was released.
fn update_parts(
    parts: &mut OverlayParts,
    stats: &RenderFrameStats,
    params: &Graphic3dRenderingParams,
    extent: [u32; 2],
) -> Result<(), RenderError> {
    let key = counters_text_key(params);
    let released = parts.text_key != Some(key);
    parts
        .counters
        .structure
        .set_transform_persistence(&params.stats.stats_position.clone().cast());
    parts
        .chart
        .structure
        .set_transform_persistence(&params.stats.chart_position.clone().cast());
    if !stats.is_frame_updated(&mut parts.previous) && !released {
        return Ok(());
    }
    parts.text_key = Some(key);
    let text = stats.format_stats(params.stats.collected_stats);
    fill_counters(
        &parts.counters,
        params,
        key,
        &String::from_utf8_lossy(text.as_bytes()),
    );
    let viewport = extent.map(|value| value as i32);
    fill_chart(
        &parts.chart,
        params,
        ChartContent::build(stats, params, viewport),
    );
    Ok(())
}

/// Text height and the alignment toward the counters corner.
fn counters_text_key(params: &Graphic3dRenderingParams) -> TextKey {
    let corner = params
        .stats
        .stats_position
        .get()
        .and_then(|pers| pers.corner_2d().ok())
        .map_or(0, |corner| corner.0);
    let horizontal = match corner {
        c if c & Corner::LEFT.0 != 0 => Graphic3dHorizontalTextAlignment::Left,
        c if c & Corner::RIGHT.0 != 0 => Graphic3dHorizontalTextAlignment::Right,
        _ => Graphic3dHorizontalTextAlignment::Center,
    };
    let vertical = match corner {
        c if c & Corner::TOP.0 != 0 => Graphic3dVerticalTextAlignment::Top,
        c if c & Corner::BOTTOM.0 != 0 => Graphic3dVerticalTextAlignment::Bottom,
        _ => Graphic3dVerticalTextAlignment::Center,
    };
    (params.stats.stats_text_height, horizontal, vertical)
}

fn overlay_text(height: i32, value: &str, position: Point3) -> Handle<Graphic3dText> {
    let text = Handle::new(Graphic3dText::new(height as f32));
    text.set_text_str(value);
    text.set_position(&position);
    text
}

fn fill_counters(
    counters: &OverlayStructure,
    params: &Graphic3dRenderingParams,
    (height, horizontal, vertical): TextKey,
    value: &str,
) {
    counters.group.clear(false);
    counters
        .group
        .set_group_primitives_aspect(&params.stats.stats_text_aspect.clone().cast());
    let text = overlay_text(height, value, Point3::default());
    text.set_horizontal_alignment(horizontal);
    text.set_vertical_alignment(vertical);
    counters.group.add_text(&text, false);
}

fn fill_chart(
    chart: &OverlayStructure,
    params: &Graphic3dRenderingParams,
    content: Option<ChartContent>,
) {
    chart.group.clear(false);
    let Some(content) = content else {
        return;
    };
    chart.group.set_group_primitives_aspect(&chart_aspect());
    chart.group.add_primitive_array(&content.bins, false);
    chart.group.add_primitive_array(&content.frame_lines, false);
    chart
        .group
        .set_primitives_aspect(&params.stats.stats_text_aspect.clone().cast());
    for (value, position, horizontal) in content.labels {
        let text = overlay_text(params.stats.stats_text_height, &value, position);
        text.set_horizontal_alignment(horizontal);
        text.set_vertical_alignment(content.label_vertical);
        chart.group.add_text(&text, false);
    }
}

/// The chart program: unlit, blended per-vertex colors; white lines (interior color).
fn chart_aspect() -> Handle<dyn Graphic3dAspectsSource> {
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    aspect.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    aspect.set_interior_color(martensite_tkernel::quantity::Color::named(
        martensite_tkernel::quantity::NamedColor::WHITE,
    ));
    aspect.cast()
}

/// Counters then chart, depth-tested without depth writes (GlFrameStatsPrs::Render).
fn overlay_layer(
    parts: &OverlayParts,
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    params: &Graphic3dRenderingParams,
) -> Result<RenderLayer, RenderError> {
    let camera = crate::view_render_camera::owned_render_camera(view);
    let mut program = crate::stencil_program::StencilProgram::default();
    for part in [&parts.counters, &parts.chart] {
        let cstructure = part.structure.cstructure();
        // Overlay structures live in no layer, so no culling traversal clears the initial flag.
        if let Some(cstructure) = cstructure.get() {
            cstructure.mark_as_not_culled();
        }
        let render = cstructure.cast::<RenderStructure>();
        let render = render.get().ok_or_else(|| {
            rejected(
                cstructure.object_address(),
                "a Martensite overlay structure",
            )
        })?;
        let clipping = Handle::default();
        // GlFrameStatsPrs draws without depth writes; the overlay holds no user elements.
        let pass = crate::render_element_pass::RenderElementPass::opaque(false);
        program.append(render.draws(
            &camera,
            view.identification(),
            extent,
            params,
            &clipping,
            &pass,
        )?);
    }
    let mut layer = RenderLayer::from_stencil_program(program, true, false, false);
    for draw in &mut layer.draws {
        draw.depth_clamp = true;
    }
    Ok(layer)
}

#[cfg(test)]
#[path = "frame_stats_overlay_tests.rs"]
mod tests;
