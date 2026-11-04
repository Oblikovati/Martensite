//! Only commands that recorded transform feedback need its host visibility dependency.
use super::*;
trait FeedbackBarrierSink {
    fn record(&mut self, barrier: vk::MemoryBarrier2<'static>);
}
struct VulkanFeedbackBarrier<'a> {
    runtime: &'a VulkanRuntime,
    command: vk::CommandBuffer,
}
impl FeedbackBarrierSink for VulkanFeedbackBarrier<'_> {
    fn record(&mut self, barrier: vk::MemoryBarrier2<'static>) {
        unsafe {
            self.runtime.device().cmd_pipeline_barrier2(
                self.command,
                &vk::DependencyInfo::default().memory_barriers(&[barrier]),
            );
        }
    }
}
pub(in crate::backend) fn host_barrier(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    frame: &FrameResources,
) {
    let mut sink = VulkanFeedbackBarrier { runtime, command };
    let invocations = frame
        .draws
        .iter()
        .filter_map(|draw| draw.vertex_output.as_ref())
        .map(|capture| capture.recorded_invocations());
    record_feedback_barrier(&mut sink, invocations);
}
fn record_feedback_barrier(
    sink: &mut impl FeedbackBarrierSink,
    mut invocations: impl Iterator<Item = usize>,
) {
    if !invocations.any(|count| count > 0) {
        return;
    }
    sink.record(
        vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFORM_FEEDBACK_EXT)
            .src_access_mask(vk::AccessFlags2::TRANSFORM_FEEDBACK_WRITE_EXT)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ),
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct RecordedFeedbackBarriers(Vec<vk::MemoryBarrier2<'static>>);
    impl FeedbackBarrierSink for RecordedFeedbackBarriers {
        fn record(&mut self, barrier: vk::MemoryBarrier2<'static>) {
            self.0.push(barrier);
        }
    }
    #[test]
    fn vertex_output_absent_and_unselected_frames_record_no_tf_dependency() {
        for invocations in [vec![], vec![0], vec![0, 0]] {
            let mut sink = RecordedFeedbackBarriers::default();
            record_feedback_barrier(&mut sink, invocations.into_iter());
            assert!(
                sink.0.is_empty(),
                "no recorded TF draw must emit no TF stage/access bits"
            );
        }
    }
    #[test]
    fn vertex_output_recorded_dispatch_has_one_tf_to_host_dependency() {
        let mut sink = RecordedFeedbackBarriers::default();
        record_feedback_barrier(&mut sink, [0, 1, 2].into_iter());
        assert_eq!(sink.0.len(), 1);
        assert_eq!(
            sink.0[0].src_stage_mask,
            vk::PipelineStageFlags2::TRANSFORM_FEEDBACK_EXT
        );
        assert_eq!(
            sink.0[0].src_access_mask,
            vk::AccessFlags2::TRANSFORM_FEEDBACK_WRITE_EXT
        );
        assert_eq!(sink.0[0].dst_stage_mask, vk::PipelineStageFlags2::HOST);
        assert_eq!(sink.0[0].dst_access_mask, vk::AccessFlags2::HOST_READ);
    }
}
