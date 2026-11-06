//! Read actual coherent upload bytes after completion; no desired fixture values are substituted.
use super::*;
use crate::user_program::UserStage;
use std::path::Path;

#[derive(Debug)]
pub(super) struct OwnedResourceCapture {
    binding: u32,
    target_set: u64,
    buffer: u64,
    descriptor_type: i32,
    offset: u64,
    range: u64,
}
impl OwnedResourceCapture {
    pub(super) fn new(
        write: &vk::WriteDescriptorSet<'_>,
        buffer: &vk::DescriptorBufferInfo,
    ) -> Self {
        use ash::vk::Handle;
        Self {
            binding: write.dst_binding,
            target_set: write.dst_set.as_raw(),
            buffer: buffer.buffer.as_raw(),
            descriptor_type: write.descriptor_type.as_raw(),
            offset: buffer.offset,
            range: buffer.range,
        }
    }
    fn json(&self, uploaded: &UploadedUserProgram) -> String {
        let Self {
            binding,
            target_set,
            buffer,
            descriptor_type,
            offset,
            range,
        } = self;
        let sampler_count = uploaded.pipeline.samplers.len();
        let index_count = uploaded.count;
        format!(
            "{{\"observation_kind\":\"descriptor-update-inputs\",\"set\":1,\"target_set_handle\":{target_set},\"uniform_buffer_handle\":{buffer},\"binding\":{binding},\"descriptor_type\":{descriptor_type},\"offset\":{offset},\"range\":{range},\"sampler_count\":{sampler_count},\"index_count\":{index_count},\"stride\":64}}"
        )
    }
}

impl UploadedUserProgram {
    pub(crate) fn owned_write_uploads(
        &self,
        directory: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir(directory)?;
        let vertices = self.vertices.read(3 * USER_CAPTURE_STRIDE)?;
        let indices = self.indices.read(self.count as usize * 4)?;
        let uniforms = self._uniforms.read(self.block.len())?;
        std::fs::write(directory.join("vertices.bin"), vertices)?;
        std::fs::write(directory.join("indices.bin"), indices)?;
        std::fs::write(directory.join("uniforms.bin"), uniforms)?;
        self.owned_write_modules(directory)?;
        self.owned_write_layout(directory)?;
        Ok(())
    }
    fn owned_write_modules(&self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        write_stages(directory, &self.modules)?;
        let capture = self
            .pipeline
            .owned_capture
            .as_ref()
            .ok_or("missing creation snapshot")?;
        std::fs::write(
            directory.join("vertex.specialization.bin"),
            capture.specialization,
        )?;
        std::fs::write(directory.join("pipeline.json"), &capture.json)?;
        Ok(())
    }
    fn owned_write_layout(&self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let (set, binding, members) = uniform_members(&self.modules)?;
        let rows: Vec<String> = members.iter().map(owned_member_row).collect();
        std::fs::write(
            directory.join("uniforms.layout.tsv"),
            rows.join("\n") + "\n",
        )?;
        let resource = self
            .owned_resources
            .as_ref()
            .ok_or("missing actual descriptor write")?;
        validate_binding((*set, *binding), resource.binding)?;
        std::fs::write(directory.join("resources.json"), resource.json(self))?;
        Ok(())
    }
}
fn validate_binding(reflected: (u32, u32), binding: u32) -> Result<(), Box<dyn std::error::Error>> {
    if reflected != (1, binding) {
        return Err(format!("reflected={reflected:?}; expected uploaded UBO (1,{binding})").into());
    }
    Ok(())
}
fn write_stages(
    directory: &Path,
    modules: &crate::user_program::UserProgramModules,
) -> Result<(), Box<dyn std::error::Error>> {
    for (reflection, words) in &modules.stages {
        let name = match reflection.stage {
            UserStage::Vertex => "actual.vert.spv",
            UserStage::Fragment => "actual.frag.spv",
            stage => return Err(format!("stage={stage:?}; expected known vertex+fragment").into()),
        };
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        std::fs::write(directory.join(name), bytes)?;
    }
    Ok(())
}
fn uniform_members(
    modules: &crate::user_program::UserProgramModules,
) -> Result<&(u32, u32, Vec<crate::user_program::UniformMember>), Box<dyn std::error::Error>> {
    let reflection = modules
        .stage(UserStage::Vertex)
        .ok_or("missing actual vertex reflection")?;
    let block = reflection
        .uniform_block
        .as_ref()
        .ok_or("missing actual uniform block")?;
    Ok(block)
}
const USER_CAPTURE_STRIDE: usize = 64;
fn owned_member_row(member: &crate::user_program::UniformMember) -> String {
    let (count, stride) = member.array.unwrap_or((1, 0));
    format!(
        "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}",
        member.name,
        member.offset,
        member.shape.scalar,
        member.shape.rows,
        member.shape.columns,
        member.shape.matrix_stride,
        count,
        stride
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descriptor_capture_reads_actual_write_and_buffer_range() {
        let write = vk::WriteDescriptorSet::default()
            .dst_binding(9)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER);
        let buffer = vk::DescriptorBufferInfo::default().offset(32).range(1920);
        let captured = OwnedResourceCapture::new(&write, &buffer);
        assert_eq!(
            (captured.binding, captured.offset, captured.range),
            (9, 32, 1920)
        );
        assert_eq!(
            captured.descriptor_type,
            vk::DescriptorType::UNIFORM_BUFFER.as_raw()
        );
        assert!(validate_binding((1, 9), 9).is_ok());
        assert!(validate_binding((0, 9), 9).is_err());
    }
    #[test]
    fn capture_stride_matches_the_production_upload() {
        assert_eq!(
            USER_CAPTURE_STRIDE,
            super::super::super::user_program_pipeline::USER_VERTEX_STRIDE as usize
        );
        assert_eq!(3 * USER_CAPTURE_STRIDE, 192);
    }
}
