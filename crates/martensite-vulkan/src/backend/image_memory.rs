//! Device-local image bytes for frame statistics: render-target attachments and sampled
//! texture images, counted from their actual Vulkan allocation sizes.
use std::sync::atomic::{AtomicU64, Ordering};

/// Live image allocations of one Vulkan device; `runtime.image_memory.sampled_bytes()`.
#[derive(Debug, Default)]
pub(crate) struct ImageMemoryLedger {
    sampled: AtomicU64,
    last_frame_targets: AtomicU64,
}

impl ImageMemoryLedger {
    /// Count a sampled image allocation; `ledger.allocate_sampled(requirements.size)`.
    pub fn allocate_sampled(&self, bytes: u64) {
        self.sampled.fetch_add(bytes, Ordering::Relaxed);
    }
    /// Forget a destroyed sampled image; `ledger.release_sampled(bytes)`.
    pub fn release_sampled(&self, bytes: u64) {
        self.sampled.fetch_sub(bytes, Ordering::Relaxed);
    }
    /// Publish the attachments of this frame, excluding unrelated cached generations.
    pub fn sample_frame(&self, bytes: u64) {
        self.last_frame_targets.store(bytes, Ordering::Relaxed);
    }
    /// Attachment bytes of the last prepared frame; `ledger.frame_target_bytes()`.
    pub fn frame_target_bytes(&self) -> u64 {
        self.last_frame_targets.load(Ordering::Relaxed)
    }
    /// Bytes of the sampled images alive now; `ledger.sampled_bytes()`.
    pub fn sampled_bytes(&self) -> u64 {
        self.sampled.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::ImageMemoryLedger;

    #[test]
    fn frame_sample_counts_only_the_most_recent_attachments() {
        let ledger = ImageMemoryLedger::default();
        ledger.sample_frame(1024);
        ledger.sample_frame(5120);
        assert_eq!(ledger.frame_target_bytes(), 5120);
    }

    #[test]
    fn sampled_bytes_follow_live_images() {
        let ledger = ImageMemoryLedger::default();
        ledger.allocate_sampled(256);
        ledger.allocate_sampled(64);
        ledger.release_sampled(256);
        assert_eq!(ledger.sampled_bytes(), 64);
    }
}
