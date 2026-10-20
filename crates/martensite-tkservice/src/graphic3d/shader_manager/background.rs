//! The background and image-based lighting programs: environment baking, cubemap, skydome
//! and coloured quad.
use super::glsl;
use super::program_parts::{
    Declarations, FRAG, VERT, VERT_FRAG, attach_pair, new_program, program, set_unlit_limits,
};
use super::snippets::THE_FUNC_CUBEMAP_VECTOR_TRANSFORM;
use super::texts_programs::{
    CUBEMAP_FRAG_HEAD, CUBEMAP_VERT_MAIN, QUAD_FRAG, QUAD_VERT, SKYDOME_VERT,
};
use super::{Graphic3dGlslExtension as Ext, Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::aspect::AspectGraphicsLibrary;
use crate::graphic3d::Graphic3dShaderProgram;
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;

/// The program names of the baking indices (`THE_BAKE_NAMES`).
const THE_BAKE_NAMES: [&str; 3] = [
    "pbr_env_baking_diffuse",
    "pbr_env_baking_specular",
    "pbr_env_baking_difffallback",
];

/// The fragment stage of baking index `index`.
fn env_baking_frag(index: usize) -> String {
    let target = if index == 0 || index == 2 {
        "\n#define THE_TO_BAKE_DIFFUSE\n"
    } else {
        "\n#define THE_TO_BAKE_SPECULAR\n"
    };
    let pack = if index == 2 {
        "\n#define THE_TO_PACK_FLOAT\n"
    } else {
        ""
    };
    format!(
        "{THE_FUNC_CUBEMAP_VECTOR_TRANSFORM}{}{target}{pack}{}",
        glsl::PBR_DISTRIBUTION,
        glsl::PBR_ENV_BAKING_FS
    )
}

impl Graphic3dShaderManager {
    /// `getPBREnvBakingProgram`: the IBL baking program of `index` (0 diffuse, 1 specular,
    /// 2 diffuse packed as floats); another index raises `Standard_ProgramError` as the
    /// native assertion does; `manager.get_pbr_env_baking_program(0)?`.
    pub fn get_pbr_env_baking_program(
        &self,
        index: i32,
    ) -> Result<Handle<Graphic3dShaderProgram>, Failure> {
        let Some(slot) = usize::try_from(index).ok().filter(|slot| *slot <= 2) else {
            let message = format!(
                "*** ERROR: ASSERT in Graphic3d_ShaderManager: \n (theIndex >= 0 && theIndex <= 2); got {index}"
            );
            return Err(Failure::with_type("Standard_ProgramError", &message, ""));
        };
        Ok(self.snapshot().pbr_env_baking_program(slot))
    }
    /// `getBgCubeMapProgram`: the environment cubemap background;
    /// `manager.get_bg_cube_map_program()`.
    pub fn get_bg_cube_map_program(&self) -> Handle<Graphic3dShaderProgram> {
        self.snapshot().bg_cube_map_program()
    }
    /// `getBgSkydomeProgram`: the skydome background (no id or limits are set);
    /// `manager.get_bg_skydome_program()`.
    pub fn get_bg_skydome_program(&self) -> Handle<Graphic3dShaderProgram> {
        let fields = self.snapshot();
        let mut declarations = Declarations::default();
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        let handle = new_program();
        if fields.gapi == AspectGraphicsLibrary::OpenGL {
            program(&handle).set_header("#version 130".into());
        } else if fields.is_gapi_greater_equal(3, 0) {
            program(&handle).set_header("#version 300 es".into());
        }
        attach_pair(
            &handle,
            SKYDOME_VERT.into(),
            glsl::SKYDOM_BACKGROUND_FS.into(),
            &declarations,
        );
        handle
    }
    /// `getColoredQuadProgram`: the two-colour gradient quad; `manager.get_colored_quad_program()`.
    pub fn get_colored_quad_program(&self) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        declarations.uniform("vec3 uColor1", FRAG);
        declarations.uniform("vec3 uColor2", FRAG);
        let handle = new_program();
        self.snapshot()
            .default_glsl_version(&handle, "colored_quad", 0, false);
        attach_pair(&handle, QUAD_VERT.into(), QUAD_FRAG.into(), &declarations);
        handle
    }
}

impl Graphic3dShaderManagerFields {
    /// `getPBREnvBakingProgram` on the fields, for a checked index.
    fn pbr_env_baking_program(&self, index: usize) -> Handle<Graphic3dShaderProgram> {
        let handle = new_program();
        program(&handle).set_pbr(true); // should be set before defaultGlslVersion()
        let vertex = format!(
            "{THE_FUNC_CUBEMAP_VECTOR_TRANSFORM}{}",
            glsl::PBR_ENV_BAKING_VS
        );
        // constant array definition requires OpenGL 2.1+ or OpenGL ES 3.0+
        if !self.is_gles() {
            program(&handle).set_header("#version 120".into());
        } else if self.is_gapi_greater_equal(3, 0) {
            program(&handle).set_header("#version 300 es".into());
        } else if self.has(Ext::GlExtShaderTextureLod) {
            program(&handle).set_header(
                "#extension GL_EXT_shader_texture_lod : enable\n#define textureCubeLod textureCubeLodEXT".into(),
            );
        } else {
            Message::send_warning_text(&AsciiString::from(
                "Warning: incomplete PBR lighting implementation due to missing OpenGL ES 3.0 or GL_EXT_shader_texture_lod support.",
            ));
        }
        self.default_glsl_version(&handle, THE_BAKE_NAMES[index], 0, false);
        set_unlit_limits(&handle, 0);
        attach_pair(
            &handle,
            vertex,
            env_baking_frag(index),
            &Declarations::default(),
        );
        handle
    }

    /// `getBgCubeMapProgram` on the fields.
    fn bg_cube_map_program(&self) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.in_out("vec3 ViewDirection", VERT_FRAG);
        declarations.uniform("samplerCube occSampler0", FRAG);
        declarations.uniform("int uYCoeff", VERT);
        declarations.uniform("int uZCoeff", VERT);
        let vertex = format!("{THE_FUNC_CUBEMAP_VECTOR_TRANSFORM}{CUBEMAP_VERT_MAIN}");
        let handle = new_program();
        let depth_clamp = self.cubemap_depth_clamp(&handle);
        let fragment = format!("{CUBEMAP_FRAG_HEAD}{depth_clamp}\n}}");
        self.default_glsl_version(&handle, "background_cubemap", 0, false);
        set_unlit_limits(&handle, 0);
        attach_pair(&handle, vertex, fragment, &declarations);
        handle
    }

    /// The depth clamp emulation (a workaround for Z clamping on some GPUs) and its header.
    fn cubemap_depth_clamp(&self, handle: &Handle<Graphic3dShaderProgram>) -> &'static str {
        if !self.to_emulate_depth_clamp {
            return "";
        }
        let clamp = "\n  gl_FragDepth = clamp (gl_FragDepth, 0.0, 1.0);";
        if !self.is_gles() {
            return clamp;
        }
        if self.is_gapi_greater_equal(3, 0) {
            program(handle).set_header("#version 300 es".into());
        } else if self.has(Ext::GlExtFragDepth) {
            program(handle).set_header(
                "#extension GL_EXT_frag_depth : enable\n#define gl_FragDepth gl_FragDepthEXT"
                    .into(),
            );
        } else {
            return "";
        }
        clamp
    }
}
