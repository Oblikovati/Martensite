//! Resident streams become visible only after their frame's transfer completes.
use super::geometry_buffer::{GeometryBuffer, GeometryBufferPool};
use super::runtime::VulkanRuntime;
use crate::{PrimitiveDraw, PrimitiveTopology, RenderVertex};
use ash::vk;
use std::collections::{HashMap, VecDeque, hash_map::Entry};
use std::num::NonZeroUsize;
use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone)]
pub(super) struct ResidentGeometry {
    pub vertices: Arc<GeometryBuffer>,
    pub indices: Arc<GeometryBuffer>,
}

impl ResidentGeometry {
    fn bytes(&self) -> u64 {
        self.vertices
            .allocation_size()
            .saturating_add(self.indices.allocation_size())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct GeometryKey {
    streams: [Option<NonZeroUsize>; 8],
    encoding: GeometryEncoding,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct GeometryEncoding {
    pub(super) surface_coordinates: Option<[u32; 4]>,
    pub(super) surface_uv: bool,
    pub(super) sphere_normals: bool,
    pub(super) text_uv: bool,
    pub(super) font_replace: bool,
    pub(super) flat_text: bool,
    pub(super) topology: GeometryTopology,
    pub(super) has_lighting: bool,
    pub(super) fixed_lighting: bool,
    pub(super) provoking_vertices: bool,
    pub(super) fixed_grid_points: bool,
    pub(super) xor_color: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub(super) enum GeometryTopology {
    Triangles,
    Points,
    Lines,
    CompactLines,
}

impl From<PrimitiveTopology> for GeometryTopology {
    fn from(topology: PrimitiveTopology) -> Self {
        match topology {
            PrimitiveTopology::Triangles => Self::Triangles,
            PrimitiveTopology::Points => Self::Points,
            PrimitiveTopology::Lines => Self::Lines,
        }
    }
}

impl GeometryKey {
    pub(super) fn new(draw: &PrimitiveDraw, encoding: GeometryEncoding) -> Self {
        let normals = draw
            .lighting
            .as_ref()
            .and_then(|lighting| lighting.normals.identity());
        let uv = surface_uv(draw, encoding.surface_uv)
            .and_then(crate::surface_texture::SurfaceUvStream::identity);
        let sphere = sphere_normals(draw, encoding.sphere_normals)
            .and_then(crate::draw::DrawStream::identity);
        let text = text_uv(draw, encoding.text_uv).and_then(crate::draw::DrawStream::identity);
        Self {
            streams: [
                draw.vertices.identity(),
                draw.indices.identity(),
                draw.quad_corners.identity(),
                encoding
                    .provoking_vertices
                    .then(|| draw.native_flat_sources.identity())
                    .flatten(),
                normals,
                uv,
                sphere,
                text,
            ],
            encoding,
        }
    }

    pub(super) fn with_unlit_vertices(mut self, identity: NonZeroUsize) -> Option<Self> {
        if self.encoding.has_lighting
            || self.encoding.fixed_grid_points
            || self.encoding.surface_uv
            || self.encoding.sphere_normals
            || self.encoding.text_uv
        {
            return None;
        }
        self.streams[0] = Some(identity);
        Some(self)
    }

    pub(super) fn with_compact_line(self, identity: NonZeroUsize) -> Option<Self> {
        let mut key = self.with_unlit_vertices(identity)?;
        key.encoding.topology = GeometryTopology::CompactLines;
        Some(key)
    }

    pub(super) fn previous_indices(mut self, identity: NonZeroUsize) -> Option<Self> {
        // These paths derive vertex data or device indices from more than the packed index stream.
        if self.encoding.provoking_vertices
            || self.encoding.fixed_grid_points
            || self.encoding.flat_text
            || self.streams[2].is_some()
        {
            return None;
        }
        self.streams[1] = Some(identity);
        Some(self)
    }
}

struct GeometryLease {
    geometry: ResidentGeometry,
    vertices: Option<Weak<Vec<RenderVertex>>>,
    indices: Option<Weak<Vec<u32>>>,
    quad_corners: Option<Weak<Vec<[u32; 4]>>>,
    flat_sources: Option<Weak<Vec<u32>>>,
    normals: Option<Weak<Vec<[f32; 3]>>>,
    uv: Option<Weak<Vec<[f32; 4]>>>,
    sphere: Option<Weak<Vec<[f32; 3]>>>,
    text: Option<Weak<Vec<[f32; 2]>>>,
}

impl GeometryLease {
    fn new(draw: &PrimitiveDraw, geometry: ResidentGeometry, key: GeometryKey) -> Self {
        Self {
            geometry,
            vertices: draw.vertices.witness(),
            indices: draw.indices.witness(),
            quad_corners: draw.quad_corners.witness(),
            flat_sources: key
                .encoding
                .provoking_vertices
                .then(|| draw.native_flat_sources.witness())
                .flatten(),
            normals: draw
                .lighting
                .as_ref()
                .and_then(|lighting| lighting.normals.witness()),
            uv: surface_uv(draw, key.encoding.surface_uv)
                .and_then(crate::surface_texture::SurfaceUvStream::witness),
            sphere: sphere_normals(draw, key.encoding.sphere_normals)
                .and_then(crate::draw::DrawStream::witness),
            text: text_uv(draw, key.encoding.text_uv).and_then(crate::draw::DrawStream::witness),
        }
    }

    fn is_live(&self) -> bool {
        is_live(&self.vertices)
            && is_live(&self.indices)
            && is_live(&self.quad_corners)
            && is_live(&self.flat_sources)
            && is_live(&self.normals)
            && is_live(&self.uv)
            && is_live(&self.sphere)
            && is_live(&self.text)
    }
}

fn surface_uv(
    draw: &PrimitiveDraw,
    used: bool,
) -> Option<&crate::surface_texture::SurfaceUvStream> {
    draw.surface_texture
        .as_ref()
        .filter(|_| used)
        .map(|style| &style.uv)
}

fn sphere_normals(draw: &PrimitiveDraw, used: bool) -> Option<&crate::draw::DrawStream<[f32; 3]>> {
    draw.surface_texture
        .as_ref()
        .filter(|_| used)
        .and_then(|style| style.sphere_map.as_ref())
        .map(|sphere| &sphere.normals)
}

fn text_uv(draw: &PrimitiveDraw, used: bool) -> Option<&crate::draw::DrawStream<[f32; 2]>> {
    draw.textured_style
        .as_ref()
        .filter(|_| used)
        .map(|style| &style.uv)
}

fn is_live<T>(stream: &Option<Weak<Vec<T>>>) -> bool {
    stream
        .as_ref()
        .is_none_or(|witness| witness.strong_count() != 0)
}

pub(super) struct GeometryStore {
    retained: Mutex<RetainedGeometry>,
    byte_limit: u64,
    pinned_batch_bytes: Arc<AtomicU64>,
    pub(super) buffers: GeometryBufferPool,
}

pub(super) struct BatchCharge {
    pinned_bytes: Arc<AtomicU64>,
    bytes: u64,
}

impl Drop for BatchCharge {
    fn drop(&mut self) {
        self.pinned_bytes.fetch_sub(self.bytes, Ordering::Relaxed);
    }
}

const RETIREMENT_CHECKS_PER_FRAME: usize = 512;
const MAX_RESIDENT_ENTRIES: usize = 131_072;
const MAX_RESIDENT_BYTES: u64 = 2 << 30;

#[derive(Default)]
struct RetainedGeometry {
    entries: HashMap<GeometryKey, GeometryLease>,
    check_order: VecDeque<GeometryKey>,
    bytes: u64,
}

impl RetainedGeometry {
    fn retire_expired(&mut self, budget: usize) {
        for _ in 0..budget.min(self.check_order.len()) {
            let Some(key) = self.check_order.pop_front() else {
                break;
            };
            let Entry::Occupied(entry) = self.entries.entry(key) else {
                continue;
            };
            if entry.get().is_live() {
                self.check_order.push_back(key);
            } else {
                self.bytes -= entry.remove().geometry.bytes();
            }
        }
    }

    fn insert(&mut self, key: GeometryKey, lease: GeometryLease, byte_limit: u64) {
        let bytes = lease.geometry.bytes();
        if bytes > byte_limit || self.entries.contains_key(&key) {
            return;
        }
        self.make_room(byte_limit - bytes);
        self.bytes += bytes;
        self.entries.insert(key, lease);
        self.check_order.push_back(key);
    }

    fn make_room(&mut self, available_bytes: u64) {
        while self.bytes > available_bytes || self.entries.len() >= MAX_RESIDENT_ENTRIES {
            let Some(oldest) = self.check_order.pop_front() else {
                break;
            };
            if let Some(lease) = self.entries.remove(&oldest) {
                self.bytes -= lease.geometry.bytes();
            }
        }
    }
}

impl GeometryStore {
    pub(super) fn new(runtime: &VulkanRuntime) -> Self {
        let heaps = &runtime.memory.memory_heaps[..runtime.memory.memory_heap_count as usize];
        let local_bytes = heaps
            .iter()
            .filter(|heap| heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|heap| heap.size)
            .max()
            .unwrap_or_default();
        Self {
            retained: Mutex::default(),
            byte_limit: (local_bytes / 4).min(MAX_RESIDENT_BYTES),
            pinned_batch_bytes: Arc::default(),
            buffers: GeometryBufferPool::new((local_bytes / 4).min(MAX_RESIDENT_BYTES)),
        }
    }

    fn retained(&self) -> std::sync::MutexGuard<'_, RetainedGeometry> {
        self.retained
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(super) fn can_pin_batch(&self, bytes: u64) -> bool {
        let retained = self.retained();
        bytes <= self.batch_capacity(&retained)
    }

    pub(super) fn pin_batch(&self, bytes: u64) -> Option<Arc<BatchCharge>> {
        let retained = self.retained();
        if bytes > self.batch_capacity(&retained) {
            return None;
        }
        self.pinned_batch_bytes.fetch_add(bytes, Ordering::Relaxed);
        Some(Arc::new(BatchCharge {
            pinned_bytes: self.pinned_batch_bytes.clone(),
            bytes,
        }))
    }

    fn batch_capacity(&self, retained: &RetainedGeometry) -> u64 {
        self.byte_limit
            .saturating_sub(self.pinned_batch_bytes.load(Ordering::Relaxed))
            .saturating_sub(retained.bytes)
    }

    fn find(&self, key: GeometryKey) -> Option<ResidentGeometry> {
        self.retained()
            .entries
            .get(&key)
            .map(|lease| lease.geometry.clone())
    }

    /// Store a frame's pending uploads. `retire` holds whether the frame was empty and how many
    /// uploads it made; it also checks the liveness of that many older entries, twice over.
    fn publish(&self, pending: &mut PendingGeometry, retire: Option<(bool, usize)>) {
        let mut retained = self.retained();
        if let Some((empty_frame, uploads)) = retire {
            let budget = if empty_frame {
                retained.entries.len()
            } else {
                RETIREMENT_CHECKS_PER_FRAME.max(uploads.saturating_mul(2))
            };
            retained.retire_expired(budget);
        }
        let available = self
            .byte_limit
            .saturating_sub(self.pinned_batch_bytes.load(Ordering::Relaxed));
        for key in pending.order.drain(..) {
            if let Some(lease) = pending.leases.remove(&key).filter(GeometryLease::is_live) {
                retained.insert(key, lease, available);
            }
        }
    }
}

pub(super) struct FrameGeometryResidency {
    store: Arc<GeometryStore>,
    pending: PendingGeometry,
    /// Uploads already shared at submission; they still size the retirement budget (#594).
    shared: usize,
    pending_batch: Option<(Arc<super::GeometryBatch>, ResidentGeometry)>,
}

#[derive(Default)]
struct PendingGeometry {
    leases: HashMap<GeometryKey, GeometryLease>,
    order: Vec<GeometryKey>,
}

impl FrameGeometryResidency {
    pub(super) fn new(store: &Arc<GeometryStore>) -> Self {
        Self {
            store: Arc::clone(store),
            pending: PendingGeometry::default(),
            shared: 0,
            pending_batch: None,
        }
    }

    pub(super) fn buffers(&self) -> &GeometryBufferPool {
        &self.store.buffers
    }

    pub(super) fn find(&self, key: GeometryKey) -> Option<ResidentGeometry> {
        if let Some(lease) = self.pending.leases.get(&key) {
            return Some(lease.geometry.clone());
        }
        self.store.find(key)
    }

    /// An entry that an earlier frame uploaded, never one of this frame's own uploads: the
    /// copies of one frame are not ordered against each other. `residency.find_earlier(key)`.
    pub(super) fn find_earlier(&self, key: GeometryKey) -> Option<ResidentGeometry> {
        self.store.find(key)
    }

    /// Let later frames use this frame's uploads once it is submitted (#594); the queue orders
    /// the uploads before every later frame. `residency.share()`.
    pub(super) fn share(&mut self) {
        self.publish_batch();
        self.shared += self.pending.order.len();
        self.store.publish(&mut self.pending, None);
    }

    pub(super) fn queue(
        &mut self,
        key: GeometryKey,
        draw: &PrimitiveDraw,
        geometry: ResidentGeometry,
    ) {
        if self
            .pending
            .leases
            .insert(key, GeometryLease::new(draw, geometry, key))
            .is_none()
        {
            self.pending.order.push(key);
        }
    }

    pub(super) fn queue_batch(
        &mut self,
        owner: Arc<super::GeometryBatch>,
        geometry: ResidentGeometry,
    ) {
        assert!(self.pending_batch.is_none());
        self.pending_batch = Some((owner, geometry));
    }

    fn publish_batch(&mut self) {
        if let Some((owner, geometry)) = self.pending_batch.take() {
            owner.publish(geometry);
        }
    }

    pub(super) fn publish(&mut self, empty_frame: bool) {
        self.publish_batch();
        let uploads = self.pending.order.len() + std::mem::take(&mut self.shared);
        self.store
            .publish(&mut self.pending, Some((empty_frame, uploads)));
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FrameGeometryResidency, GeometryBuffer, GeometryEncoding, GeometryKey, GeometryLease,
        GeometryStore, GeometryTopology, ResidentGeometry, RetainedGeometry,
    };
    use crate::{PrimitiveDraw, RenderVertex, backend::buffer::DeviceBuffer};
    use ash::vk;

    #[test]
    fn index_patches_require_a_direct_index_stream() {
        let mut draw = test_draw();
        let mut key = test_key(&draw);
        let previous = draw.indices.identity().unwrap();
        assert!(key.previous_indices(previous).is_some());
        key.encoding.fixed_grid_points = true;
        assert!(key.previous_indices(previous).is_none());
        key.encoding.fixed_grid_points = false;
        key.encoding.provoking_vertices = true;
        assert!(key.previous_indices(previous).is_none());
        draw.quad_corners = vec![[0, 1, 2, 0]].into();
        assert!(test_key(&draw).previous_indices(previous).is_none());
    }

    #[test]
    fn device_cache_budget_evicts_a_live_key_without_releasing_its_draw_owner() {
        let renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
        let first = test_draw();
        let second = test_draw();
        let (first_key, second_key) = (test_key(&first), test_key(&second));
        let retained_draw = test_geometry(&renderer.runtime);
        let budget = retained_draw.bytes();
        let mut cache = RetainedGeometry::default();
        cache.insert(
            first_key,
            GeometryLease::new(&first, retained_draw.clone(), first_key),
            budget,
        );
        assert_eq!(Arc::strong_count(&retained_draw.vertices), 2);
        cache.insert(
            second_key,
            GeometryLease::new(&second, test_geometry(&renderer.runtime), second_key),
            budget,
        );
        assert!(!cache.entries.contains_key(&first_key));
        assert!(cache.entries.contains_key(&second_key));
        assert_eq!(cache.bytes, budget);
        assert_eq!(Arc::strong_count(&retained_draw.vertices), 1);
    }

    #[test]
    fn batch_budget_counts_regular_cache_and_retained_buffer_owners() {
        let renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
        let regular = test_geometry(&renderer.runtime);
        let draw = test_draw();
        let regular_key = test_key(&draw);
        let mut store = GeometryStore::new(&renderer.runtime);
        let buffer =
            DeviceBuffer::device_local(&renderer.runtime, 96, vk::BufferUsageFlags::VERTEX_BUFFER)
                .unwrap();
        let batch_bytes = buffer.allocation_size();
        store.byte_limit = regular.bytes() + batch_bytes;
        let store = Arc::new(store);
        let mut uploads = FrameGeometryResidency::new(&store);
        uploads.queue(regular_key, &draw, regular);
        uploads.share();

        assert!(!store.can_pin_batch(batch_bytes + 1));
        let charge = store.pin_batch(batch_bytes).unwrap();
        let batch = Arc::new(GeometryBuffer::charged(buffer, charge));
        let flight_owner = Arc::clone(&batch);
        assert!(store.find(regular_key).is_some());
        assert!(store.pin_batch(1).is_none());
        drop(batch);
        assert!(!store.can_pin_batch(1));
        drop(flight_owner);
        assert!(store.can_pin_batch(batch_bytes));
    }

    #[test]
    fn a_submitted_upload_serves_later_frames_but_not_its_own() {
        let renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
        let store = Arc::new(GeometryStore::new(&renderer.runtime));
        let draw = test_draw();
        let key = test_key(&draw);
        let mut submitted = FrameGeometryResidency::new(&store);
        submitted.queue(key, &draw, test_geometry(&renderer.runtime));
        assert!(submitted.find(key).is_some());
        assert!(submitted.find_earlier(key).is_none());
        submitted.share();
        let later = FrameGeometryResidency::new(&store);
        assert!(later.find(key).is_some());
        assert!(later.find_earlier(key).is_some());
    }

    use std::sync::Arc;

    fn test_draw() -> PrimitiveDraw {
        PrimitiveDraw::new(
            vec![RenderVertex::new([0.; 3], [1.; 4]); 3],
            vec![0, 1, 2],
            crate::identity_clip(),
        )
        .unwrap()
    }

    fn test_key(draw: &PrimitiveDraw) -> GeometryKey {
        GeometryKey::new(
            draw,
            GeometryEncoding {
                surface_coordinates: None,
                surface_uv: false,
                sphere_normals: false,
                text_uv: false,
                font_replace: false,
                flat_text: false,
                topology: GeometryTopology::Triangles,
                has_lighting: false,
                fixed_lighting: false,
                provoking_vertices: false,
                fixed_grid_points: false,
                xor_color: false,
            },
        )
    }

    fn test_geometry(runtime: &Arc<crate::backend::runtime::VulkanRuntime>) -> ResidentGeometry {
        let vertices =
            DeviceBuffer::device_local(runtime, 96, vk::BufferUsageFlags::VERTEX_BUFFER).unwrap();
        let indices =
            DeviceBuffer::device_local(runtime, 12, vk::BufferUsageFlags::INDEX_BUFFER).unwrap();
        ResidentGeometry {
            vertices: Arc::new(GeometryBuffer::dedicated(vertices, None)),
            indices: Arc::new(GeometryBuffer::dedicated(indices, None)),
        }
    }
}
