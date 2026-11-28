//! `RenderWindowPresenter` implementation for browser `WasmWindow` canvases (#634, #642).
use crate::context::WebGpuContext;
#[cfg(target_arch = "wasm32")]
use crate::realm::WebGpuRealm;
use martensite_render_core::{RenderError, RenderWindowPresenter};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::any::Any;

#[cfg(any(target_arch = "wasm32", test))]
const SURFACE_BLIT_WGSL: &str = r#"
@group(0) @binding(0) var src_tex: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(idx & 1u) * 4 - 1);
    let y = f32(i32(idx & 2u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<i32>(textureDimensions(src_tex, 0));
    let coord = clamp(vec2<i32>(pos.xy), vec2<i32>(0, 0), max(dims - vec2<i32>(1, 1), vec2<i32>(0, 0)));
    return textureLoad(src_tex, coord, 0);
}
"#;

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct SurfaceBlitPipeline {
    format: wgpu::TextureFormat,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
}

#[cfg(any(target_arch = "wasm32", test))]
impl SurfaceBlitPipeline {
    pub(crate) fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("martensite-surface-blit-shader"),
            source: wgpu::ShaderSource::Wgsl(SURFACE_BLIT_WGSL.into()),
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("martensite-surface-blit-bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("martensite-surface-blit-layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("martensite-surface-blit-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            format: target_format,
            bind_group_layout,
            pipeline,
        }
    }
}

/// Copy or format-preservingly blit `scene_tex` (`Rgba8UnormSrgb`) into `target_tex` (#642).
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn present_scene_to_target(
    device: &wgpu::Device,
    blit_cache: &mut Option<SurfaceBlitPipeline>,
    encoder: &mut wgpu::CommandEncoder,
    scene_tex: &wgpu::Texture,
    target_tex: &wgpu::Texture,
    extent: [u32; 2],
) {
    let target_format = target_tex.format();
    if target_format == scene_tex.format() {
        encoder.copy_texture_to_texture(
            wgpu::TexelCopyTextureInfo {
                texture: scene_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyTextureInfo {
                texture: target_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: extent[0].min(target_tex.width()),
                height: extent[1].min(target_tex.height()),
                depth_or_array_layers: 1,
            },
        );
        return;
    }

    let blit = match blit_cache {
        Some(cached) if cached.format == target_format => cached,
        slot => slot.insert(SurfaceBlitPipeline::new(device, target_format)),
    };

    // Preserve sRGB bytes 1:1 when presenting into non-sRGB browser surfaces (Bgra8Unorm/Rgba8Unorm).
    let src_view_format = if target_format.is_srgb() {
        scene_tex.format()
    } else {
        wgpu::TextureFormat::Rgba8Unorm
    };
    let src_view = scene_tex.create_view(&wgpu::TextureViewDescriptor {
        format: Some(src_view_format),
        ..Default::default()
    });
    let dst_view = target_tex.create_view(&wgpu::TextureViewDescriptor::default());
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("martensite-surface-blit-bg"),
        layout: &blit.bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&src_view),
        }],
    });
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("martensite-surface-blit-pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &dst_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(&blit.pipeline);
    pass.set_bind_group(0, &bind_group, &[]);
    pass.draw(0..3, 0..1);
}

#[cfg(target_arch = "wasm32")]
struct WasmSurfaceState {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    blit: Option<SurfaceBlitPipeline>,
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
                state: WebGpuRealm::new(WasmSurfaceState {
                    surface,
                    config,
                    blit: None,
                }),
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

    /// Copy or blit the rendered offscreen scene color texture into the live canvas surface (#634, #642).
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
        present_scene_to_target(
            &dev.device,
            &mut state.blit,
            encoder,
            scene_tex,
            &surface_tex.texture,
            extent,
        );
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::options::WebGpuOptions;
    use crate::targets::WebGpuTargets;

    #[test]
    fn surface_blit_preserves_srgb_bytes_across_bgra_rgba_and_srgb_targets() {
        let ctx = WebGpuContext::new_blocking(WebGpuOptions {
            validation: true,
            ..Default::default()
        })
        .expect("WebGPU context");
        let dev = ctx.state().expect("device state");
        let extent = [4u32, 4u32];
        let targets = WebGpuTargets::new(&dev.device, extent, 1);

        // Render a known linear color into `targets.color_tex` (`Rgba8UnormSrgb`) so the GPU encodes sRGB bytes.
        let mut init_enc = dev
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("test-init-scene-color"),
            });
        {
            let _pass = init_enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("test-clear-scene-color"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &targets.color_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.7,
                            g: 0.2,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }
        let baseline_readback = crate::targets::WebGpuDeferredFrame::stage_copy(
            &dev.device,
            &mut init_enc,
            &targets.color_tex,
            extent,
        );
        dev.queue.submit(Some(init_enc.finish()));
        let baseline = baseline_readback
            .read_capture_blocking(&ctx, martensite_render_core::CapturePlanes::Color)
            .expect("baseline readback")
            .rgba;
        let expected_rgba = [baseline[0], baseline[1], baseline[2], baseline[3]];
        let expected_bgra = [baseline[2], baseline[1], baseline[0], baseline[3]];
        assert!(expected_rgba[0] > 200 && expected_rgba[2] < 100 && expected_rgba[3] == 255);

        let mut blit_cache = None;
        for (format, expected_pixel) in [
            (wgpu::TextureFormat::Bgra8Unorm, expected_bgra),
            (wgpu::TextureFormat::Rgba8Unorm, expected_rgba),
            (wgpu::TextureFormat::Bgra8UnormSrgb, expected_bgra),
            (wgpu::TextureFormat::Rgba8UnormSrgb, expected_rgba),
        ] {
            let dst_tex = dev.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("test-surface-target"),
                size: wgpu::Extent3d {
                    width: 4,
                    height: 4,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            let mut encoder = dev
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("test-surface-blit-enc"),
                });
            present_scene_to_target(
                &dev.device,
                &mut blit_cache,
                &mut encoder,
                &targets.color_tex,
                &dst_tex,
                extent,
            );
            let readback = crate::targets::WebGpuDeferredFrame::stage_copy(
                &dev.device,
                &mut encoder,
                &dst_tex,
                extent,
            );
            dev.queue.submit(Some(encoder.finish()));
            let pixels = readback
                .read_capture_blocking(&ctx, martensite_render_core::CapturePlanes::Color)
                .expect("readback blitted surface")
                .rgba;
            for ch in 0..4 {
                let diff = pixels[ch].abs_diff(expected_pixel[ch]);
                assert!(
                    diff <= 1,
                    "format {format:?} channel {ch}: got {:?}, expected {:?}",
                    &pixels[..4],
                    expected_pixel
                );
            }
        }
        assert!(
            ctx.validation_errors()
                .expect("validation errors")
                .is_empty(),
            "unexpected WebGPU validation errors"
        );
    }
}
