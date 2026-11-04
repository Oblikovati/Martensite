//! One synchronous Ash boundary supports both actual calls and named CPU fakes.
use super::super::runtime::VulkanRuntime;
use ash::vk;

pub(super) trait ExecutableApi {
    fn properties(
        &mut self,
        pipeline: vk::Pipeline,
        rows: &mut [vk::PipelineExecutablePropertiesKHR<'_>],
    ) -> (vk::Result, u32);
    fn representations(
        &mut self,
        pipeline: vk::Pipeline,
        index: u32,
        rows: &mut [vk::PipelineExecutableInternalRepresentationKHR<'_>],
    ) -> (vk::Result, u32);
}
pub(super) struct BoundExecutableApi<'a> {
    device: &'a ash::Device,
    extension: ash::khr::pipeline_executable_properties::Device,
}
impl<'a> BoundExecutableApi<'a> {
    /// Borrow the actual device dispatch; `BoundExecutableApi::new(runtime)`.
    pub fn new(runtime: &'a VulkanRuntime) -> Self {
        Self {
            device: runtime.device(),
            extension: ash::khr::pipeline_executable_properties::Device::new(
                &runtime.instance,
                runtime.device(),
            ),
        }
    }
}
impl ExecutableApi for BoundExecutableApi<'_> {
    fn properties(
        &mut self,
        pipeline: vk::Pipeline,
        rows: &mut [vk::PipelineExecutablePropertiesKHR<'_>],
    ) -> (vk::Result, u32) {
        let info = vk::PipelineInfoKHR::default().pipeline(pipeline);
        let mut count = rows.len() as u32;
        // Every output has initialized sType; the owned slice outlives this synchronous call.
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let status = unsafe {
            (self.extension.fp().get_pipeline_executable_properties_khr)(
                self.device.handle(),
                &info,
                &mut count,
                rows.as_mut_ptr(),
            )
        };
        (status, count)
    }
    fn representations(
        &mut self,
        pipeline: vk::Pipeline,
        index: u32,
        rows: &mut [vk::PipelineExecutableInternalRepresentationKHR<'_>],
    ) -> (vk::Result, u32) {
        let info = executable_info(pipeline, index);
        let mut count = rows.len() as u32;
        let call = self
            .extension
            .fp()
            .get_pipeline_executable_internal_representations_khr;
        let device = self.device.handle();
        let target = rows.as_mut_ptr();
        // The owned descriptors and buffers stay fixed through this synchronous call.
        // SAFETY: the entry point was loaded for this device, and its arguments are live values of this scope.
        let status = unsafe { call(device, &info, &mut count, target) };
        (status, count)
    }
}

fn executable_info(pipeline: vk::Pipeline, index: u32) -> vk::PipelineExecutableInfoKHR<'static> {
    vk::PipelineExecutableInfoKHR::default()
        .pipeline(pipeline)
        .executable_index(index)
}
