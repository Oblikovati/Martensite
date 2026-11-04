//! Temporary immutable command control: upload/state stay complete; only draw commands differ.
mod scope;
#[cfg(test)]
mod tests;
use crate::{RenderError, RenderFrame, RenderStencilState};
use std::{ffi::OsStr, fmt};

pub(super) const FLAG: &str = "MARTENSITE_LIT_ONLY_COMMAND_CONTROL";
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum LitOnlyCommands {
    #[default]
    Normal,
    PhongTriangles,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LitOnlyFailure {
    Setting(String),
    Frame {
        route: &'static str,
        actual: String,
    },
    Draw {
        ordinal: usize,
        route: &'static str,
        actual: String,
    },
}
impl fmt::Display for LitOnlyFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "lit-only command control received {self:?}; expected single-sample opaque built-in Phong triangles and side-effect-free unlit lines/binary AlphaSprite points"
        )
    }
}
impl std::error::Error for LitOnlyFailure {}
impl From<LitOnlyFailure> for RenderError {
    fn from(value: LitOnlyFailure) -> Self {
        Self::lit_only_commands(value)
    }
}

impl LitOnlyCommands {
    pub(super) fn from_environment() -> Result<Self, RenderError> {
        Self::requested(std::env::var_os(FLAG).as_deref()).map_err(Into::into)
    }
    pub(super) fn requested(value: Option<&OsStr>) -> Result<Self, LitOnlyFailure> {
        match value {
            None => Ok(Self::Normal),
            Some(value) if value == "1" => Ok(Self::PhongTriangles),
            actual => Err(LitOnlyFailure::Setting(format!(
                "{FLAG}={actual:?}; expected unset or literal 1"
            ))),
        }
    }
    pub(super) fn validate_frame(
        self,
        frame: &RenderFrame,
        stencil: RenderStencilState,
    ) -> Result<(), RenderError> {
        if self == Self::Normal {
            return Ok(());
        }
        scope::validate_frame(frame, stencil).map_err(Into::into)
    }
    pub(super) fn validate_uploads(
        self,
        frame: &RenderFrame,
        draws: &[super::frame::UploadedDraw],
        samples: ash::vk::SampleCountFlags,
    ) -> Result<(), RenderError> {
        if self == Self::Normal {
            return Ok(());
        }
        scope::validate_uploads(frame, draws, samples).map_err(Into::into)
    }
    pub(super) fn begin_frame(
        self,
        command: ash::vk::CommandBuffer,
        draws: &[super::frame::UploadedDraw],
        subpixel_bits: u32,
    ) {
        use ash::vk::Handle;
        if self == Self::Normal {
            return;
        }
        let lit = draws
            .iter()
            .filter(|draw| draw.pipeline.kind.lighting)
            .count();
        eprintln!(
            "{}",
            frame_manifest(command.as_raw(), draws.len(), lit, subpixel_bits)
        );
    }
    pub(super) fn issue(self, lit: bool, sink: &mut impl FinalDrawCommand) {
        let retained = self == Self::Normal || lit;
        if retained {
            sink.issue_draw();
        }
        if self != Self::Normal {
            sink.record_decision(retained);
        }
    }
}

pub(super) trait FinalDrawCommand {
    fn issue_draw(&mut self);
    fn record_decision(&mut self, retained: bool);
}

fn frame_manifest(command: u64, uploaded: usize, retained: usize, subpixel_bits: u32) -> String {
    format!(
        "{{\"kind\":\"lit-only-command-frame\",\"command\":{command},\"uploaded_draws\":{uploaded},\"expected_retained\":{retained},\"expected_skipped\":{},\"subpixel_precision_bits\":{subpixel_bits},\"scope\":\"diagnostic-only\",\"qualification\":false}}",
        uploaded - retained
    )
}
