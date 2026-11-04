//! Cache image revisions and sampler policies; replacement retires old resources after frame references.
use super::{
    runtime::VulkanRuntime,
    sampled_surface_image::SampledSurfaceImage,
    surface_image_descriptor::{SurfaceImageBinding, SurfaceImageLayout},
};
use crate::{RenderError, error::rejected, surface_texture::*};
use ash::vk;
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};
type SurfaceSamplerKey = (bool, i32, i32, i32, i32);
type SurfaceImageKey = (Vec<u8>, i32);
struct SurfaceImageEntry {
    first_storage: bool,
    source: Arc<SurfaceTextureImage>,
    image: Arc<SampledSurfaceImage>,
    bindings: BTreeMap<SurfaceSamplerKey, Arc<SurfaceImageBinding>>,
}
/// One snapshot image and the sampler parameters it is read with.
#[derive(Clone, Copy)]
pub(super) struct SampledSource<'a> {
    pub image: &'a Arc<SurfaceTextureImage>,
    pub sampler: &'a SurfaceSamplerPolicy,
}
#[derive(Default)]
pub(super) struct SurfaceImageCache {
    images: Mutex<BTreeMap<SurfaceImageKey, SurfaceImageEntry>>,
    live_fixed_seen: AtomicBool,
}
impl SurfaceImageCache {
    // A live capability switch does not reinitialize existing texture resources.
    pub fn observe_live_fixed_function(&self, fixed: bool) {
        if fixed {
            self.live_fixed_seen.store(true, Ordering::Relaxed);
        }
    }
    fn storage_format(
        &self,
        entries: &BTreeMap<SurfaceImageKey, SurfaceImageEntry>,
        source: &Arc<SurfaceTextureImage>,
        requested: vk::Format,
    ) -> vk::Format {
        if !self.live_fixed_seen.load(Ordering::Relaxed) {
            return requested;
        }
        entries
            .values()
            .find(|entry| entry.first_storage && Arc::ptr_eq(&entry.source, source))
            .map_or(requested, |entry| entry.image.format)
    }
    pub fn touch(
        &self,
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<SurfaceTextureImage>,
        encoded: bool,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<(), RenderError> {
        if source.id.is_empty() {
            return Ok(());
        }
        let mut images = self.images.lock().map_err(|error| {
            rejected(
                error.to_string(),
                "unpoisoned unsampled texture upload cache",
            )
        })?;
        let format = super::surface_image_format::sampled_binding_format(source.encoding, encoded);
        let format = self.storage_format(&images, source, format);
        cache_entry(
            &mut images,
            runtime,
            source,
            format,
            retirements,
            self.live_fixed_seen.load(Ordering::Relaxed),
        )?;
        Ok(())
    }
    pub fn forget(&self, resource_id: &[u8]) -> Result<(), RenderError> {
        self.images
            .lock()
            .map_err(|error| rejected(error.to_string(), "unpoisoned sampled image cache"))?
            .retain(|(id, _), _| id != resource_id);
        Ok(())
    }

    pub fn prune(&self) -> Result<(), RenderError> {
        self.images
            .lock()
            .map_err(|error| rejected(error.to_string(), "unpoisoned sampled image cache"))?
            .retain(|_, entry| entry.source.lifetime.is_alive());
        Ok(())
    }

    pub fn binding(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &SurfaceImageLayout,
        style: &SurfaceTextureStyle,
        encoded: bool,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let sampled = SampledSource {
            image: &style.image,
            sampler: &style.sampler,
        };
        self.image_binding(runtime, layout, sampled, encoded, retirements)
    }
    /// The cached view and sampler of one image; material maps share the base-colour cache.
    pub fn image_binding(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &SurfaceImageLayout,
        sampled: SampledSource<'_>,
        encoded: bool,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let format =
            super::surface_image_format::sampled_binding_format(sampled.image.encoding, encoded);
        if sampled.image.id.is_empty() {
            let mut entry = SurfaceImageEntry::new(runtime, sampled.image, format, retirements)?;
            return cached_surface_binding(&mut entry, layout, sampled.sampler);
        }
        self.cached_binding(runtime, layout, sampled, format, retirements)
    }
    fn cached_binding(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &SurfaceImageLayout,
        sampled: SampledSource<'_>,
        format: vk::Format,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let mut images = self
            .images
            .lock()
            .map_err(|e| rejected(e.to_string(), "unpoisoned sampled image cache"))?;
        images.retain(|_, entry| entry.source.lifetime.is_alive());
        let format = self.storage_format(&images, sampled.image, format);
        let entry = cache_entry(
            &mut images,
            runtime,
            sampled.image,
            format,
            retirements,
            self.live_fixed_seen.load(Ordering::Relaxed),
        )?;
        cached_surface_binding(entry, layout, sampled.sampler)
    }
}
fn cached_surface_binding(
    entry: &mut SurfaceImageEntry,
    layout: &SurfaceImageLayout,
    policy: &SurfaceSamplerPolicy,
) -> Result<Arc<SurfaceImageBinding>, RenderError> {
    let key = sampler_key(policy);
    if let Some(binding) = entry.bindings.get(&key) {
        return Ok(binding.clone());
    }
    let binding = SurfaceImageBinding::new(&entry.image, layout, policy)?;
    // Submitted draws retain their Arc; cached historical level ranges must not grow without bound.
    entry.bindings.clear();
    entry.bindings.insert(key, binding.clone());
    Ok(binding)
}
impl SurfaceImageEntry {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        source: &Arc<SurfaceTextureImage>,
        format: ash::vk::Format,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Self, RenderError> {
        Ok(Self {
            first_storage: true,
            source: source.clone(),
            image: SampledSurfaceImage::new(runtime, source, format, retirements)?,
            bindings: BTreeMap::new(),
        })
    }
}
fn cache_entry<'a>(
    entries: &'a mut BTreeMap<SurfaceImageKey, SurfaceImageEntry>,
    runtime: &Arc<VulkanRuntime>,
    source: &Arc<SurfaceTextureImage>,
    format: ash::vk::Format,
    retirements: &super::retained_submission::RetirementStore,
    live: bool,
) -> Result<&'a mut SurfaceImageEntry, RenderError> {
    let key = (source.id.clone(), format.as_raw());
    let stale = entries
        .get(&key)
        .is_none_or(|entry| !Arc::ptr_eq(&entry.source, source));
    if stale {
        replace_surface_storage(entries, runtime, source, format, retirements, live)?;
    }
    Ok(entries.get_mut(&key).expect("inserted sampled image"))
}
fn replace_surface_storage(
    entries: &mut BTreeMap<SurfaceImageKey, SurfaceImageEntry>,
    runtime: &Arc<VulkanRuntime>,
    source: &Arc<SurfaceTextureImage>,
    format: vk::Format,
    retirements: &super::retained_submission::RetirementStore,
    live: bool,
) -> Result<(), RenderError> {
    let first_storage = !entries
        .values()
        .any(|entry| Arc::ptr_eq(&entry.source, source));
    let mut replacement = SurfaceImageEntry::new(runtime, source, format, retirements)?;
    replacement.first_storage = first_storage;
    // A failed upload leaves every prior submitted/cache owner intact.
    if live {
        entries.retain(|(id, _), entry| id != &source.id || Arc::ptr_eq(&entry.source, source));
    }
    entries.insert((source.id.clone(), format.as_raw()), replacement);
    Ok(())
}
fn sampler_key(policy: &SurfaceSamplerPolicy) -> SurfaceSamplerKey {
    (
        policy.repeat,
        policy.filter as i32,
        policy.anisotropy as i32,
        policy.first_level,
        policy.last_level,
    )
}
#[cfg(test)]
#[path = "surface_image_cache_tests.rs"]
mod tests;
