//! GlView::updateRaytraceLightSources: the global ambient and the
//! `(Emission, Position)` pairs of uRaytraceLightSrcTexture, positional lights first.
use martensite_tkernel::ncollection::{Mat4, Vec4};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dCLight, Graphic3dLightSet, Graphic3dTypeOfLightSource, Graphic3dTypeOfShadingModel,
};

/// The light sources of one frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RaytraceLights {
    /// `uGlobalAmbient`: the set's ambient colour (w = 0), zero when unlit.
    pub ambient: [f32; 4],
    /// `GlRaytraceLight::Packed`: emission then position of each light.
    pub sources: Vec<[[f32; 4]; 2]>,
}

/// The enabled non-ambient lights, positional before directional, each in set order.
fn ordered_lights(set: &Graphic3dLightSet) -> Vec<Handle<Graphic3dCLight>> {
    let active: Vec<_> = set
        .lights()
        .into_iter()
        .filter(|light| {
            light.is_enabled() && light.light_type() != Graphic3dTypeOfLightSource::Ambient
        })
        .collect();
    let directional = |light: &Handle<Graphic3dCLight>| {
        light.light_type() == Graphic3dTypeOfLightSource::Directional
    };
    let (positional, distant): (Vec<_>, Vec<_>) = active.into_iter().partition(|l| !directional(l));
    positional.into_iter().chain(distant).collect()
}

/// One light: intensity-scaled colour with the smoothness (an angle's cosine for directional
/// lights) in w, and the position (w = 1) or the direction towards the light (w = 0), moved by
/// `inverse_model_view` for headlights.
fn packed_light(light: &Graphic3dCLight, inverse_model_view: &Mat4<f32>) -> [[f32; 4]; 2] {
    let color = light.packed_color();
    let intensity = light.intensity();
    let mut emission = [
        color.x * intensity,
        color.y * intensity,
        color.z * intensity,
        1.,
    ];
    let direction = light.packed_direction_range();
    let mut position = Vec4::new(-direction.x, -direction.y, -direction.z, 0.);
    if light.light_type() != Graphic3dTypeOfLightSource::Directional {
        let point = light.position();
        position = Vec4::new(point.x() as f32, point.y() as f32, point.z() as f32, 1.);
        emission[3] = std_max(light.smoothness(), 0.);
    } else {
        let angle = std_max(light.smoothness(), 0.);
        // std::min(angle, M_PI / 2): a NaN angle stays NaN, as natively.
        let bounded = if std::f32::consts::FRAC_PI_2 < angle {
            std::f32::consts::FRAC_PI_2
        } else {
            angle
        };
        emission[3] = martensite_cmath::cos(f64::from(bounded)) as f32;
    }
    if light.is_headlight() {
        position = *inverse_model_view * position;
    }
    [emission, position.data()]
}

/// `std::max(value, floor)`: `value < floor ? floor : value`, which keeps a NaN value.
fn std_max(value: f32, floor: f32) -> f32 {
    if value < floor { floor } else { value }
}

/// The frame's lights; `inverse_model_view` is the inverted camera orientation (float).
/// Example: `raytrace_lights(&set, Graphic3dTypeOfShadingModel::Phong, &inverse)`.
pub(crate) fn raytrace_lights(
    set: Option<&Graphic3dLightSet>,
    shading: Graphic3dTypeOfShadingModel,
    inverse_model_view: &Mat4<f32>,
) -> RaytraceLights {
    // Unlit shading takes neither the ambient nor any light source.
    let Some(set) = set.filter(|_| shading != Graphic3dTypeOfShadingModel::Unlit) else {
        return RaytraceLights::default();
    };
    let ambient = set.ambient_color();
    let ordered = ordered_lights(set);
    RaytraceLights {
        ambient: [ambient.x, ambient.y, ambient.z, 0.],
        sources: ordered
            .iter()
            .map(|light| packed_light(light, inverse_model_view))
            .collect(),
    }
}
