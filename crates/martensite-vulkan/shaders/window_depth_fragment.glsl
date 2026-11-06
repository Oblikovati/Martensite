// V3 owns depth directly: no second raster-plane reconstruction of rounded corrected vertices.
#include "window_depth_plane.glsl"
layout(location=7) flat in vec3 depthPlaneOrigin;
layout(location=8) flat in vec2 depthPlaneGradient;
layout(location=9) flat in uint depthPlaneEmpty;
#include "window_depth_bias.glsl"
void writeReferenceWindowDepth() {
    if (depthPlaneEmpty!=0u) { gl_FragDepth=gl_FragCoord.z; return; }
    WindowDepthPlane plane=WindowDepthPlane(depthPlaneOrigin,depthPlaneGradient);
    vec2 bias=windowDepthEffectiveBias();
    // V3 explicitly saturates its deterministic depth result to the admitted 0..1 range.
    gl_FragDepth=clamp(windowDepthBiasedSample(plane,gl_FragCoord.xy,bias.y,bias.x,windowDepthBias.operands.w),0.0,1.0);
}
