//! The upload a draw body made on an earlier frame (#594, #633).
//! Stored inside `DrawBody::upload_memo` (`OpaqueMemo`).
use super::geometry_buffer::GeometryBuffer;
use super::geometry_residency::IndexFormat;
use super::mesh_vertex_codec::MeshPositionDecode;
use super::pipeline::DrawPipeline;
use super::pipeline_memo::SelectionInputs;
use crate::OpaqueMemo;
use ash::vk;
use std::sync::{Arc, Weak};

/// Everything a kept upload depends on besides the body itself.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct UploadInputs {
    /// The runtime that uploaded, by address.
    pub runtime: usize,
    pub selection: SelectionInputs,
    pub compact_vertices: bool,
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
    pub index_format: IndexFormat,
    pub mesh_decode: Option<Arc<MeshPositionDecode>>,
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
    pub index_format: IndexFormat,
    pub mesh_decode: Option<Arc<MeshPositionDecode>>,
    pub count: u32,
    pub sample_alpha_cutoff: bool,
}

struct KeptUploadEntry {
    inputs: UploadInputs,
    upload: KeptUpload,
}

/// Extension methods storing Vulkan geometry upload memos inside [`OpaqueMemo`].
#[cfg_attr(
    any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ),
    allow(dead_code)
)]
pub(super) trait VulkanUploadMemoExt {
    fn live_vulkan(
        &self,
        inputs: &UploadInputs,
        current: impl FnOnce(&KeptUpload) -> bool,
    ) -> Option<LiveUpload>;
    fn keep_vulkan(&self, inputs: UploadInputs, upload: KeptUpload);
}

#[cfg_attr(
    any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics"
    ),
    allow(dead_code)
)]
impl VulkanUploadMemoExt for OpaqueMemo {
    fn live_vulkan(
        &self,
        inputs: &UploadInputs,
        current: impl FnOnce(&KeptUpload) -> bool,
    ) -> Option<LiveUpload> {
        self.with_ref::<KeptUploadEntry, Option<LiveUpload>>(|entry| {
            if entry.inputs != *inputs || !current(&entry.upload) {
                return None;
            }
            Some(LiveUpload {
                pipeline: Arc::clone(&entry.upload.pipeline),
                vertices: entry.upload.vertices.upgrade()?,
                indices: entry.upload.indices.upgrade()?,
                index_format: entry.upload.index_format,
                mesh_decode: entry.upload.mesh_decode.clone(),
                count: entry.upload.count,
                sample_alpha_cutoff: entry.upload.sample_alpha_cutoff,
            })
        })
        .flatten()
    }

    fn keep_vulkan(&self, inputs: UploadInputs, upload: KeptUpload) {
        self.store(KeptUploadEntry { inputs, upload });
    }
}
