//! Initialized `wgpu` instance, adapter, device, queue, and validation-scope tracking (#634).
use crate::options::WebGpuOptions;
use crate::realm::WebGpuRealm;
use martensite_render_core::view_diagnostics::RenderDiagnosticFacts;
use martensite_render_core::{
    FrameWorkLedger, RenderDeviceLimits, RenderDriverIdentity, RenderError,
    RenderValidationObservationFailure,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Inner `wgpu` device handles confined to their creating thread / Web Worker realm.
pub(crate) struct WebGpuDeviceState {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub instance: wgpu::Instance,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// Initialized WebGPU Core device context shared by `WebGpuRenderer` and `WebGpuWindowPresenter` (#634).
///
/// Always `Send + Sync` (zero `unsafe`, zero `thread_local!`) via checked `WebGpuRealm` confinement.
pub struct WebGpuContext {
    pub(crate) state: WebGpuRealm<WebGpuDeviceState>,
    options: WebGpuOptions,
    info: wgpu::AdapterInfo,
    limits: wgpu::Limits,
    work_ledger: Arc<FrameWorkLedger>,
    device_lost: Arc<AtomicBool>,
    validation_errors: Arc<Mutex<Vec<String>>>,
}

impl WebGpuContext {
    /// Asynchronously initialize a WebGPU Core adapter and logical device (#634).
    ///
    /// Works identically in browser Web Workers (`wasm32`) and desktop native targets.
    /// Example: `WebGpuContext::new_async(WebGpuOptions::default()).await?`.
    pub async fn new_async(options: WebGpuOptions) -> Result<Self, RenderError> {
        let backends = default_backends();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: options.power_preference.to_wgpu(),
                force_fallback_adapter: options.force_fallback_adapter,
                compatible_surface: None,
                apply_limit_buckets: false,
            })
            .await
            .map_err(|err| {
                RenderError::rejected(
                    format!("request_adapter failed ({err})"),
                    "an available WebGPU Core adapter",
                )
            })?;

        let info = adapter.get_info();
        let adapter_limits = adapter.limits();
        let required_limits = wgpu::Limits::default().using_resolution(adapter_limits);
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("martensite-webgpu-device"),
                required_features: wgpu::Features::empty(),
                required_limits: required_limits.clone(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|err| {
                RenderError::rejected(
                    format!("request_device failed ({err})"),
                    "a WebGPU Core logical device",
                )
            })?;

        let device_lost = Arc::new(AtomicBool::new(false));
        let lost_flag = Arc::clone(&device_lost);
        device.set_device_lost_callback(move |_reason, _msg| {
            lost_flag.store(true, Ordering::Release);
        });

        let validation_errors = Arc::new(Mutex::new(Vec::new()));
        let err_sink = Arc::clone(&validation_errors);
        device.on_uncaptured_error(Arc::new(move |error| {
            if let Ok(mut guard) = err_sink.lock() {
                guard.push(error.to_string());
            }
        }));

        let limits = device.limits();
        let state = WebGpuRealm::new(WebGpuDeviceState {
            instance,
            adapter,
            device,
            queue,
        });

        Ok(Self {
            state,
            options,
            info,
            limits,
            work_ledger: Arc::new(FrameWorkLedger::default()),
            device_lost,
            validation_errors,
        })
    }

    /// Synchronously initialize a WebGPU Core context on native desktop platforms (#634).
    ///
    /// Example: `WebGpuContext::new_blocking(WebGpuOptions::default())?`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_blocking(options: WebGpuOptions) -> Result<Self, RenderError> {
        crate::realm::block_on(Self::new_async(options))
    }

    /// Active options used to initialize this context.
    pub fn options(&self) -> WebGpuOptions {
        self.options
    }

    /// Adapter display name reported by WebGPU.
    pub fn device_name(&self) -> &str {
        if self.info.name.is_empty() {
            "WebGPU Core Device"
        } else {
            &self.info.name
        }
    }

    /// Maximum supported 2D capture dimension clamped by adapter limits.
    pub fn capture_limit(&self) -> u32 {
        self.limits
            .max_texture_dimension_2d
            .min(self.options.max_capture_dimension)
            .max(1)
    }

    /// Whether the WebGPU device lost callback has fired.
    pub fn is_device_lost(&self) -> bool {
        self.device_lost.load(Ordering::Acquire)
    }

    /// Shared frame profiling work ledger.
    pub fn work_ledger(&self) -> Arc<FrameWorkLedger> {
        Arc::clone(&self.work_ledger)
    }

    /// Record a validation error observed via `pop_error_scope` or uncaptured error handler.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub(crate) fn record_validation_error(&self, message: String) {
        if let Ok(mut guard) = self.validation_errors.lock() {
            guard.push(message);
        }
    }

    /// Share the validation error sink for async `wasm32` error-scope polling.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn clone_validation_sink(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.validation_errors)
    }

    /// Snapshot collected WebGPU validation errors, or return unavailable when disabled.
    pub fn validation_errors(&self) -> Result<Vec<String>, RenderValidationObservationFailure> {
        if !self.options.validation {
            return Err(RenderValidationObservationFailure::unavailable());
        }
        let guard = self
            .validation_errors
            .lock()
            .map_err(|_| RenderValidationObservationFailure::unavailable())?;
        Ok(guard.clone())
    }

    /// Return the WebGPU Core capacities mapped into `RenderDeviceLimits`.
    pub fn device_limits(&self) -> RenderDeviceLimits {
        let max_tex = self.limits.max_texture_dimension_2d as i32;
        let max_dump = self.capture_limit() as i32;
        RenderDeviceLimits {
            max_texture_size: max_tex,
            max_dump_size: [max_dump, max_dump],
            max_msaa_samples: 4,
            max_combined_texture_units: self.limits.max_sampled_textures_per_shader_stage as i32,
            max_clip_planes: crate::gpu_types::MAX_WEBGPU_CLIP_PLANES as i32,
            max_color_attachments: self.limits.max_color_attachments as i32,
            max_draw_buffers: self.limits.max_color_attachments as i32,
            max_degree_of_anisotropy: 16,
            has_srgb: true,
            has_texture_multisampling: true,
            has_mesh_edges: true,
            has_blended_oit: true,
            has_blended_oit_msaa: false,
            sub_texel_precision_bits: 8,
            has_ray_tracing_textures: false,
            has_ray_tracing_adaptive_sampling: false,
            has_ray_tracing_adaptive_sampling_atomic: false,
            has_hardware_ray_tracing: false,
            has_ray_query: false,
            min_as_scratch_alignment: 0,
        }
    }

    /// Return selected WebGPU adapter provenance.
    pub fn driver_identity(&self) -> RenderDriverIdentity {
        RenderDriverIdentity {
            vendor_id: self.info.vendor,
            device_id: self.info.device,
            driver_id: 0,
            driver_version: 0,
            driver_name: format!("WebGPU ({:?})", self.info.backend),
            driver_info: self.info.driver_info.clone(),
            device_name: self.device_name().to_string(),
        }
    }

    /// Return diagnostic dictionary facts for `V3dView::diagnostic_information`.
    pub fn diagnostic_facts(&self, frame_prepared: bool) -> RenderDiagnosticFacts {
        RenderDiagnosticFacts {
            instance_version: (1 << 22),
            api_version: (1 << 22),
            validation: self.options.validation,
            instance_extensions: vec!["webgpu-core".to_string()],
            device_extensions: vec![format!("backend:{:?}", self.info.backend)],
            scene_formats: [
                "Rgba8UnormSrgb".to_string(),
                "Depth24PlusStencil8".to_string(),
            ],
            frame_prepared,
        }
    }

    /// Borrow the realm-confined `wgpu` handles, rejecting foreign-thread access.
    pub(crate) fn state(&self) -> Result<&WebGpuDeviceState, RenderError> {
        if self.is_device_lost() {
            return Err(RenderError::rejected(
                "WebGPU device lost",
                "a live WebGPU logical device",
            ));
        }
        self.state.get("WebGpuContext")
    }
}

fn default_backends() -> wgpu::Backends {
    #[cfg(target_arch = "wasm32")]
    {
        wgpu::Backends::BROWSER_WEBGPU
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12
    }
}
