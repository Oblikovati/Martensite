//! Retained text labels (#594, plan item 5). A label keeps the draws it lowered with the
//! revisions they came from; a later frame computes only the label's clip transforms, which
//! follow the camera, and places the kept draws with them.
use crate::PrimitiveDraw;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use std::sync::{Arc, Mutex};

/// The record of one text command; the clones of a command share it.
#[derive(Clone, Default)]
pub(crate) struct LabelSlot(Arc<Mutex<Option<LabelRecord>>>);

struct LabelRecord {
    inputs: LabelInputs,
    replay: LabelReplay,
}

/// The draws a label lowered and what a replay needs besides them.
#[derive(Clone)]
pub(crate) struct LabelReplay {
    /// The layout's point size, which the clip transforms scale by.
    pub point_size: u32,
    /// What the element upload slot recorded: glyph indices, subtitle and formatter bounds.
    pub upload: (usize, bool, martensite_tkservice::font::FontRect),
    pub draws: Vec<PrimitiveDraw>,
}

/// Everything a label's draws depend on besides the camera. Read it before lowering: an
/// edit made while the label lowers then leaves a record that the next frame finds stale.
#[derive(Clone, PartialEq)]
pub(crate) struct LabelInputs {
    text: u64,
    aspects: (usize, u64),
    /// The font name's own bytes: the name is a shared string that can change in place.
    font_name: Option<Vec<u8>>,
    resolution: u32,
    hinting: i32,
}

impl LabelInputs {
    /// The current inputs of a label; `LabelInputs::current(&text, &aspects, &params)`.
    pub(crate) fn current(
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        rendering: &Graphic3dRenderingParams,
    ) -> Option<Self> {
        let aspect = aspects.get()?;
        let name = aspect.text_font();
        Some(Self {
            text: text.revision(),
            aspects: (
                aspects.object_address().addr(),
                aspect.aspects_base().fields.revision(),
            ),
            font_name: name.get().map(|name| name.string().as_bytes().to_vec()),
            resolution: rendering.resolution,
            hinting: rendering.font_hinting.0,
        })
    }

    /// Whether nothing changed, compared in place without copying the font name.
    fn still_hold(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        rendering: &Graphic3dRenderingParams,
    ) -> bool {
        let Some(aspect) = aspects.get() else {
            return false;
        };
        let name = aspect.text_font();
        self.text == text.revision()
            && self.aspects
                == (
                    aspects.object_address().addr(),
                    aspect.aspects_base().fields.revision(),
                )
            && match (&self.font_name, name.get()) {
                (None, None) => true,
                (Some(kept), Some(name)) => kept.as_slice() == name.string().as_bytes(),
                _ => false,
            }
            && (self.resolution, self.hinting) == (rendering.resolution, rendering.font_hinting.0)
    }
}

impl LabelSlot {
    /// The kept draws when the label's text, aspects, font and resolution are unchanged;
    /// `slot.unchanged(&text, &aspects, &params)`.
    pub(crate) fn unchanged(
        &self,
        text: &Graphic3dText,
        aspects: &Handle<dyn Graphic3dAspectsSource>,
        rendering: &Graphic3dRenderingParams,
    ) -> Option<LabelReplay> {
        let slot = self.0.lock().ok()?;
        slot.as_ref()
            .filter(|record| record.inputs.still_hold(text, aspects, rendering))
            .map(|record| record.replay.clone())
    }

    /// Keep what a label lowered from `inputs`; `slot.keep(inputs, replay)`.
    pub(crate) fn keep(&self, inputs: LabelInputs, replay: LabelReplay) {
        if let Ok(mut slot) = self.0.lock() {
            *slot = Some(LabelRecord { inputs, replay });
        }
    }
}

#[cfg(test)]
#[path = "text_label_record_tests.rs"]
mod tests;
