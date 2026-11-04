//! World-space light records mirror GlShaderManager::PushLightSourceState.
use crate::{RenderError, error::rejected};
use martensite_tkernel::{
    ncollection::{Mat4, Vec4},
    standard::Handle,
};
use martensite_tkservice::graphic3d::*;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PackedRenderLight {
    pub kind: u32,
    /// Graphic3dCLight::ToCastShadows; sampled only when the view renders shadow maps.
    pub cast_shadows: bool,
    pub color: [f32; 4],
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub params: [f32; 4],
}
/// Immutable enabled lights in world space; `RenderLightSet::snapshot(&lights, inverse_view)?`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderLightSet {
    pub(crate) ambient: [f32; 4],
    pub(crate) sources: Vec<PackedRenderLight>,
}
impl RenderLightSet {
    // GlView::render replaces sources for an Unlit view,
    // even when individual aspects request a lit shader.
    // `local_origin` is the Z layer origin the lights render around (GlShaderManager's
    // myLocalOrigin while GlLayerList::renderLayer draws a layer with a local origin).
    pub(crate) fn for_view(
        shading: Graphic3dTypeOfShadingModel,
        lights: &Handle<Graphic3dLightSet>,
        inverse_view: [f32; 16],
        local_origin: Option<[f64; 3]>,
    ) -> Result<Self, RenderError> {
        if shading == Graphic3dTypeOfShadingModel::Unlit {
            return Ok(Self {
                ambient: [1.; 4],
                sources: Vec::new(),
            });
        }
        Self::snapshot_around(lights, inverse_view, local_origin.unwrap_or_default())
    }

    /// Read live lights and transform headlights through the inverse view; `RenderLightSet::snapshot(&lights, inverse_view)?`.
    pub fn snapshot(
        lights: &Handle<Graphic3dLightSet>,
        inverse_view: [f32; 16],
    ) -> Result<Self, RenderError> {
        Self::snapshot_around(lights, inverse_view, [0.; 3])
    }

    /// `snapshot` with world positions of positional and spot lights that are not headlights
    /// taken relative to `origin`, as pushLightSourceState subtracts the layer's local origin
    /// in double before the float cast; `RenderLightSet::snapshot_around(&lights, inverse, [0.; 3])?`.
    pub(crate) fn snapshot_around(
        lights: &Handle<Graphic3dLightSet>,
        inverse_view: [f32; 16],
        origin: [f64; 3],
    ) -> Result<Self, RenderError> {
        if inverse_view.iter().any(|value| !value.is_finite()) {
            return Err(rejected(inverse_view, "a finite inverse world-view matrix"));
        }
        let Some(lights) = lights.get() else {
            return Ok(Self::default());
        };
        lights.update_revision();
        let inverse = Mat4::from_data(inverse_view);
        let sources = lights
            .lights()
            .iter()
            .filter(|light| {
                light.is_enabled() && light.light_type() != Graphic3dTypeOfLightSource::Ambient
            })
            .map(|light| packed_light(light, &inverse, origin))
            .collect();
        Ok(Self {
            ambient: lights.ambient_color().data(),
            sources,
        })
    }
    /// Read the summed enabled ambient RGB, including intensity; `lights.ambient_color()`.
    pub fn ambient_color(&self) -> [f32; 4] {
        self.ambient
    }
    /// Count enabled non-ambient sources; `lights.source_count()`.
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }
}
fn packed_light(
    light: &Graphic3dCLight,
    inverse: &Mat4<f32>,
    origin: [f64; 3],
) -> PackedRenderLight {
    let kind = light.light_type();
    let mut color = light.packed_color().data();
    color[3] = light.intensity(); // Legacy Phong ignores this channel; PBR consumes it.
    let direction = light.packed_direction().data();
    let mut packed = PackedRenderLight {
        kind: kind as u32,
        cast_shadows: light.to_cast_shadows(),
        color,
        position: [0.; 4],
        direction: [0.; 4],
        params: light.packed_params().data(),
    };
    if kind == Graphic3dTypeOfLightSource::Directional {
        packed.position =
            world_direction(direction.map(|value| -value), light.is_headlight(), inverse);
        return packed;
    }
    if kind == Graphic3dTypeOfLightSource::Spot {
        packed.direction = world_direction(direction, light.is_headlight(), inverse);
    }
    packed.direction[3] = light.range();
    packed.position = world_position(light, inverse, origin);
    packed
}
fn world_direction(direction: [f32; 3], headlight: bool, inverse: &Mat4<f32>) -> [f32; 4] {
    let [x, y, z] = direction;
    if !headlight {
        return [x, y, z, 0.];
    }
    let transformed = *inverse * Vec4::new(x, y, z, 0.);
    let length = (transformed.x() * transformed.x()
        + transformed.y() * transformed.y()
        + transformed.z() * transformed.z())
    .sqrt();
    [
        transformed.x() / length,
        transformed.y() / length,
        transformed.z() / length,
        0.,
    ]
}
fn world_position(light: &Graphic3dCLight, inverse: &Mat4<f32>, origin: [f64; 3]) -> [f32; 4] {
    let world = light.position_coords();
    if light.is_headlight() {
        let [x, y, z] = world.map(|value| value as f32);
        return (*inverse * Vec4::new(x, y, z, 1.)).data();
    }
    let [x, y, z] = [0, 1, 2].map(|k| (world[k] - origin[k]) as f32);
    [x, y, z, 0.]
}

#[cfg(test)]
#[path = "light_snapshot_tests.rs"]
mod tests;
