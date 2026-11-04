//! Alpha-to-coverage (Graphic3dRenderingParams::ToEnableAlphaToCoverage) on real devices.
//!
//! Native fingerprints come from the recorded alpha_to_coverage capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/alpha_to_coverage/native.txt (build and run as
//! the recorded depth_prepass capture). A white unlit Mask-mode rectangle with alpha 0.5
//! and cutoff 0.5 is drawn without MSAA and with 4x MSAA, alpha-to-coverage off and on.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use rgb_fingerprint::{Fingerprint, FingerprintScene, native_fingerprints};

const NATIVE: &str = include_str!("fixtures/alpha_to_coverage/native.txt");

fn mask_rectangle(scene: &FingerprintScene) {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(1., 1., 1., 0.5).unwrap());
    style.set_alpha_mode(Graphic3dAlphaMode::Mask, 0.5);
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 6, 0, 0, 0)
            .unwrap(),
    );
    let (lo, hi) = (-20.3, 20.3);
    for [x, y] in [[lo, lo], [hi, lo], [hi, hi], [lo, lo], [hi, hi], [lo, hi]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    let structure = scene.structure();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&style.cast());
    group.add_primitive_array(&array, true);
    structure.display();
}

fn dump(scene: &FingerprintScene, samples: i32, coverage: bool) -> (String, Fingerprint) {
    let actual = scene.dump(&mut |params| {
        params.nb_msaa_samples = samples;
        params.to_enable_alpha_to_coverage = coverage;
    });
    let label = format!(
        "msaa={samples} coverage={}",
        if coverage { "on" } else { "off" }
    );
    (label, actual)
}

#[test]
fn coverage_toggle_matches_native_without_multisampling_and_with_4x_msaa_off() {
    let scene = FingerprintScene::new();
    mask_rectangle(&scene);
    for (samples, coverage) in [(0, false), (0, true), (4, false)] {
        let (label, actual) = dump(&scene, samples, coverage);
        let [sw, host] = native_fingerprints(NATIVE, &label);
        assert_eq!(sw, host, "native drivers disagree on {label}");
        assert_eq!(actual, sw, "{label}");
    }
}

/// sRGB encoding of a linear intensity, as an 8-bit level before rounding.
fn srgb_level(linear: f64) -> f64 {
    let encoded = if linear <= 0.0031308 {
        12.92 * linear
    } else {
        1.055 * martensite_cmath::pow(linear, 1. / 2.4) - 0.055
    };
    255. * encoded
}

/// Cross-driver reference (user decision 2026-10-27): the native drivers disagree on the resolved
/// interior level (llvmpipe 0xbb, radeonsi 0xbc) and each driver's sample-mask dither keeps a
/// different half of the one-sample edge pixels. Alpha 0.5 covers 2 of 4 samples, so the interior
/// must resolve to sRGB(0.5) = 187.5 within one 8-bit step; the colour histogram must otherwise
/// equal the native one (dropped and kept edge pixels alike); pixel positions are not compared.
#[test]
fn coverage_follows_alpha_with_4x_msaa_within_the_analytic_bound() {
    let scene = FingerprintScene::new();
    mask_rectangle(&scene);
    let (label, actual) = dump(&scene, 4, true);
    let [sw, host] = native_fingerprints(NATIVE, &label);
    // The interior is the most frequent lit colour (the background stays black).
    let lit = |print: &Fingerprint| {
        let lit = print.counts.iter().filter(|(color, _)| *color != "000000");
        lit.max_by_key(|(_, count)| **count)
            .map(|(color, count)| (color.clone(), *count))
            .unwrap()
    };
    let interior = |print: &Fingerprint| u8::from_str_radix(&lit(print).0[..2], 16).unwrap();
    let expected = srgb_level(0.5);
    for print in [&sw, &host, &actual] {
        let level = f64::from(interior(print));
        assert!(
            (level - expected).abs() <= 1.,
            "{label}: interior {level} in {print:?}"
        );
    }
    let without_interior = |print: &Fingerprint| {
        let mut counts = print.counts.clone();
        counts.retain(|color, _| u8::from_str_radix(&color[..2], 16).unwrap() != interior(print));
        counts
    };
    assert_eq!(without_interior(&actual), without_interior(&sw), "{label}");
    assert_eq!(without_interior(&sw), without_interior(&host), "{label}");
    assert_eq!(lit(&actual).1, lit(&sw).1, "{label}");
    // Coverage changed the image: without it the interior stays white.
    let (_, off) = dump(&scene, 4, false);
    assert_eq!(off.counts.get("ffffff"), Some(&1600));
}
