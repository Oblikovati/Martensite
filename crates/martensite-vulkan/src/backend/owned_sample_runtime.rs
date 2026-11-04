//! Explicit v2 carrier selection happens before publication, after querying the selected device.
use super::*;
use crate::backend::depth_equivalence::{DepthEquivalence, DepthFormats};

impl VulkanRuntime {
    pub(crate) fn create_for_owned_v2(options: VulkanOptions) -> Result<Arc<Self>, RenderError> {
        let mut runtime =
            Self::configured_runtime(options, Default::default(), Default::default())?;
        runtime.initialize_device(options.device_index)?;
        emit_owned_device(&runtime);
        let support = owned_support(&runtime)?;
        runtime.owned_v2_original_depth = Some(runtime.depth);
        runtime.depth = owned_v2_depth(runtime.depth, support.0, support.1)?;
        Ok(Arc::new(runtime))
    }
}
fn emit_owned_device(runtime: &VulkanRuntime) {
    let identity = crate::backend::owned_sample_metadata::device_identity(runtime);
    let identifiers = crate::backend::owned_sample_metadata::id_properties(runtime);
    println!(
        "{{\"event\":\"owned_v2_selected_device\",\"driver\":{identity},\"identifiers\":{identifiers}}}"
    );
}
fn owned_support(
    runtime: &VulkanRuntime,
) -> Result<(vk::FormatFeatureFlags, vk::SampleCountFlags), RenderError> {
    let format = vk::Format::D32_SFLOAT_S8_UINT;
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let features = unsafe {
        runtime
            .instance
            .get_physical_device_format_properties(runtime.physical, format)
    };
    let flags = features.optimal_tiling_features.as_raw();
    let raw_format = format.as_raw();
    println!(
        "{{\"event\":\"owned_v2_carrier_query\",\"format\":{raw_format},\"optimal_features\":{flags}}}"
    );
    let properties = owned_image_properties(runtime, format)?;
    emit_owned_samples(properties.sample_counts);
    Ok((features.optimal_tiling_features, properties.sample_counts))
}
fn emit_owned_samples(samples: vk::SampleCountFlags) {
    let raw = samples.as_raw();
    println!("{{\"event\":\"owned_v2_carrier_samples_query\",\"sample_counts\":{raw}}}");
    // The same returned flags govern the pre-publication sample1 refusal.
}
fn owned_image_properties(
    runtime: &VulkanRuntime,
    format: vk::Format,
) -> Result<vk::ImageFormatProperties, RenderError> {
    let usage = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_image_format_properties(
                runtime.physical,
                format,
                vk::ImageType::TYPE_2D,
                vk::ImageTiling::OPTIMAL,
                usage,
                vk::ImageCreateFlags::empty(),
            )
    }
    .map_err(|error| vk_error("query owned v2 D32S8 sample support", error))?;
    Ok(properties)
}
fn owned_v2_depth(
    actual: DepthEquivalence,
    support: vk::FormatFeatureFlags,
    samples: vk::SampleCountFlags,
) -> Result<DepthEquivalence, RenderError> {
    require_owned_support(support, samples)?;
    let mut selected = actual;
    selected.formats.stencil = vk::Format::D32_SFLOAT_S8_UINT;
    Ok(selected)
}
fn require_owned_support(
    support: vk::FormatFeatureFlags,
    samples: vk::SampleCountFlags,
) -> Result<(), RenderError> {
    let required =
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
    if !support.contains(required) {
        return Err(rejected(
            (vk::Format::D32_SFLOAT_S8_UINT, support),
            "v2 D32S8 attachment+TRANSFER_SRC support, no fallback",
        ));
    }
    if !samples.contains(vk::SampleCountFlags::TYPE_1) {
        return Err(rejected(samples, "v2 explicit D32S8 sample1 support"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_v2_carrier_preserves_actual_plain_format_and_enabled_clip_mode() {
        let support =
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
        for formats in [DepthFormats::UNORM24, DepthFormats::FLOAT] {
            for clip_control in [false, true] {
                let actual = DepthEquivalence {
                    formats,
                    clip_control,
                };
                let selected =
                    owned_v2_depth(actual, support, vk::SampleCountFlags::TYPE_1).unwrap();
                assert_eq!(selected.formats.plain, actual.formats.plain);
                assert_eq!(selected.clip_control, actual.clip_control);
                assert_eq!(selected.formats.stencil, vk::Format::D32_SFLOAT_S8_UINT);
            }
        }
    }
    #[test]
    fn v2_sample_support_query_cannot_be_substituted_with_multisample_only_support() {
        let flags =
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
        assert!(
            owned_v2_depth(
                DepthEquivalence::default(),
                flags,
                vk::SampleCountFlags::TYPE_4
            )
            .is_err()
        );
        assert!(
            owned_v2_depth(
                DepthEquivalence::default(),
                flags,
                vk::SampleCountFlags::empty()
            )
            .is_err()
        );
    }
    #[test]
    fn unsupported_v2_carrier_has_no_default_or_precision_fallback() {
        let depth = DepthEquivalence::default();
        assert!(
            owned_v2_depth(
                depth,
                vk::FormatFeatureFlags::empty(),
                vk::SampleCountFlags::TYPE_1
            )
            .is_err()
        );
        assert!(
            owned_v2_depth(
                depth,
                vk::FormatFeatureFlags::TRANSFER_SRC,
                vk::SampleCountFlags::TYPE_1
            )
            .is_err()
        );
    }
}
