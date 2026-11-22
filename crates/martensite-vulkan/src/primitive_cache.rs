//! Reuse validated local geometry across views and presentations of the same Graphic3d buffers.
use crate::draw::{DrawStream, MutableGeometry};
use crate::primitive::PrimitiveSubmission;
use crate::{LineStyle, PointStyle, PrimitiveDraw};
use martensite_tkernel::standard::{Handle, SharedState, WeakHandle};
use martensite_tkservice::graphic3d::{
    Graphic3dBndBox4f, Graphic3dBoundBuffer, Graphic3dBufferSource, Graphic3dCView,
    Graphic3dCViewMembers, Graphic3dIndexBufferSource, Graphic3dStructureMembers,
    Graphic3dStructureSource, Graphic3dTypeOfPrimitiveArray,
};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Weak};

mod source;
pub(crate) use source::{
    AttributeStamp, BatchGeometryStamp, BoundsStamp, GeometryStamp, IndexStamp, UvStamp,
};

const SOURCE_LIMIT: usize = 131_072;
const SOURCES_CHECKED_PER_INSERT: usize = 4;
const STYLE_LIMIT: usize = 4;

/// The driver shares cache entries between groups, including groups created on successive frames.
#[derive(Default)]
pub(crate) struct RetainedPrimitives(SharedState<SourceStore>);

#[derive(Default)]
struct SourceStore {
    entries: HashMap<SourceAddress, SourceEntry>,
    expiry_checks: VecDeque<SourceAddress>,
    // A group can be replaced while its source buffers are still held by the application.
    // Keep the latest slot warm across that gap without retaining every closed presentation.
    warm: Option<(SourceAddress, Arc<RetainedPrimitive>)>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct SourceAddress {
    buffers: [usize; 3],
    kind: Graphic3dTypeOfPrimitiveArray,
}

struct SourceEntry {
    attributes: WeakHandle<dyn Graphic3dBufferSource>,
    indices: WeakHandle<dyn Graphic3dIndexBufferSource>,
    bounds: WeakHandle<Graphic3dBoundBuffer>,
    cache: Weak<RetainedPrimitive>,
}

impl RetainedPrimitives {
    pub(crate) fn for_structure(structure: &Handle<dyn Graphic3dStructureSource>) -> Arc<Self> {
        structure
            .structure_manager()
            .graphic_driver()
            .cast::<crate::Graphic3dRenderDriver>()
            .get()
            .map_or_else(Arc::default, |driver| driver.primitive_sources.clone())
    }

    pub(crate) fn entry(&self, source: &PrimitiveSubmission) -> Option<Arc<RetainedPrimitive>> {
        let key = SourceAddress::of(source);
        if let Some(found) = self.0.read().get(key) {
            return Some(found);
        }
        self.0.write().insert(key, source)
    }

    pub(crate) fn prune_for_view(view: &dyn Graphic3dCView) {
        let driver = view
            .structure_manager()
            .graphic_driver()
            .cast::<crate::Graphic3dRenderDriver>();
        if let Some(driver) = driver.get() {
            driver.primitive_sources.0.write().prune();
        }
    }
}

impl SourceAddress {
    fn of(source: &PrimitiveSubmission) -> Self {
        Self {
            buffers: [
                source.attributes.object_address().addr(),
                source.indices.object_address().addr(),
                source.bounds.object_address().addr(),
            ],
            kind: source.kind,
        }
    }
}

impl SourceStore {
    fn get(&self, key: SourceAddress) -> Option<Arc<RetainedPrimitive>> {
        self.entries.get(&key)?.cache.upgrade()
    }

    fn insert(
        &mut self,
        key: SourceAddress,
        source: &PrimitiveSubmission,
    ) -> Option<Arc<RetainedPrimitive>> {
        if let Some(found) = self.get(key) {
            return Some(found);
        }
        self.prune();
        if self.entries.len() >= SOURCE_LIMIT && !self.entries.contains_key(&key) {
            return None;
        }
        let cache = Arc::new(RetainedPrimitive::default());
        if self
            .entries
            .insert(key, SourceEntry::new(source, &cache))
            .is_none()
        {
            self.expiry_checks.push_back(key);
        }
        self.warm = Some((key, cache.clone()));
        Some(cache)
    }

    fn prune(&mut self) {
        self.expire_warm();
        let checks = self.expiry_checks.len().min(SOURCES_CHECKED_PER_INSERT);
        for _ in 0..checks {
            let Some(key) = self.expiry_checks.pop_front() else {
                break;
            };
            if self.entries.get(&key).is_some_and(SourceEntry::is_live) {
                self.expiry_checks.push_back(key);
            } else {
                self.entries.remove(&key);
            }
        }
    }

    fn expire_warm(&mut self) {
        if let Some((key, _)) = &self.warm
            && !self.entries.get(key).is_some_and(SourceEntry::is_live)
        {
            self.warm = None;
        }
    }
}

impl SourceEntry {
    fn new(source: &PrimitiveSubmission, cache: &Arc<RetainedPrimitive>) -> Self {
        Self {
            attributes: source.attributes.downgrade(),
            indices: source.indices.downgrade(),
            bounds: source.bounds.downgrade(),
            cache: Arc::downgrade(cache),
        }
    }

    fn is_live(&self) -> bool {
        self.cache.strong_count() > 0
            && self.attributes.ref_count() > 0
            && (self.indices.is_null() || self.indices.ref_count() > 0)
            && (self.bounds.is_null() || self.bounds.ref_count() > 0)
    }
}

#[derive(Default)]
pub(crate) struct RetainedPrimitive {
    draws: SharedState<DrawVariants>,
    pub(crate) bounds: RetainedAttribute<Graphic3dBndBox4f>,
    pub(crate) normals: RetainedAttribute<Option<DrawStream<[f32; 3]>>>,
    pub(crate) texture_streams: crate::surface_texture_uv::RetainedTextureStreams,
}

#[derive(Default)]
pub(crate) struct RetainedAttribute<T>(SharedState<Option<(AttributeStamp, T)>>);

impl<T: Clone> RetainedAttribute<T> {
    pub(crate) fn get(&self, source: &AttributeStamp) -> Option<T> {
        self.0
            .read()
            .as_ref()
            .filter(|(saved, _)| saved == source)
            .map(|(_, value)| value.clone())
    }

    pub(crate) fn record(&self, source: AttributeStamp, value: T) {
        *self.0.write() = Some((source, value));
    }
}

#[derive(Default)]
struct DrawVariants(VecDeque<DrawVariant>);

struct DrawVariant {
    style: GeometryStyle,
    source: GeometryStamp,
    templates: Vec<PrimitiveDraw>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct GeometryStyle {
    color_bits: [u32; 4],
    topology: crate::PrimitiveTopology,
}

impl GeometryStyle {
    pub(crate) fn resolve(color: [f32; 4], line: Option<LineStyle>, point: bool) -> Self {
        let topology = if point {
            crate::PrimitiveTopology::Points
        } else if line.is_some() {
            crate::PrimitiveTopology::Lines
        } else {
            crate::PrimitiveTopology::Triangles
        };
        Self {
            color_bits: color.map(f32::to_bits),
            topology,
        }
    }
}

impl RetainedPrimitive {
    pub(crate) fn matches_batch_streams(
        &self,
        source: &GeometryStamp,
        draw: &PrimitiveDraw,
    ) -> bool {
        let Some(normals) = draw
            .lighting
            .as_ref()
            .and_then(|lighting| lighting.normals.identity())
        else {
            return false;
        };
        self.normals
            .get(&source.attributes)
            .flatten()
            .is_some_and(|current| current.identity() == Some(normals))
            && self.draws.read().0.iter().any(|variant| {
                &variant.source == source
                    && matches!(&variant.templates[..], [template]
                        if template.vertices.identity() == draw.vertices.identity()
                            && template.indices.identity() == draw.indices.identity())
            })
    }

    pub(crate) fn draws(
        &self,
        source: &GeometryStamp,
        style: GeometryStyle,
        matrix: [f32; 16],
        line: Option<LineStyle>,
        point: Option<&PointStyle>,
    ) -> Option<Vec<PrimitiveDraw>> {
        let cell = self.draws.read();
        let cached = cell
            .0
            .iter()
            .find(|draw| draw.style == style && &draw.source == source)?;
        Some(
            cached
                .templates
                .iter()
                .cloned()
                .map(|mut draw| {
                    draw.clip_transform = matrix;
                    draw.line_style = if point.is_some() { None } else { line };
                    draw.point_style = point.cloned();
                    draw
                })
                .collect(),
        )
    }

    pub(crate) fn record(
        &self,
        source: GeometryStamp,
        style: GeometryStyle,
        draws: &mut [PrimitiveDraw],
        mutable: MutableGeometry,
    ) {
        if style.topology == crate::PrimitiveTopology::Lines
            && !mutable.any()
            && !source.has_vertex_colors()
        {
            retain_uniform_line_streams(draws);
        }
        self.draws.write().insert(source, style, draws, mutable);
    }
}

fn retain_uniform_line_streams(draws: &mut [PrimitiveDraw]) {
    // Require at least 64 KiB of vertex savings to amortize a separate pipeline.
    const MIN_COMPACT_LINE_VERTICES: usize = (64 << 10) / 16;
    for draw in draws {
        if draw.vertices.len() >= MIN_COMPACT_LINE_VERTICES {
            draw.retain_uniform_line();
        }
    }
}

impl DrawVariants {
    fn insert(
        &mut self,
        source: GeometryStamp,
        style: GeometryStyle,
        draws: &mut [PrimitiveDraw],
        mutable: MutableGeometry,
    ) {
        if let Some(slot) = self.0.iter().position(|variant| variant.style == style) {
            let old = &self.0[slot];
            if old.source == source {
                return;
            }
            if mutable.any() && old.templates.len() == draws.len() {
                for (draw, previous) in draws.iter_mut().zip(&old.templates) {
                    draw.retain_geometry_edit(previous, mutable);
                }
            }
            self.0.remove(slot);
        } else if self.0.len() == STYLE_LIMIT {
            self.0.pop_front();
        }
        self.0.push_back(DrawVariant::new(source, style, draws));
    }
}

impl DrawVariant {
    fn new(source: GeometryStamp, style: GeometryStyle, draws: &[PrimitiveDraw]) -> Self {
        let templates = draws
            .iter()
            .cloned()
            .map(|mut draw| {
                // Marker pixmaps and line policies are live even when vertices are shared.
                draw.point_style = None;
                draw.line_style = None;
                draw
            })
            .collect();
        Self {
            style,
            source,
            templates,
        }
    }
}

#[cfg(test)]
#[path = "primitive_cache_tests.rs"]
mod tests;
