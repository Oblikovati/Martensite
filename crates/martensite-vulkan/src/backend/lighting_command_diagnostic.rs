//! Temporary point-lighting input evidence; remove with command-state diagnostics.
//! Bytes are CPU upload inputs, never GPU readback, derivative values or shader identities.
use ash::{vk, vk::Handle};

pub(super) struct DescriptorArguments<'a> {
    set: u64,
    buffer: u64,
    bytes: &'a [u8],
}
impl<'a> DescriptorArguments<'a> {
    pub(super) fn from_storage(
        storage: &'a super::super::storage_descriptor::UploadedStorage,
    ) -> Self {
        let (buffer, bytes) = storage.diagnostic_storage();
        Self {
            set: storage.set.as_raw(),
            buffer: buffer.as_raw(),
            bytes,
        }
    }
    fn json(&self) -> String {
        format!(
            "{{\"set\":{},\"buffer\":{},\"binding\":0,\"byte_count\":{},\"uploaded_bytes\":{:?}}}",
            self.set,
            self.buffer,
            self.bytes.len(),
            self.bytes
        )
    }
}

pub(super) fn optional_json(descriptor: Option<&DescriptorArguments<'_>>) -> String {
    descriptor.map_or_else(|| "null".to_owned(), DescriptorArguments::json)
}

pub(super) fn binding_status(user: bool, present: bool) -> &'static str {
    if user {
        return "unknown-user-program";
    }
    if present {
        "pending-stock-bind"
    } else {
        "absent"
    }
}

pub(super) fn bound(
    location: (vk::CommandBuffer, vk::Pipeline, vk::PipelineLayout),
    set: vk::DescriptorSet,
) {
    eprintln!("{}", binding_json(location, set));
}

fn binding_json(
    (command, pipeline, layout): (vk::CommandBuffer, vk::Pipeline, vk::PipelineLayout),
    set: vk::DescriptorSet,
) -> String {
    format!(
        "{{\"kind\":\"draw-state-lighting-bound\",\"command\":{},\"pipeline\":{},\"pipeline_layout\":{},\"set_index\":0,\"set\":{}}}",
        command.as_raw(),
        pipeline.as_raw(),
        layout.as_raw(),
        set.as_raw()
    )
}

#[cfg(test)]
#[path = "lighting_command_diagnostic_tests.rs"]
mod tests;
