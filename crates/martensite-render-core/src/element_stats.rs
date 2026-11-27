//! Per-element frame counters: GlPrimitiveArray::UpdateDrawStats, GlText::UpdateDrawStats
//! and GlElement::UpdateMemStats, over the Vulkan element residency.
use crate::RenderElement;
use crate::element_upload::{ElementUpload, PrimitiveUpload};
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsCounter as Counter, Graphic3dFrameStatsDataTmp,
    Graphic3dTypeOfPrimitiveArray as Kind,
};
use std::sync::Arc;

/// One node GlFrameStats::updateStructures visits: a built-in element, whose counters are
/// computed here, or a user element, whose own hooks update the active frame counters (native
/// calls both virtuals on every node). Example: `CountedElement::User(element)`.
#[derive(Clone)]
pub(crate) enum CountedElement {
    BuiltIn(GroupElementStats),
    User(Arc<dyn RenderElement>),
}

impl CountedElement {
    /// `UpdateMemStats` on the active frame; `element.update_mem_stats(&mut tmp)`.
    pub fn update_mem_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp) {
        match self {
            Self::BuiltIn(element) => {
                add_counter(stats, Counter::EstimatedBytesGeom, element.memory_bytes());
            }
            Self::User(element) => element.update_mem_stats(stats),
        }
    }
    /// `UpdateDrawStats` on the active frame; `element.update_draw_stats(&mut tmp, true)`.
    pub fn update_draw_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp, detailed: bool) {
        match self {
            Self::BuiltIn(element) => element.draw_stats(detailed).apply_to(stats),
            Self::User(element) => element.update_draw_stats(stats, detailed),
        }
    }
}

/// Native `size_t +=` on one counter of the active frame (wrapping like the C++ conversion).
fn add_counter(stats: &mut Graphic3dFrameStatsDataTmp, counter: Counter, delta: usize) {
    let slot = stats.change_counter_value(counter);
    *slot = slot.wrapping_add(delta);
}

/// One retained group element as the statistics see it; `GroupElementStats::Text(upload)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum GroupElementStats {
    /// A primitive array: its type, `NbBounds` (None without bounds) and residency.
    Primitive {
        kind: Kind,
        bounds: Option<usize>,
        upload: ElementUpload,
    },
    /// A text element and its residency.
    Text(ElementUpload),
}

/// Counter increments of one element; `ElementTally::default().add(Counter::NbElemsNotCulled, 1)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ElementTally(pub Vec<(Counter, usize)>);

impl ElementTally {
    fn add(&mut self, counter: Counter, delta: usize) {
        self.0.push((counter, delta));
    }
    /// Native `size_t += int`: a negative strip count wraps exactly like the C++ conversion.
    fn add_signed(&mut self, counter: Counter, delta: i64) {
        self.add(counter, delta as isize as usize);
    }
    /// Add every increment to the active frame; `tally.apply_to(&mut tmp)`.
    pub fn apply_to(self, stats: &mut Graphic3dFrameStatsDataTmp) {
        for (counter, delta) in self.0 {
            add_counter(stats, counter, delta);
        }
    }
}

impl GroupElementStats {
    /// `UpdateMemStats`: the estimated bytes of the element's built buffers.
    pub fn memory_bytes(&self) -> usize {
        match self {
            Self::Primitive { upload, .. } | Self::Text(upload) => upload.estimated_bytes(),
        }
    }
    /// `UpdateDrawStats(stats, detailed)`; `element.draw_stats(true)`.
    pub fn draw_stats(&self, detailed: bool) -> ElementTally {
        let mut tally = ElementTally::default();
        tally.add(Counter::NbElemsNotCulled, 1);
        match *self {
            Self::Text(upload) => text_stats(&mut tally, upload, detailed),
            Self::Primitive {
                kind,
                bounds,
                upload,
            } => primitive_stats(&mut tally, kind, bounds, upload, detailed),
        }
        tally
    }
}

fn text_stats(tally: &mut ElementTally, upload: ElementUpload, detailed: bool) {
    tally.add(Counter::NbElemsTextNotCulled, 1);
    if let (true, ElementUpload::Text(text)) = (detailed, upload) {
        // Two non-indexed triangles per glyph.
        tally.add(Counter::NbTrianglesNotCulled, text.glyph_vertices / 3);
    }
}

fn primitive_stats(
    tally: &mut ElementTally,
    kind: Kind,
    bounds: Option<usize>,
    upload: ElementUpload,
    detailed: bool,
) {
    tally.add(element_class(kind), 1);
    // Native UpdateDrawStats reads the built buffers, which an invalidation keeps.
    let (true, Some(upload)) = (detailed, upload.primitive()) else {
        return;
    };
    if let Some((counter, value)) = primitive_count(kind, &upload, bounds.unwrap_or(1)) {
        tally.add_signed(counter, value);
    }
}

/// `myIsFillType` selects fill; then `GL_POINTS`; every other mode (including none) is a line.
fn element_class(kind: Kind) -> Counter {
    match kind {
        Kind::Triangles
        | Kind::Trianglestrips
        | Kind::Trianglefans
        | Kind::TrianglesAdjacency
        | Kind::TriangleStripAdjacency
        | Kind::Quadrangles
        | Kind::Quadranglestrips
        | Kind::Polygons => Counter::NbElemsFillNotCulled,
        Kind::Points => Counter::NbElemsPointNotCulled,
        _ => Counter::NbElemsLineNotCulled,
    }
}

/// Arrays drawn as lines: neither fill nor points; `is_line_array(Kind::Segments)`.
pub(crate) fn is_line_array(kind: Kind) -> bool {
    element_class(kind) == Counter::NbElemsLineNotCulled
}

/// The native draw-mode switch; polygons and undefined arrays add no primitive count.
fn primitive_count(kind: Kind, upload: &PrimitiveUpload, bounds: usize) -> Option<(Counter, i64)> {
    let n = upload.index_count.unwrap_or(upload.vertex_count) as i64;
    let b = bounds as i64;
    Some(match kind {
        Kind::Points => (Counter::NbPointsNotCulled, n),
        Kind::Segments => (Counter::NbLinesNotCulled, n / 2),
        Kind::Polylines => (Counter::NbLinesNotCulled, n - b),
        Kind::LinesAdjacency => (Counter::NbLinesNotCulled, n / 4),
        Kind::LineStripAdjacency => (Counter::NbLinesNotCulled, n - 4 * b),
        Kind::Triangles => (Counter::NbTrianglesNotCulled, n / 3),
        Kind::Trianglestrips | Kind::Trianglefans => (Counter::NbTrianglesNotCulled, n - 2 * b),
        Kind::TrianglesAdjacency => (Counter::NbTrianglesNotCulled, n / 6),
        Kind::TriangleStripAdjacency => (Counter::NbTrianglesNotCulled, n - 4 * b),
        Kind::Quadrangles => (Counter::NbTrianglesNotCulled, n / 2),
        Kind::Quadranglestrips => (Counter::NbTrianglesNotCulled, (n / 2 - b) * 2),
        _ => return None,
    })
}

/// `NbBounds` of a bound buffer, None without one; `bound_count(&primitive.bounds)`.
pub(crate) fn bound_count(
    bounds: &martensite_tkernel::standard::Handle<
        martensite_tkservice::graphic3d::Graphic3dBoundBuffer,
    >,
) -> Option<usize> {
    let bounds = bounds.get()?;
    Some(usize::try_from(bounds.state().nb_bounds).unwrap_or(0))
}

#[cfg(test)]
#[path = "element_stats_tests.rs"]
mod tests;
