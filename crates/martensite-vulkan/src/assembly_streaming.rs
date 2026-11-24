//! Versioned presentation of cold assembly geometry.

/// Select how a mapped Vulkan view draws cold assembly geometry.
///
/// Example: `RenderAssemblyStreaming::VisibleBudgetedV2(NonZeroU64::MIN)`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderAssemblyStreaming {
    /// Upload all geometry needed for a complete redraw.
    #[default]
    CompleteV1,
    /// Limit each eligible redraw's vertex and index uploads to this many bytes.
    ///
    /// The view draws resident geometry and adds visible parts in screen-center order.
    /// Unsupported frames and parts larger than the budget use a complete redraw.
    VisibleBudgetedV2(std::num::NonZeroU64),
}

impl RenderAssemblyStreaming {
    pub(crate) fn budget(self) -> Option<u64> {
        match self {
            Self::CompleteV1 => None,
            Self::VisibleBudgetedV2(bytes) => Some(bytes.get()),
        }
    }
}

/// Progress of a published budgeted redraw.
///
/// Example: `if progress.complete() { /* every visible draw was included */ }`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderAssemblyStreamingProgress {
    /// Draws issued for the current visible set, including resident placements.
    pub drawn: usize,
    /// Draws in the current visible set before applying the upload budget.
    pub visible: usize,
    /// Vertex and index bytes staged by this frame.
    pub uploaded_bytes: u64,
    /// Maximum vertex and index bytes to stage in the redraw.
    pub budget_bytes: u64,
}

impl RenderAssemblyStreamingProgress {
    /// Whether every currently visible draw was included. Example: `progress.complete()`.
    pub fn complete(self) -> bool {
        self.drawn == self.visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU64;

    #[test]
    fn streaming_policy_and_progress_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderAssemblyStreaming>();
        assert_send_sync::<RenderAssemblyStreamingProgress>();
    }

    #[test]
    fn complete_is_default_and_progress_uses_the_current_visible_set() {
        assert_eq!(RenderAssemblyStreaming::default().budget(), None);
        let budget = NonZeroU64::new(120).unwrap();
        assert_eq!(
            RenderAssemblyStreaming::VisibleBudgetedV2(budget).budget(),
            Some(120)
        );
        let progress = RenderAssemblyStreamingProgress {
            drawn: 2,
            visible: 3,
            uploaded_bytes: 100,
            budget_bytes: budget.get(),
        };
        assert!(!progress.complete());
        assert!(
            RenderAssemblyStreamingProgress {
                visible: 2,
                ..progress
            }
            .complete()
        );
    }
}
