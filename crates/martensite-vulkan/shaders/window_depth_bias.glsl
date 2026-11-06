#ifndef MARTENSITE_WINDOW_DEPTH_BIAS_GLSL
#define MARTENSITE_WINDOW_DEPTH_BIAS_GLSL
// One actual ordered push is consumed by both validation and fragment evaluation.
layout(push_constant) uniform WindowDepthBias { layout(offset=128) vec4 operands; } windowDepthBias;
vec2 windowDepthEffectiveBias() {
    return windowDepthBias.operands.z!=0.0?windowDepthBias.operands.xy:vec2(0.0);
}
#endif
