//! The lighting part of the retained main-pass key: one world view, its lights and each lit
//! draw's material and transforms.
use super::StreamWitness;
use crate::{PrimitiveDraw, RenderLightSet, RenderLighting, RenderMaterial, RenderShading};
use std::collections::HashMap;

// Ordinary lit structures share a world-view and its transformed headlights. A different
// effective view or light set within the same main pass uses the full renderer.
#[derive(Default)]
pub(super) struct MainLightingKey {
    pub(super) view: Option<MainViewLights>,
    pub(super) materials: Vec<MaterialBits>,
    pub(super) draws: Vec<MainLitDraw>,
}

type MaterialBits = [[u32; 4]; 8];

impl MainLightingKey {
    pub(super) fn capture(draws: &[PrimitiveDraw]) -> Option<Self> {
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

    pub(super) fn matches(&self, draws: &[PrimitiveDraw]) -> bool {
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

pub(super) struct MainViewLights {
    view: [u32; 16],
    eye: [u32; 4],
    pub(super) lights: RenderLightSet,
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

pub(super) struct MainLitDraw {
    draw: usize,
    material: usize,
    pub(super) normals: StreamWitness<[f32; 3]>,
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
