//! Alpha descriptor selection retains the ordinary and OIT override bindings.
use ash::vk;
pub(super) fn selected_alpha_set(
    draw: &super::super::frame::UploadedDraw,
    selection: Option<vk::DescriptorSet>,
) -> Option<vk::DescriptorSet> {
    if let Some(base) = selection {
        return Some(
            draw.oit_alpha
                .iter()
                .find(|(key, _)| *key == base)
                .map_or(base, |(_, override_set)| override_set.set),
        );
    }
    draw.native_alpha.as_ref().map(|parameters| parameters.set)
}
