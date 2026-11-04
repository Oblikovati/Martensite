//! Explicit raster widths preserve requested presentation aspects for inspection and replay.
/// Choose how line aspects become raster widths, independently of their colour and pattern.
/// Example: `frame.with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderLineWidthPolicy {
    /// Preserve existing requested aspect widths, including ordered width inheritance.
    #[default]
    AspectWidthsV1,
    /// Rasterize ordinary lines and polygon edges at one pixel, retaining requested aspects.
    SinglePixelV1,
}

impl crate::RenderFrame {
    /// Select effective raster widths without changing geometry or requested aspects.
    /// Example: `frame.with_line_width_policy(RenderLineWidthPolicy::SinglePixelV1)`.
    pub fn with_line_width_policy(mut self, policy: RenderLineWidthPolicy) -> Self {
        for draw in &mut self.draws {
            for style in [&mut draw.line_style, &mut draw.polygon_edge_style]
                .into_iter()
                .flatten()
            {
                style.raster_policy = policy;
            }
        }
        self
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderLineWidthPolicy>();
};

#[cfg(test)]
#[path = "line_width_policy_tests.rs"]
mod tests;
