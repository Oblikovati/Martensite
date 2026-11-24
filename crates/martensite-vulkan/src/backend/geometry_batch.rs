use super::super::{
    geometry_buffer::{GeometryBuffer, create_geometry_buffer, optional_geometry_refusal},
    geometry_residency::{GeometryStore, IndexFormat, ResidentGeometry},
    geometry_transfer::GeometryTransfers,
    host_upload_pages::HostUploadPool,
    pipeline_state::LIT_VERTEX_STRIDE,
    runtime::VulkanRuntime,
};
use crate::{PrimitiveDraw, RenderError};
use ash::vk;
use std::sync::{Arc, Mutex};

pub(crate) struct GeometryBatch {
    vertex_count: usize,
    index_count: u32,
    sources: Mutex<BatchSources>,
}

enum BatchSources {
    Pending {
        draws: Vec<PrimitiveDraw>,
        buffers: ResidentGeometry,
    },
    Resident(ResidentGeometry),
}

impl std::fmt::Debug for GeometryBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometryBatch")
            .field("vertices", &self.vertex_count)
            .field("indices", &self.index_count)
            .finish()
    }
}

impl GeometryBatch {
    pub(in crate::backend) fn prepare(
        draws: Vec<PrimitiveDraw>,
        runtime: &Arc<VulkanRuntime>,
        store: &GeometryStore,
    ) -> Result<Option<Self>, RenderError> {
        let Some((vertices, indices)) = draws.iter().try_fold((0u32, 0u32), batch_sizes) else {
            return Ok(None);
        };
        let Some(buffers) = batch_buffers(runtime, store, vertices, indices)? else {
            return Ok(None);
        };
        Ok(Some(Self {
            vertex_count: vertices as usize,
            index_count: indices,
            sources: Mutex::new(BatchSources::Pending { draws, buffers }),
        }))
    }

    pub(super) fn vertex_count(&self) -> usize {
        self.vertex_count
    }
    pub(super) fn index_count(&self) -> u32 {
        self.index_count
    }

    pub(super) fn resident(
        self: &Arc<Self>,
        runtime: &Arc<VulkanRuntime>,
        pool: &HostUploadPool,
        transfers: &mut GeometryTransfers,
    ) -> Result<ResidentGeometry, RenderError> {
        let source = self
            .sources
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (draws, buffers) = match &*source {
            BatchSources::Pending { draws, buffers } => (draws, buffers),
            BatchSources::Resident(resident) => return Ok(resident.clone()),
        };
        let (mut vertex_base, mut index_base) = (0u32, 0u64);
        let mut index_bytes = Vec::new();
        for draw in draws {
            let vertex_bytes = super::encode_vertices(draw);
            if vertex_bytes.len() != draw.vertices.len() * LIT_VERTEX_STRIDE as usize {
                return Err(crate::error::rejected(
                    vertex_bytes.len(),
                    "the packed Phong vertex stream size",
                ));
            }
            transfers.stage(
                runtime,
                pool,
                &buffers.vertices,
                u64::from(vertex_base) * u64::from(LIT_VERTEX_STRIDE),
                &vertex_bytes,
            )?;
            index_bytes.clear();
            index_bytes.extend(
                draw.indices
                    .iter()
                    .flat_map(|index| (index + vertex_base).to_ne_bytes()),
            );
            transfers.stage(runtime, pool, &buffers.indices, index_base, &index_bytes)?;
            vertex_base += draw.vertices.len() as u32;
            index_base += index_bytes.len() as u64;
        }
        let resident = buffers.clone();
        transfers
            .residency
            .queue_batch(Arc::clone(self), resident.clone());
        Ok(resident)
    }

    pub(in crate::backend) fn publish(&self, geometry: ResidentGeometry) {
        let mut sources = self
            .sources
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *sources = BatchSources::Resident(geometry);
    }
}

fn batch_buffers(
    runtime: &Arc<VulkanRuntime>,
    store: &GeometryStore,
    vertices: u32,
    indices: u32,
) -> Result<Option<ResidentGeometry>, RenderError> {
    let vertex_bytes = u64::from(vertices) * u64::from(LIT_VERTEX_STRIDE);
    let index_bytes = u64::from(indices) * 4;
    if vertices == 0 || indices == 0 || !store.can_pin_batch(vertex_bytes + index_bytes) {
        return Ok(None);
    }
    let (vertices, indices) = match allocate_pair(runtime, vertex_bytes, index_bytes) {
        Ok(pair) => pair,
        Err(error) if optional_geometry_refusal(&error) => return Ok(None),
        Err(error) => return Err(error),
    };
    let bytes = vertices
        .allocation_size()
        .saturating_add(indices.allocation_size());
    Ok(store.pin_batch(bytes).map(|charge| ResidentGeometry {
        index_format: IndexFormat::U32,
        mesh_decode: None,
        vertices: Arc::new(GeometryBuffer::charged(vertices, charge.clone())),
        indices: Arc::new(GeometryBuffer::charged(indices, charge)),
    }))
}

fn allocate_pair(
    runtime: &Arc<VulkanRuntime>,
    vertex_bytes: u64,
    index_bytes: u64,
) -> Result<
    (
        super::super::buffer::DeviceBuffer,
        super::super::buffer::DeviceBuffer,
    ),
    RenderError,
> {
    Ok((
        create_geometry_buffer(runtime, vk::BufferUsageFlags::VERTEX_BUFFER, vertex_bytes)?,
        create_geometry_buffer(runtime, vk::BufferUsageFlags::INDEX_BUFFER, index_bytes)?,
    ))
}

fn batch_sizes((vertices, indices): (u32, u32), draw: &PrimitiveDraw) -> Option<(u32, u32)> {
    Some((
        vertices.checked_add(u32::try_from(draw.vertices.len()).ok()?)?,
        indices.checked_add(u32::try_from(draw.indices.len()).ok()?)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RenderLighting, RenderMaterial, RenderShading, RenderVertex, identity_clip};

    #[test]
    fn abandoned_batch_upload_preserves_sources_for_retry() {
        let renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
        let draw = PrimitiveDraw::new(
            vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
            vec![0, 1, 2],
            identity_clip(),
        )
        .unwrap()
        .with_lighting(
            RenderLighting::new(
                vec![[0., 0., 1.]; 3],
                RenderMaterial::default(),
                identity_clip(),
                identity_clip(),
                false,
                RenderShading::Phong,
            )
            .unwrap(),
        )
        .unwrap();
        let pool = &renderer.pipeline.geometry_staging;
        let store = &renderer.pipeline.geometry_residency;
        let batch = Arc::new(
            GeometryBatch::prepare(vec![draw], &renderer.runtime, store)
                .unwrap()
                .unwrap(),
        );
        let mut abandoned = GeometryTransfers::new(store);
        let first = batch
            .resident(&renderer.runtime, pool, &mut abandoned)
            .unwrap();
        drop(abandoned);
        assert!(matches!(
            *batch.sources.lock().unwrap(),
            BatchSources::Pending { .. }
        ));

        let mut retry = GeometryTransfers::new(store);
        let submitted = batch.resident(&renderer.runtime, pool, &mut retry).unwrap();
        assert!(Arc::ptr_eq(&first.vertices, &submitted.vertices));
        assert!(matches!(
            *batch.sources.lock().unwrap(),
            BatchSources::Pending { .. }
        ));
        retry.residency.share();
        let mut later = GeometryTransfers::new(store);
        let reused = batch.resident(&renderer.runtime, pool, &mut later).unwrap();
        assert!(Arc::ptr_eq(&reused.vertices, &submitted.vertices));
        assert!(Arc::ptr_eq(&reused.indices, &submitted.indices));
    }
}
