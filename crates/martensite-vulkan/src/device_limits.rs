//! Device capacities behind `Graphic3dGraphicDriver::inquire_limit`, read from the selected
//! Vulkan physical device. Each OpenGL context limit maps to the Vulkan limit or feature
//! with the same meaning; the recorded driver_limits capture holds the expected values.

/// Capacities of the selected device as the renderer uses them; `driver.device_limits()`.
///
/// Example: `driver.device_limits().map(|limits| limits.max_clip_planes())`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderDeviceLimits {
    pub(crate) max_texture_size: i32,
    pub(crate) max_dump_size: [i32; 2],
    pub(crate) max_msaa_samples: i32,
    pub(crate) max_combined_texture_units: i32,
    pub(crate) max_clip_planes: i32,
    pub(crate) max_color_attachments: i32,
    pub(crate) max_draw_buffers: i32,
    pub(crate) max_degree_of_anisotropy: i32,
    pub(crate) has_srgb: bool,
    pub(crate) has_texture_multisampling: bool,
    pub(crate) has_mesh_edges: bool,
    pub(crate) has_blended_oit: bool,
    pub(crate) has_blended_oit_msaa: bool,
    pub(crate) sub_texel_precision_bits: u32,
    pub(crate) has_ray_tracing_textures: bool,
    pub(crate) has_ray_tracing_adaptive_sampling: bool,
    pub(crate) has_ray_tracing_adaptive_sampling_atomic: bool,
}

impl RenderDeviceLimits {
    /// Largest 2D image side (`maxImageDimension2D`); `limits.max_texture_size() >= 4096`.
    pub fn max_texture_size(&self) -> i32 {
        self.max_texture_size
    }
    /// Largest offscreen capture width: framebuffer and image bounds; `limits.max_dump_size_x()`.
    pub fn max_dump_size_x(&self) -> i32 {
        self.max_dump_size[0]
    }
    /// Largest offscreen capture height: framebuffer and image bounds; `limits.max_dump_size_y()`.
    pub fn max_dump_size_y(&self) -> i32 {
        self.max_dump_size[1]
    }
    /// Highest scene sample count the renderer selects, 0 without multisampling; `limits.max_msaa_samples()`.
    pub fn max_msaa_samples(&self) -> i32 {
        self.max_msaa_samples
    }
    /// Combined image samplers one pipeline may bind across all stages
    /// (`maxDescriptorSetSamplers` and `maxDescriptorSetSampledImages`); `limits.max_combined_texture_units()`.
    pub fn max_combined_texture_units(&self) -> i32 {
        self.max_combined_texture_units
    }
    /// Hardware clip distances (`maxClipDistances`), the GL_MAX_CLIP_PLANES counterpart; `limits.max_clip_planes()`.
    pub fn max_clip_planes(&self) -> i32 {
        self.max_clip_planes
    }
    /// Color attachments of one rendering pass (`maxColorAttachments`); `limits.max_color_attachments()`.
    pub fn max_color_attachments(&self) -> i32 {
        self.max_color_attachments
    }
    /// Fragment outputs written at once (`maxFragmentOutputAttachments`); `limits.max_draw_buffers()`.
    pub fn max_draw_buffers(&self) -> i32 {
        self.max_draw_buffers
    }
    /// Sampler anisotropy cap (`maxSamplerAnisotropy`), 1 without the feature; `limits.max_degree_of_anisotropy()`.
    pub fn max_degree_of_anisotropy(&self) -> i32 {
        self.max_degree_of_anisotropy
    }
    /// sRGB sampled images and color attachments are both supported; `limits.has_srgb()`.
    pub fn has_srgb(&self) -> bool {
        self.has_srgb
    }
    /// Multisampled color images can be sampled; `limits.has_texture_multisampling()`.
    pub fn has_texture_multisampling(&self) -> bool {
        self.has_texture_multisampling
    }
    /// Facet shading uses fragment derivatives, core in every Vulkan device; `limits.has_flat_shading()`.
    pub fn has_flat_shading(&self) -> bool {
        true
    }
    /// Mesh edges need the geometry stage (`geometryShader`); `limits.has_mesh_edges()`.
    pub fn has_mesh_edges(&self) -> bool {
        self.has_mesh_edges
    }
    /// Weighted OIT needs float targets and two color attachments; `limits.has_blended_oit()`.
    pub fn has_blended_oit(&self) -> bool {
        self.has_blended_oit
    }
    /// Weighted OIT under MSAA also needs sample-rate shading; `limits.has_blended_oit_msaa()`.
    pub fn has_blended_oit_msaa(&self) -> bool {
        self.has_blended_oit_msaa
    }
    /// Bits of sub-texel precision of image sampling (`subTexelPrecisionBits`): coordinates are
    /// resolved to `2^-bits` of a texel when a sampler selects texels;
    /// `limits.sub_texel_precision_bits() >= 4`.
    pub fn sub_texel_precision_bits(&self) -> u32 {
        self.sub_texel_precision_bits
    }
    /// Ray tracing and path tracing run on every Vulkan 1.3 device: texel buffers, float
    /// targets and blits (native HasRayTracing: GL 3.1, texture buffers and FBO blit);
    /// `limits.has_ray_tracing()`.
    pub fn has_ray_tracing(&self) -> bool {
        true
    }
    /// Materials of ray-traced scenes may be textured (native HasRayTracingTextures: bindless
    /// textures); `limits.has_ray_tracing_textures()`.
    pub fn has_ray_tracing_textures(&self) -> bool {
        self.has_ray_tracing_textures
    }
    /// Adaptive screen sampling stores into R32F and R32I images from fragment shaders with
    /// 32-bit integer image atomics (`fragmentStoresAndAtomics`; the formats and integer
    /// atomics are required by Vulkan 1.3; native: GL 4.4);
    /// `limits.has_ray_tracing_adaptive_sampling()`.
    pub fn has_ray_tracing_adaptive_sampling(&self) -> bool {
        self.has_ray_tracing_adaptive_sampling
    }
    /// Its single-pass variant adds float image atomics (`shaderImageFloat32AtomicAdd` of
    /// VK_EXT_shader_atomic_float; native GL_NV_shader_atomic_float);
    /// `limits.has_ray_tracing_adaptive_sampling_atomic()`.
    pub fn has_ray_tracing_adaptive_sampling_atomic(&self) -> bool {
        self.has_ray_tracing_adaptive_sampling_atomic
    }
    /// Captures are tiled below the texture size, as native `IsWorkaroundFBO`; `limits.is_workaround_fbo()`.
    pub fn is_workaround_fbo(&self) -> bool {
        self.max_texture_size != self.max_dump_size[0]
    }
}
