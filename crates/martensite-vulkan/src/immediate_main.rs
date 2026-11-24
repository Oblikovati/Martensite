//! Preserve a completed main pass only while its effective input matches exactly.
//!
//! An immediate redraw proves this in one of two ways. With an elision (immediate_elision.rs),
//! the view had no main-layer notice since the main pass and its in-place state matches, so the
//! redraw builds only the immediate layers. Otherwise the redraw builds the whole frame and
//! this key compares its main draws with those of the retained pass.
use crate::{
    PrimitiveDraw, RenderFrame, RenderLayerSpan, RenderPolygonOffset, RenderProjectionFactors,
    backend::immediate_main_record::MainLayerTicket,
    draw::DrawStream,
    immediate_elision::{ImmediateViewInputs, MainElision},
};
use lighting::MainLightingKey;
use std::sync::{Arc, Mutex, PoisonError, Weak};

#[path = "immediate_main_lighting.rs"]
mod lighting;

#[derive(Default)]
pub(crate) struct ImmediateMainCache(Mutex<Option<RetainedMain>>);

#[derive(Clone)]
struct RetainedMain {
    key: Arc<MainPassKey>,
    ticket: MainLayerTicket,
    elision: Option<Arc<MainElision>>,
}

#[derive(Clone)]
pub(crate) enum ImmediateMainPlan {
    Remember(MainSeed),
    Restore {
        start: usize,
        ticket: MainLayerTicket,
        fallback: MainSeed,
    },
    /// The frame holds only empty layers before `start`: without the retained images the
    /// backend refuses it, and the view builds the whole frame.
    Elided {
        start: usize,
        ticket: MainLayerTicket,
    },
}

#[derive(Clone)]
pub(crate) struct MainSeed {
    pub(crate) snapshot_start: Option<usize>,
    cache: Arc<ImmediateMainCache>,
    key: Arc<MainPassKey>,
    elision: Option<Arc<MainElision>>,
}

/// The notice count and the in-place view state read before a frame was built.
pub(crate) type MainObservation = (u64, ImmediateViewInputs);

impl std::fmt::Debug for ImmediateMainPlan {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Remember(_) => output.write_str("RememberMain"),
            Self::Restore { start, .. } => output.debug_tuple("RestoreMain").field(start).finish(),
            Self::Elided { start, .. } => output.debug_tuple("ElidedMain").field(start).finish(),
        }
    }
}

impl ImmediateMainCache {
    pub(crate) fn discard(&self) {
        let retired = self.lock().take();
        drop(retired);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Option<RetainedMain>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The plan of a whole frame. `observed`, read before the frame was built, becomes the
    /// elision of the main pass that this frame draws or proves equal.
    pub(crate) fn plan(
        self: &Arc<Self>,
        frame: &RenderFrame,
        observed: Option<MainObservation>,
    ) -> Option<ImmediateMainPlan> {
        let start = eligible_start(frame)?;
        let neutral = frame.layers[start..].iter().all(neutral_layer);
        let elision = observed
            .map(|(notices, inputs)| Arc::new(MainElision::capture(notices, inputs, frame, start)));
        let active = self.lock().clone();
        let snapshot_start = (!neutral).then_some(start);
        if let Some(main) = active.filter(|main| main.key.matches(frame, start)) {
            self.refresh(&main.ticket, elision.clone());
            return Some(ImmediateMainPlan::Restore {
                start,
                ticket: main.ticket,
                fallback: self.seed(main.key, snapshot_start, elision),
            });
        }
        let key = MainPassKey::from_frame(frame, start)?;
        Some(ImmediateMainPlan::Remember(self.seed(
            Arc::new(key),
            snapshot_start,
            elision,
        )))
    }

    /// The plan of an immediate-only frame when the retained main pass still holds for the
    /// view's `notices` and `inputs`, with the elision that proves it.
    pub(crate) fn elided(
        &self,
        notices: u64,
        inputs: &ImmediateViewInputs,
    ) -> Option<(ImmediateMainPlan, Arc<MainElision>)> {
        let main = self.lock().clone()?;
        let elision = main
            .elision
            .filter(|elision| elision.holds(notices, inputs))?;
        let plan = ImmediateMainPlan::Elided {
            start: elision.start(),
            ticket: main.ticket,
        };
        Some((plan, elision))
    }

    /// Under `draw-record-verification`, an elided redraw also builds the whole frame: its main
    /// draws must equal those of the retained pass, or a main-layer notice is missing.
    #[cfg(feature = "draw-record-verification")]
    pub(crate) fn verify_elided(&self, frame: &RenderFrame, start: usize) {
        let main = self.lock().clone();
        let held = main.is_some_and(|main| main.key.matches(frame, start));
        assert!(
            held,
            "an elided immediate redraw reused a main pass that the view no longer draws; \
             expected a main-layer notice for every change (#594)"
        );
    }

    // A key match proves the main pass unchanged at the observed notice count.
    fn refresh(&self, ticket: &MainLayerTicket, elision: Option<Arc<MainElision>>) {
        let mut active = self.lock();
        if let Some(main) = active.as_mut().filter(|main| main.ticket.same(ticket)) {
            main.elision = elision;
        }
    }

    fn seed(
        self: &Arc<Self>,
        key: Arc<MainPassKey>,
        snapshot_start: Option<usize>,
        elision: Option<Arc<MainElision>>,
    ) -> MainSeed {
        MainSeed {
            snapshot_start,
            cache: self.clone(),
            key,
            elision,
        }
    }
}

impl MainSeed {
    /// The cache this seed remembers into; `seed.cache().discard()` forgets a main pass whose
    /// frame never reached the queue (#594).
    pub(crate) fn cache(&self) -> &Arc<ImmediateMainCache> {
        &self.cache
    }

    pub(crate) fn remember(&self, ticket: MainLayerTicket) {
        let retained = RetainedMain {
            key: self.key.clone(),
            ticket,
            elision: self.elision.clone(),
        };
        let retired = self.cache.lock().replace(retained);
        drop(retired);
    }
}

impl ImmediateMainPlan {
    pub(crate) fn restore(&self) -> Option<(usize, &MainLayerTicket)> {
        match self {
            Self::Restore { start, ticket, .. } | Self::Elided { start, ticket } => {
                Some((*start, ticket))
            }
            Self::Remember(_) => None,
        }
    }

    /// The seed that remembers a newly drawn main pass; an elided frame draws none.
    pub(crate) fn seed(&self) -> Option<&MainSeed> {
        match self {
            Self::Remember(seed) | Self::Restore { fallback: seed, .. } => Some(seed),
            Self::Elided { .. } => None,
        }
    }

    pub(crate) fn is_elided(&self) -> bool {
        matches!(self, Self::Elided { .. })
    }
}

/// The span index of the first immediate layer, when the frame can keep its main pass.
pub(crate) fn eligible_start(frame: &RenderFrame) -> Option<usize> {
    let start = frame.immediate_start?;
    if start == 0 || start > frame.layers.len() || !direct_single_sample(frame) {
        return None;
    }
    let stencil = frame.initial_stencil(Default::default());
    let alpha = frame.initial_fragment_alpha(Default::default());
    (stencil == Default::default() && alpha == Default::default()).then_some(start)
}

fn direct_single_sample(frame: &RenderFrame) -> bool {
    !frame.uses_mesh_lod
        && !cfg!(any(
            feature = "command-state-diagnostics",
            feature = "vertex-output-diagnostics",
            feature = "raytrace-input-diagnostics",
            feature = "lit-only-command-diagnostics"
        ))
        && frame.raster.requested_samples <= 1
        && !frame.raster.alpha_writes
        && !frame.raster.fixed_function
        && frame.transparency() == crate::RenderTransparency::Unordered
        && frame.stencil_clear_value == 0
        && frame
            .layers
            .iter()
            .all(|layer| layer.stencil_boundaries.is_empty())
        && frame
            .draws
            .iter()
            .all(|draw| draw.capping_pass == crate::RenderCappingPass::None)
        && !frame.flip_output
        && frame.environment.is_none()
        && frame.colored_quad.is_none()
        && frame.shader_grid.is_none()
        && frame.shadow_maps.is_none()
        && frame.stereo_pair.is_none()
        && frame.raytrace.is_none()
        && frame.subview_blits.is_empty()
        && frame.gui_draw_start.is_none()
}

fn neutral_layer(layer: &RenderLayerSpan) -> bool {
    layer.draw_range.is_empty() && !layer.clear_depth
}

struct MainPassKey {
    extent: [u32; 2],
    clear: [u32; 4],
    layers: Vec<MainLayerKey>,
    draws: Vec<MainDrawKey>,
    lighting: MainLightingKey,
}

impl MainPassKey {
    fn from_frame(frame: &RenderFrame, start: usize) -> Option<Self> {
        let main = &frame.layers[..start];
        let end = main.last()?.draw_range.end;
        let prefix = &frame.draws[..end];
        let draws = prefix
            .iter()
            .map(MainDrawKey::capture)
            .collect::<Option<_>>()?;
        Some(Self {
            extent: frame.extent(),
            clear: frame.clear_color.map(f32::to_bits),
            layers: main.iter().map(MainLayerKey::capture).collect(),
            draws,
            lighting: MainLightingKey::capture(prefix)?,
        })
    }

    fn matches(&self, frame: &RenderFrame, start: usize) -> bool {
        let layers = &frame.layers[..start];
        if self.extent != frame.extent()
            || self.clear != frame.clear_color.map(f32::to_bits)
            || self.layers.len() != start
            || !self
                .layers
                .iter()
                .zip(layers)
                .all(|(key, layer)| *key == MainLayerKey::capture(layer))
        {
            return false;
        }
        let draws = &frame.draws[..layers[start - 1].draw_range.end];
        self.draws.len() == draws.len()
            && self
                .draws
                .iter()
                .zip(draws)
                .all(|(key, draw)| key.matches(draw))
            && self.lighting.matches(draws)
    }
}

#[derive(PartialEq)]
struct MainLayerKey {
    draw_count: usize,
    depth: (bool, bool, bool),
    offset: (i32, u32, u32),
}

impl MainLayerKey {
    fn capture(layer: &RenderLayerSpan) -> Self {
        Self {
            draw_count: layer.draw_range.len(),
            depth: layer.depth_policy(),
            offset: offset_bits(layer.polygon_offset),
        }
    }
}

struct MainDrawKey {
    vertices: StreamWitness<crate::RenderVertex>,
    indices: StreamWitness<u32>,
    native_flat_sources: StreamWitness<u32>,
    clip: [u32; 16],
    projection: Option<ProjectionBits>,
    fixed_projection: Option<ProjectionBits>,
    offset: Option<(i32, u32, u32)>,
    culling: crate::RenderFaceCulling,
    mirrored: bool,
    color_array: bool,
}

type ProjectionBits = ([[u32; 16]; 3], bool);

impl MainDrawKey {
    fn capture(draw: &PrimitiveDraw) -> Option<Self> {
        if !plain_triangle(draw) {
            return None;
        }
        Some(Self {
            vertices: StreamWitness::capture(&draw.vertices),
            indices: StreamWitness::capture(&draw.indices),
            native_flat_sources: StreamWitness::capture(&draw.native_flat_sources),
            clip: draw.clip_transform.map(f32::to_bits),
            projection: draw.projection_factors.map(projection_bits),
            fixed_projection: draw.fixed_function_factors.map(projection_bits),
            offset: draw.polygon_offset.map(offset_bits),
            culling: draw.face_culling,
            mirrored: draw.mirrored,
            color_array: draw.source_color_array,
        })
    }

    fn matches(&self, draw: &PrimitiveDraw) -> bool {
        plain_triangle(draw)
            && self.vertices.matches(&draw.vertices)
            && self.indices.matches(&draw.indices)
            && self.native_flat_sources.matches(&draw.native_flat_sources)
            && self.clip == draw.clip_transform.map(f32::to_bits)
            && self.projection == draw.projection_factors.map(projection_bits)
            && self.fixed_projection == draw.fixed_function_factors.map(projection_bits)
            && self.offset == draw.polygon_offset.map(offset_bits)
            && self.culling == draw.face_culling
            && self.mirrored == draw.mirrored
            && self.color_array == draw.source_color_array
    }
}

/// Admit only states where the keyed inputs account for every main-pass raster command.
fn plain_triangle(draw: &PrimitiveDraw) -> bool {
    matches!(
        draw,
        PrimitiveDraw {
            clip_transform: _,
            projection_factors: _,
            fixed_function_factors: _,
            depth_clamp: false,
            face_culling: _,
            lighting: _,
            depth_pass: crate::depth_prepass::RenderDepthPass::Shaded,
            instances: None,
            body: _,
        }
    ) && matches!(&**draw, crate::draw::DrawBody {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output_target: None,
        projected_dash: None, vertices: _, indices: _, geometry_revision: _, mesh_lod: None,
        guard_band_source: None, guard_band_bounds: _, quad_corners,
        polygon_offset: _, mirrored: _,
        capping_pass: crate::RenderCappingPass::None, clipping: None, hatch_rows: None,
        fixed_function_lighting: false, fixed_function_mask: false,
        compatibility_color_target: false, native_material_cutoff: None, source_color_array: _,
        unlit_back_color: None, source_line_strip: false, native_flat_sources: _, annotation: false,
        stencil_rect: false, text_shadow_bias: false, text_logic_xor: false,
        text_background_blend: false, surface_blend: false, surface_cutoff: None,
        surface_forced_blend: false, surface_texture_binding_only: false, mesh_edges: None,
        polygon_edge_style: None, polygon_face_lines: false, polygon_points: false,
        fixed_grid_point: false, fixed_grid_point_unit12: false, fixed_grid_point_sprite: false,
        edge_background_fallback: false, line_style: None, textured_style: None, surface_texture: None,
        surface_texture_unavailable: None, point_style: None, user_program: None,
        pipeline_memo: _,
        upload_memo: _,
    } if quad_corners.is_empty())
}

/// Keep a pointer identity witness without owning otherwise dead geometry bytes.
struct StreamWitness<T>(Option<Weak<Vec<T>>>);

impl<T> StreamWitness<T> {
    fn capture(stream: &DrawStream<T>) -> Self {
        Self(stream.witness())
    }

    fn matches(&self, stream: &DrawStream<T>) -> bool {
        let address = self.0.as_ref().map(|weak| weak.as_ptr().addr());
        address == stream.identity().map(std::num::NonZeroUsize::get)
    }
}

fn projection_bits(factors: RenderProjectionFactors) -> ProjectionBits {
    (
        factors.matrices().map(|matrix| matrix.map(f32::to_bits)),
        factors.has_camera_isometry(),
    )
}

fn offset_bits(offset: RenderPolygonOffset) -> (i32, u32, u32) {
    let (mode, factor, units) = offset.parameters();
    (mode, factor.to_bits(), units.to_bits())
}

#[cfg(test)]
#[path = "immediate_main_lighting_tests.rs"]
mod lighting_tests;
