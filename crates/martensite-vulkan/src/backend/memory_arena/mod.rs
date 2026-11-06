//! Sub-allocate buffers and images from shared device-memory blocks.
//!
//! One `vkAllocateMemory` per small resource exhausts allocation counts and is flagged by the
//! best-practices layer. Resources at or above `DEDICATED_SIZE` keep a dedicated allocation; all
//! others share `BLOCK_SIZE` blocks per memory type. Buffers and images use separate pools, so
//! `bufferImageGranularity` never applies within a block. Host-visible blocks are mapped once,
//! for their lifetime; every selected host type is coherent, so no flush is needed.
mod block_space;
use super::runtime::{VulkanRuntime, vk_error};
use crate::error::RenderError;
use ash::vk;
use block_space::BlockSpace;
use std::sync::{Arc, Mutex};

const BLOCK_SIZE: u64 = 16 << 20;
const DEDICATED_SIZE: u64 = 4 << 20;

/// Which resource family a pool serves; linear buffers never share a block with images.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ResourceTiling {
    Buffer,
    OptimalImage,
}

struct Block {
    memory: vk::DeviceMemory,
    mapped: Option<usize>,
    space: BlockSpace,
    type_index: u32,
    tiling: ResourceTiling,
}

/// All blocks of one device; owned by its runtime and freed before the device.
#[derive(Default)]
pub(super) struct MemoryArena {
    blocks: Mutex<Vec<Block>>,
}

/// One bound range; dropping it returns the range (or frees a dedicated allocation).
pub(super) struct MemoryLease {
    runtime: Arc<VulkanRuntime>,
    pub memory: vk::DeviceMemory,
    pub offset: u64,
    size: u64,
    mapped: Option<usize>,
    dedicated: bool,
}

/// The requested placement of one resource; `MemoryRequest { requirements, type_index, .. }`.
#[derive(Clone, Copy)]
pub(super) struct MemoryRequest {
    pub requirements: vk::MemoryRequirements,
    pub type_index: u32,
    pub tiling: ResourceTiling,
}

impl MemoryLease {
    /// Reserve memory for one resource; `MemoryLease::new(&runtime, request)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>, request: MemoryRequest) -> Result<Self, RenderError> {
        if request.requirements.size >= DEDICATED_SIZE {
            return dedicated_lease(runtime, request);
        }
        let (memory, offset, mapped) = runtime.memory_arena.reserve(runtime, request)?;
        Ok(Self {
            runtime: runtime.clone(),
            memory,
            offset,
            size: request.requirements.size,
            mapped,
            dedicated: false,
        })
    }
    /// The resource's required byte size (not the shared block); `lease.size()`.
    pub fn size(&self) -> u64 {
        self.size
    }
    /// The host address of this range in a mapped block; `lease.host_pointer()`.
    pub fn host_pointer(&self) -> Option<*mut u8> {
        self.mapped
            .map(|base| (base + self.offset as usize) as *mut u8)
    }
}

impl Drop for MemoryLease {
    fn drop(&mut self) {
        if self.dedicated {
            // The resource bound to this memory was destroyed before its lease drops.
            // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
            unsafe { self.runtime.device().free_memory(self.memory, None) };
            return;
        }
        let device = self.runtime.device();
        self.runtime
            .memory_arena
            .release(device, self.memory, self.offset, self.size);
    }
}

fn dedicated_lease(
    runtime: &Arc<VulkanRuntime>,
    request: MemoryRequest,
) -> Result<MemoryLease, RenderError> {
    let (memory, mapped) = allocate_block(runtime, request.type_index, request.requirements.size)?;
    Ok(MemoryLease {
        runtime: runtime.clone(),
        memory,
        offset: 0,
        size: request.requirements.size,
        mapped,
        dedicated: true,
    })
}

/// Allocate (and map, when host-visible) one device-memory object; `allocate_block(rt, 0, 64)?`.
fn allocate_block(
    runtime: &VulkanRuntime,
    type_index: u32,
    size: u64,
) -> Result<(vk::DeviceMemory, Option<usize>), RenderError> {
    let info = vk::MemoryAllocateInfo::default()
        .allocation_size(size)
        .memory_type_index(type_index);
    let device = runtime.device();
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let memory = unsafe { device.allocate_memory(&info, None) }
        .map_err(|error| vk_error("allocate device memory block", error))?;
    let flags = runtime.memory.memory_types[type_index as usize].property_flags;
    if !flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
        return Ok((memory, None));
    }
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    match unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty()) } {
        Ok(pointer) => Ok((memory, Some(pointer as usize))),
        Err(error) => {
            // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
            unsafe { device.free_memory(memory, None) };
            Err(vk_error("map device memory block", error))
        }
    }
}

impl MemoryArena {
    fn reserve(
        &self,
        runtime: &VulkanRuntime,
        request: MemoryRequest,
    ) -> Result<(vk::DeviceMemory, u64, Option<usize>), RenderError> {
        let mut blocks = self.blocks.lock().expect("memory arena lock");
        if let Some(found) = reserve_in(&mut blocks, request) {
            return Ok(found);
        }
        let (memory, mapped) = allocate_block(runtime, request.type_index, BLOCK_SIZE)?;
        let (requirements, type_index, tiling) =
            (request.requirements, request.type_index, request.tiling);
        let mut space = BlockSpace::new(BLOCK_SIZE);
        let offset = space
            .reserve(requirements.size, requirements.alignment)
            .expect("fresh block");
        blocks.push(Block {
            memory,
            mapped,
            space,
            type_index,
            tiling,
        });
        Ok((memory, offset, mapped))
    }

    fn release(&self, device: &ash::Device, memory: vk::DeviceMemory, offset: u64, size: u64) {
        let mut blocks = self.blocks.lock().expect("memory arena lock");
        let Some(index) = blocks.iter().position(|block| block.memory == memory) else {
            return;
        };
        blocks[index].space.release(offset, size);
        let (type_index, tiling) = (blocks[index].type_index, blocks[index].tiling);
        let empty = |block: &Block| {
            block.type_index == type_index && block.tiling == tiling && block.space.is_empty()
        };
        // Keep one empty block per pool so per-frame readbacks do not reallocate it.
        if blocks[index].space.is_empty() && blocks.iter().filter(|block| empty(block)).count() > 1
        {
            let block = blocks.swap_remove(index);
            // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
            unsafe { device.free_memory(block.memory, None) };
        }
    }

    /// Free every block; called by the runtime after all leases dropped, before the device.
    pub fn free_all(&self, device: &ash::Device) {
        let mut blocks = self.blocks.lock().expect("memory arena lock");
        for block in blocks.drain(..) {
            // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
            unsafe { device.free_memory(block.memory, None) };
        }
    }
}

fn reserve_in(
    blocks: &mut [Block],
    request: MemoryRequest,
) -> Option<(vk::DeviceMemory, u64, Option<usize>)> {
    let requirements = request.requirements;
    blocks
        .iter_mut()
        .filter(|block| block.type_index == request.type_index && block.tiling == request.tiling)
        .find_map(|block| {
            let offset = block
                .space
                .reserve(requirements.size, requirements.alignment)?;
            Some((block.memory, offset, block.mapped))
        })
}

/// Reserve device-local memory for an optimal image and bind it; `bind_device_image(&rt, image)?`.
pub(super) fn bind_device_image(
    runtime: &Arc<VulkanRuntime>,
    image: vk::Image,
) -> Result<MemoryLease, RenderError> {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let requirements = unsafe { runtime.device().get_image_memory_requirements(image) };
    let type_index = runtime.memory_type(
        requirements.memory_type_bits,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;
    let request = MemoryRequest {
        requirements,
        type_index,
        tiling: ResourceTiling::OptimalImage,
    };
    let lease = MemoryLease::new(runtime, request)?;
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe {
        runtime
            .device()
            .bind_image_memory(image, lease.memory, lease.offset)
    }
    .map_err(|error| vk_error("bind image memory", error))?;
    Ok(lease)
}
