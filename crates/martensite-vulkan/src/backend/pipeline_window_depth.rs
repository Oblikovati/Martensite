use super::*;
impl DrawPipeline {
    pub(super) fn apply_window_depth_geometry(&mut self) -> Result<(), RenderError> {
        let Some(bytes) = super::super::window_depth_shader::geometry(self.kind) else {
            return Ok(());
        };
        if self.shaders.len() != 2 {
            return Err(crate::RenderWindowDepthFailure::UnsupportedInterface {
                actual: "existing geometry stage before window-depth reconstruction".into(),
            }
            .into());
        }
        if self
            .kind
            .window_depth
            .is_some_and(|parameters| parameters.fragment_depth())
        {
            self.window_depth_status_layout = Some(
                super::super::storage_descriptor::StorageDescriptorLayout::new(
                    &self.runtime,
                    if self.kind.window_depth.is_some_and(|p| p.clipped_fan()) {
                        vk::ShaderStageFlags::GEOMETRY | vk::ShaderStageFlags::FRAGMENT
                    } else {
                        vk::ShaderStageFlags::GEOMETRY
                    },
                )?,
            );
            let fragment = if self.kind.window_depth.is_some_and(|p| p.shared_incidence()) {
                super::super::clipped_depth_shader::incidence_fragment(self.fragment_bytes)?
            } else if self.kind.window_depth.is_some_and(|p| p.clipped_fan()) {
                super::super::clipped_depth_shader::fragment(self.fragment_bytes)?
            } else if self.kind.window_depth.is_some_and(|p| p.float64()) {
                super::super::window_depth_fragment::float64_fragment(self.fragment_bytes)?
            } else {
                super::super::window_depth_fragment::selected(self.fragment_bytes)?
            };
            self.replace_fragment(fragment)?;
        }
        self.add_shader(bytes)
    }
}
