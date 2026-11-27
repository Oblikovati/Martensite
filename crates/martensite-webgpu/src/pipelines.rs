//! Lazy WebGPU Core pipeline caches, bind-group layouts, and texture uploads (#634, #635).
use crate::gpu_types::GpuSurfaceVertex;
use crate::shaders;
use crate::targets::{
    OIT_ACCUM_FORMAT, OIT_REVEAL_FORMAT, SCENE_COLOR_FORMAT, SCENE_DEPTH_FORMAT,
    SHADOW_DEPTH_FORMAT,
};
use martensite_render_core::surface_texture::{SurfaceImageEncoding, SurfaceTextureImage};
use martensite_render_core::user_program::{UserProgramKey, WgslUserProgram};
use martensite_render_core::{RasterImage, RenderCappingPass, RenderError};
use std::collections::BTreeMap;

/// Surface blend destination selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SurfaceBlendMode {
    Opaque,
    AlphaBlend,
    Wboit,
}

/// Cache key for unified `surface.wgsl` render pipelines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SurfacePipelineKey {
    pub shading_mode: u32,
    pub blend_mode: SurfaceBlendMode,
    pub cull_mode: u8, // 0=None, 1=Back, 2=Front
    pub sample_count: u32,
    pub depth_test: bool,
    pub depth_write: bool,
    pub capping: u8, // 0=None, 1=Mask, 2=Surface
}

/// Cache key for auxiliary pipelines (lines, points, grids, backgrounds, OIT, stereo, display).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AuxKey {
    sample_count: u32,
    depth_test: bool,
    depth_write: bool,
    blend: bool,
}

/// Uploaded GPU texture view cached by resource identity + revision.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TextureCacheKey {
    id: Vec<u8>,
    revision: usize,
    extent: [u32; 2],
}

/// Owned `wgpu` shader modules, bind-group layouts, samplers, and lazy render pipelines.
pub(crate) struct WebGpuPipelines {
    surface_module: wgpu::ShaderModule,
    background_module: wgpu::ShaderModule,
    wide_line_module: wgpu::ShaderModule,
    point_sprite_module: wgpu::ShaderModule,
    oit_composite_module: wgpu::ShaderModule,
    grid_module: wgpu::ShaderModule,
    raytrace_display_module: wgpu::ShaderModule,
    stereo_subview_module: wgpu::ShaderModule,

    pub surface_uniform_bgl: wgpu::BindGroupLayout,
    pub surface_tex_bgl: wgpu::BindGroupLayout,
    pub background_bgl: wgpu::BindGroupLayout,
    pub wide_line_bgl: wgpu::BindGroupLayout,
    pub point_data_bgl: wgpu::BindGroupLayout,
    pub point_tex_bgl: wgpu::BindGroupLayout,
    pub oit_composite_bgl: wgpu::BindGroupLayout,
    pub shadow_bgl: wgpu::BindGroupLayout,
    pub grid_bgl: wgpu::BindGroupLayout,
    pub raytrace_display_bgl: wgpu::BindGroupLayout,
    pub stereo_subview_bgl: wgpu::BindGroupLayout,

    pub linear_sampler: wgpu::Sampler,
    pub nearest_sampler: wgpu::Sampler,
    pub shadow_sampler: wgpu::Sampler,
    pub white_view: wgpu::TextureView,

    surface_pipelines: BTreeMap<SurfacePipelineKey, wgpu::RenderPipeline>,
    background_pipelines: BTreeMap<u32, wgpu::RenderPipeline>,
    wide_line_pipelines: BTreeMap<AuxKey, wgpu::RenderPipeline>,
    point_pipelines: BTreeMap<AuxKey, wgpu::RenderPipeline>,
    oit_composite_pipelines: BTreeMap<u32, wgpu::RenderPipeline>,
    shadow_pipeline: wgpu::RenderPipeline,
    grid_pipelines: BTreeMap<u32, wgpu::RenderPipeline>,
    raytrace_display_pipelines: BTreeMap<u32, wgpu::RenderPipeline>,
    stereo_subview_pipelines: BTreeMap<(u32, bool), wgpu::RenderPipeline>,
    user_wgsl_pipelines: BTreeMap<(UserProgramKey, u32, bool), wgpu::RenderPipeline>,
    texture_cache: BTreeMap<TextureCacheKey, wgpu::TextureView>,
}

impl WebGpuPipelines {
    /// Compile built-in WGSL shader modules and shared bind-group layouts on `device`.
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mk_shader = |label: &str, src: &str| {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            })
        };
        let surface_module = mk_shader("surface.wgsl", shaders::SURFACE_WGSL);
        let background_module = mk_shader("background.wgsl", shaders::BACKGROUND_WGSL);
        let wide_line_module = mk_shader("wide_line.wgsl", shaders::WIDE_LINE_WGSL);
        let point_sprite_module = mk_shader("point_sprite.wgsl", shaders::POINT_SPRITE_WGSL);
        let oit_composite_module = mk_shader("oit_composite.wgsl", shaders::OIT_COMPOSITE_WGSL);
        let shadow_depth_module = mk_shader("shadow_depth.wgsl", shaders::SHADOW_DEPTH_WGSL);
        let grid_module = mk_shader("grid.wgsl", shaders::GRID_WGSL);
        let raytrace_display_module =
            mk_shader("raytrace_display.wgsl", shaders::RAYTRACE_DISPLAY_WGSL);
        let stereo_subview_module = mk_shader("stereo_subview.wgsl", shaders::STEREO_SUBVIEW_WGSL);

        let surface_uniform_bgl =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("surface-uniform-bgl"),
                entries: &[
                    uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT),
                    uniform_entry(1, wgpu::ShaderStages::VERTEX_FRAGMENT),
                ],
            });
        let surface_tex_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("surface-tex-bgl"),
            entries: &[
                tex2d_entry(0, wgpu::ShaderStages::FRAGMENT),
                sampler_entry(1, wgpu::ShaderStages::FRAGMENT),
                depth_tex_entry(2, wgpu::ShaderStages::FRAGMENT),
                cmp_sampler_entry(3, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let background_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("background-bgl"),
            entries: &[
                uniform_entry(0, wgpu::ShaderStages::FRAGMENT),
                tex2d_entry(1, wgpu::ShaderStages::FRAGMENT),
                sampler_entry(2, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let wide_line_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("wide-line-bgl"),
            entries: &[
                uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT),
                storage_ro_entry(1, wgpu::ShaderStages::VERTEX),
            ],
        });
        let point_data_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("point-data-bgl"),
            entries: &[
                uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT),
                storage_ro_entry(1, wgpu::ShaderStages::VERTEX),
            ],
        });
        let point_tex_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("point-tex-bgl"),
            entries: &[
                tex2d_entry(0, wgpu::ShaderStages::FRAGMENT),
                sampler_entry(1, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let oit_composite_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("oit-composite-bgl"),
            entries: &[
                nonfilter_tex2d_entry(0, wgpu::ShaderStages::FRAGMENT),
                nonfilter_tex2d_entry(1, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let shadow_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shadow-bgl"),
            entries: &[uniform_entry(0, wgpu::ShaderStages::VERTEX)],
        });
        let grid_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("grid-bgl"),
            entries: &[uniform_entry(0, wgpu::ShaderStages::VERTEX_FRAGMENT)],
        });
        let raytrace_display_bgl =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("raytrace-display-bgl"),
                entries: &[nonfilter_tex2d_entry(0, wgpu::ShaderStages::FRAGMENT)],
            });
        let stereo_subview_bgl =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("stereo-subview-bgl"),
                entries: &[
                    uniform_entry(0, wgpu::ShaderStages::FRAGMENT),
                    tex2d_entry(1, wgpu::ShaderStages::FRAGMENT),
                    tex2d_entry(2, wgpu::ShaderStages::FRAGMENT),
                    sampler_entry(3, wgpu::ShaderStages::FRAGMENT),
                ],
            });

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("martensite-linear-sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });
        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("martensite-nearest-sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("martensite-shadow-sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let white_view = upload_rgba8_texture(
            device,
            queue,
            "martensite-white-1x1",
            [1, 1],
            &[255, 255, 255, 255],
        );

        let shadow_pipeline = build_shadow_pipeline(device, &shadow_bgl, &shadow_depth_module);

        Self {
            surface_module,
            background_module,
            wide_line_module,
            point_sprite_module,
            oit_composite_module,
            grid_module,
            raytrace_display_module,
            stereo_subview_module,
            surface_uniform_bgl,
            surface_tex_bgl,
            background_bgl,
            wide_line_bgl,
            point_data_bgl,
            point_tex_bgl,
            oit_composite_bgl,
            shadow_bgl,
            grid_bgl,
            raytrace_display_bgl,
            stereo_subview_bgl,
            linear_sampler,
            nearest_sampler,
            shadow_sampler,
            white_view,
            surface_pipelines: BTreeMap::new(),
            background_pipelines: BTreeMap::new(),
            wide_line_pipelines: BTreeMap::new(),
            point_pipelines: BTreeMap::new(),
            oit_composite_pipelines: BTreeMap::new(),
            shadow_pipeline,
            grid_pipelines: BTreeMap::new(),
            raytrace_display_pipelines: BTreeMap::new(),
            stereo_subview_pipelines: BTreeMap::new(),
            user_wgsl_pipelines: BTreeMap::new(),
            texture_cache: BTreeMap::new(),
        }
    }

    /// Return the depth-only shadow-map render pipeline.
    pub fn shadow_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.shadow_pipeline
    }

    /// Get or lazily compile a `surface.wgsl` pipeline specialized for `key`.
    pub fn surface_pipeline(
        &mut self,
        device: &wgpu::Device,
        key: SurfacePipelineKey,
    ) -> &wgpu::RenderPipeline {
        if !self.surface_pipelines.contains_key(&key) {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("surface-layout"),
                bind_group_layouts: &[Some(&self.surface_uniform_bgl), Some(&self.surface_tex_bgl)],
                immediate_size: 0,
            });
            let pipeline = build_surface_pipeline(device, &layout, &self.surface_module, key);
            self.surface_pipelines.insert(key, pipeline);
        }
        &self.surface_pipelines[&key]
    }

    /// Get or lazily compile the background pipeline for `sample_count`.
    pub fn background_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
    ) -> &wgpu::RenderPipeline {
        self.background_pipelines
            .entry(sample_count)
            .or_insert_with(|| {
                let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("background-layout"),
                    bind_group_layouts: &[Some(&self.background_bgl)],
                    immediate_size: 0,
                });
                build_fullscreen_pipeline(
                    device,
                    &layout,
                    &self.background_module,
                    "background",
                    sample_count,
                    None,
                    true,
                )
            })
    }

    /// Get or lazily compile the instanced wide/stippled line pipeline.
    pub fn wide_line_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
        depth_test: bool,
        depth_write: bool,
        blend: bool,
    ) -> &wgpu::RenderPipeline {
        let key = AuxKey {
            sample_count,
            depth_test,
            depth_write,
            blend,
        };
        self.wide_line_pipelines.entry(key).or_insert_with(|| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("wide-line-layout"),
                bind_group_layouts: &[Some(&self.wide_line_bgl)],
                immediate_size: 0,
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("wide-line-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &self.wide_line_module,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(scene_depth_stencil(
                    depth_test,
                    depth_write,
                    RenderCappingPass::None,
                )),
                multisample: multisample_state(sample_count),
                fragment: Some(wgpu::FragmentState {
                    module: &self.wide_line_module,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(color_target(blend))],
                }),
                multiview_mask: None,
                cache: None,
            })
        })
    }

    /// Get or lazily compile the instanced point/marker sprite pipeline.
    pub fn point_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
        depth_test: bool,
        depth_write: bool,
        blend: bool,
    ) -> &wgpu::RenderPipeline {
        let key = AuxKey {
            sample_count,
            depth_test,
            depth_write,
            blend,
        };
        self.point_pipelines.entry(key).or_insert_with(|| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("point-sprite-layout"),
                bind_group_layouts: &[Some(&self.point_data_bgl), Some(&self.point_tex_bgl)],
                immediate_size: 0,
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("point-sprite-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &self.point_sprite_module,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(scene_depth_stencil(
                    depth_test,
                    depth_write,
                    RenderCappingPass::None,
                )),
                multisample: multisample_state(sample_count),
                fragment: Some(wgpu::FragmentState {
                    module: &self.point_sprite_module,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(color_target(blend))],
                }),
                multiview_mask: None,
                cache: None,
            })
        })
    }

    /// Get or lazily compile the WBOIT composite pipeline for `sample_count`.
    pub fn oit_composite_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
    ) -> &wgpu::RenderPipeline {
        self.oit_composite_pipelines
            .entry(sample_count)
            .or_insert_with(|| {
                let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("oit-composite-layout"),
                    bind_group_layouts: &[Some(&self.oit_composite_bgl)],
                    immediate_size: 0,
                });
                build_fullscreen_pipeline(
                    device,
                    &layout,
                    &self.oit_composite_module,
                    "oit-composite",
                    sample_count,
                    Some(wgpu::BlendState::ALPHA_BLENDING),
                    false,
                )
            })
    }

    /// Get or lazily compile the procedural `RenderShaderGrid` pipeline.
    pub fn grid_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
    ) -> &wgpu::RenderPipeline {
        self.grid_pipelines.entry(sample_count).or_insert_with(|| {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("grid-layout"),
                bind_group_layouts: &[Some(&self.grid_bgl)],
                immediate_size: 0,
            });
            build_fullscreen_pipeline(
                device,
                &layout,
                &self.grid_module,
                "grid",
                sample_count,
                Some(wgpu::BlendState::ALPHA_BLENDING),
                true,
            )
        })
    }

    /// Get or lazily compile the compute raytrace `Rgba16Float` display pipeline.
    pub fn raytrace_display_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
    ) -> &wgpu::RenderPipeline {
        self.raytrace_display_pipelines
            .entry(sample_count)
            .or_insert_with(|| {
                let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("raytrace-display-layout"),
                    bind_group_layouts: &[Some(&self.raytrace_display_bgl)],
                    immediate_size: 0,
                });
                build_fullscreen_pipeline(
                    device,
                    &layout,
                    &self.raytrace_display_module,
                    "raytrace-display",
                    sample_count,
                    None,
                    true,
                )
            })
    }

    /// Get or lazily compile the stereo pair / subview blit pipeline.
    pub fn stereo_subview_pipeline(
        &mut self,
        device: &wgpu::Device,
        sample_count: u32,
        has_depth_attachment: bool,
    ) -> &wgpu::RenderPipeline {
        self.stereo_subview_pipelines
            .entry((sample_count, has_depth_attachment))
            .or_insert_with(|| {
                let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("stereo-subview-layout"),
                    bind_group_layouts: &[Some(&self.stereo_subview_bgl)],
                    immediate_size: 0,
                });
                build_fullscreen_pipeline(
                    device,
                    &layout,
                    &self.stereo_subview_module,
                    "stereo-subview",
                    sample_count,
                    None,
                    has_depth_attachment,
                )
            })
    }

    /// Compile and cache a WGSL user shader program pipeline (#635).
    pub fn user_wgsl_pipeline(
        &mut self,
        device: &wgpu::Device,
        key: UserProgramKey,
        wgsl: &WgslUserProgram,
        sample_count: u32,
        depth_test: bool,
    ) -> Result<&wgpu::RenderPipeline, RenderError> {
        let cache_key = (key, sample_count, depth_test);
        if !self.user_wgsl_pipelines.contains_key(&cache_key) {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("user-wgsl-program"),
                source: wgpu::ShaderSource::Wgsl(wgsl.source.as_ref().into()),
            });
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("user-wgsl-layout"),
                bind_group_layouts: &[Some(&self.surface_uniform_bgl), Some(&self.surface_tex_bgl)],
                immediate_size: 0,
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("user-wgsl-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some(wgsl.vertex_entry.as_ref()),
                    compilation_options: Default::default(),
                    buffers: &[Some(surface_vertex_layout())],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: Some(scene_depth_stencil(
                    depth_test,
                    depth_test,
                    RenderCappingPass::None,
                )),
                multisample: multisample_state(sample_count),
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some(wgsl.fragment_entry.as_ref()),
                    compilation_options: Default::default(),
                    targets: &[Some(color_target(false))],
                }),
                multiview_mask: None,
                cache: None,
            });
            self.user_wgsl_pipelines.insert(cache_key, pipeline);
        }
        Ok(&self.user_wgsl_pipelines[&cache_key])
    }

    /// Upload or reuse a cached `SurfaceTextureImage` as an RGBA8 texture view.
    pub fn upload_surface_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &SurfaceTextureImage,
    ) -> wgpu::TextureView {
        let key = TextureCacheKey {
            id: image.id.clone(),
            revision: image.revision,
            extent: image.extent,
        };
        if !self.texture_cache.contains_key(&key) {
            let rgba = convert_surface_image_to_rgba8(image);
            let view = upload_rgba8_texture(device, queue, "surface-texture", image.extent, &rgba);
            self.texture_cache.insert(key.clone(), view);
        }
        self.texture_cache[&key].clone()
    }

    /// Upload a `RasterImage` (text glyph atlas or marker sprite) as an RGBA8 texture view.
    pub fn upload_raster_image(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &RasterImage,
    ) -> wgpu::TextureView {
        let mut rgba = Vec::with_capacity(image.pixels().len() * 4);
        for px in image.pixels() {
            for c in px {
                rgba.push((c.clamp(0.0, 1.0) * 255.0).round() as u8);
            }
        }
        upload_rgba8_texture(device, queue, "raster-image", image.extent, &rgba)
    }
}

pub(crate) fn upload_rgba8_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    extent: [u32; 2],
    rgba: &[u8],
) -> wgpu::TextureView {
    let w = extent[0].max(1);
    let h = extent[1].max(1);
    let expected = (w as usize) * (h as usize) * 4;
    let data = if rgba.len() >= expected {
        &rgba[..expected]
    } else {
        &[255u8; 4]
    };
    let (tw, th) = if rgba.len() >= expected {
        (w, h)
    } else {
        (1, 1)
    };
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: tw,
            height: th,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(tw * 4),
            rows_per_image: Some(th),
        },
        wgpu::Extent3d {
            width: tw,
            height: th,
            depth_or_array_layers: 1,
        },
    );
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}

fn convert_surface_image_to_rgba8(image: &SurfaceTextureImage) -> Vec<u8> {
    let px_count = (image.extent[0] as usize) * (image.extent[1] as usize);
    if image.alpha_only && image.bytes.len() >= px_count {
        let mut out = Vec::with_capacity(px_count * 4);
        for &a in &image.bytes[..px_count] {
            out.extend_from_slice(&[255, 255, 255, a]);
        }
        return out;
    }
    if matches!(
        image.encoding,
        SurfaceImageEncoding::Srgb8 | SurfaceImageEncoding::Linear8
    ) && image.bytes.len() >= px_count * 4
    {
        return image.bytes[..px_count * 4].to_vec();
    }
    vec![255u8; px_count.max(1) * 4]
}

fn surface_vertex_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<GpuSurfaceVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 16,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 32,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 48,
                shader_location: 3,
            },
        ],
    }
}

fn build_surface_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    module: &wgpu::ShaderModule,
    key: SurfacePipelineKey,
) -> wgpu::RenderPipeline {
    let cull_mode = match key.cull_mode {
        1 => Some(wgpu::Face::Back),
        2 => Some(wgpu::Face::Front),
        _ => None,
    };
    let capping = match key.capping {
        1 => RenderCappingPass::MaskStart,
        2 => RenderCappingPass::Surface,
        _ => RenderCappingPass::None,
    };
    let is_wboit = key.blend_mode == SurfaceBlendMode::Wboit;
    let constants = [
        ("SHADING_MODE", f64::from(key.shading_mode)),
        ("USE_WBOIT", if is_wboit { 1.0 } else { 0.0 }),
    ];
    let wboit_targets = [
        Some(wgpu::ColorTargetState {
            format: OIT_ACCUM_FORMAT,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }),
        Some(wgpu::ColorTargetState {
            format: OIT_REVEAL_FORMAT,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Zero,
                    dst_factor: wgpu::BlendFactor::OneMinusSrc,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent::OVER,
            }),
            write_mask: wgpu::ColorWrites::RED,
        }),
    ];
    let single_target = [Some(wgpu::ColorTargetState {
        format: SCENE_COLOR_FORMAT,
        blend: (key.blend_mode == SurfaceBlendMode::AlphaBlend)
            .then_some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: if capping.is_mask() {
            wgpu::ColorWrites::empty()
        } else {
            wgpu::ColorWrites::ALL
        },
    })];
    let targets: &[Option<wgpu::ColorTargetState>] = if is_wboit {
        &wboit_targets
    } else {
        &single_target
    };
    let entry_point = if is_wboit { "fs_wboit" } else { "fs_main" };
    let samples = if is_wboit { 1 } else { key.sample_count };
    let depth_stencil = if is_wboit {
        None
    } else {
        Some(scene_depth_stencil(
            key.depth_test,
            key.depth_write,
            capping,
        ))
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("surface-pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
            buffers: &[Some(surface_vertex_layout())],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode,
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil,
        multisample: multisample_state(samples),
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(entry_point),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
            targets,
        }),
        multiview_mask: None,
        cache: None,
    })
}

fn build_shadow_pipeline(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    module: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("shadow-layout"),
        bind_group_layouts: &[Some(bgl)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("shadow-depth-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<GpuSurfaceVertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                }],
            })],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: SHADOW_DEPTH_FORMAT,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState {
                constant: 2,
                slope_scale: 1.5,
                clamp: 0.0,
            },
        }),
        multisample: multisample_state(1),
        fragment: None,
        multiview_mask: None,
        cache: None,
    })
}

fn build_fullscreen_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    module: &wgpu::ShaderModule,
    label: &str,
    sample_count: u32,
    blend: Option<wgpu::BlendState>,
    has_depth_attachment: bool,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: has_depth_attachment.then(|| wgpu::DepthStencilState {
            format: SCENE_DEPTH_FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::Always),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: multisample_state(sample_count),
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: SCENE_COLOR_FORMAT,
                blend,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

fn scene_depth_stencil(
    depth_test: bool,
    depth_write: bool,
    capping: RenderCappingPass,
) -> wgpu::DepthStencilState {
    let stencil = match capping {
        RenderCappingPass::MaskStart | RenderCappingPass::MaskContinue => wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Invert,
            },
            back: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::Always,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Invert,
            },
            read_mask: 0x01,
            write_mask: 0x01,
        },
        RenderCappingPass::Surface => wgpu::StencilState {
            front: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::NotEqual,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Keep,
            },
            back: wgpu::StencilFaceState {
                compare: wgpu::CompareFunction::NotEqual,
                fail_op: wgpu::StencilOperation::Keep,
                depth_fail_op: wgpu::StencilOperation::Keep,
                pass_op: wgpu::StencilOperation::Keep,
            },
            read_mask: 0x01,
            write_mask: 0x00,
        },
        RenderCappingPass::None => wgpu::StencilState::default(),
    };
    wgpu::DepthStencilState {
        format: SCENE_DEPTH_FORMAT,
        depth_write_enabled: Some(if capping.is_mask() {
            false
        } else {
            depth_write
        }),
        depth_compare: Some(if capping.is_mask() || !depth_test {
            wgpu::CompareFunction::Always
        } else {
            wgpu::CompareFunction::LessEqual
        }),
        stencil,
        bias: wgpu::DepthBiasState::default(),
    }
}

fn color_target(blend: bool) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
        format: SCENE_COLOR_FORMAT,
        blend: blend.then_some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    }
}

fn multisample_state(sample_count: u32) -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: if sample_count >= 4 { 4 } else { 1 },
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}

fn uniform_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage_ro_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn tex2d_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn nonfilter_tex2d_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn depth_tex_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Depth,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn sampler_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    }
}

fn cmp_sampler_entry(binding: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
        count: None,
    }
}
