//! `stdComputeLighting`: the `computeLighting` GLSL function of a light configuration.
use super::glsl;
use super::snippets::{
    THE_FUNC_DIRECTIONAL_LIGHT_FIRST, THE_FUNC_LIGHT_DEF, THE_FUNC_PBR_LIGHT_DEF,
};
use super::texts_shading::{
    PBR_LIGHTING_HEAD, PBR_LIGHTING_MATERIAL, PBR_LIGHTING_TAIL, PHONG_LIGHTING_HEAD,
    PHONG_LIGHTING_MATERIAL, PHONG_LIGHTING_TAIL, PHONG_LIGHTING_TEXTURE,
};
use super::version::{THE_NB_UNROLLED_LIGHTS_MAX, round_up_max_light_sources};
use super::{Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::graphic3d::{
    Graphic3dLightSet, Graphic3dLightSetIterationFilter, Graphic3dLightSetIterator,
    Graphic3dTypeOfLightSource as Light,
};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;

/// The lighting options of [`Graphic3dShaderManager::std_compute_lighting`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Graphic3dLightingOptions {
    /// Multiply by `getVertColor()` (`theHasVertColor`).
    pub has_vert_color: bool,
    /// The PBR pipeline (`theIsPBR`).
    pub is_pbr: bool,
    /// Multiply by the base colour texture (`theHasTexColor`).
    pub has_tex_color: bool,
    /// The number of shadow maps (`theNbShadowMaps`).
    pub nb_shadow_maps: i32,
}

/// The shadow lookup argument of light `index`.
fn shadow_arg(index: i32) -> String {
    format!("occLightShadow (occShadowMapSamplers[{index}], {index}, theNormal)")
}

/// The unrolled call of light `index` of `kind`, or `None` for an ambient light.
fn unrolled_call(kind: Light, index: i32, casts_shadow: bool) -> Option<String> {
    let call = match (kind, casts_shadow) {
        (Light::Ambient, _) => return None,
        (Light::Directional, true) => format!(
            "\n    occDirectionalLight ({index}, theNormal, theView, theIsFront,\n                         {});",
            shadow_arg(index)
        ),
        (Light::Directional, false) => {
            format!("\n    occDirectionalLight ({index}, theNormal, theView, theIsFront, 1.0);")
        }
        (Light::Positional, _) => {
            format!("\n    occPointLight ({index}, theNormal, theView, aPoint, theIsFront);")
        }
        (Light::Spot, true) => format!(
            "\n    occSpotLight ({index}, theNormal, theView, aPoint, theIsFront,\n                  {});",
            shadow_arg(index)
        ),
        (Light::Spot, false) => {
            format!("\n    occSpotLight ({index}, theNormal, theView, aPoint, theIsFront, 1.0);")
        }
    };
    Some(call)
}

/// The unrolled light calls of the enabled non-ambient lights.
fn unrolled_loop(lights: &Handle<Graphic3dLightSet>, nb_shadow_maps: i32) -> String {
    let filter = Graphic3dLightSetIterationFilter::ExcludeDisabledAndAmbient;
    let mut cursor = Graphic3dLightSetIterator::from_handle(lights, filter);
    let (mut source, mut index) = (String::new(), 0);
    while cursor.more() {
        if let Ok(light) = cursor.value()
            && let Some(light) = light.get()
        {
            let shadow = nb_shadow_maps > 0 && light.to_cast_shadows();
            if let Some(call) = unrolled_call(light.light_type(), index, shadow) {
                source.push_str(&call);
                index += 1;
            }
        }
        cursor.next();
    }
    source
}

/// The branches of the light-type loop over more than 32 lights.
const LOOP_BRANCHES: [(Light, &str); 3] = [
    (
        Light::Directional,
        "\n      if (aType == OccLightType_Direct)\n      {\n        occDirectionalLight (anIndex, theNormal, theView, theIsFront, 1.0);\n      }",
    ),
    (
        Light::Positional,
        "\n      if (aType == OccLightType_Point)\n      {\n        occPointLight (anIndex, theNormal, theView, aPoint, theIsFront);\n      }",
    ),
    (
        Light::Spot,
        "\n      if (aType == OccLightType_Spot)\n      {\n        occSpotLight (anIndex, theNormal, theView, aPoint, theIsFront, 1.0);\n      }",
    ),
];

/// The loop over the light array, with one branch per enabled light type.
fn typed_loop(set: &Graphic3dLightSet) -> String {
    let mut source = String::from(
        "\n    for (int anIndex = 0; anIndex < occLightSourcesCount; ++anIndex)\n    {\n      int aType = occLight_Type (anIndex);",
    );
    let mut is_first = true;
    for (kind, branch) in LOOP_BRANCHES {
        if set.nb_enabled_lights_of_type(kind) > 0 {
            if !is_first {
                source.push_str("\n      else ");
            }
            is_first = false;
            source.push_str(branch);
        }
    }
    source.push_str("\n    }");
    source
}

/// The light functions of the enabled light types and the loop (the hard-coded first
/// directional light for a single Phong directional light without shadows).
fn light_functions(
    set: &Graphic3dLightSet,
    nb_lights: i32,
    options: Graphic3dLightingOptions,
    mut lights_loop: String,
) -> (String, String) {
    let mut functions = String::new();
    if options.is_pbr {
        for table in [
            glsl::PBR_DISTRIBUTION,
            glsl::PBR_GEOMETRY,
            glsl::PBR_FRESNEL,
            glsl::PBR_COOK_TORRANCE,
            glsl::PBR_ILLUMINATION,
        ] {
            functions.push_str(table);
        }
    }
    let shadows = options.nb_shadow_maps > 0;
    let mut is_shadow_added = false;
    let directional = set.nb_enabled_lights_of_type(Light::Directional);
    if directional == 1 && nb_lights == 1 && !options.is_pbr && !shadows {
        // use the version with hard-coded first index
        lights_loop = "\n    directionalLightFirst(theNormal, theView, theIsFront, 1.0);".into();
        functions.push_str(THE_FUNC_DIRECTIONAL_LIGHT_FIRST);
    } else if directional > 0 {
        if shadows {
            functions.push_str(glsl::LIGHT_SHADOW);
            is_shadow_added = true;
        }
        functions.push_str(if options.is_pbr {
            glsl::PBR_DIRECTIONAL_LIGHT
        } else {
            glsl::PHONG_DIRECTIONAL_LIGHT
        });
    }
    if set.nb_enabled_lights_of_type(Light::Positional) > 0 {
        functions.push_str(if options.is_pbr {
            glsl::PBR_POINT_LIGHT
        } else {
            glsl::PHONG_POINT_LIGHT
        });
    }
    if set.nb_enabled_lights_of_type(Light::Spot) > 0 {
        if shadows && !is_shadow_added {
            functions.push_str(glsl::LIGHT_SHADOW);
        }
        functions.push_str(if options.is_pbr {
            glsl::PBR_SPOT_LIGHT
        } else {
            glsl::PHONG_SPOT_LIGHT
        });
    }
    (functions, lights_loop)
}

/// The light functions, the loop and the number of lights of a (possibly null) light set.
fn lights_parts(
    lights: &Handle<Graphic3dLightSet>,
    options: Graphic3dLightingOptions,
) -> (i32, String, String) {
    let Some(set) = lights.get() else {
        return (0, String::new(), String::new());
    };
    let mut nb_lights = set.nb_enabled();
    let lights_loop = if nb_lights <= THE_NB_UNROLLED_LIGHTS_MAX {
        unrolled_loop(lights, options.nb_shadow_maps)
    } else {
        nb_lights = round_up_max_light_sources(nb_lights);
        typed_loop(set)
    };
    let (functions, lights_loop) = light_functions(set, nb_lights, options, lights_loop);
    (nb_lights, functions, lights_loop)
}

/// The Phong `computeLighting` around the light functions and loop.
fn phong_lighting(functions: &str, lights_loop: &str, options: Graphic3dLightingOptions) -> String {
    let vert_color = if options.has_vert_color {
        "\n  aColor *= getVertColor(theIsFront);"
    } else {
        ""
    };
    let texture = if options.has_tex_color {
        PHONG_LIGHTING_TEXTURE
    } else {
        ""
    };
    format!(
        "{THE_FUNC_LIGHT_DEF}{}{functions}{PHONG_LIGHTING_HEAD}{lights_loop}{PHONG_LIGHTING_MATERIAL}{vert_color}{texture}{PHONG_LIGHTING_TAIL}",
        glsl::POINT_LIGHT_ATTENUATION
    )
}

/// The PBR `computeLighting` around the light functions and loop.
fn pbr_lighting(functions: &str, lights_loop: &str, options: Graphic3dLightingOptions) -> String {
    let vert_color = if options.has_vert_color {
        " * getVertColor(theIsFront)"
    } else {
        ""
    };
    format!(
        "{THE_FUNC_PBR_LIGHT_DEF}{}{functions}{PBR_LIGHTING_HEAD}{vert_color};{PBR_LIGHTING_MATERIAL}{lights_loop}{PBR_LIGHTING_TAIL}",
        glsl::POINT_LIGHT_ATTENUATION
    )
}

impl Graphic3dShaderManager {
    /// `stdComputeLighting`: the number of lights and the light declarations, functions and
    /// `computeLighting` of `lights` (none for a null set);
    /// `let (nb_lights, source) = manager.std_compute_lighting(&lights, options)`.
    pub fn std_compute_lighting(
        &self,
        lights: &Handle<Graphic3dLightSet>,
        options: Graphic3dLightingOptions,
    ) -> (i32, AsciiString) {
        let (nb_lights, source) = self.snapshot().compute_lighting(lights, options);
        (nb_lights, AsciiString::from(source.as_str()))
    }
}

impl Graphic3dShaderManagerFields {
    /// `stdComputeLighting` on the fields.
    pub(super) fn compute_lighting(
        &self,
        lights: &Handle<Graphic3dLightSet>,
        options: Graphic3dLightingOptions,
    ) -> (i32, String) {
        let (nb_lights, functions, lights_loop) = lights_parts(lights, options);
        let source = if options.is_pbr {
            pbr_lighting(&functions, &lights_loop, options)
        } else {
            phong_lighting(&functions, &lights_loop, options)
        };
        (nb_lights, source)
    }
}
