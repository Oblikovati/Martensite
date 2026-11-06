//! Which fragment output a native user program writes. GlShaderProgram::Initialize
//! defines OCC_ENABLE_draw_buffers only for more than one fragment output; without it a GLSL
//! version below 1.30 maps `occFragColor` to `gl_FragColor` (Declarations.glsl), which GL writes
//! into every enabled draw buffer. A program without a `#version` line gets `#version 400` with
//! tessellation stages, `#version 150` with a geometry stage, and no line (GLSL 1.10) otherwise.
use martensite_tkservice::graphic3d::{Graphic3dShaderProgram, Graphic3dTypeOfShaderObject};

/// The first GLSL version with user-declared fragment outputs instead of `gl_FragColor`.
const FIRST_OUT_VERSION: u32 = 130;

/// Whether native writes the program's one fragment colour into every draw buffer;
/// `broadcasts_frag_color(&program)` is true for a program with the header `#version 110`.
pub fn broadcasts_frag_color(program: &Graphic3dShaderProgram) -> bool {
    program.nb_fragment_outputs() <= 1 && effective_glsl_version(program) < FIRST_OUT_VERSION
}

/// The GLSL version native compiles the program with: the header's `#version`, or the
/// stage-dependent default of GlShaderProgram::Initialize on desktop OpenGL.
pub fn effective_glsl_version(program: &Graphic3dShaderProgram) -> u32 {
    if let Some(version) = header_version(&String::from_utf8_lossy(program.header().as_bytes())) {
        return version;
    }
    let kinds: Vec<_> = program
        .shader_objects()
        .iter()
        .map(|shader| shader.shader_type())
        .collect();
    let has = |kind| kinds.contains(&kind);
    if has(Graphic3dTypeOfShaderObject::TessControl)
        || has(Graphic3dTypeOfShaderObject::TessEvaluation)
    {
        return 400;
    }
    if has(Graphic3dTypeOfShaderObject::Geometry) {
        return 150;
    }
    110
}

/// The number of the header's first `#version` line; `header_version("#version 110") == Some(110)`.
fn header_version(header: &str) -> Option<u32> {
    let line = header
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("#version"))?;
    let number = line["#version".len()..].split_whitespace().next()?;
    number.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::graphic3d::Graphic3dShaderObject;

    fn program(header: &str, kinds: &[Graphic3dTypeOfShaderObject]) -> Graphic3dShaderProgram {
        let program = Graphic3dShaderProgram::default();
        program.set_header(header.into());
        for &kind in kinds {
            let shader = Graphic3dShaderObject::create_from_source(kind, "void main(){}".into());
            assert!(program.attach_shader(&shader));
        }
        program
    }

    #[test]
    fn a_glsl_110_program_with_one_output_broadcasts_its_colour() {
        use Graphic3dTypeOfShaderObject::{Fragment, Vertex};
        assert!(broadcasts_frag_color(&program(
            "#version 110",
            &[Vertex, Fragment]
        )));
        assert!(broadcasts_frag_color(&program("", &[Vertex, Fragment])));
        assert!(broadcasts_frag_color(&program(
            "#version 120\n#define X",
            &[Vertex, Fragment]
        )));
    }

    #[test]
    fn declared_outputs_and_draw_buffers_do_not_broadcast() {
        use Graphic3dTypeOfShaderObject::{Fragment, Geometry, TessEvaluation, Vertex};
        assert!(!broadcasts_frag_color(&program(
            "#version 150",
            &[Vertex, Fragment]
        )));
        assert!(!broadcasts_frag_color(&program(
            "",
            &[Vertex, Geometry, Fragment]
        )));
        assert_eq!(
            effective_glsl_version(&program("", &[Vertex, TessEvaluation])),
            400
        );
        let two = program("#version 110", &[Vertex, Fragment]);
        two.set_nb_fragment_outputs(2);
        assert!(!broadcasts_frag_color(&two));
    }

    #[test]
    fn a_header_without_a_number_falls_back_to_the_stage_default() {
        assert_eq!(header_version("#version"), None);
        assert_eq!(header_version("  #version 330 core"), Some(330));
    }
}
