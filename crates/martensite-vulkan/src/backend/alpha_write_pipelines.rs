//! Pipelines that write the attachment alpha, for frames rendered with native
//! `GlCaps::buffersOpaqueAlpha` off (`GlContext::SetColorMask` lets every
//! draw write alpha then). Each ordinary pipeline gets one twin built from the same variant with
//! alpha writes, cached by the source pipeline; multisample, alpha-cutoff and OIT variants derive
//! from the twin as they do from the source.
use super::{pipeline::DrawPipeline, runtime::VulkanRuntime};
use crate::{RenderError, error::rejected};
use ash::vk::Handle;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Alpha-writing twins keyed by the raw handle of their source pipeline.
#[derive(Default)]
pub(super) struct AlphaWritePipelines(Mutex<BTreeMap<u64, Arc<DrawPipeline>>>);

impl AlphaWritePipelines {
    /// The twin of `source` that writes alpha; `cache.twin(&runtime, &pipeline)?`.
    pub fn twin(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<DrawPipeline>,
    ) -> Result<Arc<DrawPipeline>, RenderError> {
        if source.kind.alpha_writes {
            return Ok(source.clone());
        }
        let key = source.handle.as_raw();
        let mut twins = self
            .0
            .lock()
            .map_err(|_| rejected("poisoned alpha-write pipeline cache", "an available cache"))?;
        if let Some(twin) = twins.get(&key) {
            return Ok(twin.clone());
        }
        let mut kind = source.kind;
        kind.alpha_writes = true;
        let twin = Arc::new(DrawPipeline::new(runtime, kind)?);
        twins.insert(key, twin.clone());
        Ok(twin)
    }
}
