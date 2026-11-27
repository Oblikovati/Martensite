// Unified 256-byte std430 OpenPBR Surface v1.1 GPU material layout shared by rasterization
// and compute path tracing (matches `crates/martensite-vulkan/src/openpbr_material.rs`).
#ifndef OPENPBR_TYPES_GLSL
#define OPENPBR_TYPES_GLSL

const uint OPENPBR_LOBE_FUZZ          = 1u << 0;
const uint OPENPBR_LOBE_COAT          = 1u << 1;
const uint OPENPBR_LOBE_METAL         = 1u << 2;
const uint OPENPBR_LOBE_TRANSMISSION  = 1u << 3;
const uint OPENPBR_LOBE_THIN_FILM     = 1u << 4;
const uint OPENPBR_LOBE_SSS           = 1u << 5;
const uint OPENPBR_LOBE_EMISSION      = 1u << 6;
const uint OPENPBR_LOBE_THIN_WALLED   = 1u << 7;
const uint OPENPBR_LOBE_ANISO         = 1u << 8;
const uint OPENPBR_LOBE_DIFFUSE_ROUGH = 1u << 9;
const uint OPENPBR_LOBE_VOLUME        = 1u << 10;
const uint OPENPBR_LOBE_DISPERSION    = 1u << 11;

struct OpenPbrGpuMaterial {
    vec4 base_color_weight;       // rgb=base_color, a=base_weight
    vec4 base_spec_params;        // x=metalness, y=diffuse_roughness, z=specular_weight, w=specular_roughness
    vec4 specular_color_aniso;    // rgb=specular_color, a=specular_roughness_anisotropy
    vec4 specular_ior_trans;      // x=specular_ior, y=transmission_weight, z=transmission_depth, w=transmission_scatter_anisotropy
    vec4 transmission_color_disp; // rgb=transmission_color, a=transmission_dispersion_scale
    vec4 transmission_scatter_abbe; // rgb=transmission_scatter, a=transmission_dispersion_abbe_number
    vec4 subsurface_color_weight; // rgb=subsurface_color, a=subsurface_weight
    vec4 subsurface_radius_scale; // rgb=subsurface_radius_scale, a=subsurface_radius
    vec4 coat_color_weight;       // rgb=coat_color, a=coat_weight
    vec4 coat_params;             // x=coat_roughness, y=coat_roughness_anisotropy, z=coat_ior, w=coat_darkening
    vec4 fuzz_color_weight;       // rgb=fuzz_color, a=fuzz_weight
    vec4 fuzz_thin_film;          // x=fuzz_roughness, y=thin_film_weight, z=thin_film_thickness_um, w=thin_film_ior
    vec4 emission_color_lum;      // rgb=emission_color, a=emission_luminance
    vec4 geometry_meta;           // x=opacity, y=sss_aniso, z=bitcast(lobe_flags), w=bitcast(tex_mask)
    vec4 uv_transform;            // xy=scale, zw=offset
    vec4 tex_indices;             // 4x uint32 packing 8x int16 bindless texture indices (-1 = none)
};

struct OpenPbrParams {
    vec3 base_color;
    float base_weight;
    float metalness;
    float diffuse_roughness;

    float specular_weight;
    vec3 specular_color;
    float specular_roughness;
    float specular_anisotropy;
    float specular_ior;

    float transmission_weight;
    vec3 transmission_color;
    float transmission_depth;
    vec3 transmission_scatter;
    float transmission_scatter_aniso;
    float dispersion_scale;
    float dispersion_abbe;

    float subsurface_weight;
    vec3 subsurface_color;
    vec3 subsurface_radius;
    float subsurface_aniso;

    float coat_weight;
    vec3 coat_color;
    float coat_roughness;
    float coat_anisotropy;
    float coat_ior;
    float coat_darkening;

    float fuzz_weight;
    vec3 fuzz_color;
    float fuzz_roughness;

    float thin_film_weight;
    float thin_film_thickness_nm;
    float thin_film_ior;

    vec3 emission_radiance;
    float opacity;
    bool thin_walled;
    uint lobe_flags;
};

int openpbrUnpackTexIndex(OpenPbrGpuMaterial g, uint slot) {
    uint word = floatBitsToUint(g.tex_indices[slot >> 1u]);
    uint shift = (slot & 1u) * 16u;
    int raw = int((word >> shift) & 0xFFFFu);
    return (raw >= 32768) ? (raw - 65536) : raw;
}

OpenPbrParams openpbrUnpack(OpenPbrGpuMaterial g) {
    OpenPbrParams p;
    p.base_weight = clamp(g.base_color_weight.w, 0.0, 1.0);
    p.base_color = clamp(g.base_color_weight.rgb, 0.0, 1.0) * p.base_weight;
    p.metalness = clamp(g.base_spec_params.x, 0.0, 1.0);
    p.diffuse_roughness = clamp(g.base_spec_params.y, 0.0, 1.0);

    p.specular_weight = max(g.base_spec_params.z, 0.0);
    p.specular_roughness = clamp(g.base_spec_params.w, 0.0, 1.0);
    p.specular_color = clamp(g.specular_color_aniso.rgb, 0.0, 1.0);
    p.specular_anisotropy = clamp(g.specular_color_aniso.w, 0.0, 1.0);
    p.specular_ior = max(g.specular_ior_trans.x, 1e-5);

    p.transmission_weight = clamp(g.specular_ior_trans.y, 0.0, 1.0);
    p.transmission_depth = max(g.specular_ior_trans.z, 0.0);
    p.transmission_scatter_aniso = clamp(g.specular_ior_trans.w, -1.0, 1.0);
    p.transmission_color = clamp(g.transmission_color_disp.rgb, vec3(1e-5), vec3(1.0));
    p.dispersion_scale = clamp(g.transmission_color_disp.w, 0.0, 1.0);
    p.transmission_scatter = max(g.transmission_scatter_abbe.rgb, vec3(0.0));
    p.dispersion_abbe = max(g.transmission_scatter_abbe.w, 1.0);

    p.subsurface_weight = clamp(g.subsurface_color_weight.w, 0.0, 1.0);
    p.subsurface_color = clamp(g.subsurface_color_weight.rgb, 0.0, 1.0);
    p.subsurface_radius = max(g.subsurface_radius_scale.w, 0.0) * clamp(g.subsurface_radius_scale.rgb, 0.0, 1.0);
    p.subsurface_aniso = clamp(g.geometry_meta.y, -1.0, 1.0);

    p.coat_weight = clamp(g.coat_color_weight.w, 0.0, 1.0);
    p.coat_color = clamp(g.coat_color_weight.rgb, 0.0, 1.0);
    p.coat_roughness = clamp(g.coat_params.x, 0.0, 1.0);
    p.coat_anisotropy = clamp(g.coat_params.y, 0.0, 1.0);
    p.coat_ior = max(g.coat_params.z, 1e-5);
    p.coat_darkening = clamp(g.coat_params.w, 0.0, 1.0);

    p.fuzz_weight = clamp(g.fuzz_color_weight.w, 0.0, 1.0);
    p.fuzz_color = clamp(g.fuzz_color_weight.rgb, 0.0, 1.0);
    p.fuzz_roughness = clamp(g.fuzz_thin_film.x, 0.0, 1.0);

    p.thin_film_weight = clamp(g.fuzz_thin_film.y, 0.0, 1.0);
    p.thin_film_thickness_nm = max(g.fuzz_thin_film.z, 0.0) * 1000.0;
    p.thin_film_ior = max(g.fuzz_thin_film.w, 1e-5);

    p.emission_radiance = max(g.emission_color_lum.rgb, vec3(0.0)) * max(g.emission_color_lum.w, 0.0);
    p.opacity = clamp(g.geometry_meta.x, 0.0, 1.0);
    p.lobe_flags = floatBitsToUint(g.geometry_meta.z);
    p.thin_walled = (p.lobe_flags & OPENPBR_LOBE_THIN_WALLED) != 0u;
    return p;
}

// Orthonormal tangent basis (Duff et al. 2017) when no explicit vertex tangent is supplied.
void openpbrBuildOrthonormalBasis(vec3 N, out vec3 T, out vec3 B) {
    float s = (N.z >= 0.0) ? 1.0 : -1.0;
    float a = -1.0 / (s + N.z);
    float b = N.x * N.y * a;
    T = vec3(1.0 + s * N.x * N.x * a, s * b, -s * N.x);
    B = vec3(b, s + N.y * N.y * a, -N.y);
}

#endif
