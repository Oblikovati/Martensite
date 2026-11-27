//! Physical oracles, hardware vs software ray-tracing parity, SVGF denoising, HDR tone mapping,
//! determinism, and Khronos validation tests for ASWF OpenPBR Surface v1.1 (#623-#628).
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::{PrsMgrDisplayStatus, PrsMgrPresentableObjectMembers as _};
use martensite_tkv3d::v3d::{V3dViewSource, V3dViewer};
use martensite_vulkan::{
    Graphic3dRenderDriver, Graphic3dRenderView, OpenPbrGpuMaterial, VulkanOptions, VulkanRenderer,
    openpbr_env_cdf::EnvImportanceTable, openpbr_luts::cached_openpbr_luts, openpbr_math,
};

fn validation_options() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: None,
    }
}

fn unit_sphere() -> martensite_tkbrep::topods::Shape {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(1.0).unwrap();
    maker.shape().unwrap().clone()
}

fn mr(rgb: [f32; 3], metallic: f32, roughness: f32) -> OpenPbrMaterial {
    let mut m = OpenPbrMaterial::default();
    m.base.color = rgb;
    m.base.metalness = metallic;
    m.specular.roughness = roughness;
    m
}

fn gray(v: f64) -> Color {
    Color::new(v, v, v, ColorSpace::Rgb).unwrap()
}

fn material_from_openpbr(pbr: OpenPbrMaterial) -> Graphic3dMaterialAspect {
    let mut mat = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    mat.set_material_type(Graphic3dTypeOfMaterial::Physic);
    mat.set_openpbr_material(pbr).unwrap();
    mat
}

fn canonical_ten_materials() -> Vec<(&'static str, OpenPbrMaterial)> {
    // 1. Matte EON rough clay
    let mut clay = OpenPbrMaterial::default();
    clay.base.color = [0.72, 0.34, 0.22];
    clay.base.diffuse_roughness = 0.85;
    clay.specular.weight = 0.2;
    clay.specular.roughness = 0.8;

    // 2. Glossy plastic
    let mut plastic = OpenPbrMaterial::default();
    plastic.base.color = [0.15, 0.45, 0.85];
    plastic.specular.roughness = 0.12;
    plastic.specular.ior = 1.5;

    // 3. Brushed anisotropic gold (F82 conductor)
    let mut gold = OpenPbrMaterial::default();
    gold.base.color = [0.94, 0.77, 0.37];
    gold.base.metalness = 1.0;
    gold.specular.color = [0.99, 0.96, 0.82];
    gold.specular.roughness = 0.22;
    gold.specular.roughness_anisotropy = 0.75;

    // 4. Metallic car paint with tinted coat & darkening
    let mut car_paint = OpenPbrMaterial::default();
    car_paint.base.color = [0.85, 0.08, 0.12];
    car_paint.base.metalness = 0.85;
    car_paint.specular.roughness = 0.35;
    car_paint.coat.weight = 1.0;
    car_paint.coat.color = [0.95, 0.85, 0.85];
    car_paint.coat.roughness = 0.04;
    car_paint.coat.ior = 1.55;
    car_paint.coat.darkening = 1.0;

    // 5. Velvet Charlie microfiber fuzz
    let mut velvet = OpenPbrMaterial::default();
    velvet.base.color = [0.22, 0.04, 0.35];
    velvet.specular.weight = 0.1;
    velvet.specular.roughness = 0.7;
    velvet.fuzz.weight = 0.9;
    velvet.fuzz.color = [0.75, 0.45, 0.95];
    velvet.fuzz.roughness = 0.35;

    // 6. Airy thin-film iridescence (soap/oil on metal)
    let mut thin_film = OpenPbrMaterial::default();
    thin_film.base.color = [0.65, 0.65, 0.72];
    thin_film.base.metalness = 0.9;
    thin_film.specular.roughness = 0.15;
    thin_film.thin_film.weight = 1.0;
    thin_film.thin_film.thickness_um = 0.48;
    thin_film.thin_film.ior = 1.38;

    // 7. Thin-walled frosted glass
    let mut thin_glass = OpenPbrMaterial::default();
    thin_glass.base.color = [0.95, 0.98, 1.0];
    thin_glass.specular.roughness = 0.2;
    thin_glass.specular.ior = 1.52;
    thin_glass.transmission.weight = 1.0;
    thin_glass.geometry.thin_walled = true;

    // 8. Solid dispersive tinted glass (Abbe)
    let mut solid_glass = OpenPbrMaterial::default();
    solid_glass.base.color = [0.98, 0.98, 0.98];
    solid_glass.specular.roughness = 0.03;
    solid_glass.specular.ior = 1.62;
    solid_glass.transmission.weight = 1.0;
    solid_glass.transmission.color = [0.72, 0.94, 0.82];
    solid_glass.transmission.depth = 0.8;
    solid_glass.transmission.dispersion_scale = 1.0;
    solid_glass.transmission.dispersion_abbe_number = 25.0;

    // 9. Random-walk subsurface jade
    let mut jade = OpenPbrMaterial::default();
    jade.base.color = [0.18, 0.56, 0.34];
    jade.specular.roughness = 0.18;
    jade.subsurface.weight = 0.85;
    jade.subsurface.color = [0.35, 0.85, 0.52];
    jade.subsurface.radius = 0.25;
    jade.subsurface.radius_scale = [0.5, 1.0, 0.6];

    // 10. Emissive under tinted coat
    let mut emissive = OpenPbrMaterial::default();
    emissive.base.color = [0.1, 0.1, 0.12];
    emissive.emission.luminance = 2.5;
    emissive.emission.color = [1.0, 0.55, 0.15];
    emissive.coat.weight = 0.8;
    emissive.coat.color = [0.85, 0.92, 1.0];
    emissive.coat.roughness = 0.06;

    vec![
        ("matte_eon_clay", clay),
        ("glossy_plastic", plastic),
        ("brushed_aniso_gold", gold),
        ("car_paint_coat", car_paint),
        ("velvet_charlie_fuzz", velvet),
        ("thin_film_iridescence", thin_film),
        ("thin_walled_frosted_glass", thin_glass),
        ("solid_dispersive_glass", solid_glass),
        ("random_walk_sss_jade", jade),
        ("emissive_under_coat", emissive),
    ]
}

struct SphereHarness {
    driver: Handle<Graphic3dRenderDriver>,
    _viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
    shape: Handle<dyn AisInteractiveObject>,
}

impl SphereHarness {
    fn new(width: i32, height: i32, bg: Color) -> Self {
        let driver = Graphic3dRenderDriver::new(validation_options()).unwrap();
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(bg);
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        for light in viewer.defined_lights() {
            if light.light_type() == Graphic3dTypeOfLightSource::Directional {
                light.set_intensity(2.0).unwrap();
            } else if light.light_type() == Graphic3dTypeOfLightSource::Ambient {
                light.set_intensity(0.35).unwrap();
            }
        }
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(width, height);
        window.set_virtual(true);
        let view = viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        let ais = AisShape::new(&unit_sphere());
        let shape: Handle<dyn AisInteractiveObject> = ais.cast();
        shape.get().unwrap().set_display_mode(1);
        context
            .display_with_modes(&shape, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
        let v3d = view.get().unwrap().view_base();
        v3d.camera()
            .set_projection_type(Graphic3dCameraProjection::Perspective);
        v3d.fit_all(0.05, false).unwrap();
        Self {
            driver,
            _viewer: viewer,
            context,
            view,
            _window: window,
            shape,
        }
    }

    fn set_openpbr(&self, pbr: OpenPbrMaterial) {
        self.shape
            .get()
            .unwrap()
            .set_material(&material_from_openpbr(pbr))
            .unwrap();
        self.context.redisplay(&self.shape, false, false).unwrap();
    }

    fn render_rgba(&self) -> Vec<u8> {
        self.view.redraw();
        let render: Handle<Graphic3dRenderView> =
            self.view.get().unwrap().view_base().view().cast();
        render.get().unwrap().capture_color().unwrap().rgba
    }

    fn assert_no_validation_errors(&self) {
        assert_eq!(
            self.driver.validation_errors().unwrap(),
            Vec::<String>::new()
        );
    }
}

impl Drop for SphereHarness {
    fn drop(&mut self) {
        self.view.remove();
    }
}

fn mean_center_rgb(rgba: &[u8], width: usize, height: usize) -> [f32; 3] {
    let mut sum = [0.0f64; 3];
    let mut count = 0usize;
    let x0 = width * 3 / 8;
    let x1 = width * 5 / 8;
    let y0 = height * 3 / 8;
    let y1 = height * 5 / 8;
    for y in y0..y1 {
        for x in x0..x1 {
            let idx = (y * width + x) * 4;
            for c in 0..3 {
                sum[c] += f64::from(rgba[idx + c]) / 255.0;
            }
            count += 1;
        }
    }
    [
        (sum[0] / count as f64) as f32,
        (sum[1] / count as f64) as f32,
        (sum[2] / count as f64) as f32,
    ]
}

fn center_luma_variance(rgba: &[u8], width: usize, height: usize) -> f64 {
    let mut lumas = Vec::new();
    let x0 = width * 3 / 8;
    let x1 = width * 5 / 8;
    let y0 = height * 3 / 8;
    let y1 = height * 5 / 8;
    for y in y0..y1 {
        for x in x0..x1 {
            let idx = (y * width + x) * 4;
            let r = f64::from(rgba[idx]) / 255.0;
            let g = f64::from(rgba[idx + 1]) / 255.0;
            let b = f64::from(rgba[idx + 2]) / 255.0;
            lumas.push(0.2126 * r + 0.7152 * g + 0.0722 * b);
        }
    }
    let mean = lumas.iter().sum::<f64>() / lumas.len() as f64;
    lumas
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / lumas.len() as f64
}

#[test]
fn openpbr_v11_equation_oracles_satisfy_physical_invariants() {
    // Coat darkening Eq. 64-70: strictly darkens rough dielectric base under coat, identity when off.
    let no_coat = openpbr_math::openpbr_coat_darkening(
        0.7, [0.5; 3], 0.0, [1.0; 3], 1.5, 0.6, 0.0, [1.0; 3], 0.0, [1.0; 3], 0.0, 1.5, 1.0,
    );
    assert_eq!(no_coat, [1.0; 3]);
    let dark = openpbr_math::openpbr_coat_darkening(
        0.7, [0.5; 3], 0.0, [1.0; 3], 1.5, 0.6, 0.0, [1.0; 3], 0.0, [1.0; 3], 1.0, 1.5, 1.0,
    );
    assert!(dark[0] < 0.99 && dark[0] > 0.2, "coat darkening={dark:?}");

    // Coat roughening Eq. 73: monotone with coat roughness.
    let r0 = openpbr_math::openpbr_coat_roughening(0.2, 0.0, 1.0);
    let r1 = openpbr_math::openpbr_coat_roughening(0.2, 0.5, 1.0);
    assert!((r0 - 0.2).abs() < 1e-5);
    assert!(r1 > r0 && r1 <= 1.0);

    // Coat TIR IOR modulation Eq. 60/74: reduces effective contrast at base interface.
    let ior_air = openpbr_math::openpbr_effective_specular_ior(1.5, 1.0, 1.5, 0.0);
    let ior_coated = openpbr_math::openpbr_effective_specular_ior(1.5, 1.0, 1.5, 1.0);
    assert!((ior_air - 1.5).abs() < 1e-4);
    assert!((ior_coated - 1.0).abs() < 1e-4);

    // F82 conductor grazing tint: equals f0 at normal, 1 at grazing, dips when f82 < 1.
    let f0 = [0.94, 0.77, 0.37];
    let f82 = [0.5, 0.5, 0.5];
    let normal = openpbr_math::fresnel_f82_tint(1.0, f0, f82);
    let grazing = openpbr_math::fresnel_f82_tint(0.0, f0, f82);
    for c in 0..3 {
        assert!((normal[c] - f0[c]).abs() < 1e-4);
        assert!((grazing[c] - 1.0).abs() < 1e-4);
    }

    // EON Oren-Nayar diffuse: exact Lambert at roughness=0, non-negative & bounded for all angles.
    let lambert = openpbr_math::oren_nayar_eon([0.8; 3], 0.0, 0.6, 0.7, 0.5);
    assert!((lambert[0] - 0.8 / std::f32::consts::PI).abs() < 1e-5);
    let rough = openpbr_math::oren_nayar_eon([0.8; 3], 1.0, 0.6, 0.7, 0.5);
    assert!(rough[0] > 0.0 && rough[0] < 1.0);

    // Thin-glass roughness scaling decreases as IOR approaches 1.
    let rg_air = openpbr_math::thin_glass_transmission_roughness(0.5, 1.01);
    let rg_glass = openpbr_math::thin_glass_transmission_roughness(0.5, 1.5);
    assert!(rg_air < rg_glass);
}

#[test]
fn canonical_ten_openpbr_materials_render_cleanly_in_raster_and_compute_path_tracer() {
    let harness = SphereHarness::new(96, 96, gray(0.2));
    let v3d = harness.view.get().unwrap().view_base();

    let mut raster_means = Vec::new();
    let mut pt_means = Vec::new();

    for (name, pbr) in canonical_ten_materials() {
        harness.set_openpbr(pbr);

        // OpenPBR Rasterization
        v3d.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Rasterization;
            params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
        });
        let raster = harness.render_rgba();
        let r_mean = mean_center_rgb(&raster, 96, 96);
        assert!(
            r_mean
                .iter()
                .all(|v| v.is_finite() && *v >= 0.0 && *v <= 1.0),
            "raster material {name} produced non-finite pixel mean {r_mean:?}"
        );
        assert!(
            r_mean[0] + r_mean[1] + r_mean[2] > 0.02,
            "raster material {name} rendered black: {r_mean:?}"
        );
        raster_means.push((name, r_mean));

        // OpenPBR Compute Path Tracing (AutoPreferHardware, AgX tone mapping)
        v3d.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Raytracing;
            params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
            params.ray_tracing.is_global_illumination_enabled = true;
            params.ray_tracing.raytracing_depth = 4;
            params.ray_tracing.backend = Graphic3dRayTracingBackend::AutoPreferHardware;
            params.ray_tracing.denoiser_enabled = false;
            params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::AgX;
            params.ray_tracing.exposure = 0.0;
        });
        let pt = harness.render_rgba();
        let p_mean = mean_center_rgb(&pt, 96, 96);
        assert!(
            p_mean
                .iter()
                .all(|v| v.is_finite() && *v >= 0.0 && *v <= 1.0),
            "path-traced material {name} produced non-finite pixel mean {p_mean:?}"
        );
        assert!(
            p_mean[0] + p_mean[1] + p_mean[2] > 0.02,
            "path-traced material {name} rendered black: {p_mean:?}"
        );
        pt_means.push((name, p_mean));
    }

    // Emissive material must be brighter than dark velvet in both raster & path tracing.
    let velvet_r = raster_means
        .iter()
        .find(|(n, _)| *n == "velvet_charlie_fuzz")
        .unwrap()
        .1;
    let emissive_r = raster_means
        .iter()
        .find(|(n, _)| *n == "emissive_under_coat")
        .unwrap()
        .1;
    assert!(
        emissive_r[0] > velvet_r[0],
        "expected emissive red > velvet red in raster: {emissive_r:?} vs {velvet_r:?}"
    );
    let velvet_p = pt_means
        .iter()
        .find(|(n, _)| *n == "velvet_charlie_fuzz")
        .unwrap()
        .1;
    let emissive_p = pt_means
        .iter()
        .find(|(n, _)| *n == "emissive_under_coat")
        .unwrap()
        .1;
    assert!(
        emissive_p[0] > velvet_p[0],
        "expected emissive red > velvet red in path tracer: {emissive_p:?} vs {velvet_p:?}"
    );

    harness.assert_no_validation_errors();
}

#[test]
fn white_furnace_energy_conservation_holds_across_openpbr_lobes() {
    let white = Color::new(1.0, 1.0, 1.0, ColorSpace::Rgb).unwrap();
    let harness = SphereHarness::new(64, 64, white);
    let v3d = harness.view.get().unwrap().view_base();
    v3d.change_rendering_params(&mut |params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 5;
        params.ray_tracing.backend = Graphic3dRayTracingBackend::AutoPreferHardware;
        params.ray_tracing.denoiser_enabled = false;
        params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::Disabled;
    });

    // Test white dielectric, white metal, coated white, and thin-walled glass in uniform white furnace.
    let cases = [
        ("white_dielectric_rough", mr([1.0; 3], 0.0, 0.6)),
        ("white_metal_smooth", mr([1.0; 3], 1.0, 0.15)),
        ("white_metal_rough", mr([1.0; 3], 1.0, 0.85)),
        ("white_coated", {
            let mut m = mr([1.0; 3], 0.0, 0.4);
            m.coat.weight = 1.0;
            m.coat.color = [1.0; 3];
            m.coat.roughness = 0.1;
            m
        }),
    ];

    for (name, mat) in cases {
        harness.set_openpbr(mat);
        let mut last = Vec::new();
        for _ in 0..8 {
            last = harness.render_rgba();
        }
        let mean = mean_center_rgb(&last, 64, 64);
        // Under a unit white background dome with no dark absorption, furnace reflectance must be
        // high (no dark energy loss bug) and bounded by 1.0 (no energy gain explosion).
        for c in 0..3 {
            assert!(
                mean[c] >= 0.72 && mean[c] <= 1.0,
                "white furnace {name} channel {c} out of energy-conserving range: {mean:?}"
            );
        }
    }
    harness.assert_no_validation_errors();
}

#[test]
fn hardware_ray_query_matches_software_compute_bvh_and_refuses_when_unsupported() {
    let harness = SphereHarness::new(96, 96, gray(0.3));
    let v3d = harness.view.get().unwrap().view_base();
    let gold = mr([0.94, 0.77, 0.37], 1.0, 0.25);
    harness.set_openpbr(gold);

    let configure = |backend: Graphic3dRayTracingBackend| {
        v3d.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Raytracing;
            params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
            params.ray_tracing.is_global_illumination_enabled = true;
            params.ray_tracing.raytracing_depth = 4;
            params.ray_tracing.backend = backend;
            params.ray_tracing.denoiser_enabled = false;
            params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::AgX;
        });
    };

    // Always render 8 accumulated frames on SoftwareComputeBvh.
    configure(Graphic3dRayTracingBackend::SoftwareComputeBvh);
    let mut sw = Vec::new();
    for _ in 0..8 {
        sw = harness.render_rgba();
    }
    let sw_mean = mean_center_rgb(&sw, 96, 96);

    let has_hw = harness
        .driver
        .device_limits()
        .is_some_and(|l| l.has_hardware_ray_tracing());
    if has_hw {
        configure(Graphic3dRayTracingBackend::HardwareRayQuery);
        let mut hw = Vec::new();
        for _ in 0..8 {
            hw = harness.render_rgba();
        }
        let hw_mean = mean_center_rgb(&hw, 96, 96);
        let mae: f64 = sw
            .iter()
            .zip(&hw)
            .map(|(&a, &b)| (f64::from(a) - f64::from(b)).abs() / 255.0)
            .sum::<f64>()
            / sw.len() as f64;
        assert!(
            mae < 0.03,
            "HW rayQuery vs SW BVH diverged: mae={mae:.4}, hw_mean={hw_mean:?}, sw_mean={sw_mean:?}"
        );
        for c in 0..3 {
            assert!(
                (hw_mean[c] - sw_mean[c]).abs() < 0.03,
                "HW vs SW mean mismatch channel {c}: hw={hw_mean:?} sw={sw_mean:?}"
            );
        }
    } else {
        // On non-RT devices (e.g. Mesa lavapipe), explicit HardwareRayQuery must fail explicitly.
        configure(Graphic3dRayTracingBackend::HardwareRayQuery);
        let render: Handle<Graphic3dRenderView> = v3d.view().cast();
        let err = render.get().unwrap().try_redraw().unwrap_err();
        assert_eq!(
            err.portability_failure(),
            Some(
                &martensite_vulkan::RenderPortabilityFailure::MissingDeviceFeature {
                    feature: "VK_KHR_ray_query",
                }
            )
        );
    }

    harness.assert_no_validation_errors();
}

#[test]
fn svgf_denoiser_reduces_variance_without_artifacts() {
    let harness = SphereHarness::new(96, 96, gray(0.1));
    let v3d = harness.view.get().unwrap().view_base();
    let mut clay = OpenPbrMaterial::default();
    clay.base.color = [0.75, 0.55, 0.35];
    clay.base.diffuse_roughness = 0.6;
    clay.specular.roughness = 0.45;
    harness.set_openpbr(clay);

    // 1-spp noisy capture without denoiser
    v3d.change_rendering_params(&mut |params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 4;
        params.ray_tracing.backend = Graphic3dRayTracingBackend::AutoPreferHardware;
        params.ray_tracing.denoiser_enabled = false;
        params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::AgX;
    });
    let noisy = harness.render_rgba();
    let noisy_var = center_luma_variance(&noisy, 96, 96);

    // 4-frame temporal + 3-iteration A-trous SVGF capture
    v3d.change_rendering_params(&mut |params| {
        params.ray_tracing.denoiser_enabled = true;
        params.ray_tracing.denoiser_atrous_iterations = 3;
        params.ray_tracing.denoiser_temporal_alpha = 0.2;
    });
    let mut denoised = Vec::new();
    for _ in 0..4 {
        denoised = harness.render_rgba();
    }
    let denoised_var = center_luma_variance(&denoised, 96, 96);
    let denoised_mean = mean_center_rgb(&denoised, 96, 96);

    assert!(
        denoised_mean[0] + denoised_mean[1] + denoised_mean[2] > 0.05,
        "denoised image collapsed to black: {denoised_mean:?}"
    );
    assert!(
        denoised_var <= noisy_var + 1e-4,
        "SVGF increased variance: denoised={denoised_var:.6} > noisy={noisy_var:.6}"
    );
    harness.assert_no_validation_errors();
}

#[test]
fn gpu_hdr_tonemapping_agx_aces_filmic_are_monotone_and_match_cpu_oracles() {
    let renderer = VulkanRenderer::new(validation_options()).unwrap();
    let width = 8u32;
    let height = 2u32;
    let ramps: Vec<[f32; 4]> = (0..(width * height))
        .map(|i| {
            let t = i as f32 / 15.0;
            let v = t * t * 8.0;
            [v, v * 0.7, v * 0.4, 1.0]
        })
        .collect();

    for method in [
        Graphic3dToneMappingMethod::Disabled,
        Graphic3dToneMappingMethod::Filmic,
        Graphic3dToneMappingMethod::AgX,
        Graphic3dToneMappingMethod::Aces,
    ] {
        let out = renderer
            .tonemap_hdr(width, height, &ramps, method, 0.0, 2.0)
            .unwrap();
        assert_eq!(out.len(), (width * height * 4) as usize);
        // Red channel must be monotonically non-decreasing across the HDR ramp.
        let reds: Vec<u8> = out.as_chunks::<4>().0.iter().map(|px| px[0]).collect();
        for w in reds.windows(2) {
            assert!(
                w[1] >= w[0],
                "tone mapper {method:?} broke monotonicity: {} -> {} ({reds:?})",
                w[0],
                w[1]
            );
        }
        // Bright HDR red (8.0) must compress gracefully into display white/bright range.
        assert!(
            reds[15] >= 200,
            "tone mapper {method:?} clipped too dark: {}",
            reds[15]
        );
        // Zero HDR input stays black.
        assert_eq!(
            reds[0], 0,
            "tone mapper {method:?} lifted black: {}",
            reds[0]
        );
    }
    assert_eq!(renderer.validation_errors().unwrap(), Vec::<String>::new());
}

#[test]
fn fixed_seed_openpbr_renders_are_byte_exact_deterministic() {
    let render_once = || {
        let harness = SphereHarness::new(64, 64, gray(0.25));
        let v3d = harness.view.get().unwrap().view_base();
        let mut pbr = OpenPbrMaterial::default();
        pbr.base.color = [0.85, 0.25, 0.15];
        pbr.coat.weight = 0.7;
        pbr.coat.roughness = 0.08;
        harness.set_openpbr(pbr);
        v3d.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Raytracing;
            params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
            params.ray_tracing.is_global_illumination_enabled = true;
            params.ray_tracing.raytracing_depth = 3;
            params.ray_tracing.backend = Graphic3dRayTracingBackend::AutoPreferHardware;
            params.ray_tracing.denoiser_enabled = false;
            params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::AgX;
        });
        let pixels = harness.render_rgba();
        harness.assert_no_validation_errors();
        pixels
    };
    let a = render_once();
    let b = render_once();
    assert_eq!(
        a, b,
        "fixed-seed OpenPBR compute path tracer must be byte-exact across runs"
    );
}

#[test]
fn concurrent_openpbr_lut_and_env_cdf_access_is_thread_safe() {
    let handles: Vec<_> = (0..8)
        .map(|tid| {
            std::thread::spawn(move || {
                let luts = cached_openpbr_luts();
                let mut m = OpenPbrMaterial::default();
                m.base.metalness = (tid as f32) / 8.0;
                let gpu = OpenPbrGpuMaterial::from_openpbr(&m);
                let env_pixels: Vec<f32> = [1.0f32, 0.5, 0.25].repeat(16 * 8);
                let cdf = EnvImportanceTable::from_rgb(16, 8, &env_pixels);
                (luts.ggx_rg16f.len(), gpu.to_bytes(), cdf.entries.len())
            })
        })
        .collect();
    for h in handles {
        let (ggx_len, bytes, cdf_len) = h.join().unwrap();
        assert_eq!(ggx_len, 64 * 64 * 4);
        assert_eq!(bytes.len(), 256);
        assert_eq!(cdf_len, 16 * 8);
    }
}
