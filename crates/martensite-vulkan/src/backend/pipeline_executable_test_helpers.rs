#[cfg(test)]
impl DrawPipeline {
    /// Borrow the exact creation inputs; `pipeline.executable_modules()?` never rebuilds a shader.
    pub(super) fn executable_modules(&self) -> Result<Vec<ExecutableModuleInput>, ExecutableGap> {
        use ash::vk::Handle;
        if self.runtime.executable_profile.is_none() {
            return Ok(Vec::new());
        }
        if self.shaders.len() != self.executable_bytes.len() {
            return Err(super::diagnostic_executable::module_count_gap(
                &self.shaders,
                &self.executable_bytes,
            ));
        }
        Ok(shader_stages(&self.shaders)
            .iter()
            .zip(&self.executable_bytes)
            .map(|(stage, bytes)| (stage.stage.as_raw(), stage.module.as_raw(), *bytes))
            .collect())
    }
}
