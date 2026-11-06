//! Install admitted device handles and its explicitly selected extension commands.
use super::*;
impl VulkanRuntime {
    pub(super) fn install_selected_device(
        &mut self,
        physical: vk::PhysicalDevice,
        family: u32,
    ) -> Result<(), RenderError> {
        self.device = Some(create_device(self, physical)?);
        self.depth_bias_command = super::super::depth_bias_device::command(
            &self.instance,
            self.device(),
            self.depth_bias_behavior,
        )?;
        self.initialize_line_stipple();
        self.initialize_queue_handles(physical, family);
        #[cfg(feature = "command-state-diagnostics")]
        super::super::replay_device_diagnostic::observed(self);
        Ok(())
    }
    fn initialize_line_stipple(&mut self) {
        self.line_stipple = self
            .lines
            .extension
            .filter(|_| self.lines.stippled)
            .map(|name| {
                super::super::line_device::LineStippleDevice::new(
                    &self.instance,
                    self.device(),
                    name,
                )
            });
    }
    fn initialize_queue_handles(&mut self, physical: vk::PhysicalDevice, family: u32) {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let families = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(physical)
        };
        self.queues = (0..families.len())
            // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
            .map(|i| unsafe { self.device().get_device_queue(i as u32, 0) })
            .collect();
        self.queue = self.queues[family as usize];
        self.physical = physical;
        self.queue_family = family;
    }
}
