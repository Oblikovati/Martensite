//! Graphic3d_ShaderManager program generators.
use super::common::{Reader, end, flag, int, text};
use super::print::{Output, show};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectGraphicsLibrary;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_GLSL_EXTENSION_NB, Graphic3dCLight, Graphic3dGlslExtension, Graphic3dLightSet,
    Graphic3dLightingOptions, Graphic3dPhongVariant, Graphic3dShaderManager,
    Graphic3dShaderProgram, Graphic3dStereoMode, Graphic3dTypeOfLightSource,
};

const EXTENSIONS: [Graphic3dGlslExtension; GRAPHIC3D_GLSL_EXTENSION_NB as usize] = [
    Graphic3dGlslExtension::GlOesStandardDerivatives,
    Graphic3dGlslExtension::GlExtShaderTextureLod,
    Graphic3dGlslExtension::GlExtFragDepth,
    Graphic3dGlslExtension::GlExtGpuShader4,
];

fn light_type(value: i32) -> Graphic3dTypeOfLightSource {
    use Graphic3dTypeOfLightSource as T;
    [T::Ambient, T::Directional, T::Positional, T::Spot][value as usize]
}

fn stereo(value: i32) -> Graphic3dStereoMode {
    use Graphic3dStereoMode as S;
    let modes = [
        S::QuadBuffer,
        S::Anaglyph,
        S::RowInterlaced,
        S::ColumnInterlaced,
        S::ChessBoard,
        S::SideBySide,
        S::OverUnder,
        S::SoftPageFlip,
        S::OpenVR,
    ];
    modes[value as usize]
}

/// `Count (Type Enabled Shadow)*`: a light set, null for a negative count.
fn lights(r: &mut Reader<'_>) -> Handle<Graphic3dLightSet> {
    let count = r.integer();
    if count < 0 {
        return Handle::default();
    }
    let set = Graphic3dLightSet::default();
    for _ in 0..count {
        let light = Graphic3dCLight::new(light_type(r.integer()));
        light.set_enabled(r.flag());
        if r.flag() {
            light
                .set_cast_shadows(true)
                .expect("corpus shadows are on directional or spot lights");
        }
        set.add(&Handle::new(light))
            .expect("corpus lights are not null");
    }
    Handle::new(set)
}

/// The program state and every stage; ids made from the program counter are not printed.
fn program(output: &Output, handle: &Handle<Graphic3dShaderProgram>) {
    let p = handle.get().expect("generated programs are not null");
    let id = p.get_id();
    let counted = id.as_bytes().starts_with(b"Graphic3d_ShaderProgram_");
    text(output, if counted { b"" } else { id.as_bytes() });
    text(output, p.header().as_bytes());
    for value in [
        p.nb_lights_max(),
        p.nb_shadow_maps(),
        p.nb_clip_planes_max(),
    ] {
        int(output, value);
    }
    int(output, p.nb_fragment_outputs());
    int(output, p.texture_set_bits());
    int(output, p.oit_output() as i32);
    for value in [p.has_default_sampler(), p.has_alpha_test(), p.is_pbr()] {
        flag(output, value);
    }
    let stages = p.shader_objects();
    int(output, stages.len() as i32);
    for stage in stages {
        let stage = stage.get().expect("attached stages are not null");
        int(output, stage.shader_type() as i32);
        text(output, stage.source().as_bytes());
    }
}

fn phong(output: &Output, r: &mut Reader<'_>, m: &Graphic3dShaderManager) {
    let l = lights(r);
    let bits = r.integer();
    let variant = Graphic3dPhongVariant {
        is_flat_normal: r.flag(),
        is_pbr: r.flag(),
        nb_shadow_maps: r.integer(),
    };
    show(output, m.get_std_program_phong(&l, bits, variant), |p| {
        program(output, &p)
    });
}

fn lighting(output: &Output, r: &mut Reader<'_>, m: &Graphic3dShaderManager) {
    let l = lights(r);
    let options = Graphic3dLightingOptions {
        has_vert_color: r.flag(),
        is_pbr: r.flag(),
        has_tex_color: r.flag(),
        nb_shadow_maps: r.integer(),
    };
    let (count, source) = m.std_compute_lighting(&l, options);
    int(output, count);
    text(output, source.as_bytes());
}

/// The program operations 0-19 of the native `sv_sm_run`.
fn run(output: &Output, r: &mut Reader<'_>, m: &Graphic3dShaderManager) {
    match r.integer() {
        0 => program(output, &m.get_std_program_font()),
        1 => {
            let bits = r.integer();
            program(output, &m.get_std_program_unlit(bits, r.flag()));
        }
        2 => {
            let l = lights(r);
            let bits = r.integer();
            show(output, m.get_std_program_gouraud(&l, bits), |p| {
                program(output, &p)
            });
        }
        3 => phong(output, r, m),
        4 => program(output, &m.get_std_program_bound_box()),
        5 => program(output, &m.get_bg_cube_map_program()),
        6 => program(output, &m.get_bg_skydome_program()),
        7 => program(output, &m.get_colored_quad_program()),
        8 => program(output, &m.get_grid_program()),
        9 => show(output, m.get_pbr_env_baking_program(r.integer()), |p| {
            program(output, &p)
        }),
        10 => {
            let samples = r.integer();
            program(output, &m.get_std_program_fbo_blit(samples, r.flag()));
        }
        11 => program(output, &m.get_std_program_stereo(stereo(r.integer()))),
        12 => program(output, &m.get_std_program_oit_compositing(r.flag())),
        13 => program(output, &m.get_std_program_oit_depth_peeling_blend(r.flag())),
        14 => program(output, &m.get_std_program_oit_depth_peeling_flush(r.flag())),
        15 => {
            let l = lights(r);
            show(output, m.gen_light_key(&l, r.flag()), |key| {
                text(output, key.as_bytes())
            });
        }
        16 => lighting(output, r, m),
        17 => text(output, m.point_sprite_alpha_src(r.integer()).as_bytes()),
        18 => {
            let bits = r.integer();
            let source = m.point_sprite_shading_src(&"occColor".into(), bits);
            text(output, source.as_bytes());
        }
        _ => flag(output, m.has_glsl_bitwise_ops()),
    }
}

/// `sv_shader_manager Gapi Major Minor ExtensionMask Settings Program Args...`.
pub fn manager_case(output: &Output, values: &[f64]) {
    Message::default_messenger().printers_mut().clear();
    let mut r = Reader::new(values);
    let gapi = if r.integer() == 0 {
        AspectGraphicsLibrary::OpenGL
    } else {
        AspectGraphicsLibrary::OpenGLES
    };
    let m = Graphic3dShaderManager::new(gapi);
    let major = r.integer();
    m.set_gapi_version(major, r.integer());
    let mask = r.integer();
    for (index, extension) in EXTENSIONS.into_iter().enumerate() {
        m.enable_glsl_extension(extension, (mask >> index) & 1 != 0);
    }
    let settings = r.integer();
    m.set_use_red_alpha(settings & 1 != 0);
    m.set_flat_shading(settings & 2 == 0, settings & 4 != 0);
    m.set_emulate_depth_clamp(settings & 8 == 0);
    run(output, &mut r, &m);
    end(output);
}
