#include "window_depth_float64_plane.glsl"
#include "window_depth_bias.glsl"
layout(location=7) flat in dvec3 depthPlaneOrigin64;
layout(location=9) flat in dvec2 depthPlaneGradient64;
layout(location=10) flat in uint depthPlaneEmpty;
void writeReferenceWindowDepth() {
    if(depthPlaneEmpty!=0u) { gl_FragDepth=gl_FragCoord.z; return; }
    WindowDepthPlane64 plane=WindowDepthPlane64(depthPlaneOrigin64,depthPlaneGradient64);
    double depth=windowDepthBiased64(plane,dvec2(gl_FragCoord.xy),windowDepthEffectiveBias(),windowDepthBias.operands.w);
    // Saturation and one final binary32 conversion are explicit V5 semantics.
    gl_FragDepth=float(clamp(depth,0.0lf,1.0lf));
}
