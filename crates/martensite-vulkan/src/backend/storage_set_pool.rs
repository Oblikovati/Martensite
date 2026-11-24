//! Storage descriptor pools reset only after every set using them has retired.
use super::super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

pub(super) const SETS_PER_PAGE: u32 = 512;
const CACHED_PAGES: usize = 3;

pub(in crate::backend) struct StorageSetPool {
    pages: Mutex<[Option<Arc<DescriptorPage>>; CACHED_PAGES]>,
    descriptor: vk::DescriptorType,
}

/// The pool owner is held by each allocated set until the submitting frame is retired.
pub(super) struct AllocatedStorageSet {
    owner: StorageSetOwner,
    pub(super) handle: vk::DescriptorSet,
}

enum StorageSetOwner {
    Dedicated(DescriptorPage),
    Frame(Arc<DescriptorPage>),
}

struct DescriptorPage {
    runtime: Arc<VulkanRuntime>,
    handle: vk::DescriptorPool,
}

impl AllocatedStorageSet {
    pub(super) fn share_frame(&self) -> Result<Self, RenderError> {
        let StorageSetOwner::Frame(page) = &self.owner else {
            return Err(crate::error::rejected(
                self.handle,
                "a frame-owned descriptor set",
            ));
        };
        Ok(Self {
            owner: StorageSetOwner::Frame(Arc::clone(page)),
            handle: self.handle,
        })
    }

    pub(super) fn dedicated(
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
        descriptor: vk::DescriptorType,
    ) -> Result<Self, RenderError> {
        let page = DescriptorPage::new(runtime, 1, descriptor)?;
        let handle = page.allocate(layout)?;
        Ok(Self {
            owner: StorageSetOwner::Dedicated(page),
            handle,
        })
    }

    fn page(&self) -> &DescriptorPage {
        match &self.owner {
            StorageSetOwner::Dedicated(page) => page,
            StorageSetOwner::Frame(page) => page,
        }
    }

    pub(super) fn runtime(&self) -> &VulkanRuntime {
        &self.page().runtime
    }

    #[cfg(all(test, not(feature = "command-state-diagnostics")))]
    pub(super) fn pool_handle(&self) -> vk::DescriptorPool {
        self.page().handle
    }
}

impl Default for StorageSetPool {
    fn default() -> Self {
        Self::new(vk::DescriptorType::STORAGE_BUFFER)
    }
}

impl StorageSetPool {
    pub(in crate::backend) fn new(descriptor: vk::DescriptorType) -> Self {
        Self {
            pages: Mutex::default(),
            descriptor,
        }
    }

    fn acquire(&self, runtime: &Arc<VulkanRuntime>) -> Result<OpenStoragePage, RenderError> {
        let mut pages = self.pages.lock().unwrap_or_else(PoisonError::into_inner);
        for cached in pages.iter_mut() {
            if let Some(page) = reuse_page(cached)? {
                return Ok(OpenStoragePage::new(page));
            }
        }
        let page = Arc::new(DescriptorPage::new(
            runtime,
            SETS_PER_PAGE,
            self.descriptor,
        )?);
        if let Some(slot) = pages.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(Arc::clone(&page));
        }
        Ok(OpenStoragePage::new(page))
    }
}

fn reuse_page(
    cached: &mut Option<Arc<DescriptorPage>>,
) -> Result<Option<Arc<DescriptorPage>>, RenderError> {
    let Some(page) = cached.as_mut().and_then(Arc::get_mut) else {
        return Ok(None);
    };
    if let Err(error) = page.reset() {
        // A failed reset is not proof that the descriptor pool can service fresh sets.
        *cached = None;
        return Err(error);
    }
    Ok(cached.clone())
}

/// The descriptor set cursor of one preparing frame.
pub(in crate::backend) struct FrameStorageSets<'a> {
    pool: &'a StorageSetPool,
    current: Option<OpenStoragePage>,
}

struct OpenStoragePage {
    page: Arc<DescriptorPage>,
    remaining: u32,
}

impl<'a> FrameStorageSets<'a> {
    /// Open a descriptor cursor for a frame preparing storage bindings.
    pub(in crate::backend) fn new(pool: &'a StorageSetPool) -> Self {
        Self {
            pool,
            current: None,
        }
    }

    pub(super) fn allocate(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
    ) -> Result<AllocatedStorageSet, RenderError> {
        self.allocate_bindings(runtime, layout, 1)
    }

    pub(super) fn allocate_bindings(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
        bindings: u32,
    ) -> Result<AllocatedStorageSet, RenderError> {
        if let Some(page) = self.current.as_mut()
            && page.remaining >= bindings
        {
            return page.allocate(layout, bindings);
        }
        let mut page = self.pool.acquire(runtime)?;
        let set = page.allocate(layout, bindings)?;
        self.current = Some(page);
        Ok(set)
    }
}

impl OpenStoragePage {
    fn new(page: Arc<DescriptorPage>) -> Self {
        Self {
            page,
            remaining: SETS_PER_PAGE,
        }
    }

    fn allocate(
        &mut self,
        layout: vk::DescriptorSetLayout,
        bindings: u32,
    ) -> Result<AllocatedStorageSet, RenderError> {
        let handle = self.page.allocate(layout)?;
        self.remaining -= bindings;
        Ok(AllocatedStorageSet {
            owner: StorageSetOwner::Frame(Arc::clone(&self.page)),
            handle,
        })
    }
}

impl DescriptorPage {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        count: u32,
        descriptor: vk::DescriptorType,
    ) -> Result<Self, RenderError> {
        let sizes = [vk::DescriptorPoolSize {
            ty: descriptor,
            descriptor_count: count,
        }];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(count)
            .pool_sizes(&sizes);
        // SAFETY: this device is owned by runtime and the creation info borrows local values.
        let handle = unsafe { runtime.device().create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create storage descriptor pool", e))?;
        Ok(Self {
            runtime: Arc::clone(runtime),
            handle,
        })
    }

    fn reset(&mut self) -> Result<(), RenderError> {
        // SAFETY: reset needs exclusive host access and all earlier use retired. The caller
        // acquires exclusive Arc access, which proves all earlier frame/set owners are gone.
        unsafe {
            self.runtime
                .device()
                .reset_descriptor_pool(self.handle, vk::DescriptorPoolResetFlags::empty())
        }
        .map_err(|e| vk_error("reset storage descriptor pool", e))
    }

    fn allocate(&self, layout: vk::DescriptorSetLayout) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [layout];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.handle)
            .set_layouts(&layouts);
        // SAFETY: the layout is live on this device. One frame cursor exclusively allocates
        // from each page, and a dedicated page can allocate only its one set.
        let sets = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate storage descriptor", e))?;
        self.runtime.work.add_sets(&info);
        Ok(sets[0])
    }
}

impl Drop for DescriptorPage {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to the owned device. The page only drops after every
        // allocated set's owner releases it and no pending submission can still use those sets.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.handle, None)
        };
    }
}
