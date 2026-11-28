//! Offscreen render targets, MSAA, WBOIT, shadow maps, and async/blocking host readback (#634).
use crate::context::WebGpuContext;
use crate::realm::WebGpuRealm;
#[cfg(not(target_arch = "wasm32"))]
use martensite_render_core::CapturePlanes;
use martensite_render_core::{FrameCapture, RenderDeferredFrame, RenderError};
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

/// Standard scene color format matching Martensite's sRGB color convention.
pub const SCENE_COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
/// Standard depth-stencil format guaranteed in WebGPU Core.
pub const SCENE_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24PlusStencil8;
/// Standard shadow-map depth format guaranteed in WebGPU Core.
pub const SHADOW_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// WBOIT accumulation format guaranteed in WebGPU Core (`Rgba16Float`).
pub const OIT_ACCUM_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
/// WBOIT revealage format guaranteed in WebGPU Core (`R16Float`).
pub const OIT_REVEAL_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R16Float;

/// Offscreen GPU attachments allocated for a target extent and sample count.
pub(crate) struct WebGpuTargets {
    pub extent: [u32; 2],
    pub sample_count: u32,
    pub color_tex: wgpu::Texture,
    pub color_view: wgpu::TextureView,
    pub msaa_color_view: Option<wgpu::TextureView>,
    pub depth_view: wgpu::TextureView,
    pub oit_accum_view: wgpu::TextureView,
    pub oit_reveal_view: wgpu::TextureView,
    pub shadow_resolution: u32,
    pub shadow_tex: wgpu::Texture,
    pub shadow_view: wgpu::TextureView,
    pub rt_accum_views: [wgpu::TextureView; 2],
}

impl WebGpuTargets {
    /// Allocate or recreate all offscreen render targets for `extent` and `sample_count` (1 or 4).
    pub fn new(device: &wgpu::Device, extent: [u32; 2], sample_count: u32) -> Self {
        let samples = if sample_count >= 4 { 4 } else { 1 };
        let size = wgpu::Extent3d {
            width: extent[0].max(1),
            height: extent[1].max(1),
            depth_or_array_layers: 1,
        };
        let color_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("martensite-scene-color"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: SCENE_COLOR_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });
        let color_view = color_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let msaa_color_view = if samples > 1 {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("martensite-scene-msaa-color"),
                size,
                mip_level_count: 1,
                sample_count: samples,
                dimension: wgpu::TextureDimension::D2,
                format: SCENE_COLOR_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            Some(tex.create_view(&wgpu::TextureViewDescriptor::default()))
        } else {
            None
        };

        let depth_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("martensite-scene-depth"),
            size,
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: SCENE_DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let oit_accum_tex = create_2d_target(
            device,
            "martensite-oit-accum",
            size,
            OIT_ACCUM_FORMAT,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let oit_accum_view = oit_accum_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let oit_reveal_tex = create_2d_target(
            device,
            "martensite-oit-reveal",
            size,
            OIT_REVEAL_FORMAT,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let oit_reveal_view = oit_reveal_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let (shadow_tex, shadow_view) = create_shadow_target(device, 1);

        let rt_usage = wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING;
        let rt0 = create_2d_target(
            device,
            "martensite-rt-accum-0",
            size,
            OIT_ACCUM_FORMAT,
            rt_usage,
        );
        let rt1 = create_2d_target(
            device,
            "martensite-rt-accum-1",
            size,
            OIT_ACCUM_FORMAT,
            rt_usage,
        );
        let rt_accum_views = [
            rt0.create_view(&wgpu::TextureViewDescriptor::default()),
            rt1.create_view(&wgpu::TextureViewDescriptor::default()),
        ];

        Self {
            extent,
            sample_count: samples,
            color_tex,
            color_view,
            msaa_color_view,
            depth_view,
            oit_accum_view,
            oit_reveal_view,
            shadow_resolution: 1,
            shadow_tex,
            shadow_view,
            rt_accum_views,
        }
    }

    /// Resize the `Depth32Float` shadow-map target when a shadow pass requests a different size.
    pub fn ensure_shadow_resolution(&mut self, device: &wgpu::Device, resolution: u32) {
        let res = resolution.max(1);
        if self.shadow_resolution == res {
            return;
        }
        let (tex, view) = create_shadow_target(device, res);
        self.shadow_resolution = res;
        self.shadow_tex = tex;
        self.shadow_view = view;
    }
}

fn create_2d_target(
    device: &wgpu::Device,
    label: &str,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage,
        view_formats: &[],
    })
}

fn create_shadow_target(
    device: &wgpu::Device,
    resolution: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("martensite-shadow-depth"),
        size: wgpu::Extent3d {
            width: resolution,
            height: resolution,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: SHADOW_DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}

/// Compute WebGPU-aligned bytes per row (multiple of `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT = 256`).
pub fn aligned_bytes_per_row(width: u32) -> u32 {
    let unpadded = width.saturating_mul(4);
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    unpadded.div_ceil(align) * align
}

/// Deferred WebGPU frame holding a staged `MAP_READ` copy of the rendered scene color attachment (#634).
pub struct WebGpuDeferredFrame {
    extent: [u32; 2],
    padded_bytes_per_row: u32,
    staging: WebGpuRealm<wgpu::Buffer>,
}

impl WebGpuDeferredFrame {
    /// Allocate a `MAP_READ | COPY_DST` staging buffer and record `copy_texture_to_buffer`.
    pub(crate) fn stage_copy(
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        color_tex: &wgpu::Texture,
        extent: [u32; 2],
    ) -> Self {
        let padded_bytes_per_row = aligned_bytes_per_row(extent[0]);
        let byte_size = u64::from(padded_bytes_per_row) * u64::from(extent[1]);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("martensite-readback-staging"),
            size: byte_size.max(4),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: color_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(extent[1]),
                },
            },
            wgpu::Extent3d {
                width: extent[0],
                height: extent[1],
                depth_or_array_layers: 1,
            },
        );
        Self {
            extent,
            padded_bytes_per_row,
            staging: WebGpuRealm::new(buffer),
        }
    }

    /// Initiate asynchronous buffer mapping without holding any external lock across `.await` (#634).
    pub(crate) fn begin_map_async(&self, ctx: &WebGpuContext) -> Result<MapFuture, RenderError> {
        let buffer = self.staging.get("WebGpuDeferredFrame")?;
        let state = Arc::new(Mutex::new(MapSignal::default()));
        let signal = Arc::clone(&state);
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                if let Ok(mut guard) = signal.lock() {
                    guard.result = Some(result);
                    if let Some(waker) = guard.waker.take() {
                        waker.wake();
                    }
                }
            });
        #[cfg(not(target_arch = "wasm32"))]
        {
            let dev = ctx.state()?;
            let _ = dev.device.poll(wgpu::PollType::wait_indefinitely());
        }
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;
        Ok(MapFuture { state })
    }

    /// Finish unpacking mapped RGBA8 pixels once the `MapFuture` completes (#634).
    pub(crate) fn finish_map_capture(
        &self,
        map_result: Result<(), wgpu::BufferAsyncError>,
    ) -> Result<FrameCapture, RenderError> {
        map_result.map_err(|err| {
            RenderError::rejected(
                format!("WebGPU buffer map_async failed ({err})"),
                "a mappable staging buffer",
            )
        })?;
        let buffer = self.staging.get("WebGpuDeferredFrame")?;
        let slice = buffer.slice(..);
        let mapped = slice.get_mapped_range().map_err(|err| {
            RenderError::rejected(
                format!("WebGPU buffer get_mapped_range failed ({err})"),
                "a mapped staging buffer slice",
            )
        })?;
        let rgba = unpack_rgba_rows(&mapped, self.extent, self.padded_bytes_per_row);
        drop(mapped);
        buffer.unmap();
        let depth = vec![1.0_f32; (self.extent[0] as usize) * (self.extent[1] as usize)];
        Ok(FrameCapture {
            width: self.extent[0],
            height: self.extent[1],
            rgba,
            depth,
        })
    }

    /// Asynchronously map the staged color buffer and unpack tightly packed RGBA8 pixels (#634).
    ///
    /// Never blocks the browser event loop on `wasm32`.
    pub async fn read_capture_async(
        &self,
        ctx: &WebGpuContext,
    ) -> Result<FrameCapture, RenderError> {
        let future = self.begin_map_async(ctx)?;
        let res = future.await;
        self.finish_map_capture(res)
    }

    /// Synchronously map and unpack pixels on native desktop platforms (#634).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_capture_blocking(
        &self,
        ctx: &WebGpuContext,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        let mut capture = crate::realm::block_on(self.read_capture_async(ctx))?;
        if planes == CapturePlanes::Color {
            capture.depth.clear();
        }
        Ok(capture)
    }
}

impl RenderDeferredFrame for WebGpuDeferredFrame {
    fn logical_size(&self) -> [u32; 2] {
        self.extent
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
struct MapSignal {
    result: Option<Result<(), wgpu::BufferAsyncError>>,
    waker: Option<Waker>,
}

pub(crate) struct MapFuture {
    state: Arc<Mutex<MapSignal>>,
}

impl Future for MapFuture {
    type Output = Result<(), wgpu::BufferAsyncError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Ok(mut guard) = self.state.lock() else {
            return Poll::Ready(Err(wgpu::BufferAsyncError));
        };
        if let Some(res) = guard.result.take() {
            return Poll::Ready(res);
        }
        guard.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

fn unpack_rgba_rows(mapped: &[u8], extent: [u32; 2], padded_row: u32) -> Vec<u8> {
    let row_bytes = (extent[0] as usize) * 4;
    let stride = padded_row as usize;
    let mut out = Vec::with_capacity(row_bytes * extent[1] as usize);
    for y in 0..extent[1] as usize {
        let start = y * stride;
        out.extend_from_slice(&mapped[start..start + row_bytes]);
    }
    out
}
