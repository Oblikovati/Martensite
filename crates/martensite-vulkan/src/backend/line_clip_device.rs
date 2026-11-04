//! Derive optional line clipping from the selected implementation, never from case geometry.
use crate::{RenderLineClippingBehavior, RenderLineClippingRequirement};
use ash::{Instance, vk};

#[derive(Clone, Copy, Default)]
pub(super) struct LineClipDevice {
    required: bool,
    shader_distances: bool,
    maximum_distances: u32,
    maximum_combined_distances: u32,
}
impl LineClipDevice {
    pub fn query(
        instance: &Instance,
        physical: vk::PhysicalDevice,
        behavior: RenderLineClippingBehavior,
    ) -> Self {
        let identity = super::driver_identity::query_driver_identity(instance, physical);
        let (supported, clip, combined) = queried_clip_limits(instance, physical);
        Self::observed(behavior, identity.driver_id, supported, clip, combined)
    }
    fn observed(
        behavior: RenderLineClippingBehavior,
        driver: u32,
        shader_distances: bool,
        maximum_distances: u32,
        maximum_combined_distances: u32,
    ) -> Self {
        Self {
            required: behavior != RenderLineClippingBehavior::VulkanDriverV1
                && driver == vk::DriverId::MESA_LLVMPIPE.as_raw() as u32,
            shader_distances,
            maximum_distances,
            maximum_combined_distances,
        }
    }
    pub fn refusal(
        self,
        behavior: RenderLineClippingBehavior,
        device: crate::RenderDriverIdentity,
        reason: RenderLineClippingRequirement,
    ) -> crate::RenderLineClippingFailure {
        crate::RenderLineClippingFailure {
            behavior,
            device,
            reason,
            shader_clip_distance: self.shader_distances,
            max_clip_distances: self.maximum_distances,
            max_combined_clip_and_cull_distances: self.maximum_combined_distances,
        }
    }

    pub fn enabled(self) -> bool {
        self.required && self.admit(false).is_ok()
    }
    pub fn admit(self, registered_program: bool) -> Result<bool, RenderLineClippingRequirement> {
        if !self.required {
            return Ok(false);
        }
        if !self.shader_distances {
            return Err(RenderLineClippingRequirement::ShaderClipDistance);
        }
        if self.maximum_distances < 4 || self.maximum_combined_distances < 4 {
            return Err(RenderLineClippingRequirement::FourClipDistances);
        }
        if registered_program {
            return Err(RenderLineClippingRequirement::RegisteredLineProgram);
        }
        Ok(true)
    }
}

fn queried_clip_limits(instance: &Instance, physical: vk::PhysicalDevice) -> (bool, u32, u32) {
    // SAFETY: runtime enumerated this physical handle from this retained live instance.
    // Ash initializes and returns an owned feature structure before this borrow ends.
    let features = unsafe { instance.get_physical_device_features(physical) };
    // SAFETY: the same enumerated handle and live-instance lifetime apply; Ash owns
    // initialized output storage, and only copied limit values leave this function.
    let limits = unsafe { instance.get_physical_device_properties(physical) }.limits;
    (
        features.shader_clip_distance == vk::TRUE,
        limits.max_clip_distances,
        limits.max_combined_clip_and_cull_distances,
    )
}

#[cfg(test)]
#[path = "line_clip_device_tests.rs"]
mod tests;
