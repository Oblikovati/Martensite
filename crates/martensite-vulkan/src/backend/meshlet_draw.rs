//! Ordered, capability-gated meshlet draws with a bounded renderer-owned geometry cache.
use super::{
    directional_phong, frame::UploadedDraw, meshlet_pipeline::MeshletPipeline, meshlet_words,
    pipeline_variants::RasterPipelines, runtime::VulkanRuntime,
    storage_descriptor::UploadedStorage,
};
use crate::{PrimitiveDraw, RenderError, RenderFrame, RenderVertex};
use ash::vk;
use std::{
    collections::VecDeque,
    num::NonZeroUsize,
    sync::{Arc, Mutex, Weak},
};

const MAX_CACHE_BYTES: usize = 384 << 20;
const MIN_INDICES: usize = 512 * 3;
const MAX_CACHED_MESHES: usize = 256;

type MeshletKey = (vk::Pipeline, [NonZeroUsize; 3]);
type FrameEntry = Option<MeshletDraw>;
pub(super) type FrameMeshlets = Vec<FrameEntry>;

#[derive(Default)]
pub(super) struct MeshletCache(Mutex<VecDeque<CachedMeshlet>>);

struct CachedMeshlet {
    key: MeshletKey,
    vertices: Weak<Vec<RenderVertex>>,
    indices: Weak<Vec<u32>>,
    normals: Weak<Vec<[f32; 3]>>,
    bytes: usize,
    geometry: Arc<MeshletAsset>,
}

struct MeshletAsset {
    pipeline: Arc<MeshletPipeline>,
    storage: UploadedStorage,
    groups: u32,
}

pub(super) struct MeshletDraw {
    asset: Arc<MeshletAsset>,
    lighting: UploadedStorage,
    cone_eye: [f32; 4],
}

pub(super) fn prepare(
    runtime: &Arc<VulkanRuntime>,
    frame: &RenderFrame,
    pipelines: &RasterPipelines,
    samples: vk::SampleCountFlags,
    uploaded: &[UploadedDraw],
    first: usize,
) -> Result<FrameMeshlets, RenderError> {
    if !frame_eligible(runtime, frame, pipelines, samples) {
        return Ok(Vec::new());
    }
    frame.draws[first..]
        .iter()
        .zip(uploaded)
        .map(|(source, draw)| {
            runtime
                .lifecycle
                .observe(prepare_draw(runtime, pipelines, source, draw))
        })
        .map(|result| match result {
            Err(error) if !super::device_lifecycle::contains_device_loss(&error) => Ok(None),
            other => other,
        })
        .collect()
}

fn frame_eligible(
    runtime: &VulkanRuntime,
    frame: &RenderFrame,
    pipelines: &RasterPipelines,
    samples: vk::SampleCountFlags,
) -> bool {
    runtime.meshlet_group_limit > 0
        && samples == vk::SampleCountFlags::TYPE_1
        && directional_phong::supported_behavior(runtime)
        && !pipelines.stencil
        && !frame.raster.alpha_writes
        && frame.shadow_maps.is_none()
        && frame.raytrace.is_none()
        && frame.geometry_batch.is_none()
        // The packed meshlet storage is independent of the indexed upload budget.
        && frame.assembly_upload_budget.is_none()
        && frame.immediate_main.is_none()
        && !runtime.raster_policy.requires_last_vertex()
}

fn prepare_draw(
    runtime: &Arc<VulkanRuntime>,
    pipelines: &RasterPipelines,
    source: &PrimitiveDraw,
    draw: &UploadedDraw,
) -> Result<FrameEntry, RenderError> {
    let Some(light) = source.lighting.as_ref().filter(|light| {
        light.shading == crate::RenderShading::Phong && !light.pbr && light.fixed.is_none()
    }) else {
        return Ok(None);
    };
    if !eligible(source, draw) {
        return Ok(None);
    }
    let Some(asset) = pipelines.meshlets.load(runtime, source, draw)? else {
        return Ok(None);
    };
    let mut bytes = Vec::new();
    light.encode_into(&mut bytes);
    let lighting = UploadedStorage::new(runtime, &asset.pipeline.lighting, &bytes)?;
    Ok(Some(MeshletDraw {
        asset,
        lighting,
        cone_eye: cone_eye(source),
    }))
}

fn eligible(source: &PrimitiveDraw, draw: &UploadedDraw) -> bool {
    source.indices.len() >= MIN_INDICES
        && directional_phong::ordinary_surface(source)
        && !draw.borrowed_lighting
        && draw.table_run.is_none()
        && draw.oit.is_none()
        && draw.native_alpha.is_none()
        && draw.mesh_decode.is_none()
        && draw.pipeline.meshlet_fragment_module().is_some()
        && source.instances.is_none()
}

impl MeshletCache {
    fn load(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &PrimitiveDraw,
        draw: &UploadedDraw,
    ) -> Result<Option<Arc<MeshletAsset>>, RenderError> {
        let Some(key) = key(source, draw) else {
            return Ok(None);
        };
        let mut cache = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = cache.iter().position(|entry| entry.key == key)
            && let Some(entry) = cache.remove(index)
        {
            let asset = entry.geometry.clone();
            cache.push_back(entry);
            return Ok(Some(asset));
        }
        cache.retain(CachedMeshlet::live);
        let pipeline = cache
            .iter()
            .find(|entry| entry.key.0 == key.0)
            .map(|entry| entry.geometry.pipeline.clone());
        let Some(asset) = CachedMeshlet::upload(runtime, source, draw, key, pipeline)? else {
            return Ok(None);
        };
        let geometry = asset.geometry.clone();
        make_room(&mut cache, asset.bytes);
        cache.push_back(asset);
        Ok(Some(geometry))
    }
}

impl CachedMeshlet {
    fn live(&self) -> bool {
        self.vertices.strong_count() != 0
            && self.indices.strong_count() != 0
            && self.normals.strong_count() != 0
    }

    fn upload(
        runtime: &Arc<VulkanRuntime>,
        source: &PrimitiveDraw,
        draw: &UploadedDraw,
        key: MeshletKey,
        pipeline: Option<Arc<MeshletPipeline>>,
    ) -> Result<Option<Self>, RenderError> {
        let Some((vertices, indices, normals)) = witnesses(source) else {
            return Ok(None);
        };
        let Some((groups, bytes)) = meshlet_words::encode(source, storage_limit(runtime)) else {
            return Ok(None);
        };
        let pipeline = match pipeline {
            Some(pipeline) => pipeline,
            None => Arc::new(MeshletPipeline::new(runtime, draw.pipeline.clone())?),
        };
        let storage = UploadedStorage::new(runtime, &pipeline.geometry, &bytes)?;
        let geometry = Arc::new(MeshletAsset {
            pipeline,
            storage,
            groups,
        });
        Ok(Some(Self {
            key,
            vertices,
            indices,
            normals,
            bytes: bytes.len(),
            geometry,
        }))
    }
}

fn storage_limit(runtime: &VulkanRuntime) -> usize {
    // SAFETY: the physical device belongs to this live instance.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_properties(runtime.physical)
    };
    MAX_CACHE_BYTES.min(properties.limits.max_storage_buffer_range as usize)
}

fn key(source: &PrimitiveDraw, draw: &UploadedDraw) -> Option<MeshletKey> {
    Some((
        draw.pipeline.handle,
        [
            source.vertices.identity()?,
            source.indices.identity()?,
            source.lighting.as_ref()?.normals.identity()?,
        ],
    ))
}

type MeshletWitnesses = (Weak<Vec<RenderVertex>>, Weak<Vec<u32>>, Weak<Vec<[f32; 3]>>);

fn witnesses(source: &PrimitiveDraw) -> Option<MeshletWitnesses> {
    Some((
        source.vertices.witness()?,
        source.indices.witness()?,
        source.lighting.as_ref()?.normals.witness()?,
    ))
}

fn make_room(cache: &mut VecDeque<CachedMeshlet>, bytes: usize) {
    let mut total: usize = cache.iter().map(|entry| entry.bytes).sum();
    while total + bytes > MAX_CACHE_BYTES || cache.len() >= MAX_CACHED_MESHES {
        let Some(expired) = cache.pop_front() else {
            return;
        };
        total -= expired.bytes;
    }
}

fn cone_eye(source: &PrimitiveDraw) -> [f32; 4] {
    let m = source.clip_transform;
    if source.face_culling != crate::RenderFaceCulling::Back
        || source.mirrored
        || [m[3], m[7], m[11]] != [0.; 3]
        || m[15] <= 0.
    {
        return [0.; 4];
    }
    // The projected winding sign is the dot product with the cross of the two clip rows.
    // Lighting eye direction is independent of these rows in the public draw API.
    let vec = martensite_tkernel::ncollection::Vec3::new;
    let forward = martensite_tkernel::ncollection::Vec3::cross(
        &vec(m[0] as f64, m[4] as f64, m[8] as f64),
        &vec(m[1] as f64, m[5] as f64, m[9] as f64),
    );
    let length = forward.dot(&forward).sqrt();
    if length <= 0. || !length.is_finite() {
        return [0.; 4];
    }
    let forward = forward / length;
    [
        forward.x() as f32,
        forward.y() as f32,
        forward.z() as f32,
        1.,
    ]
}

impl MeshletDraw {
    pub(super) fn record(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        draw: &UploadedDraw,
        extent: vk::Extent2D,
    ) {
        self.bind(runtime, command, draw);
        let mut first = 0;
        while first < self.asset.groups {
            let count = (self.asset.groups - first).min(runtime.meshlet_group_limit);
            let constants = self.constants(runtime, draw, first, extent);
            let program = &self.asset.pipeline;
            // SAFETY: command is inside its rendering scope and this frame retains its mesh resources.
            unsafe {
                runtime.device().cmd_push_constants(
                    command,
                    program.layout,
                    vk::ShaderStageFlags::MESH_EXT,
                    0,
                    &constants,
                );
                program.command.cmd_draw_mesh_tasks(command, count, 1, 1);
            }
            first += count;
        }
    }

    fn constants(
        &self,
        runtime: &VulkanRuntime,
        draw: &UploadedDraw,
        first: u32,
        extent: vk::Extent2D,
    ) -> [u8; super::meshlet_pipeline::PUSH_BYTES as usize] {
        let mut words = [0; super::meshlet_pipeline::PUSH_BYTES as usize];
        words[..64].copy_from_slice(&draw.transform[..64]);
        words[64..80].copy_from_slice(self.cone_eye.map(f32::to_ne_bytes).as_flattened());
        let control = [
            first,
            u32::from(!runtime.depth.clip_control),
            u32::from(!draw.pipeline.kind.clamp),
            extent.width,
            extent.height,
        ];
        words[80..].copy_from_slice(control.map(u32::to_ne_bytes).as_flattened());
        words
    }

    fn bind(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer, draw: &UploadedDraw) {
        let program = &self.asset.pipeline;
        let device = runtime.device();
        let graphics = vk::PipelineBindPoint::GRAPHICS;
        // SAFETY: the ordered draw owns its pipeline, geometry, lighting and optional mock shadows.
        unsafe {
            device.cmd_bind_pipeline(command, graphics, program.handle);
            device.cmd_bind_descriptor_sets(
                command,
                graphics,
                program.layout,
                0,
                &[self.lighting.set],
                &[],
            );
            device.cmd_bind_descriptor_sets(
                command,
                graphics,
                program.layout,
                7,
                &[self.asset.storage.set],
                &[],
            );
            if let Some(shadows) = &draw.bindings.shadow_maps {
                device.cmd_bind_descriptor_sets(
                    command,
                    graphics,
                    program.layout,
                    6,
                    &[shadows.set],
                    &[],
                );
            }
        }
    }
}

#[cfg(all(
    test,
    not(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "lit-only-command-diagnostics"
    ))
))]
#[path = "meshlet_draw_tests.rs"]
mod tests;
