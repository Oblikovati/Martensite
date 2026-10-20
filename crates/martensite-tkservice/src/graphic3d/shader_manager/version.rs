//! Program ids, GLSL version headers and the light configuration key.
use super::program_parts::{any, program, textures_are};
use super::{Graphic3dGlslExtension as Ext, Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::aspect::AspectGraphicsLibrary;
use crate::graphic3d::{Graphic3dLightSet, Graphic3dShaderFlags as Flags, Graphic3dShaderProgram};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;

/// The maximum number of lights of a program with an unrolled light loop
/// (`THE_NB_UNROLLED_LIGHTS_MAX`).
pub(super) const THE_NB_UNROLLED_LIGHTS_MAX: i32 = 32;

/// The light array size for `nb_lights` lights: the unrolled maximum doubled until it fits
/// (`roundUpMaxLightSources`); `round_up_max_light_sources(40) == 64`.
pub(super) fn round_up_max_light_sources(nb_lights: i32) -> i32 {
    let mut limit = THE_NB_UNROLLED_LIGHTS_MAX;
    while limit < nb_lights {
        limit = limit.wrapping_mul(2);
    }
    limit
}

/// The text of an ASCII string; `text(&"abc".into()) == "abc"`.
pub(super) fn text(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

/// The set behind `lights`, or the `Standard_NullObject` a native dereference would hit.
pub(super) fn light_set(lights: &Handle<Graphic3dLightSet>) -> Result<&Graphic3dLightSet, Failure> {
    lights.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "Graphic3d_ShaderManager: null light set; expected a Graphic3d_LightSet",
            "",
        )
    })
}

impl Graphic3dShaderManager {
    /// `genLightKey`: `l_`/`ls_` (with shadow maps) and the long enabled-lights key, or for more
    /// than 32 lights `l_`, the short key and the rounded light count. A null set raises
    /// `Standard_NullObject`; `manager.gen_light_key(&lights, false)?`.
    pub fn gen_light_key(
        &self,
        lights: &Handle<Graphic3dLightSet>,
        has_shadow_map: bool,
    ) -> Result<AsciiString, Failure> {
        let set = light_set(lights)?;
        let key = if set.nb_enabled() <= THE_NB_UNROLLED_LIGHTS_MAX {
            let prefix = if has_shadow_map { "ls_" } else { "l_" };
            format!("{prefix}{}", text(&set.key_enabled_long()))
        } else {
            let limit = round_up_max_light_sources(set.nb_enabled());
            format!("l_{}{limit}", text(&set.key_enabled_short()))
        };
        Ok(AsciiString::from(key.as_str()))
    }
    /// `hasGlslBitwiseOps`: OpenGL 3.0+ (or `GL_EXT_gpu_shader4`), OpenGL ES 3.0+;
    /// `manager.has_glsl_bitwise_ops()`.
    pub fn has_glsl_bitwise_ops(&self) -> bool {
        self.snapshot().has_glsl_bitwise_ops()
    }
    /// `defaultGlslVersion`: sets the version header and the id `occt_<name><bits %04x>` of
    /// `program` and returns `bits` without the features the API cannot provide;
    /// `manager.default_glsl_version(&program, &"unlit".into(), bits, false)`.
    pub fn default_glsl_version(
        &self,
        program: &Handle<Graphic3dShaderProgram>,
        name: &AsciiString,
        bits: i32,
        uses_derivates: bool,
    ) -> i32 {
        self.snapshot()
            .default_glsl_version(program, &text(name), bits, uses_derivates)
    }
    /// `defaultOitGlslVersion`: the version header of the OIT composition programs and the id
    /// `occt_<name>[_msaa]`; `manager.default_oit_glsl_version(&program, &"weight_oit".into(), false)`.
    pub fn default_oit_glsl_version(
        &self,
        program: &Handle<Graphic3dShaderProgram>,
        name: &AsciiString,
        msaa: bool,
    ) {
        self.snapshot()
            .default_oit_glsl_version(program, &text(name), msaa);
    }
}

impl Graphic3dShaderManagerFields {
    /// `hasGlslBitwiseOps` on the fields.
    pub(super) fn has_glsl_bitwise_ops(&self) -> bool {
        match self.gapi {
            AspectGraphicsLibrary::OpenGL => {
                self.is_gapi_greater_equal(3, 0) || self.has(Ext::GlExtGpuShader4)
            }
            AspectGraphicsLibrary::OpenGLES => self.is_gapi_greater_equal(3, 0),
        }
    }
    /// `defaultGlslVersion` on the fields.
    pub(super) fn default_glsl_version(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        name: &str,
        bits: i32,
        uses_derivates: bool,
    ) -> i32 {
        let filtered = match self.gapi {
            AspectGraphicsLibrary::OpenGL => {
                self.desktop_glsl_header(handle, bits);
                bits
            }
            AspectGraphicsLibrary::OpenGLES => self.gles_glsl_header(handle, bits, uses_derivates),
        };
        // should fit Graphic3d_ShaderFlags_NB
        let id = format!("occt_{name}{filtered:04x}");
        program(handle).set_id(AsciiString::from(id.as_str()));
        filtered
    }
    /// The desktop OpenGL header: `#version 150` from 3.2, else 120/130 or `GL_EXT_gpu_shader4`
    /// for the features that need them.
    fn desktop_glsl_header(&self, handle: &Handle<Graphic3dShaderProgram>, bits: i32) {
        let program = program(handle);
        if self.is_gapi_greater_equal(3, 2) {
            program.set_header("#version 150".into());
            return;
        }
        // TangentSpaceNormal() uses mat2x3; gl_PointCoord is GLSL 1.2+
        let needs_120 = textures_are(bits, Flags::TEXTURE_NORMAL) || any(bits, Flags::POINT_SPRITE);
        if needs_120 && self.is_gapi_greater_equal(2, 1) {
            program.set_header("#version 120".into());
        }
        if !(any(bits, Flags::STIPPLE_LINE) || program.is_pbr()) {
            return;
        }
        if self.is_gapi_greater_equal(3, 0) {
            program.set_header("#version 130".into());
        } else if self.has(Ext::GlExtGpuShader4) {
            // GL_EXT_gpu_shader4 defines "unsigned int", core GLSL defines "uint"
            program.set_header(
                "#extension GL_EXT_gpu_shader4 : enable\n#define uint unsigned int".into(),
            );
        }
    }
    /// The OpenGL ES header (the build without Emscripten): `300 es`/`310 es`/`320 es` from
    /// 3.1, else `300 es` or ES 2.0 extensions per feature; returns the supported bits.
    fn gles_glsl_header(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        uses_derivates: bool,
    ) -> i32 {
        let program = program(handle);
        if self.is_gapi_greater_equal(3, 1) {
            let geometry = any(bits, Flags::NEEDS_GEOM_SHADER);
            let header = match (geometry, self.is_gapi_greater_equal(3, 2)) {
                (true, true) => "#version 320 es",
                (true, false) => "#version 310 es",
                _ => "#version 300 es",
            };
            program.set_header(header.into());
            return bits;
        }
        let to_use_derivates = uses_derivates
            || any(bits, Flags::STIPPLE_LINE)
            || textures_are(bits, Flags::TEXTURE_NORMAL);
        let mut extensions = String::new();
        if program.is_pbr() {
            self.gles2_pbr_header(program, &mut extensions);
        }
        let filtered = self.gles2_oit_header(program, bits);
        if to_use_derivates {
            if self.is_gapi_greater_equal(3, 0) {
                program.set_header("#version 300 es".into());
            } else if self.has(Ext::GlOesStandardDerivatives) {
                extensions.push_str("#extension GL_OES_standard_derivatives : enable\n");
            }
        }
        if !extensions.is_empty() {
            program.set_header(AsciiString::from(extensions.as_str()));
        }
        filtered
    }
    /// The PBR part of the OpenGL ES 2/3.0 header.
    fn gles2_pbr_header(&self, program: &Graphic3dShaderProgram, extensions: &mut String) {
        if self.is_gapi_greater_equal(3, 0) {
            program.set_header("#version 300 es".into());
        } else if self.has(Ext::GlExtShaderTextureLod) {
            extensions.push_str(
                "#extension GL_EXT_shader_texture_lod : enable\n#define textureCubeLod textureCubeLodEXT\n",
            );
        }
    }
    /// The OIT and stipple part of the OpenGL ES 2/3.0 header; returns the supported bits.
    fn gles2_oit_header(&self, program: &Graphic3dShaderProgram, bits: i32) -> i32 {
        let needs_300 = Flags::WRITE_OIT.0 | Flags::OIT_DEPTH_PEELING.0 | Flags::STIPPLE_LINE.0;
        if bits & needs_300 == 0 {
            return bits;
        }
        if self.is_gapi_greater_equal(3, 0) {
            program.set_header("#version 300 es".into());
            return bits;
        }
        let mut filtered = bits & !Flags::WRITE_OIT.0 & !Flags::OIT_DEPTH_PEELING.0;
        if !self.has(Ext::GlOesStandardDerivatives) {
            filtered &= !Flags::STIPPLE_LINE.0;
        }
        filtered
    }
    /// `defaultOitGlslVersion` on the fields.
    pub(super) fn default_oit_glsl_version(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        name: &str,
        msaa: bool,
    ) {
        if let Some(header) = self.oit_header(msaa) {
            program(handle).set_header(header.into());
        }
        let id = format!("occt_{name}{}", if msaa { "_msaa" } else { "" });
        program(handle).set_id(AsciiString::from(id.as_str()));
    }
    /// The OIT composition header, if the API version needs one.
    fn oit_header(&self, msaa: bool) -> Option<&'static str> {
        let at_least = |major, minor| self.is_gapi_greater_equal(major, minor);
        match (self.gapi, msaa) {
            (AspectGraphicsLibrary::OpenGL, true) => at_least(4, 0).then_some("#version 400"),
            (AspectGraphicsLibrary::OpenGL, false) => at_least(3, 2).then_some("#version 150"),
            (AspectGraphicsLibrary::OpenGLES, true) if at_least(3, 2) => Some("#version 320 es"),
            // with GL_OES_sample_variables extension
            (AspectGraphicsLibrary::OpenGLES, _) => at_least(3, 0).then_some("#version 300 es"),
        }
    }
}
