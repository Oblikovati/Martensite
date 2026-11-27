//! End-to-end WebGPU Core Phase 4 (#634) & Phase 5 (#635) integration tests.
//!
//! Runs against real `wgpu` (Mesa lavapipe / Vulkan / Metal / DX12) with `validation: true`
//! and asserts zero WebGPU validation errors across every CAD rendering pipeline.
use martensite_render_core::colored_quad_background::ColoredQuadBackground;
use martensite_render_core::stereo_pair::{
    StereoProgram, tests_support::composite as stereo_composite,
};
use martensite_render_core::subview_composition::RenderSubviewBlit;
use martensite_render_core::user_program::{
    RenderUserProgramFailure, UserProgramDraw, UserProgramKey, UserProgramModules, UserUniformScene,
};
use martensite_render_core::{
    Graphic3dRenderView, LineStyle, PointStyle, PrimitiveDraw, RenderBackend,
    RenderBehaviorConfiguration, RenderCappingPass, RenderClipping, RenderCoordinateConvention,
    RenderFrame, RenderLayer, RenderLineClippingBehavior, RenderLineWidthPolicy, RenderMeshEdges,
    RenderPipelinePolicies, RenderRasterPolicy, RenderTransparency, RenderVertex,
    RenderViewportGridPrecision, RenderWindowDepthBehavior, RenderWorldClippingBehavior,
    identity_clip,
};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkservice::aspect::{AspectWindow, AspectWindowMembers as _};
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection, Graphic3dGraphicDriver, Graphic3dGraphicDriverFactory,
    Graphic3dMaterialAspect, Graphic3dNameOfMaterial, Graphic3dRenderingMode,
    Graphic3dShaderProgram, Graphic3dTypeOfShadingModel,
};
use martensite_tkservice::wasm::WasmWindow;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::{
    PrsMgrDisplayStatus, PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_webgpu::realm::{WebGpuRealm, block_on};
use martensite_webgpu::shaders::total_wgsl_bytes;
use martensite_webgpu::{
    WEBGPU_DRIVER_NAME, WebGpuContext, WebGpuDriverFactory, WebGpuOptions, WebGpuRenderDriver,
    WebGpuRenderViewExt, WebGpuRenderer,
};
use std::sync::Arc;

fn test_options() -> WebGpuOptions {
    WebGpuOptions {
        validation: true,
        ..WebGpuOptions::default()
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn public_types_are_send_sync_and_wgsl_fits_budget() {
    assert_send_sync::<WebGpuContext>();
    assert_send_sync::<WebGpuRenderer>();
    assert_send_sync::<WebGpuOptions>();
    assert_send_sync::<WebGpuDriverFactory>();
    assert!(
        total_wgsl_bytes() < 150_000,
        "built-in WGSL sources exceeded 150 KB budget: {} bytes",
        total_wgsl_bytes()
    );
    assert_eq!(
        WebGpuDriverFactory::new().name(),
        AsciiString::from(WEBGPU_DRIVER_NAME)
    );
}

#[test]
fn realm_rejects_foreign_thread_access_with_typed_error() {
    let realm = Arc::new(WebGpuRealm::new(42_u32));
    assert!(realm.is_valid());
    assert_eq!(*realm.get("test_resource").unwrap(), 42);

    let foreign = Arc::clone(&realm);
    let err = std::thread::spawn(move || foreign.get("test_resource").unwrap_err())
        .join()
        .unwrap();
    assert!(err.is_thread_confined(), "unexpected error: {err}");
}

#[test]
fn policy_refusals_reject_vulkan_only_extensions() {
    let raster_err = WebGpuRenderer::new_blocking(
        test_options(),
        RenderRasterPolicy::LastProvokingVertexV2,
        RenderCoordinateConvention::default(),
        RenderLineClippingBehavior::default(),
        RenderBehaviorConfiguration::default(),
    )
    .err()
    .expect("LastProvokingVertexV2 must be refused on WebGPU Core");
    assert!(
        raster_err.to_string().contains("LegacyFirstV1"),
        "{raster_err}"
    );

    let depth_cfg = RenderBehaviorConfiguration::default().with_window_depth(
        RenderWindowDepthBehavior::UnsnappedWindowDepthV2 {
            reference_grid: RenderViewportGridPrecision::Fractional8,
            raster_grid: RenderViewportGridPrecision::Fractional8,
        },
    );
    let depth_err = WebGpuRenderer::new_blocking(
        test_options(),
        RenderRasterPolicy::LegacyFirstV1,
        RenderCoordinateConvention::default(),
        RenderLineClippingBehavior::default(),
        depth_cfg,
    )
    .err()
    .expect("UnsnappedWindowDepthV2 must be refused on WebGPU Core");
    assert!(depth_err.to_string().contains("HardwareV1"), "{depth_err}");

    let clip_cfg = RenderBehaviorConfiguration::default()
        .with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV2);
    let clip_err = WebGpuRenderer::new_blocking(
        test_options(),
        RenderRasterPolicy::LegacyFirstV1,
        RenderCoordinateConvention::default(),
        RenderLineClippingBehavior::default(),
        clip_cfg,
    )
    .err()
    .expect("RasterClipDistanceV2 must be refused on WebGPU Core");
    assert!(clip_err.to_string().contains("FragmentV1"), "{clip_err}");
}

fn make_virtual_wasm_window(w: i32, h: i32) -> Handle<dyn AspectWindow> {
    let win = Handle::new(WasmWindow::new(&AsciiString::from("#test-canvas"), true));
    win.set_virtual(true);
    win.set_size_backing(martensite_tkernel::ncollection::Vec2::new(w, h));
    win.cast()
}

fn count_non_clear_pixels(rgba: &[u8], bg: [u8; 3]) -> usize {
    rgba.as_chunks::<4>()
        .0
        .iter()
        .filter(|px| {
            px[0].abs_diff(bg[0]) > 4 || px[1].abs_diff(bg[1]) > 4 || px[2].abs_diff(bg[2]) > 4
        })
        .count()
}

#[test]
fn cad_viewer_renders_openpbr_box_and_supports_capture_async() {
    let driver = WebGpuRenderDriver::new_blocking(test_options())
        .expect("WebGpuRenderDriver::new_blocking should succeed on lavapipe/Vulkan");
    let gd: Handle<dyn Graphic3dGraphicDriver> = driver.clone().cast();
    let viewer = V3dViewer::new(&gd).expect("V3dViewer::new");
    viewer.set_default_background_color(Color::named(NamedColor::BLACK));
    viewer.set_default_lights().expect("set_default_lights");
    viewer.set_light_on().expect("set_light_on");

    let view = viewer.create_view().expect("create_view");
    let window = make_virtual_wasm_window(128, 96);
    view.set_window(&window, 0).expect("set_window");

    let ctx = AisInteractiveContext::new(&viewer).expect("AisInteractiveContext::new");
    let mut sphere_maker = BRepPrimAPIMakeSphere::new(1.0).expect("MakeSphere");
    let shape = sphere_maker.shape().expect("sphere shape").clone();
    let ais_box = AisShape::new(&shape);
    ais_box
        .set_material(&Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold))
        .expect("set_material");
    let interactive: Handle<dyn AisInteractiveObject> = ais_box.cast();
    interactive.get().unwrap().set_display_mode(1);
    ctx.display_with_modes(&interactive, 1, 0, false, PrsMgrDisplayStatus::None)
        .expect("display_with_modes");

    let v3d = view.get().unwrap().view_base();
    v3d.camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    v3d.fit_all(0.05, false).expect("fit_all");

    // 1. OpenPBR Surface v1.1 shaded render
    v3d.change_rendering_params(&mut |params| {
        params.shading_model = Graphic3dTypeOfShadingModel::Pbr;
    });
    v3d.redraw();

    let render_view = v3d.view().cast::<Graphic3dRenderView>();
    let async_cap = block_on(render_view.capture_async()).expect("capture_async");
    assert_eq!((async_cap.width, async_cap.height), (128, 96));
    let lit_pixels = count_non_clear_pixels(&async_cap.rgba, [0, 0, 0]);
    assert!(
        lit_pixels > 500,
        "expected shaded OpenPBR box pixels > 500, got {lit_pixels}"
    );

    // 2. Blinn-Phong + 4x MSAA render
    v3d.change_rendering_params(&mut |params| {
        params.shading_model = Graphic3dTypeOfShadingModel::Phong;
        params.nb_msaa_samples = 4;
    });
    v3d.redraw();
    let msaa_cap = render_view
        .capture()
        .expect("sync capture after MSAA redraw");
    assert!(count_non_clear_pixels(&msaa_cap.rgba, [0, 0, 0]) > 500);

    // 3. Phase 5: Compute OpenPBR ray & progressive path tracing (2 samples)
    v3d.change_rendering_params(&mut |params| {
        params.method = Graphic3dRenderingMode::Raytracing;
        params.ray_tracing.is_global_illumination_enabled = true;
        params.ray_tracing.raytracing_depth = 2;
        params.nb_msaa_samples = 0;
    });
    v3d.redraw();
    let rt_cap_1 = render_view.capture().expect("raytrace sample 1");
    v3d.redraw();
    let rt_cap_2 = render_view.capture().expect("raytrace sample 2");
    assert_eq!((rt_cap_1.width, rt_cap_1.height), (128, 96));
    assert_eq!((rt_cap_2.width, rt_cap_2.height), (128, 96));
    let rt_pixels = count_non_clear_pixels(&rt_cap_2.rgba, [0, 0, 0]);
    assert!(
        rt_pixels > 300,
        "expected compute OpenPBR raytraced box pixels > 300, got {rt_pixels}"
    );

    let errors = driver.validation_errors().expect("validation errors query");
    assert!(
        errors.is_empty(),
        "WebGPU validation errors during CAD viewer test: {errors:?}"
    );
}

#[test]
fn primitives_lines_points_edges_wboit_clipping_stereo_subviews_zero_validation_errors() {
    let mut renderer = WebGpuRenderer::new_blocking(
        test_options(),
        RenderRasterPolicy::LegacyFirstV1,
        RenderCoordinateConvention::default(),
        RenderLineClippingBehavior::default(),
        RenderBehaviorConfiguration::default(),
    )
    .expect("WebGpuRenderer::new_blocking");

    // Triangle with mesh edges + clipping plane
    let tri_verts = vec![
        RenderVertex::new([-0.6, -0.6, 0.0], [0.8, 0.2, 0.2, 1.0]),
        RenderVertex::new([0.6, -0.6, 0.0], [0.2, 0.8, 0.2, 1.0]),
        RenderVertex::new([0.0, 0.6, 0.0], [0.2, 0.2, 0.8, 1.0]),
    ];
    let mut tri = PrimitiveDraw::new(tri_verts.clone(), vec![0, 1, 2], identity_clip()).unwrap();
    tri.mesh_edges = Some(RenderMeshEdges {
        color: [1.0, 1.0, 0.0, 1.0],
        width: 2.0,
        feather: 1.0,
        skip_first: false,
    });
    tri = tri.with_clipping(
        RenderClipping::new(vec![vec![[1.0, 0.0, 0.0, 0.25]]], identity_clip()).unwrap(),
    );

    // Wide stippled line
    let line_verts = vec![
        RenderVertex::new([-0.8, 0.7, 0.0], [1.0, 1.0, 1.0, 1.0]),
        RenderVertex::new([0.8, 0.7, 0.0], [1.0, 0.5, 0.0, 1.0]),
    ];
    let line = PrimitiveDraw::segments(
        line_verts,
        vec![0, 1],
        identity_clip(),
        LineStyle::new(5.0, 0xF0F0, 2).unwrap(),
    )
    .unwrap();

    // Large point marker
    let pt = PrimitiveDraw::points(
        vec![RenderVertex::new([0.0, -0.8, 0.0], [0.0, 1.0, 1.0, 1.0])],
        vec![0],
        identity_clip(),
        PointStyle::new(8.0, [1, 1], vec![[1.0, 1.0, 1.0, 1.0]]).unwrap(),
    )
    .unwrap();

    // Transparent quad for WBOIT
    let mut trans = PrimitiveDraw::new(
        vec![
            RenderVertex::new([-0.3, -0.3, -0.1], [1.0, 0.0, 1.0, 0.5]),
            RenderVertex::new([0.3, -0.3, -0.1], [1.0, 0.0, 1.0, 0.5]),
            RenderVertex::new([0.3, 0.3, -0.1], [1.0, 0.0, 1.0, 0.5]),
        ],
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap();
    trans.surface_blend = true;

    // Stencil capping mask + cap surface
    let cap_mask = PrimitiveDraw::new(tri_verts.clone(), vec![0, 1, 2], identity_clip())
        .unwrap()
        .with_capping_pass(RenderCappingPass::MaskStart)
        .unwrap();
    let cap_surf = PrimitiveDraw::new(tri_verts, vec![0, 1, 2], identity_clip())
        .unwrap()
        .with_capping_pass(RenderCappingPass::Surface)
        .unwrap();

    let mut frame = RenderFrame::with_layers(
        96,
        96,
        [0.05, 0.05, 0.08, 1.0],
        vec![RenderLayer::new(
            vec![tri, line, pt, trans, cap_mask, cap_surf],
            true,
            true,
            true,
        )],
    )
    .unwrap()
    .with_transparency(RenderTransparency::Weighted { depth_factor: 0.5 })
    .unwrap();
    frame.colored_quad = Some(ColoredQuadBackground {
        projection: identity_clip(),
        colors: [[0.1, 0.1, 0.2], [0.2, 0.2, 0.3]],
        view_size: [96.0, 96.0],
        corner: 0,
        encoded: false,
    });
    frame.subview_blits.push(RenderSubviewBlit {
        offset: [4, 4],
        extent: [8, 8],
        rgba: [255, 0, 0, 255].repeat(64),
    });

    let cap = renderer.render(&frame).expect("combined frame render");
    assert_eq!((cap.width, cap.height), (96, 96));
    let sub_px = cap.pixel(6, 6).unwrap();
    assert!(
        sub_px[0] > 200 && sub_px[1] < 20,
        "expected red subview pixel, got {sub_px:?}"
    );

    // Stereo pair composition (Red-Cyan + SideBySide + RowInterlaced)
    for mode in [
        StereoProgram::RedCyan,
        StereoProgram::SideBySide,
        StereoProgram::RowInterlaced,
    ] {
        let mut stereo_frame = RenderFrame::new(32, 32, [0.0, 0.0, 0.0, 1.0], vec![]).unwrap();
        stereo_frame.stereo_pair = Some(Arc::new(stereo_composite(mode)));
        let _ = renderer.render(&stereo_frame).expect("stereo compose");
    }

    let errors = renderer.context().validation_errors().unwrap();
    assert!(errors.is_empty(), "WebGPU validation errors: {errors:?}");
}

#[test]
fn wgsl_user_program_executes_and_spirv_only_program_returns_typed_refusal() {
    let driver = WebGpuRenderDriver::with_policies_async(
        test_options(),
        RenderPipelinePolicies::default(),
        RenderCoordinateConvention::default(),
        RenderLineWidthPolicy::default(),
        RenderBehaviorConfiguration::default(),
    );
    let driver = block_on(driver).expect("driver");

    let custom_wgsl = r#"
struct VsIn {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal_edge: vec4<f32>,
    @location(3) uv_bary: vec4<f32>,
};
@vertex
fn vs_custom(input: VsIn) -> @builtin(position) vec4<f32> {
    return input.position;
}
@fragment
fn fs_custom() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
"#;
    let shader = Graphic3dShaderProgram::default();
    let key = UserProgramKey::of(&shader);
    driver
        .user_programs()
        .insert_wgsl(key, custom_wgsl, "vs_custom", "fs_custom")
        .expect("insert_wgsl");
    let wgsl_modules = driver.user_programs().modules(&shader).unwrap();

    let mut draw = PrimitiveDraw::new(
        vec![
            RenderVertex::new([-1.0, -1.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
            RenderVertex::new([1.0, -1.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
            RenderVertex::new([0.0, 1.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
        ],
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap();
    draw.user_program = Some(Arc::new(UserProgramDraw::new(
        wgsl_modules,
        UserUniformScene::default(),
    )));

    let frame = RenderFrame::new(32, 32, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap();
    let mut renderer = WebGpuRenderer::new_blocking(
        test_options(),
        RenderRasterPolicy::LegacyFirstV1,
        RenderCoordinateConvention::default(),
        RenderLineClippingBehavior::default(),
        RenderBehaviorConfiguration::default(),
    )
    .unwrap();
    let cap = renderer.render(&frame).expect("WGSL user program render");
    let center = cap.pixel(16, 16).unwrap();
    assert!(
        center[1] > 200 && center[0] < 20,
        "expected green WGSL user shader output at center, got {center:?}"
    );

    // SPIR-V-only user program -> typed SpirvUnsupportedOnWebGpu refusal
    let spirv_only = Arc::new(UserProgramModules {
        key,
        stages: Vec::new(),
        wgsl: None,
    });
    let mut spirv_draw = PrimitiveDraw::new(
        vec![
            RenderVertex::new([-1.0, -1.0, 0.0], [1.0; 4]),
            RenderVertex::new([1.0, -1.0, 0.0], [1.0; 4]),
            RenderVertex::new([0.0, 1.0, 0.0], [1.0; 4]),
        ],
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap();
    spirv_draw.user_program = Some(Arc::new(UserProgramDraw::new(
        spirv_only,
        UserUniformScene::default(),
    )));
    let spirv_frame = RenderFrame::new(16, 16, [0.0; 4], vec![spirv_draw]).unwrap();
    let err = renderer
        .render(&spirv_frame)
        .expect_err("SPIR-V-only program must fail on WebGPU");
    assert_eq!(
        err.user_program_failure(),
        Some(&RenderUserProgramFailure::SpirvUnsupportedOnWebGpu { key })
    );
    assert!(renderer.context().validation_errors().unwrap().is_empty());
}
