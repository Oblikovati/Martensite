//! WebGPU Core compute pipelines: placement frustum culling & OpenPBR ray/path tracing (#634, #635).
use crate::gpu_types::{
    GpuCullUniforms, GpuDrawIndexedIndirect, GpuLight, GpuPlacementInput, GpuRayBvhNode,
    GpuRayMaterial, GpuRayTriangle, GpuRaytraceUniforms, mat4_from_slice,
};
use crate::shaders;
use crate::targets::OIT_ACCUM_FORMAT;
use bytemuck::cast_slice;
use martensite_render_core::draw_instances::{DrawInstances, InstanceBounds};
use martensite_render_core::raytrace::RaytraceFrame;
use martensite_render_core::raytrace::pt_scene::PtSceneData;
use wgpu::util::DeviceExt;

/// Compute pipelines and progressive accumulation state owned by `WebGpuRenderer`.
pub(crate) struct WebGpuComputePasses {
    cull_bgl: wgpu::BindGroupLayout,
    cull_pipeline: wgpu::ComputePipeline,
    raytrace_bgl: wgpu::BindGroupLayout,
    raytrace_pipeline: wgpu::ComputePipeline,
    rt_sample_index: u32,
    rt_ping_pong: usize,
    rt_last_geom_hash: u64,
}

impl WebGpuComputePasses {
    /// Compile compute shaders and layouts on `device`.
    pub fn new(device: &wgpu::Device) -> Self {
        let cull_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("placement_cull.wgsl"),
            source: wgpu::ShaderSource::Wgsl(shaders::PLACEMENT_CULL_WGSL.into()),
        });
        let cull_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("placement-cull-bgl"),
            entries: &[
                buf_entry(0, wgpu::BufferBindingType::Uniform),
                buf_entry(1, wgpu::BufferBindingType::Storage { read_only: true }),
                buf_entry(2, wgpu::BufferBindingType::Storage { read_only: false }),
            ],
        });
        let cull_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("placement-cull-layout"),
            bind_group_layouts: &[Some(&cull_bgl)],
            immediate_size: 0,
        });
        let cull_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("placement-cull-pipeline"),
            layout: Some(&cull_layout),
            module: &cull_module,
            entry_point: Some("cs_cull"),
            compilation_options: Default::default(),
            cache: None,
        });

        let rt_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("raytrace_render.wgsl"),
            source: wgpu::ShaderSource::Wgsl(shaders::RAYTRACE_RENDER_WGSL.into()),
        });
        let raytrace_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("raytrace-compute-bgl"),
            entries: &[
                buf_entry(0, wgpu::BufferBindingType::Uniform),
                buf_entry(1, wgpu::BufferBindingType::Storage { read_only: true }),
                buf_entry(2, wgpu::BufferBindingType::Storage { read_only: true }),
                buf_entry(3, wgpu::BufferBindingType::Storage { read_only: true }),
                buf_entry(4, wgpu::BufferBindingType::Storage { read_only: true }),
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: OIT_ACCUM_FORMAT,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let rt_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("raytrace-compute-layout"),
            bind_group_layouts: &[Some(&raytrace_bgl)],
            immediate_size: 0,
        });
        let raytrace_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("raytrace-compute-pipeline"),
            layout: Some(&rt_layout),
            module: &rt_module,
            entry_point: Some("cs_main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            cull_bgl,
            cull_pipeline,
            raytrace_bgl,
            raytrace_pipeline,
            rt_sample_index: 0,
            rt_ping_pong: 0,
            rt_last_geom_hash: 0,
        }
    }

    /// Dispatch GPU placement frustum culling, producing an indirect draw buffer (#634).
    pub fn dispatch_placement_cull(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        instances: &DrawInstances,
    ) -> Option<wgpu::Buffer> {
        let count = instances.placements.len();
        if count == 0 {
            return None;
        }
        let template_bounds: Vec<InstanceBounds> = instances
            .templates
            .iter()
            .map(|t| {
                InstanceBounds::of(t).unwrap_or(InstanceBounds {
                    minimum: [-1.0; 3],
                    maximum: [1.0; 3],
                })
            })
            .collect();
        let inputs: Vec<GpuPlacementInput> = instances
            .placements
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let tpl_idx = p.template as usize;
                let b = template_bounds
                    .get(tpl_idx)
                    .copied()
                    .unwrap_or(InstanceBounds {
                        minimum: [-1.0; 3],
                        maximum: [1.0; 3],
                    });
                let idx_count = instances
                    .templates
                    .get(tpl_idx)
                    .map_or(0, |t| t.indices.len() as u32);
                let w = mat4_from_slice(p.world);
                GpuPlacementInput {
                    world_col0: w[0],
                    world_col1: w[1],
                    world_col2: w[2],
                    world_col3: w[3],
                    aabb_min: [b.minimum[0], b.minimum[1], b.minimum[2], 1.0],
                    aabb_max: [b.maximum[0], b.maximum[1], b.maximum[2], 1.0],
                    index_count: idx_count,
                    first_index: 0,
                    base_vertex: 0,
                    instance_id: i as u32,
                }
            })
            .collect();

        let uniforms = GpuCullUniforms {
            view_proj: mat4_from_slice(instances.projection_view),
            count: count as u32,
            _pad: [0; 3],
        };
        let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("placement-cull-uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let in_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("placement-cull-inputs"),
            contents: cast_slice(&inputs),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let out_size = (count * std::mem::size_of::<GpuDrawIndexedIndirect>()) as u64;
        let out_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("placement-cull-indirect"),
            size: out_size.max(20),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::INDIRECT
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("placement-cull-bg"),
            layout: &self.cull_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: u_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: in_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: out_buf.as_entire_binding(),
                },
            ],
        });
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("placement-cull-pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.cull_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups((count as u32).div_ceil(64), 1, 1);
        }
        Some(out_buf)
    }

    /// Dispatch compute OpenPBR ray/path tracing and return the active `Rgba16Float` output slot (#635).
    pub fn dispatch_raytrace(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        rt: &RaytraceFrame,
        extent: [u32; 2],
        clear_color: [f32; 4],
        accum_views: &[wgpu::TextureView; 2],
    ) -> usize {
        if rt.rebuilt || rt.reset_history || self.rt_last_geom_hash != rt.pt_scene.geometry_hash {
            self.rt_sample_index = 0;
            self.rt_last_geom_hash = rt.pt_scene.geometry_hash;
        }
        let prev_slot = self.rt_ping_pong;
        let next_slot = 1 - prev_slot;
        let (tris, bvh, mats, lights) = pack_pt_scene(&rt.pt_scene);

        let eye = rt.uniforms.eye;
        let origins = rt.uniforms.camera.origins;
        let dirs = rt.uniforms.camera.directions;
        let origin = if eye.view.iter().any(|&v| v.abs() > 1e-6) {
            eye.origin
        } else {
            origins[2] // LT origin
        };
        let ray_tl = dirs[2]; // LT
        let ray_dx = [
            dirs[3][0] - dirs[2][0],
            dirs[3][1] - dirs[2][1],
            dirs[3][2] - dirs[2][2],
        ];
        let ray_dy = [
            dirs[0][0] - dirs[2][0],
            dirs[0][1] - dirs[2][1],
            dirs[0][2] - dirs[2][2],
        ];
        let bounces = rt.constants.bounces.clamp(1, 8) as f32;
        let uniforms = GpuRaytraceUniforms {
            eye_sample: [origin[0], origin[1], origin[2], self.rt_sample_index as f32],
            ray_tl_bounces: [ray_tl[0], ray_tl[1], ray_tl[2], bounces],
            ray_dx_width: [ray_dx[0], ray_dx[1], ray_dx[2], extent[0] as f32],
            ray_dy_height: [ray_dy[0], ray_dy[1], ray_dy[2], extent[1] as f32],
            background: clear_color,
            counts: [
                rt.pt_scene.triangles.len() as u32,
                rt.pt_scene.bvh_nodes.len() as u32,
                rt.pt_scene.lights.len() as u32,
                u32::from(rt.path_tracing.is_some()),
            ],
        };

        let u_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rt-uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let tri_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rt-triangles"),
            contents: cast_slice(&tris),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let bvh_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rt-bvh"),
            contents: cast_slice(&bvh),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let mat_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rt-materials"),
            contents: cast_slice(&mats),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let light_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rt-lights"),
            contents: cast_slice(&lights),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rt-compute-bg"),
            layout: &self.raytrace_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: u_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tri_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: bvh_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: mat_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: light_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&accum_views[prev_slot]),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::TextureView(&accum_views[next_slot]),
                },
            ],
        });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("rt-compute-pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.raytrace_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.dispatch_workgroups(extent[0].div_ceil(8), extent[1].div_ceil(8), 1);
        }

        self.rt_ping_pong = next_slot;
        self.rt_sample_index = self.rt_sample_index.saturating_add(1);
        next_slot
    }
}

fn pack_pt_scene(
    scene: &PtSceneData,
) -> (
    Vec<GpuRayTriangle>,
    Vec<GpuRayBvhNode>,
    Vec<GpuRayMaterial>,
    Vec<GpuLight>,
) {
    let mut tris = Vec::with_capacity(scene.triangles.len().max(1));
    for tri in &scene.triangles {
        let v0 = scene
            .vertices
            .get(tri.idx_mat[0] as usize)
            .map_or([0.0; 4], |v| v.pos_u);
        let v1 = scene
            .vertices
            .get(tri.idx_mat[1] as usize)
            .map_or([0.0; 4], |v| v.pos_u);
        let v2 = scene
            .vertices
            .get(tri.idx_mat[2] as usize)
            .map_or([0.0; 4], |v| v.pos_u);
        let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
        let n = cross3(e1, e2);
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-8);
        let mat_idx = tri.idx_mat[3].min(scene.materials.len().saturating_sub(1) as u32);
        tris.push(GpuRayTriangle {
            v0_mat: [v0[0], v0[1], v0[2], f32::from_bits(mat_idx)],
            e1: [e1[0], e1[1], e1[2], 0.0],
            e2: [e2[0], e2[1], e2[2], 0.0],
            normal: [n[0] / len, n[1] / len, n[2] / len, 0.0],
        });
    }
    if tris.is_empty() {
        tris.push(GpuRayTriangle::default());
    }

    let mut bvh = Vec::with_capacity(scene.bvh_nodes.len().max(1));
    for node in &scene.bvh_nodes {
        let cnt_or_neg_right = node.bmax_cnt[3].to_bits() as i32;
        let encoded_right = if cnt_or_neg_right > 0 {
            (cnt_or_neg_right as u32) | 0x8000_0000
        } else {
            (-cnt_or_neg_right) as u32
        };
        bvh.push(GpuRayBvhNode {
            min_left: node.bmin_left,
            max_right: [
                node.bmax_cnt[0],
                node.bmax_cnt[1],
                node.bmax_cnt[2],
                f32::from_bits(encoded_right),
            ],
        });
    }
    if bvh.is_empty() {
        bvh.push(GpuRayBvhNode::default());
    }

    let mut mats: Vec<GpuRayMaterial> = scene
        .materials
        .iter()
        .map(|m| {
            let w = m.base_color_weight[3];
            let lum = m.emission_color_lum[3];
            GpuRayMaterial {
                base_color: [
                    m.base_color_weight[0] * w,
                    m.base_color_weight[1] * w,
                    m.base_color_weight[2] * w,
                    m.geometry_meta[0],
                ],
                emissive_metallic: [
                    m.emission_color_lum[0] * lum,
                    m.emission_color_lum[1] * lum,
                    m.emission_color_lum[2] * lum,
                    m.base_spec_params[0],
                ],
                pbr_params: [
                    m.base_spec_params[3].clamp(0.04, 1.0),
                    m.specular_ior_trans[0].max(1.0),
                    m.coat_color_weight[3],
                    m.coat_params[0].clamp(0.04, 1.0),
                ],
            }
        })
        .collect();
    if mats.is_empty() {
        mats.push(GpuRayMaterial {
            base_color: [0.8, 0.8, 0.8, 1.0],
            emissive_metallic: [0.0; 4],
            pbr_params: [0.5, 1.5, 0.0, 0.0],
        });
    }

    let mut lights: Vec<GpuLight> = scene
        .lights
        .iter()
        .map(|l| {
            let is_point = l.kind[0] == 2;
            GpuLight {
                color_intensity: [l.color[0], l.color[1], l.color[2], 1.0],
                position_kind: [
                    l.position[0],
                    l.position[1],
                    l.position[2],
                    if is_point { 1.0 } else { 0.0 },
                ],
                direction_cutoff: l.direction,
                attenuation: l.params,
            }
        })
        .collect();
    if lights.is_empty() {
        lights.push(GpuLight::default());
    }
    (tris, bvh, mats, lights)
}

fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn buf_entry(binding: u32, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
