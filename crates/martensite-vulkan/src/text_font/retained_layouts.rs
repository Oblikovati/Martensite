//! Bound layout storage by owned vector capacity and count shared atlas pixels once.
use super::FontKey;
use crate::{
    GlyphLayout,
    text_layout::{RetainedGlyphLayout, TextLayoutSnapshot},
};
use martensite_tkernel::ncollection::String8;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment as Horizontal, Graphic3dVerticalTextAlignment as Vertical,
};
use std::{
    collections::{HashMap, VecDeque},
    hash::{Hash, Hasher},
    sync::Arc,
};

const MAX_LAYOUT_BYTES: usize = 32 * 1024 * 1024;
const MAX_LAYOUTS: usize = 2048;

#[derive(PartialEq, Eq)]
pub(super) struct LayoutKey {
    pub font: FontKey,
    pub text: String8,
    pub horizontal: Horizontal,
    pub vertical: Vertical,
}

impl Hash for LayoutKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.font.hash(state);
        self.text.to_c_string()[..self.text.size() as usize].hash(state);
        self.horizontal.hash(state);
        self.vertical.hash(state);
    }
}

#[derive(Default)]
pub(super) struct RetainedLayouts {
    entries: HashMap<Arc<LayoutKey>, StoredLayout>,
    order: VecDeque<Arc<LayoutKey>>,
    atlas_owners: HashMap<usize, usize>,
    bytes: usize,
}

struct StoredLayout {
    layout: Arc<RetainedGlyphLayout>,
    bytes: usize,
    atlas_bytes: usize,
    atlas_id: usize,
}

impl RetainedLayouts {
    pub(super) fn get(&self, key: &LayoutKey) -> Option<TextLayoutSnapshot> {
        self.entries
            .get(key)
            .map(|entry| TextLayoutSnapshot::Retained(entry.layout.clone()))
    }

    pub(super) fn record(&mut self, key: LayoutKey, layout: GlyphLayout) -> TextLayoutSnapshot {
        let bytes = vector_bytes(&layout, &key)
            + RetainedGlyphLayout::reserved_geometry_bytes(layout.positions.len());
        if bytes + atlas_bytes(&layout) > MAX_LAYOUT_BYTES {
            return TextLayoutSnapshot::Live(layout);
        }
        let entry = StoredLayout::new(bytes, layout);
        while !self.has_space(&entry) {
            self.evict_oldest();
        }
        let retained = entry.layout.clone();
        self.insert(key, entry);
        TextLayoutSnapshot::Retained(retained)
    }

    fn insert(&mut self, key: LayoutKey, entry: StoredLayout) {
        let atlas = self.atlas_owners.entry(entry.atlas_id).or_default();
        if *atlas == 0 {
            self.bytes += entry.atlas_bytes;
        }
        *atlas += 1;
        self.bytes += entry.bytes;
        let key = Arc::new(key);
        self.order.push_back(Arc::clone(&key));
        self.entries.insert(key, entry);
    }

    fn has_space(&self, entry: &StoredLayout) -> bool {
        let already_retained = self.atlas_owners.contains_key(&entry.atlas_id);
        let atlas_bytes = if already_retained {
            0
        } else {
            entry.atlas_bytes
        };
        self.entries.len() < MAX_LAYOUTS
            && self.bytes + entry.bytes + atlas_bytes <= MAX_LAYOUT_BYTES
    }

    fn evict_oldest(&mut self) {
        let Some(entry) = self
            .order
            .pop_front()
            .and_then(|key| self.entries.remove(&key))
        else {
            return;
        };
        self.bytes -= entry.bytes;
        if let std::collections::hash_map::Entry::Occupied(mut owners) =
            self.atlas_owners.entry(entry.atlas_id)
        {
            *owners.get_mut() -= 1;
            if *owners.get() == 0 {
                owners.remove();
                self.bytes -= entry.atlas_bytes;
            }
        }
    }
}

impl StoredLayout {
    fn new(bytes: usize, layout: GlyphLayout) -> Self {
        Self {
            bytes,
            atlas_bytes: atlas_bytes(&layout),
            atlas_id: Arc::as_ptr(&layout.atlas.pixels).addr(),
            layout: Arc::new(RetainedGlyphLayout::new(layout)),
        }
    }
}

fn vector_bytes(layout: &GlyphLayout, key: &LayoutKey) -> usize {
    size_of::<[f32; 2]>() * (layout.positions.capacity() + layout.uv.capacity())
        + size_of::<u32>() * layout.indices.capacity()
        + key.font.0.capacity()
        + key.text.to_c_string().len()
}

fn atlas_bytes(layout: &GlyphLayout) -> usize {
    size_of::<[f32; 4]>() * layout.atlas.pixels.capacity()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RasterImage;

    #[test]
    fn the_entry_cap_evicts_old_layouts_without_charging_shared_atlases_twice() {
        let layout = test_layout(8192);
        let mut cache = RetainedLayouts::default();
        for index in 0..=MAX_LAYOUTS {
            cache.record(key(index), layout.clone());
        }
        assert_eq!(cache.entries.len(), MAX_LAYOUTS);
        assert_eq!(cache.atlas_owners.len(), 1);
        assert!(cache.get(&key(0)).is_none());
        assert!(cache.get(&key(MAX_LAYOUTS)).is_some());
        let vectors: usize = cache.entries.values().map(|entry| entry.bytes).sum();
        assert_eq!(cache.bytes, vectors + atlas_bytes(&layout));
    }

    #[test]
    fn the_byte_cap_evicts_unique_atlases_and_declines_oversized_layouts() {
        let pixels = MAX_LAYOUT_BYTES / size_of::<[f32; 4]>();
        let mut cache = RetainedLayouts::default();
        cache.record(key(0), test_layout(pixels / 2 + 1));
        cache.record(key(1), test_layout(pixels / 2 + 1));
        cache.record(key(2), test_layout(pixels + 1));
        assert_eq!(cache.entries.len(), 1);
        assert!(cache.get(&key(0)).is_none());
        assert!(cache.get(&key(1)).is_some());
        assert!(cache.get(&key(2)).is_none());
        assert!(cache.bytes <= MAX_LAYOUT_BYTES);
    }

    fn key(index: usize) -> LayoutKey {
        let text = format!("L{index}\0");
        LayoutKey {
            font: (b"Courier".to_vec(), 0, 16, 72, 0),
            text: String8::from_unicode_units(text.as_bytes(), -1),
            horizontal: Horizontal::Left,
            vertical: Vertical::Bottom,
        }
    }

    fn test_layout(capacity: usize) -> GlyphLayout {
        let mut layout = crate::text_drawing::tests::minimal_glyph_layout();
        let mut pixels = Vec::with_capacity(capacity);
        pixels.push([1.; 4]);
        layout.atlas = RasterImage::new([1, 1], pixels).unwrap();
        layout
    }
}
