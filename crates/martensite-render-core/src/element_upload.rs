//! Native buffer residency of group elements. GlPrimitiveArray and GlText build
//! their vertex buffers on the first Render call, and GlFrameStats reads only
//! built buffers. Vulkan re-reads Graphic3d buffers every frame, so each retained element keeps
//! the counts its native buffers would hold after the frames that actually submitted it.
use crate::primitive::PrimitiveSubmission;
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkservice::graphic3d::*;
use std::sync::Arc;

/// Element counts and estimated bytes of a submitted primitive array; the native
/// `GlVertexBuffer`/`GlIndexBuffer` `GetElemsNb` and `EstimatedDataSize`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PrimitiveUpload {
    pub vertex_count: usize,
    pub vertex_bytes: usize,
    pub index_count: Option<usize>,
    pub index_bytes: usize,
}

/// Vertices of a submitted text: six per visible glyph, and four for a subtitle rectangle;
/// and the formatter box of the layout (native `GlText::myBndBox`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TextUpload {
    pub glyph_vertices: usize,
    pub background_vertices: usize,
    pub bounds: martensite_tkservice::font::FontRect,
}

/// The native buffer state of one element; `Failed` is final, as native `myIsVboInit`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum ElementUpload {
    #[default]
    Pending,
    Failed,
    Primitive(PrimitiveUpload),
    /// Built buffers kept after `GlPrimitiveArray::Invalidate`; the next submission builds
    /// them again from the current Graphic3d buffers.
    Invalidated(PrimitiveUpload),
    Text(TextUpload),
}

impl ElementUpload {
    /// `EstimatedDataSize` of the element's buffers; `upload.estimated_bytes()`.
    pub fn estimated_bytes(&self) -> usize {
        // Text positions and texture coordinates are two-float vertices (GlTextBuilder).
        const TEXT_VERTEX_BYTES: usize = 2 * std::mem::size_of::<f32>();
        match self {
            Self::Primitive(upload) | Self::Invalidated(upload) => {
                upload.vertex_bytes + upload.index_bytes
            }
            Self::Text(text) => {
                TEXT_VERTEX_BYTES * (2 * text.glyph_vertices + text.background_vertices)
            }
            Self::Pending | Self::Failed => 0,
        }
    }
    /// The built primitive buffers, still valid after an invalidation; `upload.primitive()`.
    pub fn primitive(&self) -> Option<PrimitiveUpload> {
        match *self {
            Self::Primitive(upload) | Self::Invalidated(upload) => Some(upload),
            _ => None,
        }
    }
}

/// Shared residency of one retained element; clones of a group command share it.
#[derive(Clone, Default)]
pub(crate) struct ElementUploadSlot(Arc<SharedState<ElementUpload>>);

impl ElementUploadSlot {
    /// The current residency; `slot.current()`.
    pub fn current(&self) -> ElementUpload {
        *self.0.read()
    }
    /// Record a primitive submission: build once, then follow mutable buffers like
    /// `updateVBO`; `slot.record_primitive(&primitive)`.
    pub fn record_primitive(&self, primitive: &PrimitiveSubmission) {
        let mut state = self.0.write();
        *state = match *state {
            ElementUpload::Pending | ElementUpload::Invalidated(_) => built_primitive(primitive),
            ElementUpload::Primitive(upload) => updated_primitive(primitive, upload),
            other => other,
        };
    }
    /// Whether a submission built the buffers (native `myIsVboInit`); `slot.is_initialized()`.
    pub fn is_initialized(&self) -> bool {
        !matches!(
            self.current(),
            ElementUpload::Pending | ElementUpload::Invalidated(_)
        )
    }
    /// Rebuild on the next submission, keeping the built sizes until then (`Invalidate`).
    pub fn invalidate(&self) {
        let mut state = self.0.write();
        *state = match *state {
            ElementUpload::Primitive(upload) => ElementUpload::Invalidated(upload),
            ElementUpload::Failed => ElementUpload::Pending,
            other => other,
        };
    }
    /// Drop the built buffers (`GlPrimitiveArray::Release`); `slot.release`.
    pub fn release(&self) {
        *self.0.write() = ElementUpload::Pending;
    }
    /// Record a submitted text layout and its formatter box;
    /// `slot.record_text(layout.indices.len(), false, layout.formatter_bounds)`.
    pub fn record_text(
        &self,
        glyph_vertices: usize,
        subtitle: bool,
        bounds: martensite_tkservice::font::FontRect,
    ) {
        let mut state = self.0.write();
        let previous = match *state {
            ElementUpload::Text(text) => text.background_vertices,
            _ => 0,
        };
        // GlText::drawRect creates its four-vertex rectangle once and keeps it.
        *state = ElementUpload::Text(TextUpload {
            glyph_vertices,
            background_vertices: if subtitle { 4 } else { previous },
            bounds,
        });
    }
}

/// The attribute layout read once from the buffer (`buildVBO` preconditions).
struct AttributeLayout {
    stride: usize,
    elements: usize,
    max_elements: usize,
    usable: bool,
    mutable: bool,
    interleaved: bool,
}

fn attribute_layout(attributes: &dyn Graphic3dBufferSource) -> AttributeLayout {
    let (usable, stride, elements) = {
        let state = attributes.buffer_base().state.read();
        let usable = !state.data.is_empty()
            && state.nb_elements >= 1
            && (1..=10).contains(&state.nb_attributes);
        (usable, state.stride, state.nb_elements)
    };
    AttributeLayout {
        stride: count(stride),
        elements: count(elements),
        max_elements: count(attributes.nb_max_elements()),
        usable,
        mutable: attributes.is_mutable(),
        interleaved: attributes.is_interleaved(),
    }
}

fn count(value: i32) -> usize {
    usize::try_from(value).unwrap_or(0)
}

/// Native `buildVBO`/`initNormalVbo`: vertex counts, then 16- or 32-bit indices.
fn built_primitive(primitive: &PrimitiveSubmission) -> ElementUpload {
    let Some(attributes) = primitive.attributes.get() else {
        return ElementUpload::Failed;
    };
    let layout = attribute_layout(attributes);
    let indices = live_indices(&primitive.indices);
    if !layout.usable || reserved_without_indices(&layout, indices.is_some()) {
        return ElementUpload::Failed;
    }
    let vertex_count = built_vertex_count(&layout, indices.is_some());
    let (index_count, index_bytes) = match indices.map(index_layout) {
        None => (None, 0),
        Some(Some((stride, elements))) => (Some(elements), stride * elements),
        Some(None) => return ElementUpload::Failed,
    };
    ElementUpload::Primitive(PrimitiveUpload {
        vertex_count,
        vertex_bytes: layout.stride * vertex_count,
        index_count,
        index_bytes,
    })
}

/// Native buildVBO raises Standard_ProgramError for reserved non-indexed storage it cannot draw.
fn reserved_without_indices(layout: &AttributeLayout, indexed: bool) -> bool {
    layout.elements != layout.max_elements && !indexed && (!layout.interleaved || layout.mutable)
}

fn built_vertex_count(layout: &AttributeLayout, indexed: bool) -> usize {
    if !indexed && layout.mutable && layout.interleaved {
        // Mutable interleaved arrays draw only the elements in use (SetElemsNb).
        return layout.elements;
    }
    if layout.mutable || !layout.interleaved {
        layout.max_elements
    } else {
        layout.elements
    }
}

/// The native constructor drops a dummy index buffer without elements.
fn live_indices(
    indices: &Handle<dyn Graphic3dIndexBufferSource>,
) -> Option<&dyn Graphic3dIndexBufferSource> {
    indices.get().filter(|buffer| buffer.nb_elements() >= 1)
}

fn index_layout(indices: &dyn Graphic3dIndexBufferSource) -> Option<(usize, usize)> {
    let stride = count(indices.stride());
    matches!(stride, 2 | 4).then(|| (stride, count(indices.nb_elements())))
}

/// Native `updateVBO`: invalidated mutable buffers publish their current element counts.
fn updated_primitive(
    primitive: &PrimitiveSubmission,
    mut upload: PrimitiveUpload,
) -> ElementUpload {
    if let Some(attributes) = primitive.attributes.get()
        && attributes.is_mutable()
        && attributes.is_interleaved()
        && !attributes.invalidated_range().is_empty()
    {
        upload.vertex_count = count(attributes.nb_elements());
        upload.vertex_bytes = count(attributes.stride()) * upload.vertex_count;
    }
    if let Some(indices) = live_indices(&primitive.indices)
        && indices.is_mutable()
        && upload.index_count.is_some()
        && !indices.invalidated_range().is_empty()
    {
        let elements = count(indices.nb_elements());
        upload.index_count = Some(elements);
        upload.index_bytes = count(indices.stride()) * elements;
    }
    ElementUpload::Primitive(upload)
}

#[cfg(test)]
#[path = "element_upload_tests.rs"]
mod tests;
