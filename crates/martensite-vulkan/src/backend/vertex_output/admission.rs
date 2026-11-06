use super::super::{portability_admission::advertises, runtime::vk_error};
use super::*;
use crate::VertexOutputRequest;
use ash::{Instance, vk};

pub(in crate::backend) fn admit(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    request: &VertexOutputRequest,
) -> Result<(), RenderError> {
    let extensions = unsafe { instance.enumerate_device_extension_properties(physical) }
        .map_err(|e| vk_error("enumerate transform-feedback extensions", e))?;
    for name in [
        ash::ext::transform_feedback::NAME,
        ash::ext::provoking_vertex::NAME,
    ] {
        if !advertises(&extensions, name) {
            return Err(unsupported("required TF extension", name));
        }
    }
    let mut tf = vk::PhysicalDeviceTransformFeedbackFeaturesEXT::default();
    let mut provoking = vk::PhysicalDeviceProvokingVertexFeaturesEXT::default();
    let mut query = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut tf)
        .push_next(&mut provoking);
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    require_features(
        tf.transform_feedback,
        provoking.transform_feedback_preserves_provoking_vertex,
    )?;
    properties(instance, physical, request)
}
fn properties(
    instance: &Instance,
    physical: vk::PhysicalDevice,
    request: &VertexOutputRequest,
) -> Result<(), RenderError> {
    let mut tf = vk::PhysicalDeviceTransformFeedbackPropertiesEXT::default();
    let mut query = vk::PhysicalDeviceProperties2::default().push_next(&mut tf);
    unsafe { instance.get_physical_device_properties2(physical, &mut query) };
    require_properties(
        &tf,
        request.capacity_bytes as u64,
        request.format().stride() as u32,
    )
}
fn require_features(feedback: vk::Bool32, preserves: vk::Bool32) -> Result<(), RenderError> {
    if feedback == vk::TRUE && preserves == vk::TRUE {
        return Ok(());
    }
    Err(unsupported(
        "TF preserves original provoking behavior",
        (feedback, preserves),
    ))
}
fn require_properties(
    tf: &vk::PhysicalDeviceTransformFeedbackPropertiesEXT<'_>,
    capacity: u64,
    stride: u32,
) -> Result<(), RenderError> {
    let valid = tf.transform_feedback_queries == vk::TRUE
        && tf.max_transform_feedback_buffers >= 1
        && tf.max_transform_feedback_streams >= 1
        && tf.max_transform_feedback_buffer_data_size >= stride
        && tf.max_transform_feedback_buffer_data_stride >= stride
        && tf.max_transform_feedback_stream_data_size >= stride
        && tf.max_transform_feedback_buffer_size >= capacity;
    if !valid {
        return Err(unsupported("TF query/count/record stride", tf));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertex_output_requires_preservation_and_feedback_together() {
        for (feedback, preserves) in [(0, 0), (0, 1), (1, 0)] {
            assert!(require_features(feedback, preserves).is_err());
        }
        assert!(require_features(1, 1).is_ok());
    }
    #[test]
    fn vertex_output_requires_bounded_queryable_32_byte_stream() {
        let supported = vk::PhysicalDeviceTransformFeedbackPropertiesEXT::default()
            .transform_feedback_queries(true)
            .max_transform_feedback_buffers(1)
            .max_transform_feedback_streams(1)
            .max_transform_feedback_buffer_data_size(32)
            .max_transform_feedback_buffer_data_stride(32)
            .max_transform_feedback_stream_data_size(32)
            .max_transform_feedback_buffer_size(96);
        assert!(require_properties(&supported, 96, 32).is_ok());
        assert!(require_properties(&supported, 97, 32).is_err());
        for unsupported in [
            supported.transform_feedback_queries(false),
            supported.max_transform_feedback_buffers(0),
            supported.max_transform_feedback_streams(0),
            supported.max_transform_feedback_buffer_data_size(31),
            supported.max_transform_feedback_buffer_data_stride(31),
            supported.max_transform_feedback_stream_data_size(31),
        ] {
            assert!(require_properties(&unsupported, 96, 32).is_err());
        }
        assert!(require_properties(&supported, 84, 28).is_ok());
        assert!(
            require_properties(
                &supported.max_transform_feedback_buffer_data_size(27),
                84,
                28,
            )
            .is_err()
        );
    }
}
