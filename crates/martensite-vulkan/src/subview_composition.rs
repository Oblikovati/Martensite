//! Subview composition (`GlView::SetWindow` with a parent view and
//! `GlView::blitSubviews`).
//!
//! A subview renders into its own offscreen target. After the parent view has drawn its scene
//! and immediate layers, each active subview's last frame is copied, colour only, into the parent
//! frame with its top-left corner at the subview's `SubviewTopLeft` (computed by `SubviewResized`
//! from the subview size, corner, offset and margins), clipped to the parent frame.
//! `glBlitFramebuffer` copies the colour buffer bytes unchanged, so the copy here is a byte copy
//! of the subview's captured rows; the parent's depth is untouched. A subview that has not
//! rendered yet is skipped, as native skips an invalid framebuffer.
//!
//! Native blits before the swap, so a mapped parent window shows its subviews: the presented
//! frame carries the same blits (`RenderFrame::subview_blits`), which the backend copies into
//! its colour attachment before the window blit; the read-back capture receives them on the host.
use crate::{FrameCapture, Graphic3dRenderView};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectNeutralWindow;
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dCViewMembers as _, Graphic3dCViewSubviews as _,
};

impl Graphic3dRenderView {
    /// `GlView::SetWindow`'s parent link: detach from the previous parent, then attach to
    /// `parent` and place the subview in the parent window (`SubviewResized`).
    /// Example: `view.link_parent_view(&parent, &window)`.
    pub(crate) fn link_parent_view(
        &self,
        parent: &Handle<dyn Graphic3dCView>,
        window: &Handle<dyn martensite_tkservice::aspect::AspectWindow>,
    ) {
        let this: Handle<dyn Graphic3dCView> = self.this.read().upgrade().cast();
        let previous = std::mem::take(&mut self.cview_base().fields.write().parent_view);
        if let Some(previous) = previous.upgrade().get() {
            previous.remove_subview(this.object_address());
        }
        let Some(parent_view) = parent.get() else {
            return;
        };
        self.cview_base().fields.write().parent_view = parent.downgrade();
        parent_view.add_subview(&this);
        self.subview_resized(&window.cast::<AspectNeutralWindow>());
    }
    /// `blitSubviews`: the clipped colour copy of each active subview's last frame into a
    /// parent frame of `extent`, in subview order (later subviews land on top).
    /// Example: `let blits = view.subview_blits([160, 120]);`.
    pub(crate) fn subview_blits(&self, extent: [u32; 2]) -> Vec<RenderSubviewBlit> {
        let mut blits = Vec::new();
        for subview in self.subviews() {
            let render = subview.cast::<Self>();
            let Some(render) = render.get().filter(|v| v.is_active()) else {
                continue;
            };
            let outcome = render.outcome.read();
            if let Some(Ok(frame)) = outcome.as_ref() {
                blits.extend(clipped_blit(extent, frame, render.subview_top_left()));
            }
        }
        blits
    }
}

/// One subview colour copy, already clipped to the parent frame: the logical top-left offset
/// and extent of the covered rectangle and its RGBA rows, top row first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RenderSubviewBlit {
    pub offset: [u32; 2],
    pub extent: [u32; 2],
    pub rgba: Vec<u8>,
}

/// Apply `blits` in order to a top-left `target` capture (colour only, as `glBlitFramebuffer`
/// with `GL_COLOR_BUFFER_BIT`); `apply_subview_blits(&mut capture, &blits)`.
pub(crate) fn apply_subview_blits(target: &mut FrameCapture, blits: &[RenderSubviewBlit]) {
    for blit in blits {
        let width = blit.extent[0] as usize * 4;
        for (row, pixels) in blit.rgba.chunks_exact(width).enumerate() {
            let y = blit.offset[1] as usize + row;
            let start = (y * target.width as usize + blit.offset[0] as usize) * 4;
            target.rgba[start..start + width].copy_from_slice(pixels);
        }
    }
}

/// Copy the colour of `source` into `target` with its top-left pixel at `top_left`, clipped to
/// `target`; `copy_frame(&mut parent, &child, Vec2::new(10, 4))`.
#[cfg(test)]
pub(crate) fn copy_frame(target: &mut FrameCapture, source: &FrameCapture, top_left: Vec2<i32>) {
    let extent = [target.width, target.height];
    let blits: Vec<_> = clipped_blit(extent, source, top_left).into_iter().collect();
    apply_subview_blits(target, &blits);
}

/// The part of `source` placed at `top_left` that lies inside a frame of `extent`, or `None`;
/// `clipped_blit([8, 6], &child, Vec2::new(4, 3))`.
fn clipped_blit(
    extent: [u32; 2],
    source: &FrameCapture,
    top_left: Vec2<i32>,
) -> Option<RenderSubviewBlit> {
    let (source_x, target_x, width) = clipped_span(top_left.x, source.width, extent[0])?;
    let (source_y, target_y, height) = clipped_span(top_left.y, source.height, extent[1])?;
    let rgba = (source_y..source_y + height).flat_map(|y| {
        let start = (y * source.width as usize + source_x) * 4;
        source.rgba[start..start + width * 4].iter().copied()
    });
    Some(RenderSubviewBlit {
        offset: [target_x as u32, target_y as u32],
        extent: [width as u32, height as u32],
        rgba: rgba.collect(),
    })
}

/// The source start, target start and length of a span of `length` placed at `offset` in a
/// target of `limit`, or `None` when nothing overlaps; `clipped_span(-2, 5, 10) == Some((2, 0, 3))`.
fn clipped_span(offset: i32, length: u32, limit: u32) -> Option<(usize, usize, usize)> {
    let start = i64::from(offset).max(0);
    let end = (i64::from(offset) + i64::from(length)).min(i64::from(limit));
    if end <= start {
        return None;
    }
    let source = (start - i64::from(offset)) as usize;
    Some((source, start as usize, (end - start) as usize))
}

#[cfg(test)]
#[path = "subview_composition_tests.rs"]
mod tests;
