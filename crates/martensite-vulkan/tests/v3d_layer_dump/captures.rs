use super::Outcome;
use crate::hatch_surface::scene::SourceViewOutput;
use crate::session_run::SessionRun;
use martensite_tkservice::graphic3d::Graphic3dBufferType;
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dLightName};
const ONLY: [&str; 7] = [
    "bug32752_only_b1.png",
    "bug32752_only_b2.png",
    "bug32752_only_b3.png",
    "bug32752_only_b4.png",
    "bug32752_only_b5.png",
    "bug32752_only_b6.png",
    "bug32752_only_b7.png",
];
const UPTO: [&str; 7] = [
    "bug32752_upto_b1.png",
    "bug32752_upto_b2.png",
    "bug32752_upto_b3.png",
    "bug32752_upto_b4.png",
    "bug32752_upto_b5.png",
    "bug32752_upto_b6.png",
    "bug32752_upto_b7.png",
];
const ONLY_DEPTH: [&str; 7] = [
    "bug32752_only_b1_depth.png",
    "bug32752_only_b2_depth.png",
    "bug32752_only_b3_depth.png",
    "bug32752_only_b4_depth.png",
    "bug32752_only_b5_depth.png",
    "bug32752_only_b6_depth.png",
    "bug32752_only_b7_depth.png",
];
const UPTO_DEPTH: [&str; 7] = [
    "bug32752_upto_b1_depth.png",
    "bug32752_upto_b2_depth.png",
    "bug32752_upto_b3_depth.png",
    "bug32752_upto_b4_depth.png",
    "bug32752_upto_b5_depth.png",
    "bug32752_upto_b6_depth.png",
    "bug32752_upto_b7_depth.png",
];

pub(super) fn scene_layers(run: &mut SessionRun, layers: [i32; 7]) -> Outcome {
    run.dump("bug32752_all.png")?;
    let mut options = options(run);
    options.buffer_type = Graphic3dBufferType::Depth;
    run.dump_options("bug32752_depth.png", &options)?;
    for (names, single, depth) in [
        (ONLY, true, false),
        (UPTO, false, false),
        (ONLY_DEPTH, true, true),
        (UPTO_DEPTH, false, true),
    ] {
        layer_group(run, layers, names, single, depth)?;
    }
    Ok(())
}

fn layer_group(
    run: &mut SessionRun,
    layers: [i32; 7],
    names: [&'static str; 7],
    single: bool,
    depth: bool,
) -> Outcome {
    let mut options = options(run);
    options.is_single_layer = single;
    if depth {
        options.buffer_type = Graphic3dBufferType::Depth;
    }
    for (layer, name) in layers.into_iter().zip(names) {
        options.target_z_layer_id = layer;
        run.dump_options(name, &options)?;
    }
    Ok(())
}

pub(super) fn shadow_maps(run: &mut SessionRun) -> Outcome {
    run.dump("bug32752_newlights.png")?;
    let mut options = options(run);
    options.buffer_type = Graphic3dBufferType::ShadowMap;
    for (light, name) in [
        (c"v1", "bug32752_shadowmap_v1.png"),
        (c"v2", "bug32752_shadowmap_v2.png"),
    ] {
        options.light_name = V3dLightName::from_c_str(light);
        run.dump_options(name, &options)?;
    }
    Ok(())
}

fn options(run: &mut SessionRun) -> V3dImageDumpOptions<'static> {
    let (width, height) = run.session().source_window_size();
    V3dImageDumpOptions {
        width,
        height,
        ..Default::default()
    }
}
