//! Reproducible GLSL -> SPIR-V shader compiler (`cargo xtask shaders`).
//!
//! Compiles all OpenPBR raster fragment variants (ordinary, native-alpha, top-left coordinates,
//! weighted/peeling OIT) plus the compute path tracers (`pathtrace_hw.comp`, `pathtrace_sw.comp`),
//! 3-pass SVGF denoiser, and HDR tone mapper with `glslangValidator -V --target-env vulkan1.3`.
use std::path::{Path, PathBuf};

use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};
use crate::process::CommandSpec;

struct ShaderJob {
    source: &'static str,
    output: String,
    stage: &'static str,
    defines: Vec<&'static str>,
}

/// Base OpenPBR raster fragment stems and their preprocessor defines.
/// Output names match the embedded shader tables (`pbr_pipeline.rs`, `material_map_pipeline.rs`,
/// `native_alpha_shader_table.rs`, `coordinate_shader_table/*.rs`, `oit_shader_table.rs`).
const OPENPBR_RASTER_BASES: &[(&str, &str, &[&str], bool)] = &[
    // (stem, source, defines, has_oit)
    ("lighting.pbr", "lighting.frag", &["MARTENSITE_PBR"], true),
    (
        "lighting.pbr.clipped",
        "lighting.frag",
        &["MARTENSITE_PBR", "MARTENSITE_CLIPPING"],
        true,
    ),
    (
        "lighting.pbr.line",
        "lighting.frag",
        &["MARTENSITE_PBR", "MARTENSITE_LINE_LIGHTING"],
        false,
    ),
    (
        "lighting.pbr.line.clipped",
        "lighting.frag",
        &[
            "MARTENSITE_PBR",
            "MARTENSITE_LINE_LIGHTING",
            "MARTENSITE_CLIPPING",
        ],
        false,
    ),
    (
        "lighting.pbr.point",
        "lighting.frag",
        &["MARTENSITE_PBR", "MARTENSITE_MARKER_LIGHTING"],
        false,
    ),
    (
        "lighting.pbr.point.clipped",
        "lighting.frag",
        &[
            "MARTENSITE_PBR",
            "MARTENSITE_MARKER_LIGHTING",
            "MARTENSITE_CLIPPING",
        ],
        false,
    ),
    (
        "surface.pbr",
        "surface.frag",
        &["MARTENSITE_LIGHTING", "MARTENSITE_PBR"],
        true,
    ),
    (
        "surface.pbr.image",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_SURFACE_TEXTURE",
        ],
        true,
    ),
    (
        "surface.pbr.clipped",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_CLIPPING",
        ],
        true,
    ),
    (
        "surface.pbr.clipped.image",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_CLIPPING",
            "MARTENSITE_SURFACE_TEXTURE",
        ],
        true,
    ),
    (
        "surface.edges.pbr",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
        ],
        true,
    ),
    (
        "surface.edges.pbr.image",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
            "MARTENSITE_SURFACE_TEXTURE",
        ],
        true,
    ),
    (
        "surface.edges.pbr.clipped",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
            "MARTENSITE_CLIPPING",
        ],
        true,
    ),
    (
        "surface.edges.pbr.clipped.image",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
            "MARTENSITE_CLIPPING",
            "MARTENSITE_SURFACE_TEXTURE",
        ],
        true,
    ),
    (
        "surface.pbr.image.maps",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_SURFACE_TEXTURE",
            "MARTENSITE_MATERIAL_MAPS",
        ],
        true,
    ),
    (
        "surface.pbr.clipped.image.maps",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_CLIPPING",
            "MARTENSITE_SURFACE_TEXTURE",
            "MARTENSITE_MATERIAL_MAPS",
        ],
        true,
    ),
    (
        "surface.edges.pbr.image.maps",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
            "MARTENSITE_SURFACE_TEXTURE",
            "MARTENSITE_MATERIAL_MAPS",
        ],
        true,
    ),
    (
        "surface.edges.pbr.clipped.image.maps",
        "surface.frag",
        &[
            "MARTENSITE_LIGHTING",
            "MARTENSITE_PBR",
            "MARTENSITE_MESH_EDGES",
            "MARTENSITE_CLIPPING",
            "MARTENSITE_SURFACE_TEXTURE",
            "MARTENSITE_MATERIAL_MAPS",
        ],
        true,
    ),
];

const COMPUTE_SHADERS: &[(&str, &str, &[&str])] = &[
    (
        "openpbr/pathtrace_hw.comp",
        "openpbr/pathtrace_hw.comp.spv",
        &["OPENPBR_HW_RAY_QUERY"],
    ),
    (
        "openpbr/pathtrace_sw.comp",
        "openpbr/pathtrace_sw.comp.spv",
        &[],
    ),
    (
        "openpbr/svgf_reproject.comp",
        "openpbr/svgf_reproject.comp.spv",
        &[],
    ),
    (
        "openpbr/svgf_atrous.comp",
        "openpbr/svgf_atrous.comp.spv",
        &[],
    ),
    (
        "openpbr/svgf_compose_tonemap.comp",
        "openpbr/svgf_compose_tonemap.comp.spv",
        &[],
    ),
    ("openpbr/tonemap.comp", "openpbr/tonemap.comp.spv", &[]),
];

fn all_shader_jobs() -> Vec<ShaderJob> {
    let mut jobs = Vec::new();
    for &(stem, source, base_defines, has_oit) in OPENPBR_RASTER_BASES {
        push_raster_variants(&mut jobs, stem, source, base_defines, has_oit);
    }
    for &(source, output, defines) in COMPUTE_SHADERS {
        jobs.push(ShaderJob {
            source,
            output: output.into(),
            stage: "comp",
            defines: defines.to_vec(),
        });
    }
    jobs
}

fn push_raster_variants(
    jobs: &mut Vec<ShaderJob>,
    stem: &str,
    source: &'static str,
    base: &[&'static str],
    has_oit: bool,
) {
    let ordinary_combos: &[(&str, &[&str])] = &[
        ("", &[]),
        (
            ".coordinates-v2",
            &["MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES"],
        ),
        (".native-alpha", &["MARTENSITE_NATIVE_FRAGMENT_ALPHA"]),
        (
            ".native-alpha.coordinates-v2",
            &[
                "MARTENSITE_NATIVE_FRAGMENT_ALPHA",
                "MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES",
            ],
        ),
    ];
    for &(suffix, extra) in ordinary_combos {
        let mut defines = base.to_vec();
        defines.extend_from_slice(extra);
        jobs.push(ShaderJob {
            source,
            output: format!("{stem}{suffix}.frag.spv"),
            stage: "frag",
            defines,
        });
    }
    if !has_oit {
        return;
    }
    let oit_modes: &[(&str, &[&str])] = &[
        ("oit.weighted", &["MARTENSITE_OIT_WEIGHTED"]),
        (
            "oit.weighted.fixed-output",
            &["MARTENSITE_OIT_WEIGHTED", "MARTENSITE_OIT_FIXED_OUTPUT"],
        ),
        (
            "oit.initial",
            &["MARTENSITE_OIT_PEELING", "MARTENSITE_OIT_INITIAL"],
        ),
        (
            "oit.initial.fixed-output",
            &[
                "MARTENSITE_OIT_PEELING",
                "MARTENSITE_OIT_INITIAL",
                "MARTENSITE_OIT_FIXED_OUTPUT",
            ],
        ),
        ("oit.peeling", &["MARTENSITE_OIT_PEELING"]),
        (
            "oit.peeling.fixed-output",
            &["MARTENSITE_OIT_PEELING", "MARTENSITE_OIT_FIXED_OUTPUT"],
        ),
    ];
    for &(oit_suffix, oit_defs) in oit_modes {
        for &(coord_suffix, coord_defs) in &[
            ("", &[][..]),
            (
                ".coordinates-v2",
                &["MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES"][..],
            ),
        ] {
            let mut defines = base.to_vec();
            defines.extend_from_slice(oit_defs);
            defines.extend_from_slice(coord_defs);
            jobs.push(ShaderJob {
                source,
                output: format!("{stem}.{oit_suffix}{coord_suffix}.frag.spv"),
                stage: "frag",
                defines,
            });
        }
    }
}

/// Compiles all OpenPBR shaders (or verifies committed `.spv` outputs when `check` is true).
///
/// Example: `compile_shaders(&context, false)?`.
pub fn compile_shaders(context: &BuildContext<'_>, check: bool) -> BuildResult<()> {
    let shader_root = context
        .git
        .root()
        .join("crates")
        .join("martensite-vulkan")
        .join("shaders");
    let openpbr_dir = shader_root.join("openpbr");
    if !check {
        std::fs::create_dir_all(&openpbr_dir)
            .map_err(|cause| crate::error::io_error(&openpbr_dir, cause))?;
    }
    let mut compiled = 0usize;
    for job in all_shader_jobs() {
        let source = shader_root.join(job.source);
        if !source.exists() {
            continue;
        }
        let output = shader_root.join(&job.output);
        if check {
            verify_job(context, &shader_root, &job, &source, &output)?;
        } else {
            run_glslang(context, &shader_root, &job, &source, &output)?;
        }
        compiled += 1;
    }
    context.reporter.note(&format!(
        "processed {compiled} OpenPBR SPIR-V shader targets"
    ));
    Ok(())
}

fn verify_job(
    context: &BuildContext<'_>,
    shader_root: &Path,
    job: &ShaderJob,
    source: &Path,
    output: &Path,
) -> BuildResult<()> {
    let existing = std::fs::read(output).map_err(|cause| crate::error::io_error(output, cause))?;
    let temp = std::env::temp_dir().join(format!(
        "martensite-shader-check-{}.spv",
        job.output.replace('/', "_")
    ));
    run_glslang(context, shader_root, job, source, &temp)?;
    let fresh = std::fs::read(&temp).map_err(|cause| crate::error::io_error(&temp, cause))?;
    let _ = std::fs::remove_file(&temp);
    if existing != fresh {
        return Err(BuildError::LintFailed(format!(
            "stale SPIR-V binary {}; run `cargo xtask shaders` to regenerate",
            output.display()
        )));
    }
    Ok(())
}

fn run_glslang(
    context: &BuildContext<'_>,
    shader_root: &Path,
    job: &ShaderJob,
    source: &Path,
    output: &Path,
) -> BuildResult<()> {
    let args = glslang_args(shader_root, job, source, output);
    let spec = CommandSpec::new("glslangValidator", context.git.root()).args(args);
    context.runner.run(&spec)
}

fn glslang_args(shader_root: &Path, job: &ShaderJob, source: &Path, output: &Path) -> Vec<String> {
    let include_openpbr: PathBuf = shader_root.join("openpbr");
    let mut args = vec![
        "-V".into(),
        "--target-env".into(),
        "vulkan1.3".into(),
        "-S".into(),
        job.stage.into(),
        format!("-I{}", shader_root.display()),
        format!("-I{}", include_openpbr.display()),
    ];
    for define in &job.defines {
        args.push(format!("-D{define}"));
    }
    args.push("-o".into());
    args.push(output.display().to_string());
    args.push(source.display().to_string());
    args
}
