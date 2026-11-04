#ifndef MARTENSITE_WINDOW_DEPTH_PLANE_GLSL
#define MARTENSITE_WINDOW_DEPTH_PLANE_GLSL
// Window-depth arithmetic is shared by GS compensation and logical FS coverage.
// Callers prove finite positive-W inputs, clipping, grid and arithmetic closure.
#include "viewport_grid.glsl"
#include "viewport_grid_integer.glsl"
struct WindowDepthPlane {
    vec3 origin;
    vec2 gradient;
};
vec3 windowDepthPosition(vec4 clip, vec4 viewport, bool negativeOneToOne) {
    precise vec3 ndc = clip.xyz / clip.w;
    precise vec2 normalized = ndc.xy * 0.5 + 0.5;
    precise vec2 window = normalized * viewport.zw + viewport.xy;
    precise float depth = negativeOneToOne ? ndc.z * 0.5 + 0.5 : ndc.z;
    return vec3(window, depth);
}
vec2 windowDepthGradient(vec3 original[3]) {
    precise vec3 first = original[1] - original[0];
    precise vec3 second = original[2] - original[0];
    precise float determinant = first.x * second.y - second.x * first.y;
    precise vec2 numerator = vec2(first.z * second.y - second.z * first.y,
                                 first.x * second.z - second.x * first.z);
    precise vec2 gradient = numerator / determinant;
    return gradient;
}
WindowDepthPlane windowDepthReferencePlane(vec4 clip[3], vec4 viewport,
                                         bool negativeOneToOne, uint referenceBits) {
    vec3 reference[3];
    for (uint index = 0u; index < 3u; ++index) {
        vec3 window = windowDepthPosition(clip[index], viewport, negativeOneToOne);
        reference[index] = vec3(viewportGridNearestEven(window.xy, referenceBits), window.z);
    }
    return WindowDepthPlane(reference[0], windowDepthGradient(reference));
}
float windowDepthAtSample(WindowDepthPlane plane, vec2 samplePosition) {
    precise vec2 delta = samplePosition - plane.origin.xy;
    precise float depth = (plane.gradient.x * delta.x + plane.gradient.y * delta.y) + plane.origin.z;
    return depth;
}
float windowDepthMaximumSlope(WindowDepthPlane plane) {
    return max(abs(plane.gradient.x), abs(plane.gradient.y));
}
float windowDepthBiasedSample(WindowDepthPlane plane, vec2 samplePosition,
                             float factor, float units, float unitScale) {
    precise float depth = windowDepthAtSample(plane, samplePosition);
    precise float bias = windowDepthMaximumSlope(plane) * factor + units * unitScale;
    precise float biasedDepth = depth + bias;
    return biasedDepth;
}
bool windowDepthRasterEmpty(vec4 clip[3], vec4 viewport, bool negativeOneToOne, uint rasterBits) {
    vec2 window[3];
    for (uint index = 0u; index < 3u; ++index) {
        window[index] = windowDepthPosition(clip[index], viewport, negativeOneToOne).xy;
    }
    return viewportGridTriangleEmpty(window, rasterBits);
}
void windowDepthCompensate(vec4 clip[3], vec4 viewport, bool negativeOneToOne,
                          uint referenceBits, uint rasterBits, out vec4 corrected[3]) {
    // Ordinary rasterization completes an exact zero-area physical triangle without fragments.
    // Conservative/stencil/side-effect routes are not admitted by this operation.
    if (windowDepthRasterEmpty(clip, viewport, negativeOneToOne, rasterBits)) {
        for (uint index = 0u; index < 3u; ++index) corrected[index] = clip[index];
        return;
    }
    WindowDepthPlane plane = windowDepthReferencePlane(clip, viewport, negativeOneToOne, referenceBits);
    for (uint index = 0u; index < 3u; ++index) {
        vec2 window = windowDepthPosition(clip[index], viewport, negativeOneToOne).xy;
        vec2 raster = viewportGridNearestEven(window, rasterBits);
        precise float depth = windowDepthAtSample(plane, raster);
        precise float normalized = negativeOneToOne ? depth * 2.0 - 1.0 : depth;
        corrected[index] = clip[index];
        corrected[index].z = normalized * clip[index].w;
    }
}
// Explicit clipped-fan successor: framebuffer-origin, unsnapped, separate f32 operations.
// Older vertex-origin/Q12 functions above keep their original arithmetic for replay.
bool windowDepthFinite(vec3 value) {
    bool finiteValue=!any(isnan(value))&&!any(isinf(value));
    return finiteValue;
}
bool windowDepthFramebufferEdges(vec3 window[3], out vec3 first,
                                 out vec3 second, out float inverse) {
    for (uint index=0u;index<3u;++index) if (!windowDepthFinite(window[index])) return false;
    first=window[0]-window[1];second=window[2]-window[0];
    precise float determinant=first.x*second.y-first.y*second.x;
    if (isnan(determinant)||isinf(determinant)||determinant==0.0) return false;
    inverse=1.0/determinant;
    return windowDepthFinite(first)&&windowDepthFinite(second)&&!isinf(inverse);
}
vec2 windowDepthFramebufferGradient(vec3 first, vec3 second, float inverse) {
    precise vec2 dividedY=vec2(second.y,first.y)*inverse;
    precise vec2 dividedX=vec2(second.x,first.x)*inverse;
    precise float dx=first.z*dividedY.x-second.z*dividedY.y;
    precise float dy=second.z*dividedX.y-first.z*dividedX.x;
    return vec2(dx,dy);
}
bool windowDepthFramebufferSlope(vec3 first,vec3 second,float inverse,out float slope) {
    precise vec2 numerator=vec2(second.z*first.y-second.y*first.z,
                              second.x*first.z-second.z*first.x);
    precise vec2 gradient=numerator*inverse;
    if(!windowDepthFinite(vec3(numerator,0.0))||!windowDepthFinite(vec3(gradient,0.0))) return false;
    slope=max(abs(gradient.x),abs(gradient.y));
    return true;
}
bool windowDepthFramebufferPrepare(vec3 window[3],out WindowDepthPlane plane,
                                   out float slope) {
    vec3 first,second;float inverse;
    if(!windowDepthFramebufferEdges(window,first,second,inverse)) return false;
    vec2 gradient=windowDepthFramebufferGradient(first,second,inverse);
    precise float projected=gradient.x*window[0].x+gradient.y*window[0].y;
    precise float origin=window[0].z-projected;
    plane=WindowDepthPlane(vec3(0.0,0.0,origin),gradient);
    if(!windowDepthFramebufferSlope(first,second,inverse,slope)) return false;
    return windowDepthFinite(vec3(gradient,origin))&&!isnan(slope)&&!isinf(slope);
}
float windowDepthFramebufferSample(WindowDepthPlane plane,vec2 samplePosition) {
    precise float first=plane.gradient.x*samplePosition.x+plane.origin.z;
    precise float depth=plane.gradient.y*samplePosition.y+first;
    return depth;
}
#endif
