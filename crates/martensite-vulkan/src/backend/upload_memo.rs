//! The upload a draw body made on an earlier frame (#594, retained render list step 2). A
//! recorded body uploads with the same inputs on every frame; it keeps the pipeline, a weak
//! hold on its resident geometry and its count, so a later frame skips the admission checks,
//! the selection and the geometry lookup. The view-level bindings (lighting, PBR environment,
//! shadow maps) follow the view and are bound again on every frame.
use super::geometry_buffer::GeometryBuffer;
use super::pipeline::DrawPipeline;
use super::pipeline_memo::SelectionInputs;
use ash::vk;
use std::sync::{Arc, Mutex, Weak};

/// Everything a kept upload depends on besides the body itself.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct UploadInputs {
    /// The runtime that uploaded, by address.
    pub runtime: usize,
    pub selection: SelectionInputs,
    pub extent: vk::Extent2D,
    pub samples: vk::SampleCountFlags,
}

/// The frame-independent part of an `UploadedDraw` of a plain surface.
#[cfg_attr(
    any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ),
    allow(dead_code)
)]
#[derive(Clone)]
pub(super) struct KeptUpload {
    pub pipeline: Arc<DrawPipeline>,
    /// Weak, so that a body the scene no longer draws holds no device memory: the geometry
    /// store's lease decides how long the buffers stay resident.
    pub vertices: Weak<GeometryBuffer>,
    pub indices: Weak<GeometryBuffer>,
    pub count: u32,
    pub sample_alpha_cutoff: bool,
    /// The geometry store, pinned by the weak reference, and its eviction count.
    pub store: Weak<super::geometry_residency::GeometryStore>,
    pub evictions: u64,
}

/// A kept upload whose geometry is still alive, ready for this frame.
#[cfg_attr(
    any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ),
    allow(dead_code)
)]
pub(super) struct LiveUpload {
    pub pipeline: Arc<DrawPipeline>,
    pub vertices: Arc<GeometryBuffer>,
    pub indices: Arc<GeometryBuffer>,
    pub count: u32,
    pub sample_alpha_cutoff: bool,
}

/// A body's last kept upload. A copied body starts empty, and two memos always compare
/// equal, so the memo never changes what a draw is.
#[derive(Default)]
/// Boxed, so that a body without a kept upload grows by two words only.
pub(crate) struct UploadMemo(Mutex<Option<Box<(UploadInputs, KeptUpload)>>>);

#[cfg_attr(
    any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ),
    allow(dead_code)
)]
impl UploadMemo {
    /// The upload kept with `inputs` while `current` accepts it, with its geometry upgraded
    /// under the lock: cloning the weak references first cost three more atomic pairs per
    /// draw (#594); `memo.live(&inputs, |kept| store_unchanged(kept))`.
    pub(super) fn live(
        &self,
        inputs: &UploadInputs,
        current: impl FnOnce(&KeptUpload) -> bool,
    ) -> Option<LiveUpload> {
        let slot = self.0.lock().ok()?;
        let (kept_inputs, upload) = slot.as_deref()?;
        if kept_inputs != inputs || !current(upload) {
            return None;
        }
        Some(LiveUpload {
            pipeline: Arc::clone(&upload.pipeline),
            vertices: upload.vertices.upgrade()?,
            indices: upload.indices.upgrade()?,
            count: upload.count,
            sample_alpha_cutoff: upload.sample_alpha_cutoff,
        })
    }
    /// Keep an upload made with `inputs`; `memo.keep(inputs, upload)`.
    pub(super) fn keep(&self, inputs: UploadInputs, upload: KeptUpload) {
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some(Box::new((inputs, upload)));
        }
    }
}

impl UploadMemo {
    /// Whether an upload is kept, for tests; `memo.is_kept()`.
    #[cfg(test)]
    pub(crate) fn is_kept(&self) -> bool {
        self.0.lock().is_ok_and(|slot| slot.is_some())
    }
    /// Forget the upload: the body is about to change in place.
    pub(crate) fn clear(&mut self) {
        if let Ok(slot) = self.0.get_mut() {
            *slot = None;
        }
    }
}

impl Clone for UploadMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl PartialEq for UploadMemo {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl std::fmt::Debug for UploadMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("UploadMemo")
    }
}
