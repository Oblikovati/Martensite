//! The screen-space programs: font, FBO blit, OIT composition, stereo and bounding box.
use super::program_parts::{
    Declarations, FRAG, VERT, VERT_FRAG, attach_pair, new_program, program, set_limits,
    set_unlit_limits,
};
use super::snippets::THE_VERT_GL_POSITION;
use super::texts_programs::{
    BLIT_FRAG_MAIN, BLIT_FRAG_NO_DEPTH, BLIT_MSAA_FRAG_MAIN, BLIT_VERT, BNDBOX_FRAG, BNDBOX_VERT,
    OIT_FRAG, OIT_MSAA_FRAG, OIT_VERT, PEELING_FLUSH_FRAG_TAIL, PEELING_VERT,
    STEREO_ANAGLYPH_LINEAR, STEREO_ANAGLYPH_MAIN, STEREO_ANAGLYPH_SRGB, STEREO_CHESS_FRAG,
    STEREO_COLUMN_FRAG, STEREO_DEFAULT_FRAG, STEREO_OVER_FRAG, STEREO_ROW_FRAG, STEREO_SIDE_FRAG,
    STEREO_VERT,
};
use super::{Graphic3dGlslExtension as Ext, Graphic3dShaderManager, Graphic3dShaderManagerFields};
use crate::graphic3d::{Graphic3dShaderProgram, Graphic3dStereoMode as Stereo};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;

/// The fragment stage of the multisampled FBO blit.
fn blit_msaa_frag(nb_samples: i32, srgb: bool) -> String {
    let shift = if srgb { "\n#define THE_SHIFT_sRGB" } else { "" };
    format!("\n#define THE_NUM_SAMPLES {nb_samples}{shift}{BLIT_MSAA_FRAG_MAIN}")
}

/// The id `occt_blit[_msaa<n>][_gamma]`.
fn blit_id(nb_samples: i32, srgb: bool) -> String {
    let mut id = String::from("occt_blit");
    if nb_samples > 1 {
        id.push_str(&format!("_msaa{nb_samples}"));
    }
    if srgb {
        id.push_str("_gamma");
    }
    id
}

/// The fragment stage of the depth peeling blend.
fn peeling_blend_frag(msaa: bool) -> String {
    let sample = if msaa { "gl_SampleID" } else { "0" };
    format!(
        "\nvoid main()\n{{\n  #define THE_SAMPLE_ID {sample}\n  occFragColor = texelFetch (uDepthPeelingBackColor, ivec2 (gl_FragCoord.xy), THE_SAMPLE_ID);\n  if (occFragColor.a == 0.0) {{ discard; }}\n}}"
    )
}

/// The `sampler2DMS` or `sampler2D` declaration of `name`.
fn sampler(msaa: bool, name: &str) -> String {
    format!("{} {name}", if msaa { "sampler2DMS" } else { "sampler2D" })
}

/// The stereo program name, its extra uniforms and its fragment stage.
fn stereo_fragment(
    mode: Stereo,
    srgb_state: bool,
) -> (&'static str, &'static [&'static str], String) {
    match mode {
        Stereo::Anaglyph => {
            let normalize = if srgb_state {
                STEREO_ANAGLYPH_LINEAR
            } else {
                STEREO_ANAGLYPH_SRGB
            };
            (
                "anaglyph",
                &["mat4 uMultL", "mat4 uMultR"],
                format!("{normalize}{STEREO_ANAGLYPH_MAIN}"),
            )
        }
        Stereo::RowInterlaced => (
            "row-interlaced",
            &["vec2 uTexOffset"],
            STEREO_ROW_FRAG.into(),
        ),
        Stereo::ColumnInterlaced => (
            "column-interlaced",
            &["vec2 uTexOffset"],
            STEREO_COLUMN_FRAG.into(),
        ),
        Stereo::ChessBoard => ("chessboard", &["vec2 uTexOffset"], STEREO_CHESS_FRAG.into()),
        Stereo::SideBySide => ("sidebyside", &[], STEREO_SIDE_FRAG.into()),
        Stereo::OverUnder => ("overunder", &[], STEREO_OVER_FRAG.into()),
        Stereo::QuadBuffer | Stereo::SoftPageFlip | Stereo::OpenVR => {
            ("stereo", &[], STEREO_DEFAULT_FRAG.into())
        }
    }
}

impl Graphic3dShaderManager {
    /// `getStdProgramFont`: the textured font program; `manager.get_std_program_font()`.
    pub fn get_std_program_font(&self) -> Handle<Graphic3dShaderProgram> {
        self.snapshot().std_program_font()
    }
    /// `getStdProgramFboBlit`: the FBO blit of `nb_samples` samples, gamma-shifted for the sRGB
    /// fallback; `manager.get_std_program_fbo_blit(4, false)`.
    pub fn get_std_program_fbo_blit(
        &self,
        nb_samples: i32,
        is_fallback_srgb: bool,
    ) -> Handle<Graphic3dShaderProgram> {
        self.snapshot()
            .std_program_fbo_blit(nb_samples, is_fallback_srgb)
    }
    /// `getStdProgramOitCompositing`: the weighted OIT composition;
    /// `manager.get_std_program_oit_compositing(false)`.
    pub fn get_std_program_oit_compositing(&self, msaa: bool) -> Handle<Graphic3dShaderProgram> {
        let fields = self.snapshot();
        let mut declarations = Declarations::default();
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        declarations.uniform(&sampler(msaa, "uAccumTexture"), FRAG);
        declarations.uniform(&sampler(msaa, "uWeightTexture"), FRAG);
        let fragment = if msaa { OIT_MSAA_FRAG } else { OIT_FRAG };
        let handle = new_program();
        fields.default_oit_glsl_version(&handle, "weight_oit", msaa);
        set_unlit_limits(&handle, 0);
        attach_pair(&handle, OIT_VERT.into(), fragment.into(), &declarations);
        handle
    }
    /// `getStdProgramOitDepthPeelingBlend`: the depth peeling blend;
    /// `manager.get_std_program_oit_depth_peeling_blend(false)`.
    pub fn get_std_program_oit_depth_peeling_blend(
        &self,
        msaa: bool,
    ) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.uniform(&sampler(msaa, "uDepthPeelingBackColor"), FRAG);
        let handle = new_program();
        self.snapshot()
            .default_oit_glsl_version(&handle, "oit_peeling_blend", msaa);
        set_unlit_limits(&handle, 0);
        attach_pair(
            &handle,
            PEELING_VERT.into(),
            peeling_blend_frag(msaa),
            &declarations,
        );
        handle
    }
    /// `getStdProgramOitDepthPeelingFlush`: the depth peeling flush;
    /// `manager.get_std_program_oit_depth_peeling_flush(false)`.
    pub fn get_std_program_oit_depth_peeling_flush(
        &self,
        msaa: bool,
    ) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.uniform(&sampler(msaa, "uDepthPeelingFrontColor"), FRAG);
        declarations.uniform(&sampler(msaa, "uDepthPeelingBackColor"), FRAG);
        let sample = if msaa { "gl_SampleID" } else { "0" };
        let fragment =
            format!("\nvoid main()\n{{\n  #define THE_SAMPLE_ID {sample}{PEELING_FLUSH_FRAG_TAIL}");
        let handle = new_program();
        self.snapshot()
            .default_oit_glsl_version(&handle, "oit_peeling_flush", msaa);
        set_unlit_limits(&handle, 0);
        attach_pair(&handle, PEELING_VERT.into(), fragment, &declarations);
        handle
    }
    /// `getStdProgramStereo`: the stereo composition of `mode`;
    /// `manager.get_std_program_stereo(Graphic3dStereoMode::Anaglyph)`.
    pub fn get_std_program_stereo(&self, mode: Stereo) -> Handle<Graphic3dShaderProgram> {
        let fields = self.snapshot();
        let mut declarations = Declarations::default();
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        declarations.uniform("sampler2D uLeftSampler", FRAG);
        declarations.uniform("sampler2D uRightSampler", FRAG);
        let (name, uniforms, fragment) = stereo_fragment(mode, fields.srgb_state);
        uniforms
            .iter()
            .for_each(|uniform| declarations.uniform(uniform, FRAG));
        let handle = new_program();
        fields.default_glsl_version(&handle, name, 0, false);
        set_unlit_limits(&handle, 0);
        attach_pair(&handle, STEREO_VERT.into(), fragment, &declarations);
        handle
    }
    /// `getStdProgramBoundBox`: the bounding box program; `manager.get_std_program_bound_box()`.
    pub fn get_std_program_bound_box(&self) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.uniform("vec3 occBBoxCenter", VERT);
        declarations.uniform("vec3 occBBoxSize", VERT);
        let handle = new_program();
        self.snapshot()
            .default_glsl_version(&handle, "bndbox", 0, false);
        set_unlit_limits(&handle, 0);
        attach_pair(
            &handle,
            BNDBOX_VERT.into(),
            BNDBOX_FRAG.into(),
            &declarations,
        );
        handle
    }
}

impl Graphic3dShaderManagerFields {
    /// `getStdProgramFont` on the fields.
    fn std_program_font(&self) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.uniform("sampler2D occSamplerBaseColor", FRAG);
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        let vertex =
            format!("\nvoid main()\n{{\n  TexCoord = occTexCoord.st;{THE_VERT_GL_POSITION}\n}}");
        let channel = if self.use_red_alpha { 'r' } else { 'a' };
        let fragment = format!(
            "\nfloat getAlpha(void) {{ return occTexture2D(occSamplerBaseColor, TexCoord.st).{channel}; }}\nvoid main()\n{{\n  vec4 aColor = occColor;\n  aColor.a *= getAlpha();\n  if (aColor.a <= 0.285) discard;\n  occSetFragColor (aColor);\n}}"
        );
        let handle = new_program();
        self.default_glsl_version(&handle, "font", 0, false);
        set_unlit_limits(&handle, 0);
        attach_pair(&handle, vertex, fragment, &declarations);
        handle
    }

    /// `getStdProgramFboBlit` on the fields.
    fn std_program_fbo_blit(&self, nb_samples: i32, srgb: bool) -> Handle<Graphic3dShaderProgram> {
        let mut declarations = Declarations::default();
        declarations.in_out("vec2 TexCoord", VERT_FRAG);
        let mut fragment = if nb_samples > 1 {
            let precision = if self.is_gles() { "highp " } else { "" };
            declarations.uniform(&format!("{precision}sampler2DMS uColorSampler"), FRAG);
            declarations.uniform(&format!("{precision}sampler2DMS uDepthSampler"), FRAG);
            blit_msaa_frag(nb_samples, srgb)
        } else {
            declarations.uniform("sampler2D uColorSampler", FRAG);
            declarations.uniform("sampler2D uDepthSampler", FRAG);
            let shift = if srgb { "\n#define THE_SHIFT_sRGB" } else { "" };
            format!("{shift}{BLIT_FRAG_MAIN}")
        };
        let handle = new_program();
        self.blit_header(&handle, &mut fragment);
        program(&handle).set_id(AsciiString::from(blit_id(nb_samples, srgb).as_str()));
        set_limits(&handle, (0, 0, 0));
        attach_pair(&handle, BLIT_VERT.into(), fragment, &declarations);
        handle
    }

    /// The blit header; without depth output on OpenGL ES 2.0 the fragment drops the depth.
    fn blit_header(&self, handle: &Handle<Graphic3dShaderProgram>, fragment: &mut String) {
        let program = program(handle);
        if !self.is_gles() {
            if self.is_gapi_greater_equal(3, 2) {
                program.set_header("#version 150".into());
            }
        } else if self.is_gapi_greater_equal(3, 1) {
            // required for MSAA sampler
            program.set_header("#version 310 es".into());
        } else if self.is_gapi_greater_equal(3, 0) {
            program.set_header("#version 300 es".into());
        } else if self.has(Ext::GlExtFragDepth) {
            program.set_header(
                "#extension GL_EXT_frag_depth : enable\n#define gl_FragDepth gl_FragDepthEXT"
                    .into(),
            );
        } else {
            // there is no way to draw into depth buffer
            *fragment = BLIT_FRAG_NO_DEPTH.into();
        }
    }
}
