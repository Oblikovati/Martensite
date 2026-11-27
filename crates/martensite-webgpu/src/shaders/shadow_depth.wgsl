// Depth32Float shadow-map depth-only pass (#634).

struct ShadowDrawUniforms {
    light_mvp: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> shadow_u: ShadowDrawUniforms;

struct VsIn {
    @location(0) position: vec4<f32>,
};

@vertex
fn vs_main(input: VsIn) -> @builtin(position) vec4<f32> {
    return shadow_u.light_mvp * input.position;
}
