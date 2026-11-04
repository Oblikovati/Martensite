//! Closed command scope refuses side effects rather than changing their order or semantics.
mod draw_scope;
mod point_upload;
use super::LitOnlyFailure;
use crate::{RenderFrame, RenderStencilState};

pub(super) fn validate_frame(
    frame: &RenderFrame,
    stencil: RenderStencilState,
) -> Result<(), LitOnlyFailure> {
    validate_raster(frame, stencil)?;
    validate_passes(frame)?;
    validate_layers(frame)?;
    for (ordinal, draw) in frame.draws.iter().enumerate() {
        draw_scope::validate(ordinal, draw)?;
    }
    Ok(())
}
fn validate_raster(frame: &RenderFrame, stencil: RenderStencilState) -> Result<(), LitOnlyFailure> {
    reject_frame(
        frame.transparency() != crate::RenderTransparency::Unordered,
        "OIT",
    )?;
    reject_frame(frame.raster.requested_samples > 0, "MSAA")?;
    reject_frame(stencil.is_enabled(), "initial stencil")
}
fn validate_passes(frame: &RenderFrame) -> Result<(), LitOnlyFailure> {
    reject_frame(frame.shader_grid.is_some(), "shader grid")?;
    reject_frame(frame.raytrace.is_some(), "raytrace")?;
    reject_frame(frame.stereo_pair.is_some(), "stereo")?;
    reject_frame(frame.environment.is_some(), "environment")?;
    reject_frame(frame.colored_quad.is_some(), "colored quad")?;
    reject_frame(frame.shadow_maps.is_some(), "shadow maps")
}
fn validate_layers(frame: &RenderFrame) -> Result<(), LitOnlyFailure> {
    reject_frame(frame.gui_draw_start.is_some(), "GUI")?;
    reject_frame(frame.flip_output, "output flip")?;
    reject_frame(!frame.subview_blits.is_empty(), "subview blits")?;
    reject_frame(
        frame
            .layers
            .iter()
            .any(|l| !l.stencil_boundaries.is_empty()),
        "stencil boundary",
    )
}
fn reject_frame(active: bool, route: &'static str) -> Result<(), LitOnlyFailure> {
    if active {
        return Err(LitOnlyFailure::Frame {
            route,
            actual: "active=true".into(),
        });
    }
    Ok(())
}

pub(super) fn validate_uploads(
    frame: &RenderFrame,
    draws: &[super::super::frame::UploadedDraw],
    samples: ash::vk::SampleCountFlags,
) -> Result<(), LitOnlyFailure> {
    validate_upload_count_and_samples(frame.draws.len(), draws.len(), samples)?;
    for (ordinal, (source, uploaded)) in frame.draws.iter().zip(draws).enumerate() {
        validate_uploaded_route(ordinal, source.lighting.is_some(), uploaded)?;
        if source.topology() == crate::PrimitiveTopology::Points {
            point_upload::validate_point_upload(ordinal, uploaded)?;
        }
    }
    Ok(())
}
fn validate_upload_count_and_samples(
    source_count: usize,
    uploaded_count: usize,
    samples: ash::vk::SampleCountFlags,
) -> Result<(), LitOnlyFailure> {
    if samples != ash::vk::SampleCountFlags::TYPE_1 {
        return Err(LitOnlyFailure::Frame {
            route: "actual sample count",
            actual: format!("{samples:?}"),
        });
    }
    if source_count != uploaded_count {
        return Err(LitOnlyFailure::Frame {
            route: "upload count",
            actual: format!("{}/{}", source_count, uploaded_count),
        });
    }
    Ok(())
}
fn validate_uploaded_route(
    ordinal: usize,
    lit: bool,
    uploaded: &super::super::frame::UploadedDraw,
) -> Result<(), LitOnlyFailure> {
    draw_scope::reject(
        ordinal,
        uploaded.user_program.is_some(),
        "uploaded user program",
    )?;
    draw_scope::reject(ordinal, uploaded.oit.is_some(), "uploaded OIT")?;
    validate_uploaded_assembly(ordinal, lit, uploaded)
}
fn validate_uploaded_assembly(
    ordinal: usize,
    lit: bool,
    uploaded: &super::super::frame::UploadedDraw,
) -> Result<(), LitOnlyFailure> {
    draw_scope::reject(
        ordinal,
        uploaded.line.wide.is_some(),
        "wide-line replication",
    )?;
    draw_scope::reject(
        ordinal,
        lit != uploaded.pipeline.kind.lighting,
        "uploaded lighting mismatch",
    )?;
    validate_uploaded_observation(ordinal, uploaded)
}
fn validate_uploaded_observation(
    _ordinal: usize,
    _uploaded: &super::super::frame::UploadedDraw,
) -> Result<(), LitOnlyFailure> {
    #[cfg(feature = "vertex-output-diagnostics")]
    draw_scope::reject(
        _ordinal,
        _uploaded.vertex_output.is_some(),
        "transform feedback",
    )?;
    Ok(())
}
