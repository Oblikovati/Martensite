//! `getStdProgramGouraud`: the program with per-vertex lighting.
use super::geometry::{point_sprite_shading, prepare_geom_main_src};
use super::lighting::Graphic3dLightingOptions;
use super::program_parts::{
    Declarations, FRAG, VERT, VERT_FRAG, any, apply_oit_output, attach_with_geometry,
    clip_planes_fragment, new_program, program, set_alpha_test, set_limits,
};
use super::snippets::{
    THE_FUNC_TRANSFORM_NORMAL_WORLD, THE_VARY_TEX_COORD_TRSF, THE_VERT_GL_POSITION,
};
use super::texts_shading::GOURAUD_VERT_MAIN;
use super::unlit::final_color_src;
use super::version::text;
use super::{Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::graphic3d::{
    Graphic3dLightSet, Graphic3dShaderFlags as Flags, Graphic3dShaderProgram,
    Graphic3dTextureSetBits,
};
use martensite_tkernel::standard::{Failure, Handle};

/// The source pieces of the Gouraud program.
struct GouraudParts {
    vert_color: String,
    vert_extra_main: String,
    frag_extra_main: String,
    frag_color: String,
    use_tex_color: bool,
}

impl Graphic3dShaderManager {
    /// `getStdProgramGouraud`: the per-vertex lighting program of `lights` and `bits`; a null
    /// light set raises `Standard_NullObject`; `manager.get_std_program_gouraud(&lights, bits)?`.
    pub fn get_std_program_gouraud(
        &self,
        lights: &Handle<Graphic3dLightSet>,
        bits: i32,
    ) -> Result<Handle<Graphic3dShaderProgram>, Failure> {
        self.snapshot().std_program_gouraud(self, lights, bits)
    }
}

impl Graphic3dShaderManagerFields {
    /// `getStdProgramGouraud` on the fields.
    fn std_program_gouraud(
        &self,
        manager: &Graphic3dShaderManager,
        lights: &Handle<Graphic3dLightSet>,
        bits: i32,
    ) -> Result<Handle<Graphic3dShaderProgram>, Failure> {
        let handle = new_program();
        let mut declarations = Declarations::default();
        let mut parts = GouraudParts {
            vert_color: String::new(),
            vert_extra_main: String::new(),
            frag_extra_main: String::new(),
            frag_color: "\nvec4 getColor(void) { return gl_FrontFacing ? FrontColor : BackColor; }"
                .into(),
            use_tex_color: false,
        };
        if any(bits, Flags::IS_POINT) {
            self.gouraud_point(&handle, bits, &mut parts, &mut declarations);
        } else if any(bits, Flags::TEXTURE_RGB) {
            gouraud_texture(&handle, &mut parts, &mut declarations);
        }
        gouraud_vert_color(bits, &mut parts);
        let clip_planes = gouraud_clipping(bits, &mut parts, &mut declarations);
        apply_oit_output(&handle, bits);
        declarations.in_out("vec4 FrontColor", VERT_FRAG);
        declarations.in_out("vec4 BackColor", VERT_FRAG);
        let options = Graphic3dLightingOptions {
            has_vert_color: !parts.vert_color.is_empty(),
            has_tex_color: parts.use_tex_color,
            ..Graphic3dLightingOptions::default()
        };
        let (nb_lights, lighting) = self.compute_lighting(lights, options);
        let sources = gouraud_sources(bits, &parts, &lighting, &mut declarations);
        let name = format!("gouraud-{}-", text(&manager.gen_light_key(lights, false)?));
        self.default_glsl_version(&handle, &name, bits, false);
        set_limits(&handle, (nb_lights, 0, clip_planes));
        set_alpha_test(&handle, bits);
        attach_with_geometry(&handle, sources, &declarations);
        Ok(handle)
    }

    /// The point marker part: point size, sprite shading and texture colour.
    fn gouraud_point(
        &self,
        handle: &Handle<Graphic3dShaderProgram>,
        bits: i32,
        parts: &mut GouraudParts,
        declarations: &mut Declarations,
    ) {
        if self.set_point_size {
            parts
                .vert_extra_main
                .push_str("\n  gl_PointSize = occPointSize;");
        }
        if any(bits, Flags::POINT_SPRITE) {
            declarations.uniform("sampler2D occSamplerPointSprite", FRAG);
            let base = "gl_FrontFacing ? FrontColor : BackColor";
            parts.frag_color = point_sprite_shading(base, bits, self.use_red_alpha);
        }
        if any(bits, Flags::TEXTURE_RGB) && !any(bits, Flags::VERT_COLOR) {
            program(handle).set_texture_set_bits(Graphic3dTextureSetBits::BASE_COLOR.0);
            declarations.uniform("sampler2D occSamplerBaseColor", VERT);
            parts.vert_color =
                "\nvec4 getVertColor(in bool theIsFront) { return occTexture2D (occSamplerBaseColor, occTexCoord.xy); }"
                    .into();
        }
    }
}

/// The texture part of a non-point program.
fn gouraud_texture(
    handle: &Handle<Graphic3dShaderProgram>,
    parts: &mut GouraudParts,
    declarations: &mut Declarations,
) {
    parts.use_tex_color = true;
    program(handle).set_texture_set_bits(Graphic3dTextureSetBits::BASE_COLOR.0);
    declarations.uniform("sampler2D occSamplerBaseColor", FRAG);
    declarations.in_out("vec4 TexCoord", VERT_FRAG);
    parts.vert_extra_main.push_str(THE_VARY_TEX_COORD_TRSF);
    parts.frag_color = concat!(
        "\nvec4 getColor(void)",
        "\n{",
        "\n  vec4 aColor = gl_FrontFacing ? FrontColor : BackColor;",
        "\n  return occTexture2D(occSamplerBaseColor, TexCoord.st / TexCoord.w) * aColor;",
        "\n}"
    )
    .into();
}

/// The per-vertex colour (front faces only with `VertColorFrontOnly`).
fn gouraud_vert_color(bits: i32, parts: &mut GouraudParts) {
    if !any(bits, Flags::VERT_COLOR) {
        return;
    }
    parts.vert_color = if any(bits, Flags::VERT_COLOR_FRONT_ONLY) {
        "\nvec4 getVertColor(in bool theIsFront) { return theIsFront ? occVertColor : vec4(1.0); }"
            .into()
    } else {
        "\nvec4 getVertColor(in bool theIsFront) { return occVertColor; }".into()
    };
}

/// The clipping part; returns the number of clipping planes.
fn gouraud_clipping(bits: i32, parts: &mut GouraudParts, declarations: &mut Declarations) -> i32 {
    if !any(bits, Flags::CLIP_PLANES_N) {
        return 0;
    }
    declarations.in_out("vec4 PositionWorld", VERT_FRAG);
    parts
        .vert_extra_main
        .push_str("\n  PositionWorld = aPositionWorld;");
    let (count, code) = clip_planes_fragment(bits);
    parts.frag_extra_main.push_str(code);
    count
}

/// The vertex, geometry and fragment sources.
fn gouraud_sources(
    bits: i32,
    parts: &GouraudParts,
    lighting: &str,
    declarations: &mut Declarations,
) -> [String; 3] {
    let vertex = format!(
        "{THE_FUNC_TRANSFORM_NORMAL_WORLD}\n{}{lighting}{GOURAUD_VERT_MAIN}{}{THE_VERT_GL_POSITION}\n}}",
        parts.vert_color, parts.vert_extra_main
    );
    let geometry = prepare_geom_main_src(declarations, bits);
    let fragment = format!(
        "{}{}\nvoid main()\n{{\n  if (occFragEarlyReturn()) {{ return; }}{}\n  occSetFragColor (getFinalColor());\n}}",
        parts.frag_color,
        final_color_src(bits),
        parts.frag_extra_main
    );
    [vertex, geometry, fragment]
}
