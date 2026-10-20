//! `getStdProgramPhong`: the program with per-pixel lighting (flat, PBR and shadow variants).
use super::geometry::{point_sprite_shading, prepare_geom_main_src};
use super::glsl;
use super::lighting::Graphic3dLightingOptions;
use super::program_parts::{
    Declarations, FRAG, VERT, VERT_FRAG, any, apply_oit_output, attach_with_geometry,
    clip_planes_fragment, new_program, program, set_alpha_test, set_limits, textures_are,
};
use super::snippets::{
    THE_FUNC_TRANSFORM_NORMAL_WORLD, THE_VARY_TEX_COORD_TRSF, THE_VERT_GL_POSITION,
};
use super::texts_shading::{PHONG_NORMAL_MAP, PHONG_SHADOW_VERT, PHONG_VERT_MAIN};
use super::unlit::final_color_src;
use super::version::text;
use super::{Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::graphic3d::{
    Graphic3dLightSet, Graphic3dShaderFlags as Flags, Graphic3dShaderProgram,
    Graphic3dTextureSetBits as TextureBits,
};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;

/// The lighting call of the fragment stage.
const PHONG_COMPUTE_LIGHT: &str =
    "computeLighting (normalize (Normal), normalize (View), PositionWorld, gl_FrontFacing)";

/// The variant of a Phong program (`theIsFlatNormal`, `theIsPBR`, `theNbShadowMaps`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Graphic3dPhongVariant {
    /// Compute the face normal instead of reading vertex normals (`theIsFlatNormal`).
    pub is_flat_normal: bool,
    /// The PBR pipeline (`theIsPBR`).
    pub is_pbr: bool,
    /// The number of shadow maps (`theNbShadowMaps`).
    pub nb_shadow_maps: i32,
}

/// The source pieces of the Phong program.
#[derive(Default)]
struct PhongParts {
    vert_extra_func: String,
    vert_extra_main: String,
    frag_prefix: String,
    frag_vert_color: String,
    frag_extra_main: String,
    frag_color: String,
    use_tex_color: bool,
}

/// Sends a warning to the default messenger.
fn warn(text: &str) {
    Message::send_warning_text(&AsciiString::from(text));
}

impl Graphic3dShaderManager {
    /// `getStdProgramPhong`: the per-pixel lighting program of `lights`, `bits` and `variant`;
    /// a null light set raises `Standard_NullObject`;
    /// `manager.get_std_program_phong(&lights, bits, Graphic3dPhongVariant::default())?`.
    pub fn get_std_program_phong(
        &self,
        lights: &Handle<Graphic3dLightSet>,
        bits: i32,
        variant: Graphic3dPhongVariant,
    ) -> Result<Handle<Graphic3dShaderProgram>, Failure> {
        self.snapshot()
            .std_program_phong(self, lights, bits, variant)
    }
}

impl Graphic3dShaderManagerFields {
    /// `getStdProgramPhong` on the fields.
    fn std_program_phong(
        &self,
        manager: &Graphic3dShaderManager,
        lights: &Handle<Graphic3dLightSet>,
        bits: i32,
        variant: Graphic3dPhongVariant,
    ) -> Result<Handle<Graphic3dShaderProgram>, Failure> {
        let is_flat = self.flat_normal(variant.is_flat_normal);
        let handle = new_program();
        program(&handle).set_pbr(variant.is_pbr); // should be set before defaultGlslVersion()
        let mut declarations = Declarations::default();
        let mut parts = PhongParts {
            frag_color: format!("\nvec4 getColor(void) {{ return {PHONG_COMPUTE_LIGHT}; }}"),
            ..PhongParts::default()
        };
        if any(bits, Flags::IS_POINT) {
            self.phong_point(&handle, bits, &mut parts, &mut declarations);
        } else if any(bits, Flags::TEXTURE_RGB) {
            self.phong_texture(
                &handle,
                bits,
                (is_flat, variant.is_pbr),
                &mut parts,
                &mut declarations,
            );
        }
        phong_vert_color(bits, &mut parts, &mut declarations);
        let (clip_planes, clip_code) = clip_planes_fragment(bits);
        parts.frag_extra_main.push_str(clip_code);
        apply_oit_output(&handle, bits);
        self.phong_normal(bits, is_flat, &mut parts, &mut declarations);
        phong_shadow_declarations(variant.nb_shadow_maps, &mut parts, &mut declarations);
        let options = Graphic3dLightingOptions {
            has_vert_color: !parts.frag_vert_color.is_empty(),
            is_pbr: variant.is_pbr,
            has_tex_color: parts.use_tex_color,
            nb_shadow_maps: variant.nb_shadow_maps,
        };
        let sources = phong_sources(
            bits,
            &parts,
            &mut declarations,
            |options| self.compute_lighting(lights, options),
            options,
        );
        let (nb_lights, sources) = sources;
        let name = phong_program_name(manager, lights, variant)?;
        self.default_glsl_version(&handle, &name, bits, is_flat);
        set_limits(&handle, (nb_lights, variant.nb_shadow_maps, clip_planes));
        set_alpha_test(&handle, bits);
        attach_with_geometry(&handle, sources, &declarations);
        Ok(handle)
    }

    /// The flat normal flag the API allows, with the native warnings.
    fn flat_normal(&self, requested: bool) -> bool {
        let is_flat = requested && self.has_flat_shading;
        if is_flat != requested {
            warn(
                "Warning: flat shading requires OpenGL ES 3.0+ or GL_OES_standard_derivatives extension",
            );
        } else if is_flat && self.to_reverse_dfdx_sign {
            warn(
                "Warning: applied workaround for GLSL flat shading normal computation using dFdx/dFdy on Adreno",
            );
        }
        is_flat
    }

    /// The point marker part: point size, sprite shading and texture colour.
    fn phong_point(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        parts: &mut PhongParts,
        declarations: &mut Declarations,
    ) {
        if self.set_point_size {
            parts
                .vert_extra_main
                .push_str("\n  gl_PointSize = occPointSize;");
        }
        if any(bits, Flags::POINT_SPRITE) {
            declarations.uniform("sampler2D occSamplerPointSprite", FRAG);
            parts.frag_color = point_sprite_shading(PHONG_COMPUTE_LIGHT, bits, self.use_red_alpha);
        }
        if any(bits, Flags::TEXTURE_RGB) && !any(bits, Flags::VERT_COLOR) {
            program(handle).set_texture_set_bits(TextureBits::BASE_COLOR.0);
            declarations.uniform("sampler2D occSamplerBaseColor", VERT);
            declarations.in_out("vec4 VertColor", VERT_FRAG);
            parts
                .vert_extra_main
                .push_str("\n  VertColor = occTexture2D (occSamplerBaseColor, occTexCoord.xy);");
            parts.frag_vert_color =
                "\nvec4 getVertColor(in bool theIsFront) { return VertColor; }".into();
        }
    }

    /// The texture part of a non-point program: the texture set of the material maps.
    fn phong_texture(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        (is_flat, is_pbr): (bool, bool),
        parts: &mut PhongParts,
        declarations: &mut Declarations,
    ) {
        parts.use_tex_color = true;
        declarations.uniform("sampler2D occSamplerBaseColor", FRAG);
        declarations.in_out("vec4 TexCoord", VERT_FRAG);
        parts.vert_extra_main.push_str(THE_VARY_TEX_COORD_TRSF);
        let mut texture_bits =
            TextureBits::BASE_COLOR.0 | TextureBits::OCCLUSION.0 | TextureBits::EMISSIVE.0;
        if is_pbr {
            texture_bits |= TextureBits::METALLIC_ROUGHNESS.0;
        }
        if textures_are(bits, Flags::TEXTURE_NORMAL) && !is_flat {
            if self.has_flat_shading {
                texture_bits |= TextureBits::NORMAL.0;
            } else {
                warn("Warning: ignoring Normal Map texture in GLSL due to hardware capabilities");
            }
        }
        program(handle).set_texture_set_bits(texture_bits);
    }

    /// The normal: computed from the derivatives for flat shading, else the vertex normal
    /// with an optional normal map.
    fn phong_normal(
        &self,
        bits: i32,
        is_flat: bool,
        parts: &mut PhongParts,
        declarations: &mut Declarations,
    ) {
        if is_flat {
            let sign = if self.to_reverse_dfdx_sign { "-" } else { "" };
            parts.frag_extra_main.push_str(&format!(
                "\n  Normal = {sign}normalize (cross (dFdx (PositionWorld.xyz / PositionWorld.w), dFdy (PositionWorld.xyz / PositionWorld.w)));\n  if (!gl_FrontFacing) {{ Normal = -Normal; }}"
            ));
            return;
        }
        declarations.in_out("vec3 vNormal", VERT_FRAG);
        parts
            .vert_extra_func
            .push_str(THE_FUNC_TRANSFORM_NORMAL_WORLD);
        parts
            .vert_extra_main
            .push_str("\n  vNormal = transformNormal (occNormal);");
        parts.frag_extra_main.push_str("\n  Normal = vNormal;");
        if !any(bits, Flags::IS_POINT)
            && textures_are(bits, Flags::TEXTURE_NORMAL)
            && self.has_flat_shading
        {
            parts.frag_prefix.push_str(glsl::TANGENT_SPACE_NORMAL);
            // apply normal map texture
            parts.frag_extra_main.push_str(PHONG_NORMAL_MAP);
        }
    }
}

/// The per-vertex colour (front faces only with `VertColorFrontOnly`).
fn phong_vert_color(bits: i32, parts: &mut PhongParts, declarations: &mut Declarations) {
    if !any(bits, Flags::VERT_COLOR) {
        return;
    }
    declarations.in_out("vec4 VertColor", VERT_FRAG);
    parts
        .vert_extra_main
        .push_str("\n  VertColor = occVertColor;");
    parts.frag_vert_color = if any(bits, Flags::VERT_COLOR_FRONT_ONLY) {
        "\nvec4 getVertColor(in bool theIsFront) { return theIsFront ? VertColor : vec4(1.0); }"
            .into()
    } else {
        "\nvec4 getVertColor(in bool theIsFront) { return VertColor; }".into()
    };
}

/// The position and view in/outs, and the shadow map declarations and light-space positions.
fn phong_shadow_declarations(
    nb_shadow_maps: i32,
    parts: &mut PhongParts,
    declarations: &mut Declarations,
) {
    declarations.in_out("vec4 PositionWorld", VERT_FRAG);
    declarations.in_out("vec3 View", VERT_FRAG);
    if nb_shadow_maps <= 0 {
        return;
    }
    declarations.uniform("mat4      occShadowMapMatrices[THE_NB_SHADOWMAPS]", VERT);
    declarations.uniform("sampler2D occShadowMapSamplers[THE_NB_SHADOWMAPS]", FRAG);
    declarations.uniform("vec2      occShadowMapSizeBias", FRAG);
    declarations.in_out("vec4 PosLightSpace[THE_NB_SHADOWMAPS]", VERT_FRAG);
    parts.vert_extra_main.push_str(PHONG_SHADOW_VERT);
}

/// The vertex, geometry and fragment sources; the lighting is computed after the geometry
/// stage, as natively. Returns the number of lights too.
fn phong_sources(
    bits: i32,
    parts: &PhongParts,
    declarations: &mut Declarations,
    lighting: impl FnOnce(Graphic3dLightingOptions) -> (i32, String),
    options: Graphic3dLightingOptions,
) -> (i32, [String; 3]) {
    let vertex = format!(
        "{}{PHONG_VERT_MAIN}{}{THE_VERT_GL_POSITION}\n}}",
        parts.vert_extra_func, parts.vert_extra_main
    );
    let geometry = prepare_geom_main_src(declarations, bits);
    let (nb_lights, lights) = lighting(options);
    let fragment = format!(
        "{}\n{}\nvec3  Normal;{lights}{}{}\n\nvoid main()\n{{\n  if (occFragEarlyReturn()) {{ return; }}{}\n  occSetFragColor (getFinalColor());\n}}",
        parts.frag_prefix,
        parts.frag_vert_color,
        parts.frag_color,
        final_color_src(bits),
        parts.frag_extra_main
    );
    (nb_lights, [vertex, geometry, fragment])
}

/// The program name `flat-`/`phong-`, `pbr-`, the light key and `-`.
fn phong_program_name(
    manager: &Graphic3dShaderManager,
    lights: &Handle<Graphic3dLightSet>,
    variant: Graphic3dPhongVariant,
) -> Result<String, Failure> {
    let kind = if variant.is_flat_normal {
        "flat-"
    } else {
        "phong-"
    };
    let pbr = if variant.is_pbr { "pbr-" } else { "" };
    let key = manager.gen_light_key(lights, variant.nb_shadow_maps > 0)?;
    Ok(format!("{kind}{pbr}{}-", text(&key)))
}
