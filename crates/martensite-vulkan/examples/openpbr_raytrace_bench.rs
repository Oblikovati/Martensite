//! Benchmark measuring OpenPBR Vulkan rasterization, compute path tracing (hardware
//! `VK_KHR_ray_query` vs software BVH), 3-pass SVGF denoiser overhead, and standalone GPU HDR
//! tone mapping across two CAD scenes matching the Blender 4.0 comparison benchmark.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::quantity::{Color, ColorSpace};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Transform3, Vector3};
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape as _, BRepBuilderAPITransform,
};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::{
    PrsMgrDisplayStatus, PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::{
    Graphic3dRenderDriver, Graphic3dRenderView, VulkanOptions, VulkanRenderer,
};
use std::time::Instant;

fn translate_shape(shape: &Shape, dx: f64, dy: f64, dz: f64) -> Shape {
    let mut trsf = Transform3::default();
    trsf.set_translation(Vector3::new(dx, dy, dz));
    let mut xform = BRepBuilderAPITransform::with_shape(shape, trsf, true, false).unwrap();
    xform.shape().unwrap().clone()
}

fn count_mesh_triangles(shape: &Shape) -> usize {
    let mut exp = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut total = 0usize;
    while exp.more() {
        if let Ok(face) = topods::face(exp.current())
            && let Ok((tri, _)) = BRepTool::triangulation(&face, mesh_purpose::NONE)
            && let Some(t) = tri.get()
        {
            total += t.nb_triangles() as usize;
        }
        let _ = exp.next();
    }
    total
}

fn make_aspect(pbr: OpenPbrMaterial) -> Graphic3dMaterialAspect {
    let mut mat = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    mat.set_material_type(Graphic3dTypeOfMaterial::Physic);
    mat.set_openpbr_material(pbr).unwrap();
    mat
}

fn car_paint_mat() -> OpenPbrMaterial {
    let mut m = OpenPbrMaterial::default();
    m.base.color = [0.85, 0.12, 0.15];
    m.base.metalness = 0.85;
    m.specular.roughness = 0.25;
    m.coat.weight = 1.0;
    m.coat.roughness = 0.04;
    m
}

fn build_torus_shapes() -> Vec<(Shape, OpenPbrMaterial)> {
    let mut torus = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus::new(2.0, 0.65).unwrap();
    vec![(torus.shape().unwrap().clone(), car_paint_mat())]
}

fn build_showcase_5mat_shapes() -> Vec<(Shape, OpenPbrMaterial)> {
    let mut torus =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeTorus::new(0.95, 0.32).unwrap();
    let torus = translate_shape(torus.shape().unwrap(), -2.6, 0.0, 0.0);

    let mut gold_s = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(0.95).unwrap();
    let mut gold = OpenPbrMaterial::default();
    gold.base.color = [0.944, 0.776, 0.373];
    gold.base.metalness = 1.0;
    gold.specular.roughness = 0.18;

    let mut glass_s = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(0.95).unwrap();
    let glass_shape = translate_shape(glass_s.shape().unwrap(), 2.6, 0.0, 0.0);
    let mut glass = OpenPbrMaterial::default();
    glass.base.color = [0.94, 0.99, 0.97];
    glass.specular.ior = 1.52;
    glass.specular.roughness = 0.03;
    glass.transmission.weight = 1.0;
    glass.transmission.color = [0.94, 0.99, 0.97];

    let mut cyl = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder::new(0.75, 1.6);
    let cyl_shape = translate_shape(cyl.shape().unwrap(), -1.3, 2.2, -0.8);
    let mut terracotta = OpenPbrMaterial::default();
    terracotta.base.color = [0.72, 0.28, 0.16];
    terracotta.base.diffuse_roughness = 0.8;
    terracotta.specular.roughness = 0.75;

    let mut velvet_s = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(0.95).unwrap();
    let velvet_shape = translate_shape(velvet_s.shape().unwrap(), 1.3, 2.2, 0.0);
    let mut velvet = OpenPbrMaterial::default();
    velvet.base.color = [0.14, 0.16, 0.48];
    velvet.specular.roughness = 0.85;
    velvet.fuzz.weight = 0.9;
    velvet.fuzz.roughness = 0.45;
    velvet.fuzz.color = [0.85, 0.88, 1.0];

    let mut floor = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        Point3::new(-12.0, -11.0, -1.25),
        24.0,
        24.0,
        0.2,
    )
    .unwrap();
    let mut floor_mat = OpenPbrMaterial::default();
    floor_mat.base.color = [0.45, 0.45, 0.48];
    floor_mat.specular.roughness = 0.65;

    vec![
        (torus, car_paint_mat()),
        (gold_s.shape().unwrap().clone(), gold),
        (glass_shape, glass),
        (cyl_shape, terracotta),
        (velvet_shape, velvet),
        (floor.shape().unwrap().clone(), floor_mat),
    ]
}

fn bench_scene(
    driver: &Handle<Graphic3dRenderDriver>,
    scene_name: &str,
    items: &[(Shape, OpenPbrMaterial)],
    resolutions: &[(i32, i32)],
    has_hw: bool,
) {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    viewer.set_default_background_color(Color::new(0.2, 0.2, 0.25, ColorSpace::Rgb).unwrap());
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(512, 512);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    view.set_window(&window.cast(), 0).unwrap();

    for (shape, pbr) in items {
        let ais = AisShape::new(shape);
        ais.set_material(&make_aspect(*pbr)).unwrap();
        let obj: Handle<dyn AisInteractiveObject> = ais.cast();
        obj.get().unwrap().set_display_mode(1);
        context
            .display_with_modes(&obj, 1, 0, false, PrsMgrDisplayStatus::None)
            .unwrap();
    }
    let tris: usize = items.iter().map(|(s, _)| count_mesh_triangles(s)).sum();
    println!("SCENE {scene_name} triangles={tris}");

    let v3d = view.get().unwrap().view_base();
    v3d.camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    v3d.fit_all(0.05, false).unwrap();
    let render_view: Handle<Graphic3dRenderView> = v3d.view().cast();

    for &(w, h) in resolutions {
        window.set_size(w, h);
        view.must_be_resized();

        // 1. OpenPBR real-time rasterization
        v3d.change_rendering_params(&mut |params| {
            params.method = Graphic3dRenderingMode::Rasterization;
            params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
        });
        let t_w = Instant::now();
        view.redraw();
        let _ = render_view.get().unwrap().capture_color().unwrap();
        let warm = t_w.elapsed().as_secs_f64() * 1000.0;
        let frames = 20u32;
        let t0 = Instant::now();
        for _ in 0..frames {
            view.redraw();
            let _ = render_view.get().unwrap().capture_color().unwrap();
        }
        let steady = t0.elapsed().as_secs_f64() * 1000.0 / f64::from(frames);
        println!(
            "BENCH {scene_name} engine=MARTENSITE_RASTER res={w}x{h} spp=1 warmup_ms={warm:.2} steady_ms={steady:.2}"
        );

        // 2. Compute path tracer (SW BVH and HW VK_KHR_ray_query)
        let mut modes = vec![
            (
                "MARTENSITE_PT_SW_RAW",
                Graphic3dRayTracingBackend::SoftwareComputeBvh,
                false,
            ),
            (
                "MARTENSITE_PT_SW_SVGF",
                Graphic3dRayTracingBackend::SoftwareComputeBvh,
                true,
            ),
        ];
        if has_hw {
            modes.push((
                "MARTENSITE_PT_HW_RAW",
                Graphic3dRayTracingBackend::HardwareRayQuery,
                false,
            ));
            modes.push((
                "MARTENSITE_PT_HW_SVGF",
                Graphic3dRayTracingBackend::HardwareRayQuery,
                true,
            ));
        }
        for (engine, backend, denoiser) in modes {
            v3d.change_rendering_params(&mut |params| {
                params.method = Graphic3dRenderingMode::Raytracing;
                params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
                params.ray_tracing.is_global_illumination_enabled = true;
                params.ray_tracing.raytracing_depth = 4;
                params.ray_tracing.backend = backend;
                params.ray_tracing.denoiser_enabled = denoiser;
                params.ray_tracing.denoiser_atrous_iterations = 3;
                params.ray_tracing.tone_mapping_method = Graphic3dToneMappingMethod::AgX;
            });
            let t_w = Instant::now();
            view.redraw();
            let _ = render_view.get().unwrap().capture_color().unwrap();
            let warm = t_w.elapsed().as_secs_f64() * 1000.0;

            let samples = 16u32;
            let t0 = Instant::now();
            for _ in 0..samples {
                view.redraw();
            }
            let _ = render_view.get().unwrap().capture_color().unwrap();
            let total_16spp_ms = t0.elapsed().as_secs_f64() * 1000.0;
            let per_spp_ms = total_16spp_ms / f64::from(samples);

            let readback_frames = 12u32;
            let t1 = Instant::now();
            for _ in 0..readback_frames {
                view.redraw();
                let _ = render_view.get().unwrap().capture_color().unwrap();
            }
            let with_readback_ms = t1.elapsed().as_secs_f64() * 1000.0 / f64::from(readback_frames);
            let mrays_s = (f64::from(w * h) * 3.0) / (per_spp_ms / 1000.0) / 1.0e6;
            println!(
                "BENCH {scene_name} engine={engine} res={w}x{h} denoise={denoiser} warmup_ms={warm:.2} \
                 steady_1spp_ms={per_spp_ms:.2} steady_16spp_ms={total_16spp_ms:.2} \
                 readback_1spp_ms={with_readback_ms:.2} est_mrays_s={mrays_s:.1}"
            );
        }
    }

    view.remove();
}

fn main() {
    let driver = Graphic3dRenderDriver::new(VulkanOptions::default()).unwrap();
    let device = driver.device_name();
    let limits = driver.device_limits().unwrap();
    let has_hw = limits.has_hardware_ray_tracing();
    println!("DEVICE {device}");
    println!("HAS_HW_RT {has_hw}");

    let resolutions = if device.contains("llvmpipe") {
        vec![(512, 512)]
    } else {
        vec![(512, 512), (1024, 1024)]
    };

    let torus = build_torus_shapes();
    bench_scene(&driver, "car_paint_torus", &torus, &resolutions, has_hw);

    let showcase = build_showcase_5mat_shapes();
    bench_scene(&driver, "showcase_5mat", &showcase, &resolutions, has_hw);

    // Standalone 1024x1024 HDR AgX tone mapping benchmark.
    let renderer = VulkanRenderer::new(VulkanOptions::default()).unwrap();
    let hdr = vec![[1.5f32, 0.8, 0.3, 1.0]; 1024 * 1024];
    let _ = renderer
        .tonemap_hdr(1024, 1024, &hdr, Graphic3dToneMappingMethod::AgX, 0.0, 1.0)
        .unwrap();
    let t_tm = Instant::now();
    for _ in 0..5 {
        let _ = renderer
            .tonemap_hdr(1024, 1024, &hdr, Graphic3dToneMappingMethod::AgX, 0.0, 1.0)
            .unwrap();
    }
    let tm_ms = t_tm.elapsed().as_secs_f64() * 1000.0 / 5.0;
    println!("BENCH tonemap_agx_1024x1024 steady_ms={tm_ms:.2}");
}
