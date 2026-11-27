//! `RenderWindowPresenter` implementation for browser `WasmWindow` canvases (#634).
use crate::context::WebGpuContext;
#[cfg(target_arch = "wasm32")]
use crate::realm::WebGpuRealm;
use martensite_render_core::{RenderError, RenderWindowPresenter};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::any::Any;

#[cfg(target_arch = "wasm32")]
struct WasmSurfaceState {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

/// WebGPU canvas presenter bound to a `WasmWindow` (`OffscreenCanvas` or `HtmlCanvasElement`, #634).
pub struct WebGpuWindowPresenter {
    window: Handle<dyn AspectWindow>,
    #[cfg(target_arch = "wasm32")]
    state: WebGpuRealm<WasmSurfaceState>,
}

impl WebGpuWindowPresenter {
    /// Bind a `wgpu::Surface` to `window` when it is a canvas-backed `WasmWindow` (#634).
    pub fn create(
        ctx: &WebGpuContext,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<Self, RenderError> {
        #[cfg(target_arch = "wasm32")]
        {
            use martensite_tkservice::wasm::{WasmCanvasTarget, WasmWindow};
            let wasm_win = window.cast::<WasmWindow>();
            let Some(win) = wasm_win.get() else {
                return Err(RenderError::rejected(
                    "non-WasmWindow AspectWindow",
                    "a martensite_tkservice::wasm::WasmWindow with an OffscreenCanvas or HtmlCanvasElement",
                ));
            };
            let target = win
                .canvas_target()
                .map_err(|err| RenderError::rejected(err.to_string(), "owning-realm WasmWindow"))?
                .ok_or_else(|| {
                    RenderError::rejected(
                        "canvasless WasmWindow",
                        "a WasmWindow created with from_offscreen_canvas or from_html_canvas",
                    )
                })?;
            let dev = ctx.state()?;
            let surface_target = match target {
                WasmCanvasTarget::Offscreen(c) => wgpu::SurfaceTarget::OffscreenCanvas(c),
                WasmCanvasTarget::Html(c) => wgpu::SurfaceTarget::Canvas(c),
            };
            let surface = dev.instance.create_surface(surface_target).map_err(|err| {
                RenderError::rejected(
                    format!("create_surface failed ({err})"),
                    "a WebGPU-compatible canvas",
                )
            })?;
            let caps = surface.get_capabilities(&dev.adapter);
            let format = caps
                .formats
                .iter()
                .copied()
                .find(|f| f.is_srgb())
                .or_else(|| caps.formats.first().copied())
                .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb);
            let (w, h) = win.size();
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
                format,
                width: (w.max(1)) as u32,
                height: (h.max(1)) as u32,
                present_mode: wgpu::PresentMode::Fifo,
                desired_maximum_frame_latency: 2,
                alpha_mode: caps
                    .alpha_modes
                    .first()
                    .copied()
                    .unwrap_or(wgpu::CompositeAlphaMode::Opaque),
                view_formats: vec![],
                color_space: wgpu::SurfaceColorSpace::Auto,
            };
            surface.configure(&dev.device, &config);
            Ok(Self {
                window: window.clone(),
                state: WebGpuRealm::new(WasmSurfaceState { surface, config }),
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (ctx, window);
            Err(RenderError::rejected(
                "native AspectWindow on WebGPU",
                "a virtual window offscreen target or wasm32 WasmWindow canvas",
            ))
        }
    }

    /// Copy the rendered offscreen scene color texture into the live canvas surface and present (#634).
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn present_texture(
        &mut self,
        ctx: &WebGpuContext,
        encoder: &mut wgpu::CommandEncoder,
        scene_tex: &wgpu::Texture,
        extent: [u32; 2],
    ) -> Result<Option<wgpu::SurfaceTexture>, RenderError> {
        let dev = ctx.state()?;
        let state = self.state.get_mut("WebGpuWindowPresenter")?;
        if state.config.width != extent[0] || state.config.height != extent[1] {
            state.config.width = extent[0].max(1);
            state.config.height = extent[1].max(1);
            state.surface.configure(&dev.device, &state.config);
        }
        let surface_tex = match state.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(tex)
            | wgpu::CurrentSurfaceTexture::Suboptimal(tex) => tex,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                state.surface.configure(&dev.device, &state.config);
                match state.surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(tex)
                    | wgpu::CurrentSurfaceTexture::Suboptimal(tex) => tex,
                    other => {
                        return Err(RenderError::rejected(
                            format!("WebGPU surface unavailable ({other:?})"),
                            "a configured browser canvas surface",
                        ));
                    }
                }
            }
            other => {
                return Err(RenderError::rejected(
                    format!("WebGPU surface acquire failed ({other:?})"),
                    "an active browser canvas surface",
                ));
            }
        };
        if surface_tex.texture.format() == scene_tex.format() {
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: scene_tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &surface_tex.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: extent[0].min(surface_tex.texture.width()),
                    height: extent[1].min(surface_tex.texture.height()),
                    depth_or_array_layers: 1,
                },
            );
        }
        Ok(Some(surface_tex))
    }
}

impl RenderWindowPresenter for WebGpuWindowPresenter {
    fn observed_extent(&self) -> Result<[u32; 2], RenderError> {
        let Some(win) = self.window.get() else {
            return Err(RenderError::rejected(
                "null window handle",
                "a live AspectWindow",
            ));
        };
        let (w, h) = win.size();
        Ok([w.max(1) as u32, h.max(1) as u32])
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
