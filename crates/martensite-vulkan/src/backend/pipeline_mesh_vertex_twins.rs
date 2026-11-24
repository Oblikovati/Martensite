//! Match each packed mesh draw to the same raster state as its ordinary pipeline.
use super::{DrawPipeline, VulkanRuntime};
use crate::{RenderError, error::rejected};
use ash::{vk, vk::Handle};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub(in crate::backend) struct MeshVertexTwins(Mutex<BTreeMap<u64, Arc<DrawPipeline>>>);

impl MeshVertexTwins {
    pub(in crate::backend) fn get(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<DrawPipeline>,
    ) -> Result<Arc<DrawPipeline>, RenderError> {
        let mut twins = self.0.lock().map_err(|error| {
            rejected(
                error.to_string(),
                "an unpoisoned packed mesh pipeline cache",
            )
        })?;
        let key = source.handle.as_raw();
        if let Some(twin) = twins.get(&key) {
            return Ok(twin.clone());
        }
        let twin = Arc::new(source.mesh_vertex_twin(runtime)?);
        twins.insert(key, twin.clone());
        Ok(twin)
    }
}

impl DrawPipeline {
    fn mesh_vertex_twin(&self, runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let directional = self.vertex_bytes == super::super::directional_phong::SHADERS[0];
        if !directional && !self.has_plain_lit_stages() {
            return Err(rejected(
                self.kind,
                "a stock programmable lighting pipeline for packed mesh vertices",
            ));
        }
        let mut kind = self.kind;
        kind.compact_vertices = true;
        Self::build(
            runtime,
            kind,
            None,
            vk::SampleCountFlags::TYPE_1,
            false,
            None,
            directional,
        )
    }
}
