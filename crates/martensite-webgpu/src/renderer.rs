//! `RenderDeviceBackend` implementation executing `RenderFrame` on WebGPU Core (#634, #635).
use crate::compute_passes::WebGpuComputePasses;
use crate::context::WebGpuContext;
use crate::gpu_types::{
    GpuBackgroundUniforms, GpuGridUniforms, GpuLineSegment, GpuLineUniforms, GpuPointInstance,
    GpuPointUniforms, GpuStereoSubviewUniforms, identity_mat4, mat4_from_slice, pack_draw_state,
    pack_surface_vertices,
};
use crate::options::{WebGpuOptions, validate_webgpu_policies};
use crate::pipelines::{
    SurfaceBlendMode, SurfacePipelineKey, WebGpuPipelines, upload_rgba8_texture,
};
use crate::realm::WebGpuRealm;
use crate::surface::WebGpuWindowPresenter;
use crate::targets::{WebGpuDeferredFrame, WebGpuTargets};
use bytemuck::{bytes_of, cast_slice};
use martensite_render_core::driver_options::WindowFrameQueue;
use martensite_render_core::scene::ShaderGridPass;
use martensite_render_core::stereo_pair::StereoProgram;
use martensite_render_core::user_program::RenderUserProgramFailure;
use martensite_render_core::view_diagnostics::RenderDiagnosticFacts;
use martensite_render_core::{
    CapturePlanes, FrameCapture, FrameWorkLedger, PrimitiveDraw, PrimitiveTopology, RenderBackend,
    RenderBehaviorConfiguration, RenderCappingPass, RenderCoordinateConvention,
    RenderDeferredFrame, RenderDeviceBackend, RenderDeviceLimits, RenderDeviceMemoryReport,
    RenderDriverIdentity, RenderError, RenderFaceCulling, RenderFrame, RenderGuiSurfacePolicy,
    RenderImageMemory, RenderLineClippingBehavior, RenderMemoryQueryError, RenderRasterPolicy,
    RenderTransparency, RenderValidationObservationFailure, RenderWindowPresenter, RenderedFrame,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::any::Any;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Zero-`unsafe` WebGPU Core renderer implementing `RenderDeviceBackend` (#634, #635).
pub struct WebGpuRenderer {
    context: WebGpuContext,
    pipelines: WebGpuRealm<WebGpuPipelines>,
    compute: WebGpuRealm<WebGpuComputePasses>,
    targets: WebGpuRealm<Option<WebGpuTargets>>,
    raster: RenderRasterPolicy,
    coordinates: RenderCoordinateConvention,
    frame_prepared: bool,
}

impl WebGpuRenderer {
    /// Build a `WebGpuRenderer` from an initialized `WebGpuContext` and policy selection (#634).
    pub fn from_context(
        context: WebGpuContext,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Self, RenderError> {
        validate_webgpu_policies(raster, clipping, &configuration)?;
        let (pipelines, compute) = {
            let dev = context.state()?;
            (
                WebGpuPipelines::new(&dev.device, &dev.queue),
                WebGpuComputePasses::new(&dev.device),
            )
        };
        Ok(Self {
            context,
            pipelines: WebGpuRealm::new(pipelines),
            compute: WebGpuRealm::new(compute),
            targets: WebGpuRealm::new(None),
            raster,
            coordinates,
            frame_prepared: false,
        })
    }

    /// Asynchronously initialize a `WebGpuRenderer` from `options` (#634).
    pub async fn new_async(
        options: WebGpuOptions,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Self, RenderError> {
        validate_webgpu_policies(raster, clipping, &configuration)?;
        let context = WebGpuContext::new_async(options).await?;
        Self::from_context(context, raster, coordinates, clipping, configuration)
    }

    /// Synchronously initialize a `WebGpuRenderer` on native desktop platforms (#634).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_blocking(
        options: WebGpuOptions,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Self, RenderError> {
        crate::realm::block_on(Self::new_async(
            options,
            raster,
            coordinates,
            clipping,
            configuration,
        ))
    }

    /// Access the underlying `WebGpuContext` (used by `WebGpuRenderViewExt::capture_async`).
    pub fn context(&self) -> &WebGpuContext {
        &self.context
    }

    /// Execute `frame` on the WebGPU device and return a staged `WebGpuDeferredFrame`.
    pub(crate) fn execute_frame(
        &mut self,
        frame: &RenderFrame,
        present_to: Option<&mut WebGpuWindowPresenter>,
    ) -> Result<WebGpuDeferredFrame, RenderError> {
        let dev = self.context.state()?;
        let extent = frame.extent();
        let sample_count = if frame.raster.requested_samples >= 4 {
            4
        } else {
            1
        };
        let validation = self.context.options().validation;
        let error_scope =
            validation.then(|| dev.device.push_error_scope(wgpu::ErrorFilter::Validation));

        let result = self.record_and_submit(frame, extent, sample_count, present_to);

        if let Some(scope) = error_scope {
            let pop_future = scope.pop();
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(err) = crate::realm::block_on(pop_future) {
                self.context.record_validation_error(err.to_string());
            }
            #[cfg(target_arch = "wasm32")]
            {
                let sink = self.context.clone_validation_sink();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(err) = pop_future.await {
                        if let Ok(mut guard) = sink.lock() {
                            guard.push(err.to_string());
                        }
                    }
                });
            }
        }
        self.frame_prepared = true;
        result
    }

    fn record_and_submit(
        &mut self,
        frame: &RenderFrame,
        extent: [u32; 2],
        sample_count: u32,
        present_to: Option<&mut WebGpuWindowPresenter>,
    ) -> Result<WebGpuDeferredFrame, RenderError> {
        let dev = self.context.state()?;
        let targets_slot = self.targets.get_mut("WebGpuTargets")?;
        let needs_alloc = targets_slot
            .as_ref()
            .is_none_or(|t| t.extent != extent || t.sample_count != sample_count);
        if needs_alloc {
            *targets_slot = Some(WebGpuTargets::new(&dev.device, extent, sample_count));
        }
        let targets = targets_slot.as_mut().ok_or_else(|| {
            RenderError::rejected("missing render targets", "allocated WebGpuTargets")
        })?;
        let pipelines = self.pipelines.get_mut("WebGpuPipelines")?;
        let compute = self.compute.get_mut("WebGpuComputePasses")?;

        let mut encoder = dev
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("martensite-frame-encoder"),
            });

        // 1. Optional shadow-map depth pass
        let (has_shadow, shadow_matrix) =
            record_shadow_pass(frame, &dev.device, targets, pipelines, &mut encoder);

        // 2. Optional compute placement culling for instanced batches
        for draw in &frame.draws {
            if let Some(ref instances) = draw.instances {
                let _ = compute.dispatch_placement_cull(&dev.device, &mut encoder, instances);
            }
        }

        // 3. Optional Phase 5 compute OpenPBR ray/path tracing
        let rt_slot = frame.raytrace.as_ref().map(|rt| {
            compute.dispatch_raytrace(
                &dev.device,
                &mut encoder,
                rt,
                extent,
                frame.clear_color,
                &targets.rt_accum_views,
            )
        });

        // 4. Main scene render pass (background, opaque/lit/OpenPBR/edges/lines/points/text/capping/grid)
        let mut wboit_draws: Vec<&PrimitiveDraw> = Vec::new();
        record_main_pass(
            frame,
            extent,
            sample_count,
            has_shadow,
            shadow_matrix,
            rt_slot,
            &dev.device,
            &dev.queue,
            targets,
            pipelines,
            &mut encoder,
            &mut wboit_draws,
        )?;

        // 5. Weighted Blended OIT pass + fullscreen composite (when active)
        if !wboit_draws.is_empty() {
            record_wboit_passes(
                &wboit_draws,
                extent,
                sample_count,
                has_shadow,
                shadow_matrix,
                &dev.device,
                &dev.queue,
                targets,
                pipelines,
                &mut encoder,
            );
        }

        // 6. Subview blits (Phase 5)
        if !frame.subview_blits.is_empty() {
            record_subview_blits(
                frame,
                extent,
                &dev.device,
                &dev.queue,
                targets,
                pipelines,
                &mut encoder,
            );
        }

        // 7. Optional browser canvas presentation
        #[cfg(target_arch = "wasm32")]
        let presented = if let Some(presenter) = present_to {
            presenter.present_texture(&self.context, &mut encoder, &targets.color_tex, extent)?
        } else {
            None
        };
        #[cfg(not(target_arch = "wasm32"))]
        let _ = present_to;

        // 8. Stage host readback copy
        let deferred =
            WebGpuDeferredFrame::stage_copy(&dev.device, &mut encoder, &targets.color_tex, extent);
        dev.queue.submit(std::iter::once(encoder.finish()));

        #[cfg(target_arch = "wasm32")]
        if let Some(surface_tex) = presented {
            dev.queue.present(surface_tex);
        }

        Ok(deferred)
    }
}

#[allow(clippy::too_many_arguments)]
fn record_main_pass<'a>(
    frame: &'a RenderFrame,
    extent: [u32; 2],
    sample_count: u32,
    has_shadow: bool,
    shadow_matrix: [[f32; 4]; 4],
    rt_slot: Option<usize>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    targets: &WebGpuTargets,
    pipelines: &mut WebGpuPipelines,
    encoder: &mut wgpu::CommandEncoder,
    wboit_draws: &mut Vec<&'a PrimitiveDraw>,
) -> Result<(), RenderError> {
    // Pre-validate user programs before opening the render pass so errors return cleanly.
    for draw in &frame.draws {
        if let Some(ref prog) = draw.user_program
            && prog.modules.wgsl().is_none()
        {
            return Err(RenderError::user_program(
                RenderUserProgramFailure::SpirvUnsupportedOnWebGpu {
                    key: prog.modules.key,
                },
            ));
        }
    }

    let (color_view, resolve_target) = match &targets.msaa_color_view {
        Some(msaa) => (msaa, Some(&targets.color_view)),
        None => (&targets.color_view, None),
    };
    let clear = frame.clear_color;
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("martensite-main-pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: color_view,
            depth_slice: None,
            resolve_target,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: f64::from(clear[0]),
                    g: f64::from(clear[1]),
                    b: f64::from(clear[2]),
                    a: f64::from(clear[3]),
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &targets.depth_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(u32::from(frame.stencil_clear_value)),
                store: wgpu::StoreOp::Store,
            }),
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });

    // Background (colored quad / skydome / stereo / raytrace display)
    if let Some(ref stereo) = frame.stereo_pair {
        draw_stereo_pair(stereo, sample_count, device, queue, pipelines, &mut pass);
    } else if let Some(slot) = rt_slot {
        draw_raytrace_display(slot, sample_count, device, targets, pipelines, &mut pass);
    } else if let Some(ref quad) = frame.colored_quad {
        draw_colored_quad(quad, sample_count, device, pipelines, &mut pass);
    }

    let use_wboit = matches!(
        frame.raster.transparency,
        RenderTransparency::Weighted { .. }
    );
    for (span_idx, span) in frame.layers.iter().enumerate() {
        if let Some(ref grid) = frame.shader_grid
            && grid.span == span_idx
        {
            draw_shader_grid(grid, sample_count, device, pipelines, &mut pass);
        }
        let (depth_test, depth_write, _clear_depth) = span.depth_policy();
        for draw in &frame.draws[span.draw_range.clone()] {
            if draw.vertices.is_empty() || draw.indices.is_empty() {
                continue;
            }
            if use_wboit && (draw.surface_blend || draw.surface_forced_blend) {
                wboit_draws.push(draw);
                continue;
            }
            match draw.topology() {
                PrimitiveTopology::Lines => {
                    draw_wide_lines(
                        draw,
                        extent,
                        sample_count,
                        depth_test,
                        depth_write,
                        device,
                        pipelines,
                        &mut pass,
                    );
                }
                PrimitiveTopology::Points => {
                    draw_points(
                        draw,
                        extent,
                        sample_count,
                        depth_test,
                        depth_write,
                        device,
                        queue,
                        pipelines,
                        &mut pass,
                    );
                }
                PrimitiveTopology::Triangles => {
                    draw_surface(
                        draw,
                        extent,
                        sample_count,
                        depth_test,
                        depth_write,
                        has_shadow,
                        shadow_matrix,
                        SurfaceBlendMode::Opaque,
                        device,
                        queue,
                        targets,
                        pipelines,
                        &mut pass,
                    )?;
                }
            }
        }
    }
    if let Some(ref grid) = frame.shader_grid
        && grid.span >= frame.layers.len()
    {
        draw_shader_grid(grid, sample_count, device, pipelines, &mut pass);
    }
    Ok(())
}

fn record_shadow_pass(
    frame: &RenderFrame,
    device: &wgpu::Device,
    targets: &mut WebGpuTargets,
    pipelines: &WebGpuPipelines,
    encoder: &mut wgpu::CommandEncoder,
) -> (bool, [[f32; 4]; 4]) {
    let Some(ref shadow_maps) = frame.shadow_maps else {
        return (false, identity_mat4());
    };
    let Some(first) = shadow_maps.maps.first() else {
        return (false, identity_mat4());
    };
    targets.ensure_shadow_resolution(device, shadow_maps.resolution);
    let light_mat = mat4_from_slice(first.light_matrix);
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("martensite-shadow-pass"),
        color_attachments: &[],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &targets.shadow_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipelines.shadow_pipeline());
    for draw in &frame.draws {
        if draw.topology() != PrimitiveTopology::Triangles || draw.indices.is_empty() {
            continue;
        }
        let world = draw
            .lighting
            .as_ref()
            .map_or_else(identity_mat4, |l| mat4_from_slice(l.world));
        let light_mvp = mul_mat4(light_mat, world);
        let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shadow-mvp"),
            contents: bytes_of(&light_mvp),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shadow-bg"),
            layout: &pipelines.shadow_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            }],
        });
        let (verts, inds) = pack_surface_vertices(draw);
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shadow-vb"),
            contents: cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shadow-ib"),
            contents: cast_slice(&inds),
            usage: wgpu::BufferUsages::INDEX,
        });
        pass.set_bind_group(0, &bg, &[]);
        pass.set_vertex_buffer(0, vb.slice(..));
        pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..inds.len() as u32, 0, 0..1);
    }
    (true, light_mat)
}

#[allow(clippy::too_many_arguments)]
fn draw_surface(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    sample_count: u32,
    depth_test: bool,
    depth_write: bool,
    has_shadow: bool,
    shadow_matrix: [[f32; 4]; 4],
    override_blend: SurfaceBlendMode,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    targets: &WebGpuTargets,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) -> Result<(), RenderError> {
    let (verts, inds) = pack_surface_vertices(draw);
    if verts.is_empty() || inds.is_empty() {
        return Ok(());
    }
    let owned_tex: Option<wgpu::TextureView> = draw
        .textured_style
        .as_ref()
        .map(|ts| pipelines.upload_raster_image(device, queue, &ts.image));
    let has_tex = owned_tex.is_some() || draw.surface_texture.is_some();
    let (uniforms, scene) = pack_draw_state(draw, extent, has_tex, has_shadow, shadow_matrix);

    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("surface-draw-u"),
        contents: bytes_of(&uniforms),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let s_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("surface-scene-u"),
        contents: bytes_of(&scene),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let u_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("surface-u-bg"),
        layout: &pipelines.surface_uniform_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: s_buf.as_entire_binding(),
            },
        ],
    });

    let surf_tex = draw
        .surface_texture
        .as_ref()
        .map(|st| pipelines.upload_surface_texture(device, queue, &st.image));
    let tex_view = owned_tex
        .as_ref()
        .or(surf_tex.as_ref())
        .unwrap_or(&pipelines.white_view);

    let t_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("surface-t-bg"),
        layout: &pipelines.surface_tex_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(tex_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&pipelines.linear_sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&targets.shadow_view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&pipelines.shadow_sampler),
            },
        ],
    });

    let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("surface-vb"),
        contents: cast_slice(&verts),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("surface-ib"),
        contents: cast_slice(&inds),
        usage: wgpu::BufferUsages::INDEX,
    });

    let pipeline = if let Some(ref prog) = draw.user_program {
        let wgsl = prog.modules.wgsl().ok_or_else(|| {
            RenderError::user_program(RenderUserProgramFailure::SpirvUnsupportedOnWebGpu {
                key: prog.modules.key,
            })
        })?;
        pipelines.user_wgsl_pipeline(device, prog.modules.key, wgsl, sample_count, depth_test)?
    } else {
        let blend_mode = if override_blend == SurfaceBlendMode::Wboit {
            SurfaceBlendMode::Wboit
        } else if draw.surface_blend
            || draw.surface_forced_blend
            || draw.textured_style.as_ref().is_some_and(|t| t.blend)
        {
            SurfaceBlendMode::AlphaBlend
        } else {
            SurfaceBlendMode::Opaque
        };
        let cull_mode = match draw.face_culling {
            RenderFaceCulling::Back => 1,
            RenderFaceCulling::Front => 2,
            RenderFaceCulling::None => 0,
        };
        let capping = match draw.capping_pass {
            RenderCappingPass::MaskStart | RenderCappingPass::MaskContinue => 1,
            RenderCappingPass::Surface => 2,
            RenderCappingPass::None => 0,
        };
        let key = SurfacePipelineKey {
            shading_mode: uniforms.counts_flags[0],
            blend_mode,
            cull_mode,
            sample_count,
            depth_test,
            depth_write,
            capping,
        };
        pipelines.surface_pipeline(device, key)
    };

    pass.set_pipeline(pipeline);
    if draw.capping_pass == RenderCappingPass::Surface {
        pass.set_stencil_reference(0);
    }
    pass.set_bind_group(0, &u_bg, &[]);
    pass.set_bind_group(1, &t_bg, &[]);
    pass.set_vertex_buffer(0, vb.slice(..));
    pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(0..inds.len() as u32, 0, 0..1);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_wide_lines(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    sample_count: u32,
    depth_test: bool,
    depth_write: bool,
    device: &wgpu::Device,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let m = mat4_from_slice(draw.clip_transform);
    let mut segments = Vec::with_capacity(draw.indices.len() / 2);
    for pair in draw.indices.as_chunks::<2>().0 {
        let Some(&v0) = draw.vertices.get(pair[0] as usize) else {
            continue;
        };
        let Some(&v1) = draw.vertices.get(pair[1] as usize) else {
            continue;
        };
        segments.push(GpuLineSegment {
            p0: mul_mat4_vec4(
                m,
                [v0.position[0], v0.position[1], v0.position[2], v0.weight],
            ),
            p1: mul_mat4_vec4(
                m,
                [v1.position[0], v1.position[1], v1.position[2], v1.weight],
            ),
            c0: v0.color,
            c1: v1.color,
        });
    }
    if segments.is_empty() {
        return;
    }
    let style = draw.line_style;
    let width = style.map_or(1.0, |s| s.raster_width());
    let pattern = style.map_or(0xffffu32, |s| u32::from(s.pattern));
    let factor = style.map_or(1.0, |s| f32::from(s.factor));
    let blend = style.is_some_and(|s| s.alpha_blend);
    let u = GpuLineUniforms {
        params: [extent[0] as f32, extent[1] as f32, width, factor],
        stipple: [pattern, 0, 0, 0],
    };
    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("line-u"),
        contents: bytes_of(&u),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let seg_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("line-segs"),
        contents: cast_slice(&segments),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("line-bg"),
        layout: &pipelines.wide_line_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: seg_buf.as_entire_binding(),
            },
        ],
    });
    let pipeline =
        pipelines.wide_line_pipeline(device, sample_count, depth_test, depth_write, blend);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..6, 0..segments.len() as u32);
}

#[allow(clippy::too_many_arguments)]
fn draw_points(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    sample_count: u32,
    depth_test: bool,
    depth_write: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let m = mat4_from_slice(draw.clip_transform);
    let mut pts = Vec::with_capacity(draw.indices.len());
    for &idx in draw.indices.iter() {
        let Some(&v) = draw.vertices.get(idx as usize) else {
            continue;
        };
        pts.push(GpuPointInstance {
            position: mul_mat4_vec4(m, [v.position[0], v.position[1], v.position[2], v.weight]),
            color: v.color,
        });
    }
    if pts.is_empty() {
        return;
    }
    let (size_px, blend, has_sprite, cutoff, sprite_view) = if let Some(ref ps) = draw.point_style {
        let has_img = ps.image.extent[0] > 1 || ps.image.extent[1] > 1;
        let view = has_img.then(|| pipelines.upload_raster_image(device, queue, &ps.image));
        (
            ps.size.max(1.0),
            ps.alpha_blend,
            u32::from(has_img),
            ps.fixed_material_cutoff.unwrap_or(0.05),
            view,
        )
    } else {
        (1.0, false, 0, 0.0, None)
    };
    let u = GpuPointUniforms {
        params: [extent[0] as f32, extent[1] as f32, size_px, cutoff],
        flags: [has_sprite, 0, 0, 0],
    };
    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("point-u"),
        contents: bytes_of(&u),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let pt_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("point-inst"),
        contents: cast_slice(&pts),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let data_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("point-data-bg"),
        layout: &pipelines.point_data_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: pt_buf.as_entire_binding(),
            },
        ],
    });
    let tex_view = sprite_view.as_ref().unwrap_or(&pipelines.white_view);
    let tex_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("point-tex-bg"),
        layout: &pipelines.point_tex_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(tex_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&pipelines.nearest_sampler),
            },
        ],
    });
    let pipeline = pipelines.point_pipeline(device, sample_count, depth_test, depth_write, blend);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &data_bg, &[]);
    pass.set_bind_group(1, &tex_bg, &[]);
    pass.draw(0..6, 0..pts.len() as u32);
}

fn draw_colored_quad(
    quad: &martensite_render_core::colored_quad_background::ColoredQuadBackground,
    sample_count: u32,
    device: &wgpu::Device,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let c0 = [quad.colors[0][0], quad.colors[0][1], quad.colors[0][2], 1.0];
    let c1 = [quad.colors[1][0], quad.colors[1][1], quad.colors[1][2], 1.0];
    let (tl, tr, bl, br) = match quad.corner % 4 {
        0 => (c0, c1, c1, c1),
        1 => (c1, c0, c1, c1),
        2 => (c1, c1, c1, c0),
        _ => (c1, c1, c0, c1),
    };
    let u = GpuBackgroundUniforms {
        corner_tl: tl,
        corner_tr: tr,
        corner_bl: bl,
        corner_br: br,
        mode_sun: [0.0, 0.0, 1.0, 0.0],
    };
    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("bg-u"),
        contents: bytes_of(&u),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bg-bg"),
        layout: &pipelines.background_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&pipelines.white_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&pipelines.linear_sampler),
            },
        ],
    });
    let pipeline = pipelines.background_pipeline(device, sample_count);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..3, 0..1);
}

fn draw_shader_grid(
    grid: &ShaderGridPass,
    sample_count: u32,
    device: &wgpu::Device,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let b = &grid.uniforms;
    let u = GpuGridUniforms {
        inv_view_proj: mat4_from_slice(b.projection_inverse),
        view_proj: mat4_from_slice(b.projection),
        grid_step: [
            b.rows[2][3].max(1e-3),
            b.rows[3][3].max(1e-3),
            b.rows[8][0],
            b.rows[8][1],
        ],
        color_main: [b.rows[0][0], b.rows[0][1], b.rows[0][2], 0.8],
        color_sub: [b.rows[1][0], b.rows[1][1], b.rows[1][2], 0.4],
    };
    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("grid-u"),
        contents: bytes_of(&u),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("grid-bg"),
        layout: &pipelines.grid_bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: u_buf.as_entire_binding(),
        }],
    });
    let pipeline = pipelines.grid_pipeline(device, sample_count);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..3, 0..1);
}

fn draw_raytrace_display(
    slot: usize,
    sample_count: u32,
    device: &wgpu::Device,
    targets: &WebGpuTargets,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("rt-display-bg"),
        layout: &pipelines.raytrace_display_bgl,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&targets.rt_accum_views[slot]),
        }],
    });
    let pipeline = pipelines.raytrace_display_pipeline(device, sample_count);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..3, 0..1);
}

fn draw_stereo_pair(
    stereo: &martensite_render_core::stereo_pair::StereoPairComposite,
    sample_count: u32,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipelines: &mut WebGpuPipelines,
    pass: &mut wgpu::RenderPass<'_>,
) {
    let mode = match stereo.program {
        StereoProgram::RedCyan | StereoProgram::Anaglyph => 1u32,
        StereoProgram::RowInterlaced => 4u32,
        StereoProgram::ColumnInterlaced | StereoProgram::ChessBoard => 5u32,
        StereoProgram::SideBySide | StereoProgram::OverUnder => 6u32,
    };
    let u = GpuStereoSubviewUniforms {
        mode_flags: [mode, u32::from(stereo.flip_output), 0, 0],
        rect_uv: [0.0, 0.0, 1.0, 1.0],
    };
    let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("stereo-u"),
        contents: bytes_of(&u),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let _ = pipelines.upload_surface_texture(device, queue, &stereo.samplers[0]);
    let _ = pipelines.upload_surface_texture(device, queue, &stereo.samplers[1]);
    let l_view = upload_rgba8_texture(
        device,
        queue,
        "stereo-left",
        stereo.samplers[0].extent,
        &stereo.samplers[0].bytes,
    );
    let r_view = upload_rgba8_texture(
        device,
        queue,
        "stereo-right",
        stereo.samplers[1].extent,
        &stereo.samplers[1].bytes,
    );
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("stereo-bg"),
        layout: &pipelines.stereo_subview_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&l_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&r_view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&pipelines.linear_sampler),
            },
        ],
    });
    let pipeline = pipelines.stereo_subview_pipeline(device, sample_count, true);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..3, 0..1);
}

#[allow(clippy::too_many_arguments)]
fn record_wboit_passes(
    wboit_draws: &[&PrimitiveDraw],
    extent: [u32; 2],
    sample_count: u32,
    has_shadow: bool,
    shadow_matrix: [[f32; 4]; 4],
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    targets: &WebGpuTargets,
    pipelines: &mut WebGpuPipelines,
    encoder: &mut wgpu::CommandEncoder,
) {
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("martensite-wboit-accum-pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &targets.oit_accum_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &targets.oit_reveal_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        for draw in wboit_draws {
            let _ = draw_surface(
                draw,
                extent,
                1,
                false,
                false,
                has_shadow,
                shadow_matrix,
                SurfaceBlendMode::Wboit,
                device,
                queue,
                targets,
                pipelines,
                &mut pass,
            );
        }
    }
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("oit-composite-bg"),
        layout: &pipelines.oit_composite_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&targets.oit_accum_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&targets.oit_reveal_view),
            },
        ],
    });
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("martensite-wboit-composite"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &targets.color_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let _ = sample_count;
    let pipeline = pipelines.oit_composite_pipeline(device, 1);
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bg, &[]);
    pass.draw(0..3, 0..1);
}

fn record_subview_blits(
    frame: &RenderFrame,
    extent: [u32; 2],
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    targets: &WebGpuTargets,
    pipelines: &mut WebGpuPipelines,
    encoder: &mut wgpu::CommandEncoder,
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("martensite-subview-blits"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &targets.color_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    let w = extent[0].max(1) as f32;
    let h = extent[1].max(1) as f32;
    for blit in &frame.subview_blits {
        let sub_view = upload_rgba8_texture(device, queue, "subview-rgba", blit.extent, &blit.rgba);
        let u = GpuStereoSubviewUniforms {
            mode_flags: [0, 0, 0, 0],
            rect_uv: [
                blit.offset[0] as f32 / w,
                blit.offset[1] as f32 / h,
                blit.extent[0] as f32 / w,
                blit.extent[1] as f32 / h,
            ],
        };
        let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("subview-u"),
            contents: bytes_of(&u),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("subview-bg"),
            layout: &pipelines.stereo_subview_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: u_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&sub_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&pipelines.white_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&pipelines.linear_sampler),
                },
            ],
        });
        let pipeline = pipelines.stereo_subview_pipeline(device, 1, false);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.draw(0..3, 0..1);
    }
}

fn mul_mat4(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0_f32; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            out[c][r] =
                a[0][r] * b[c][0] + a[1][r] * b[c][1] + a[2][r] * b[c][2] + a[3][r] * b[c][3];
        }
    }
    out
}

fn mul_mat4_vec4(m: [[f32; 4]; 4], v: [f32; 4]) -> [f32; 4] {
    [
        m[0][0] * v[0] + m[1][0] * v[1] + m[2][0] * v[2] + m[3][0] * v[3],
        m[0][1] * v[0] + m[1][1] * v[1] + m[2][1] * v[2] + m[3][1] * v[3],
        m[0][2] * v[0] + m[1][2] * v[1] + m[2][2] * v[2] + m[3][2] * v[3],
        m[0][3] * v[0] + m[1][3] * v[1] + m[2][3] * v[2] + m[3][3] * v[3],
    ]
}

impl RenderBackend for WebGpuRenderer {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = frame;
            Err(RenderError::async_readback_required(
                "WebGpuRenderer::render",
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let deferred = self.execute_frame(frame, None)?;
            deferred.read_capture_blocking(&self.context, CapturePlanes::ColorAndDepth)
        }
    }
}

impl RenderDeviceBackend for WebGpuRenderer {
    fn capture_limit(&self) -> u32 {
        self.context.capture_limit()
    }

    fn supports_placement_compaction(&self) -> bool {
        true
    }

    fn device_limits(&self) -> Option<RenderDeviceLimits> {
        Some(self.context.device_limits())
    }

    fn diagnostic_facts(&self) -> Option<RenderDiagnosticFacts> {
        Some(self.context.diagnostic_facts(self.frame_prepared))
    }

    fn image_memory(&self) -> RenderImageMemory {
        RenderImageMemory::default()
    }

    fn work_ledger(&self) -> Option<Arc<FrameWorkLedger>> {
        Some(self.context.work_ledger())
    }

    fn render_frame(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        <Self as RenderBackend>::render(self, frame)
    }

    fn render_deferred(&mut self, frame: &RenderFrame) -> Result<RenderedFrame, RenderError> {
        self.execute_frame(frame, None)
            .map(|deferred| RenderedFrame::Deferred(Box::new(deferred)))
    }

    fn read_frame_rgb(
        &mut self,
        frame: &dyn RenderDeferredFrame,
        flip: bool,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (frame, flip, write_row);
            Err(RenderError::async_readback_required(
                "WebGpuRenderer::read_frame_rgb",
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let capture = self.read_frame(frame, CapturePlanes::Color)?;
            let w = capture.width as usize;
            let h = capture.height as usize;
            let mut rgb_row = vec![0u8; w * 3];
            for y in 0..h {
                let src_y = if flip { h - 1 - y } else { y };
                let src = &capture.rgba[src_y * w * 4..(src_y + 1) * w * 4];
                for x in 0..w {
                    rgb_row[x * 3] = src[x * 4];
                    rgb_row[x * 3 + 1] = src[x * 4 + 1];
                    rgb_row[x * 3 + 2] = src[x * 4 + 2];
                }
                if !write_row(y, &rgb_row) {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }

    fn read_frame(
        &mut self,
        frame: &dyn RenderDeferredFrame,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (frame, planes);
            Err(RenderError::async_readback_required(
                "WebGpuRenderer::read_frame",
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(deferred) = frame.as_any().downcast_ref::<WebGpuDeferredFrame>() else {
                return Err(RenderError::rejected(
                    "foreign deferred frame",
                    "a WebGpuDeferredFrame from this WebGpuRenderer",
                ));
            };
            deferred.read_capture_blocking(&self.context, planes)
        }
    }

    fn validation_errors(&self) -> Result<Vec<String>, RenderValidationObservationFailure> {
        self.context.validation_errors()
    }

    fn driver_identity(&self) -> Option<RenderDriverIdentity> {
        Some(self.context.driver_identity())
    }

    fn memory_report(&self) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
        Ok(RenderDeviceMemoryReport::unavailable())
    }

    fn raster_policy(&self) -> Option<RenderRasterPolicy> {
        Some(self.raster)
    }

    fn coordinate_convention(&self) -> Option<RenderCoordinateConvention> {
        Some(self.coordinates)
    }

    fn device_name(&self) -> &str {
        self.context.device_name()
    }

    fn window_presenter(
        &self,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<Box<dyn RenderWindowPresenter>, RenderError> {
        WebGpuWindowPresenter::create(&self.context, window)
            .map(|p| Box::new(p) as Box<dyn RenderWindowPresenter>)
    }

    fn render_window(
        &mut self,
        frame: &RenderFrame,
        target: &mut dyn RenderWindowPresenter,
        _frame_queue: WindowFrameQueue,
        _policy: RenderGuiSurfacePolicy,
    ) -> Result<RenderedFrame, RenderError> {
        let Some(presenter) = target.as_any_mut().downcast_mut::<WebGpuWindowPresenter>() else {
            return Err(RenderError::rejected(
                "foreign window presenter",
                "a WebGpuWindowPresenter from this WebGpuRenderer",
            ));
        };
        self.execute_frame(frame, Some(presenter))
            .map(|deferred| RenderedFrame::Deferred(Box::new(deferred)))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
