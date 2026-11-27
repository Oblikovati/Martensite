//! Headless Chromium 156 multi-threaded WASM + WebGPU E2E & benchmark harness (#636).
#![cfg(target_arch = "wasm32")]

use martensite_render_core::Graphic3dRenderView;
use martensite_tkernel::osd::{
    MonotonicInstant, Parallel, init_wasm_worker_pool_from_url, wasm_worker_pool_size,
};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::bvh::{BvhBox, BvhIndexedBoxSet, BvhPrimitiveSetAccess as _, BvhVec3d};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkservice::aspect::AspectWindow;
use martensite_tkservice::graphic3d::{
    Graphic3dCameraProjection, Graphic3dGraphicDriver, Graphic3dMaterialAspect,
    Graphic3dNameOfMaterial, Graphic3dRenderingMode, Graphic3dTypeOfShadingModel,
};
use martensite_tkservice::wasm::WasmWindow;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};
use martensite_tkv3d::prs_mgr::{
    PrsMgrDisplayStatus, PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_webgpu::{WebGpuOptions, WebGpuRenderDriver, WebGpuRenderViewExt};
use std::sync::atomic::{AtomicU64, Ordering};
use wasm_bindgen::prelude::*;

fn bench_parallel_math(workers: usize, total_iters: usize) -> (f64, u64) {
    let start = MonotonicInstant::now();
    let acc = AtomicU64::new(0);
    let chunk = total_iters / workers.max(1);
    let _ = Parallel::scope_or_pool(workers.max(1), |tid| {
        let mut local = 0.0_f64;
        let base = tid * chunk;
        for i in base..base + chunk {
            let x = (i as f64) * 1e-4;
            local += x.sin().mul_add(x.cos(), (x * 0.5).sqrt());
        }
        acc.fetch_add(local.to_bits() & 0xffff_ffff, Ordering::Relaxed);
    });
    (
        start.elapsed().as_secs_f64() * 1000.0,
        acc.load(Ordering::Relaxed),
    )
}

fn bench_bvh_build(count: usize) -> (f64, i32) {
    let mut set = BvhIndexedBoxSet::<BvhVec3d, i32>::default();
    for i in 0..count {
        let x = (i % 64) as f64;
        let y = ((i / 64) % 64) as f64;
        let z = (i / 4096) as f64;
        set.add(
            i as i32,
            BvhBox::from_corners(
                BvhVec3d::new(x, y, z),
                BvhVec3d::new(x + 0.5, y + 0.5, z + 0.5),
            ),
        );
    }
    let start = MonotonicInstant::now();
    let _ = set.build();
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    let nodes = set.bvh().map_or(0, |tree| tree.length());
    (elapsed_ms, nodes)
}

fn count_non_black(rgba: &[u8]) -> usize {
    rgba.as_chunks::<4>()
        .0
        .iter()
        .filter(|px| px[0] > 4 || px[1] > 4 || px[2] > 4)
        .count()
}

/// Runs the full Chromium E2E suite (worker pool scaling, BRep/BVH parity, OffscreenCanvas WebGPU).
#[wasm_bindgen]
pub async fn run_all_e2e_checks(
    canvas: web_sys::OffscreenCanvas,
    max_workers: usize,
    glue_url: String,
) -> Result<String, JsValue> {
    let pool = init_wasm_worker_pool_from_url(max_workers, &glue_url)
        .await
        .map_err(|e| JsValue::from_str(&format!("init_wasm_worker_pool failed: {e}")))?;
    let active_workers = wasm_worker_pool_size();

    // 1. Multi-threaded scaling benchmark (1 vs 4 vs max_workers)
    let work_items = 2_400_000usize;
    let (ms_1, hash_1) = bench_parallel_math(1, work_items);
    let (ms_4, _hash_4) = bench_parallel_math(4.min(active_workers + 1), work_items);
    let (ms_max, _hash_max) = bench_parallel_math((active_workers + 1).min(8), work_items);

    // 2. Parallel BVH build
    let (bvh_ms, bvh_nodes) = bench_bvh_build(16_384);

    // 3. BRep sphere creation + WebGPU OffscreenCanvas rendering (OpenPBR + Compute Raytrace)
    let t_driver = MonotonicInstant::now();
    let driver = WebGpuRenderDriver::new_async(WebGpuOptions {
        validation: true,
        ..WebGpuOptions::default()
    })
    .await
    .map_err(|e| JsValue::from_str(&format!("WebGpuRenderDriver::new_async failed: {e}")))?;
    let driver_init_ms = t_driver.elapsed().as_secs_f64() * 1000.0;

    let gd: Handle<dyn Graphic3dGraphicDriver> = driver.clone().cast();
    let viewer = V3dViewer::new(&gd).map_err(|e| JsValue::from_str(&e.to_string()))?;
    viewer.set_default_background_color(Color::named(NamedColor::BLACK));
    viewer
        .set_default_lights()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    viewer
        .set_light_on()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let view = viewer
        .create_view()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let wasm_win = Handle::new(WasmWindow::from_offscreen_canvas(
        &AsciiString::from("#e2e-canvas"),
        canvas,
        1.0,
    ));
    let aspect_win: Handle<dyn AspectWindow> = wasm_win.cast();
    view.set_window(&aspect_win, 0)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let ctx = AisInteractiveContext::new(&viewer).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let t_brep = MonotonicInstant::now();
    let mut sphere_maker =
        BRepPrimAPIMakeSphere::new(1.0).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let shape = sphere_maker
        .shape()
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .clone();
    let brep_ms = t_brep.elapsed().as_secs_f64() * 1000.0;

    let ais_sphere = AisShape::new(&shape);
    ais_sphere
        .set_material(&Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let interactive: Handle<dyn AisInteractiveObject> = ais_sphere.cast();
    interactive
        .get()
        .ok_or_else(|| JsValue::from_str("null interactive"))?
        .set_display_mode(1);
    ctx.display_with_modes(&interactive, 1, 0, false, PrsMgrDisplayStatus::None)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let v3d = view
        .get()
        .ok_or_else(|| JsValue::from_str("null view"))?
        .view_base();
    v3d.camera()
        .set_projection_type(Graphic3dCameraProjection::Perspective);
    v3d.fit_all(0.05, false)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // OpenPBR frame
    v3d.change_rendering_params(&mut |p| {
        p.shading_model = Graphic3dTypeOfShadingModel::Pbr;
    });
    let t_pbr = MonotonicInstant::now();
    v3d.redraw();
    let render_view = v3d.view().cast::<Graphic3dRenderView>();
    let pbr_cap = match render_view.capture_async().await {
        Ok(cap) => cap,
        Err(e) => {
            // Yield once so spawn_local(pop_error_scope) can record validation errors.
            let _ = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::NULL))
                .await;
            let errs = driver.validation_errors().unwrap_or_default();
            return Err(JsValue::from_str(&format!(
                "capture_async(PBR) failed: {e}; validation_errors={errs:?}"
            )));
        }
    };
    let pbr_ms = t_pbr.elapsed().as_secs_f64() * 1000.0;
    let pbr_lit = count_non_black(&pbr_cap.rgba);

    // Compute OpenPBR Raytrace frame
    v3d.change_rendering_params(&mut |p| {
        p.method = Graphic3dRenderingMode::Raytracing;
        p.ray_tracing.is_global_illumination_enabled = true;
        p.ray_tracing.raytracing_depth = 2;
    });
    let t_rt = MonotonicInstant::now();
    v3d.redraw();
    let rt_cap = render_view
        .capture_async()
        .await
        .map_err(|e| JsValue::from_str(&format!("capture_async(RT) failed: {e}")))?;
    let rt_ms = t_rt.elapsed().as_secs_f64() * 1000.0;
    let rt_lit = count_non_black(&rt_cap.rgba);

    let val_errors = driver.validation_errors().unwrap_or_default();
    let wasm_memory_bytes = core::arch::wasm32::memory_size(0) * 65536;
    let device_name = driver.device_name().replace('"', "'");
    pool.terminate();

    Ok(format!(
        "{{\"activeWorkers\":{active_workers},\"parallelMs1\":{ms_1:.2},\"parallelMs4\":{ms_4:.2},\"parallelMsMax\":{ms_max:.2},\"parallelHash\":{hash_1},\"bvhMs\":{bvh_ms:.2},\"bvhNodes\":{bvh_nodes},\"brepMs\":{brep_ms:.2},\"driverInitMs\":{driver_init_ms:.2},\"deviceName\":\"{device_name}\",\"pbrMs\":{pbr_ms:.2},\"pbrLitPixels\":{pbr_lit},\"pbrWidth\":{},\"pbrHeight\":{},\"rtMs\":{rt_ms:.2},\"rtLitPixels\":{rt_lit},\"validationErrors\":{},\"wasmMemoryBytes\":{wasm_memory_bytes}}}",
        pbr_cap.width,
        pbr_cap.height,
        val_errors.len()
    ))
}
