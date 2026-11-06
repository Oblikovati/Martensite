//! Exact fixed units are admitted explicitly; missing features never fall back to V1.
use crate::{RenderDepthBiasBehavior as Behavior, RenderDepthBiasFailure as Failure, RenderError};
use ash::{Instance, vk};
pub(super) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    behavior: Behavior,
) -> Result<(), RenderError> {
    if behavior == Behavior::HardwareFormatV1 {
        return Ok(());
    }
    let extension = super::depth_equivalence::device_lists_extension(
        instance,
        physical,
        ash::ext::depth_bias_control::NAME,
    );
    let feature = queried_feature(instance, physical, extension);
    require_control(
        extension,
        feature.depth_bias_control == vk::TRUE,
        feature.float_representation == vk::TRUE,
        feature.depth_bias_exact == vk::TRUE,
    )
}
fn queried_feature(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    extension: bool,
) -> vk::PhysicalDeviceDepthBiasControlFeaturesEXT<'static> {
    let mut feature = vk::PhysicalDeviceDepthBiasControlFeaturesEXT::default();
    if !extension {
        return feature;
    }
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut feature);
    // SAFETY: physical belongs to this live instance; initialized nodes outlive the query.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    feature
}

pub(super) fn require_control(
    extension: bool,
    control: bool,
    float: bool,
    exact: bool,
) -> Result<(), RenderError> {
    if extension && control && float && exact {
        return Ok(());
    }
    Err(Failure::MissingExactFloatControl {
        extension,
        control,
        float,
        exact,
    }
    .into())
}
pub(super) fn feature(
    behavior: Behavior,
) -> vk::PhysicalDeviceDepthBiasControlFeaturesEXT<'static> {
    let selected = behavior == Behavior::Native24UnitsV2;
    vk::PhysicalDeviceDepthBiasControlFeaturesEXT::default()
        .depth_bias_control(selected)
        .float_representation(selected)
        .depth_bias_exact(selected)
}
pub(super) fn command(
    instance: &Instance,
    device: &ash::Device,
    behavior: Behavior,
) -> Result<Option<ash::ext::depth_bias_control::Device>, RenderError> {
    if behavior == Behavior::HardwareFormatV1 {
        return Ok(None);
    }
    // SAFETY: the device is live and was created with this extension and all required features.
    let address = unsafe {
        instance.get_device_proc_addr(device.handle(), c"vkCmdSetDepthBias2EXT".as_ptr())
    };
    require_command(address.is_some())?;
    Ok(Some(ash::ext::depth_bias_control::Device::new(
        instance, device,
    )))
}
fn require_command(available: bool) -> Result<(), RenderError> {
    if available {
        return Ok(());
    }
    Err(Failure::MissingCommand.into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_required_capability_and_command_is_indispensable() {
        for mask in 0..16 {
            let [e, c, f, x] = std::array::from_fn(|i| mask & (1 << i) != 0);
            assert_eq!(require_control(e, c, f, x).is_ok(), mask == 15);
            if mask != 15 {
                assert!(
                    require_control(e, c, f, x)
                        .unwrap_err()
                        .depth_bias_failure()
                        .is_some()
                );
            }
        }
        assert_eq!(
            require_command(false).unwrap_err().depth_bias_failure(),
            Some(&Failure::MissingCommand)
        );
        assert!(require_command(true).is_ok());
    }
    #[test]
    fn enabled_feature_chain_requests_exact_float_only_for_v2() {
        for (b, expected) in [
            (Behavior::HardwareFormatV1, vk::FALSE),
            (Behavior::Native24UnitsV2, vk::TRUE),
        ] {
            let f = feature(b);
            assert_eq!(
                [
                    f.depth_bias_control,
                    f.float_representation,
                    f.depth_bias_exact
                ],
                [expected; 3]
            );
            assert_eq!(
                f.least_representable_value_force_unorm_representation,
                vk::FALSE
            );
        }
    }
}
