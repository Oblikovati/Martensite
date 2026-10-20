//! `getStdProgramUnlit`: the program without lighting (with outline and stipple variants).
use super::geometry::{point_sprite_alpha, prepare_geom_main_src};
use super::program_parts::{
    Declarations, FRAG, VERT, VERT_FRAG, any, apply_oit_output, attach_with_geometry,
    clip_planes_fragment, new_program, program, set_alpha_test, set_unlit_limits, textures_are,
};
use super::snippets::{
    THE_FRAG_WIREFRAME_COLOR, THE_FUNC_TRANSFORM_NORMAL_VIEW, THE_VARY_TEX_COORD_TRSF,
    THE_VERT_GL_POSITION, THE_VERT_GL_POSITION_OUTLINE,
};
use super::texts_shading::{
    STIPPLE_ARRAY, STIPPLE_AXIS, STIPPLE_BITWISE, STIPPLE_TAIL, STIPPLE_VERT_END,
};
use super::{Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::graphic3d::{
    Graphic3dShaderFlags as Flags, Graphic3dShaderProgram, Graphic3dTextureSetBits,
};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;

/// The texture coordinate of the environment map.
const ENV_TEX_COORD: &str = concat!(
    "\n  vec4 aPosition = occWorldViewMatrix * occModelWorldMatrix * occVertex;",
    "\n  vec3 aNormal   = transformNormal (occNormal);",
    "\n  vec3 aReflect  = reflect (normalize (aPosition.xyz), aNormal);",
    "\n  aReflect.z += 1.0;",
    "\n  TexCoord = vec4(aReflect.xy * inversesqrt (dot (aReflect, aReflect)) * 0.5 + vec2 (0.5), 0.0, 1.0);",
);
/// The vertex colour read from the base colour texture.
const VERT_COLOR_FROM_TEXTURE: &str =
    "\n  VertColor = occTexture2D (occSamplerBaseColor, occTexCoord.xy);";
/// `getBaseColor()` returning the vertex colour.
const BASE_COLOR_VERT: &str = "\nvec4 getBaseColor(void) { return VertColor; }";
/// `getBackColor()` returning the base colour.
const BACK_COLOR_BASE: &str = "\nvec4 getBackColor(void) { return getBaseColor(); }";

/// The source pieces of the unlit program.
struct UnlitParts {
    vert_extra_func: &'static str,
    vert_extra_main: String,
    vert_end_main: String,
    get_alpha: String,
    frag_extra_main: String,
    base_color: String,
    back_color: String,
    color: String,
    main_color: String,
}
impl Default for UnlitParts {
    fn default() -> Self {
        Self {
            vert_extra_func: "",
            vert_extra_main: String::new(),
            vert_end_main: String::new(),
            get_alpha: String::new(),
            frag_extra_main: String::new(),
            base_color: "\nvec4 getBaseColor(void) { return occColor; }".into(),
            back_color: "\nvec4 getBackColor(void) { return occBackColor; }".into(),
            color: "\nvec4 getColor(void) { return getBaseColor(); }".into(),
            main_color: "\n  occSetFragColor (getFinalColor());".into(),
        }
    }
}

impl Graphic3dShaderManager {
    /// `getStdProgramUnlit`: the unlit program of `bits`, the silhouette variant when
    /// `is_outline`; `manager.get_std_program_unlit(bits, false)`.
    pub fn get_std_program_unlit(
        &self,
        bits: i32,
        is_outline: bool,
    ) -> Handle<Graphic3dShaderProgram> {
        self.snapshot().std_program_unlit(bits, is_outline)
    }
}

impl Graphic3dShaderManagerFields {
    /// `getStdProgramUnlit` on the fields.
    fn std_program_unlit(&self, bits: i32, is_outline: bool) -> Handle<Graphic3dShaderProgram> {
        let handle = new_program();
        let mut parts = UnlitParts::default();
        let mut declarations = Declarations::default();
        if any(bits, Flags::IS_POINT) {
            self.unlit_point(&handle, bits, &mut parts, &mut declarations);
        } else if any(bits, Flags::HAS_TEXTURES) {
            unlit_texture(&handle, bits, &mut parts, &mut declarations);
        }
        unlit_vert_color(bits, &mut parts, &mut declarations);
        let clip_planes = unlit_clipping(bits, &mut parts, &mut declarations);
        apply_oit_output(&handle, bits);
        if is_outline {
            declarations.uniform("float occOrthoScale", VERT);
            declarations.uniform("float occSilhouetteThickness", VERT);
            parts.vert_end_main = THE_VERT_GL_POSITION_OUTLINE.into();
        } else if any(bits, Flags::STIPPLE_LINE) {
            self.unlit_stipple(&handle, bits, &mut parts, &mut declarations);
        }
        let sources = unlit_sources(bits, parts, &mut declarations);
        self.default_glsl_version(
            &handle,
            if is_outline { "outline" } else { "unlit" },
            bits,
            false,
        );
        set_unlit_limits(&handle, clip_planes);
        set_alpha_test(&handle, bits);
        attach_with_geometry(&handle, sources, &declarations);
        handle
    }

    /// The point marker part: point size, sprite or texture colour.
    fn unlit_point(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        parts: &mut UnlitParts,
        declarations: &mut Declarations,
    ) {
        if self.set_point_size {
            parts
                .vert_extra_main
                .push_str("\n  gl_PointSize = occPointSize;");
        }
        if !any(bits, Flags::POINT_SPRITE) {
            texture_vert_color(handle, bits, parts, declarations);
            parts.main_color =
                "\n  vec4 aColor = getColor();\n  if (aColor.a <= 0.1) discard;\n  occSetFragColor (aColor);"
                    .into();
            return;
        }
        declarations.uniform("sampler2D occSamplerPointSprite", FRAG);
        if bits & Flags::POINT_SPRITE_A.0 != Flags::POINT_SPRITE_A.0 {
            parts.base_color = format!(
                "\nvec4 getBaseColor(void) {{ return occTexture2D(occSamplerPointSprite, {}); }}",
                "vec2 (gl_PointCoord.x, 1.0 - gl_PointCoord.y)"
            );
        } else {
            texture_vert_color(handle, bits, parts, declarations);
        }
        parts.get_alpha = point_sprite_alpha(bits, self.use_red_alpha);
        parts.main_color = concat!(
            "\n  vec4 aColor = getColor();",
            "\n  aColor.a = getAlpha();",
            "\n  if (aColor.a <= 0.1) discard;",
            "\n  occSetFragColor (aColor);"
        )
        .into();
    }

    /// The stipple line part: pattern uniforms and the screen-space test, or a warning when
    /// the API cannot provide it.
    fn unlit_stipple(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        parts: &mut UnlitParts,
        declarations: &mut Declarations,
    ) {
        let filtered = self.default_glsl_version(handle, "unlit", bits, false);
        if !any(filtered, Flags::STIPPLE_LINE) {
            Message::send_warning_text(&AsciiString::from(
                "Warning: stipple lines in GLSL will be ignored",
            ));
            return;
        }
        let bitwise = self.has_glsl_bitwise_ops();
        let pattern = if bitwise {
            "int   occStipplePattern"
        } else {
            "bool  occStipplePattern[16]"
        };
        declarations.uniform(pattern, FRAG);
        declarations.uniform("float occStippleFactor", FRAG);
        declarations.uniform("vec4 occViewport", VERT);
        declarations.in_out("vec2 ScreenSpaceCoord", VERT_FRAG);
        parts.vert_end_main = STIPPLE_VERT_END.into();
        let test = if bitwise {
            STIPPLE_BITWISE
        } else {
            STIPPLE_ARRAY
        };
        parts.main_color = format!("{STIPPLE_AXIS}{test}{STIPPLE_TAIL}");
    }
}

/// The vertex colour from the base colour texture of a point without per-vertex colour.
fn texture_vert_color(
    handle: &Handle<Graphic3dShaderProgram>,
    bits: i32,
    parts: &mut UnlitParts,
    declarations: &mut Declarations,
) {
    if !any(bits, Flags::TEXTURE_RGB) || any(bits, Flags::VERT_COLOR) {
        return;
    }
    program(handle).set_texture_set_bits(Graphic3dTextureSetBits::BASE_COLOR.0);
    declarations.uniform("sampler2D occSamplerBaseColor", VERT);
    declarations.in_out("vec4 VertColor", VERT_FRAG);
    parts.vert_extra_main.push_str(VERT_COLOR_FROM_TEXTURE);
    parts.base_color = BASE_COLOR_VERT.into();
}

/// The texture part of a non-point program: environment map or transformed coordinates.
fn unlit_texture(
    handle: &Handle<Graphic3dShaderProgram>,
    bits: i32,
    parts: &mut UnlitParts,
    declarations: &mut Declarations,
) {
    declarations.uniform("sampler2D occSamplerBaseColor", FRAG);
    declarations.in_out("vec4 TexCoord", VERT_FRAG);
    parts.back_color = BACK_COLOR_BASE.into();
    if textures_are(bits, Flags::TEXTURE_ENV) {
        parts.vert_extra_func = THE_FUNC_TRANSFORM_NORMAL_VIEW;
        parts.vert_extra_main.push_str(ENV_TEX_COORD);
        parts.base_color =
            "\nvec4 getBaseColor(void) { return occTexture2D (occSamplerBaseColor, TexCoord.st); }"
                .into();
        return;
    }
    program(handle).set_texture_set_bits(Graphic3dTextureSetBits::BASE_COLOR.0);
    parts.vert_extra_main.push_str(THE_VARY_TEX_COORD_TRSF);
    parts.base_color =
        "\nvec4 getBaseColor(void) { return occTexture2D(occSamplerBaseColor, TexCoord.st / TexCoord.w); }"
            .into();
}

/// The per-vertex colour part (front faces only with `VertColorFrontOnly`).
fn unlit_vert_color(bits: i32, parts: &mut UnlitParts, declarations: &mut Declarations) {
    if !any(bits, Flags::VERT_COLOR) {
        return;
    }
    declarations.in_out("vec4 VertColor", VERT_FRAG);
    parts
        .vert_extra_main
        .push_str("\n  VertColor = occVertColor;");
    parts.color = if any(bits, Flags::VERT_COLOR_FRONT_ONLY) {
        "\nvec4 getColor(void) { return gl_FrontFacing ? VertColor : getBackColor(); }".into()
    } else {
        "\nvec4 getColor(void) { return VertColor; }".into()
    };
}

/// The clipping part; returns the number of clipping planes.
fn unlit_clipping(bits: i32, parts: &mut UnlitParts, declarations: &mut Declarations) -> i32 {
    if !any(bits, Flags::CLIP_PLANES_N) {
        return 0;
    }
    declarations.in_out("vec4 PositionWorld", VERT_FRAG);
    parts
        .vert_extra_main
        .push_str("\n  PositionWorld = occModelWorldMatrix * occVertex;");
    let (count, code) = clip_planes_fragment(bits);
    parts.frag_extra_main.push_str(code);
    count
}

/// The vertex, geometry and fragment sources from the parts.
fn unlit_sources(bits: i32, parts: UnlitParts, declarations: &mut Declarations) -> [String; 3] {
    let vertex = format!(
        "{}\nvoid main()\n{{{}{THE_VERT_GL_POSITION}{}\n}}",
        parts.vert_extra_func, parts.vert_extra_main, parts.vert_end_main
    );
    let geometry = prepare_geom_main_src(declarations, bits);
    let final_color = final_color_src(bits);
    let fragment = format!(
        "{}{}{}{final_color}{}\nvoid main()\n{{\n  if (occFragEarlyReturn()) {{ return; }}{}{}\n}}",
        parts.base_color,
        parts.back_color,
        parts.color,
        parts.get_alpha,
        parts.frag_extra_main,
        parts.main_color
    );
    [vertex, geometry, fragment]
}

/// `getFinalColor()`: the wireframe colour of mesh edges, else `getColor()`.
pub(super) fn final_color_src(bits: i32) -> &'static str {
    if any(bits, Flags::MESH_EDGES) {
        THE_FRAG_WIREFRAME_COLOR
    } else {
        "\n#define getFinalColor getColor"
    }
}
