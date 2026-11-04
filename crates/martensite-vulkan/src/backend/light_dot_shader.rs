//! The versioned policy applies only to owned built-in programmable Phong modules.
//! Custom programs, PBR and fixed-function shaders retain their own untouched semantics.
use super::{fragment_variant_registry::FragmentVariant, light_dot_shader_table};
use crate::{RenderError, RenderLightDotBehavior};

pub(super) fn builtin_phong_kind(kind: super::pipeline_variants::PipelineVariant) -> bool {
    kind.lighting && !kind.fixed_lighting && !kind.pbr
}

pub(super) fn selected_fragment(
    behavior: RenderLightDotBehavior,
    builtin_phong: bool,
    stock: &[u8],
) -> Result<Option<&'static [u8]>, RenderError> {
    selected_module(
        behavior,
        builtin_phong,
        stock,
        light_dot_shader_table::FRAGMENT_GROUPS
            .iter()
            .flat_map(|group| group.iter()),
        "a registered built-in Phong final fragment for NumericMaxZeroV2",
    )
}
pub(super) fn selected_vertex(
    behavior: RenderLightDotBehavior,
    builtin_phong: bool,
    stock: &[u8],
) -> Result<Option<&'static [u8]>, RenderError> {
    selected_module(
        behavior,
        builtin_phong,
        stock,
        light_dot_shader_table::VERTEX_MODULES.iter(),
        "a registered built-in Phong vertex for NumericMaxZeroV2",
    )
}
fn selected_module<'a>(
    behavior: RenderLightDotBehavior,
    builtin_phong: bool,
    stock: &[u8],
    variants: impl Iterator<Item = &'a FragmentVariant>,
    expected: &'static str,
) -> Result<Option<&'static [u8]>, RenderError> {
    if behavior == RenderLightDotBehavior::DriverNativeV1 || !builtin_phong {
        return Ok(None);
    }
    super::fragment_variant_registry::selected_fragment_variant(stock, variants, expected).map(Some)
}
#[cfg(test)]
#[path = "light_dot_shader_tests.rs"]
mod tests;
