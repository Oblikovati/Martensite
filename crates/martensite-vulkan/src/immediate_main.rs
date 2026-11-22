//! Preserve a completed main pass only while its effective input matches exactly.
use crate::{
    PrimitiveDraw, RenderFrame, RenderLayerSpan, RenderLightSet, RenderLighting, RenderMaterial,
    RenderPolygonOffset, RenderProjectionFactors, RenderShading,
    backend::immediate_main_record::MainLayerTicket, draw::DrawStream,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, PoisonError, Weak},
};

#[derive(Default)]
pub(crate) struct ImmediateMainCache(Mutex<Option<RetainedMain>>);

#[derive(Clone)]
struct RetainedMain {
    key: Arc<MainPassKey>,
    ticket: MainLayerTicket,
}

#[derive(Clone)]
pub(crate) enum ImmediateMainPlan {
    Remember(MainSeed),
    Restore {
        start: usize,
        ticket: MainLayerTicket,
        fallback: MainSeed,
    },
}

#[derive(Clone)]
pub(crate) struct MainSeed {
    pub(crate) snapshot_start: Option<usize>,
    cache: Arc<ImmediateMainCache>,
    key: Arc<MainPassKey>,
}

impl std::fmt::Debug for ImmediateMainPlan {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Remember(_) => output.write_str("RememberMain"),
            Self::Restore { start, .. } => output.debug_tuple("RestoreMain").field(start).finish(),
        }
    }
}

impl ImmediateMainCache {
    pub(crate) fn discard(&self) {
        let retired = self.0.lock().unwrap_or_else(PoisonError::into_inner).take();
        drop(retired);
    }

    pub(crate) fn plan(self: &Arc<Self>, frame: &RenderFrame) -> Option<ImmediateMainPlan> {
        let start = eligible_start(frame)?;
        let neutral = frame.layers[start..].iter().all(neutral_layer);
        let active = self
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let snapshot_start = (!neutral).then_some(start);
        if let Some(main) = active.filter(|main| main.key.matches(frame, start)) {
            return Some(ImmediateMainPlan::Restore {
                start,
                ticket: main.ticket,
                fallback: self.seed(main.key, snapshot_start),
            });
        }
        let key = MainPassKey::from_frame(frame, start)?;
        Some(ImmediateMainPlan::Remember(
            self.seed(Arc::new(key), snapshot_start),
        ))
    }

    fn seed(self: &Arc<Self>, key: Arc<MainPassKey>, snapshot_start: Option<usize>) -> MainSeed {
        MainSeed {
            snapshot_start,
            cache: self.clone(),
            key,
        }
    }
}

impl MainSeed {
    pub(crate) fn remember(&self, ticket: MainLayerTicket) {
        let retained = RetainedMain {
            key: self.key.clone(),
            ticket,
        };
        let retired = self
            .cache
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .replace(retained);
        drop(retired);
    }
}

impl ImmediateMainPlan {
    pub(crate) fn restore(&self) -> Option<(usize, &MainLayerTicket)> {
        match self {
            Self::Restore { start, ticket, .. } => Some((*start, ticket)),
            Self::Remember(_) => None,
        }
    }

    pub(crate) fn seed(&self) -> &MainSeed {
        match self {
            Self::Remember(seed) | Self::Restore { fallback: seed, .. } => seed,
        }
    }
}

fn eligible_start(frame: &RenderFrame) -> Option<usize> {
    let start = frame.immediate_start?;
    if start == 0 || start > frame.layers.len() || !direct_single_sample(frame) {
        return None;
    }
    let stencil = frame.initial_stencil(Default::default());
    let alpha = frame.initial_fragment_alpha(Default::default());
    (stencil == Default::default() && alpha == Default::default()).then_some(start)
}

fn direct_single_sample(frame: &RenderFrame) -> bool {
    !cfg!(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "raytrace-input-diagnostics",
        feature = "lit-only-command-diagnostics"
    )) && frame.raster.requested_samples <= 1
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
    matches!(draw, PrimitiveDraw {
        #[cfg(feature = "vertex-output-diagnostics")]
        vertex_output_target: None,
        projected_dash: None, vertices: _, indices: _, geometry_revision: _,
        guard_band_source: None, guard_band_bounds: _, quad_corners,
        clip_transform: _, projection_factors: _, fixed_function_factors: _, depth_clamp: false,
        polygon_offset: _, mirrored: _, face_culling: _,
        capping_pass: crate::RenderCappingPass::None, clipping: None, hatch_rows: None,
        lighting: _, fixed_function_lighting: false, fixed_function_mask: false,
        compatibility_color_target: false, native_material_cutoff: None, source_color_array: _,
        unlit_back_color: None, source_line_strip: false, native_flat_sources: _, annotation: false,
        stencil_rect: false, text_shadow_bias: false, text_logic_xor: false,
        text_background_blend: false, surface_blend: false, surface_cutoff: None,
        surface_forced_blend: false, surface_texture_binding_only: false, mesh_edges: None,
        polygon_edge_style: None, polygon_face_lines: false, polygon_points: false,
        fixed_grid_point: false, fixed_grid_point_unit12: false, fixed_grid_point_sprite: false,
        edge_background_fallback: false, line_style: None, textured_style: None, surface_texture: None,
        surface_texture_unavailable: None, point_style: None,
        depth_pass: crate::depth_prepass::RenderDepthPass::Shaded, user_program: None,
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

// Ordinary lit structures share a world-view and its transformed headlights. A different
// effective view or light set within the same main pass uses the full renderer.
#[derive(Default)]
struct MainLightingKey {
    view: Option<MainViewLights>,
    materials: Vec<MaterialBits>,
    draws: Vec<MainLitDraw>,
}

type MaterialBits = [[u32; 4]; 8];

impl MainLightingKey {
    fn capture(draws: &[PrimitiveDraw]) -> Option<Self> {
        let mut key = Self::default();
        let mut materials = HashMap::new();
        for (index, lighting) in lit_draws(draws) {
            if !key.accepts_view(lighting) {
                return None;
            }
            let bits = material_bits(&lighting.material);
            let material = *materials.entry(bits).or_insert_with(|| {
                key.materials.push(bits);
                key.materials.len() - 1
            });
            key.draws
                .push(MainLitDraw::capture(index, material, lighting));
        }
        Some(key)
    }

    fn accepts_view(&mut self, lighting: &RenderLighting) -> bool {
        if !programmable_phong(lighting) {
            return false;
        }
        self.view
            .get_or_insert_with(|| MainViewLights::capture(lighting))
            .matches(lighting)
    }

    fn matches(&self, draws: &[PrimitiveDraw]) -> bool {
        let mut current = lit_draws(draws);
        for key in &self.draws {
            let Some((index, lighting)) = current.next() else {
                return false;
            };
            if !programmable_phong(lighting)
                || !self
                    .view
                    .as_ref()
                    .is_some_and(|view| view.matches(lighting))
                || !key.matches(index, lighting)
                || self.materials[key.material] != material_bits(&lighting.material)
            {
                return false;
            }
        }
        current.next().is_none()
    }
}

fn lit_draws(draws: &[PrimitiveDraw]) -> impl Iterator<Item = (usize, &RenderLighting)> {
    draws
        .iter()
        .enumerate()
        .filter_map(|(index, draw)| draw.lighting().map(|lighting| (index, lighting)))
}

fn programmable_phong(lighting: &RenderLighting) -> bool {
    !lighting.pbr && lighting.fixed.is_none()
}

struct MainViewLights {
    view: [u32; 16],
    eye: [u32; 4],
    lights: RenderLightSet,
}

impl MainViewLights {
    fn capture(lighting: &RenderLighting) -> Self {
        Self {
            view: lighting.view.map(f32::to_bits),
            eye: lighting.eye.map(f32::to_bits),
            lights: lighting.lights.clone(),
        }
    }

    fn matches(&self, lighting: &RenderLighting) -> bool {
        self.view == lighting.view.map(f32::to_bits)
            && self.eye == lighting.eye.map(f32::to_bits)
            && light_set_matches(&self.lights, &lighting.lights)
    }
}

fn light_set_matches(left: &RenderLightSet, right: &RenderLightSet) -> bool {
    left.ambient.map(f32::to_bits) == right.ambient.map(f32::to_bits)
        && left.sources.len() == right.sources.len()
        && left.sources.iter().zip(right.sources.iter()).all(|(a, b)| {
            a.kind == b.kind
                && a.cast_shadows == b.cast_shadows
                && a.color.map(f32::to_bits) == b.color.map(f32::to_bits)
                && a.position.map(f32::to_bits) == b.position.map(f32::to_bits)
                && a.direction.map(f32::to_bits) == b.direction.map(f32::to_bits)
                && a.params.map(f32::to_bits) == b.params.map(f32::to_bits)
        })
}

struct MainLitDraw {
    draw: usize,
    material: usize,
    normals: StreamWitness<[f32; 3]>,
    world: [u32; 16],
    normal: [u32; 16],
    fixed_source_opacity: Option<[u32; 2]>,
    shading: RenderShading,
    flags: [bool; 3],
}

impl MainLitDraw {
    fn capture(draw: usize, material: usize, lighting: &RenderLighting) -> Self {
        Self {
            draw,
            material,
            normals: StreamWitness::capture(&lighting.normals),
            world: lighting.world.map(f32::to_bits),
            normal: lighting.normal.map(f32::to_bits),
            fixed_source_opacity: lighting
                .fixed_source_opacity
                .map(|values| values.map(f32::to_bits)),
            shading: lighting.shading,
            flags: lighting_flags(lighting),
        }
    }

    fn matches(&self, draw: usize, lighting: &RenderLighting) -> bool {
        self.draw == draw
            && self.normals.matches(&lighting.normals)
            && self.world == lighting.world.map(f32::to_bits)
            && self.normal == lighting.normal.map(f32::to_bits)
            && self.fixed_source_opacity
                == lighting
                    .fixed_source_opacity
                    .map(|values| values.map(f32::to_bits))
            && self.shading == lighting.shading
            && self.flags == lighting_flags(lighting)
    }
}

fn material_bits(material: &RenderMaterial) -> MaterialBits {
    material.packed_common().map(|slot| slot.map(f32::to_bits))
}

fn lighting_flags(lighting: &RenderLighting) -> [bool; 3] {
    [
        lighting.vertex_colors,
        lighting.front_vertex_colors_only,
        lighting.normalize_fixed,
    ]
}

#[cfg(test)]
#[path = "immediate_main_lighting_tests.rs"]
mod lighting_tests;
