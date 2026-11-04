#version 450
#extension GL_GOOGLE_include_directive : require
#ifndef MARTENSITE_TEXT_FLAT_CORNERS_V4
#extension GL_EXT_fragment_shader_barycentric : require
#endif
#define MARTENSITE_FRAGMENT
#include "clipping.glsl"
#include "native_fragment_alpha.glsl"
layout(location=0) in vec4 vertexColor;
layout(location=1) in vec2 textureUV;
#ifdef MARTENSITE_TEXT_FLAT_CORNERS_V4
layout(location=4) flat in vec4 textGridClip[3];
layout(location=7) flat in vec2 textGridUv[3];
#else
layout(location=4) pervertexEXT in vec4 textGridClip[];
layout(location=5) pervertexEXT in vec2 textGridUv[];
#endif
layout(location=0) out vec4 outColor;
layout(set=0,binding=0) uniform sampler2D glyphs;
layout(push_constant) uniform Transform {
    mat4 clip;
    uint width;
    uint height;
    float cutoff;
    layout(offset=80) vec4 rectangle;
} transform;

bool textGridFinite(vec2 value) {
    return !any(isnan(value)) && !any(isinf(value));
}
bool textGridValidWorld(vec4 point) {
    if (any(isnan(point)) || any(isinf(point)) || point.w == 0.0) return false;
    float reciprocalW = 1.0 / point.w;
    if (isnan(reciprocalW) || isinf(reciprocalW)
        || abs(reciprocalW) < 1.17549435e-38) return false;
    vec3 divided = point.xyz / point.w;
    return !any(isnan(divided)) && !any(isinf(divided));
}
bool textGridWindowPoint(vec4 clip, out vec2 point) {
    // Admission screens the bounded host transform; recheck actual VS values before any division.
    if (any(isnan(clip)) || any(isinf(clip)) || clip.w <= 0.0) return false;
    float reciprocalW = 1.0 / clip.w;
    if (isnan(reciprocalW) || isinf(reciprocalW) || abs(reciprocalW) < 1.17549435e-38) return false;
    if (!textGridFinite(transform.rectangle.xy) || !textGridFinite(transform.rectangle.zw)
        || transform.rectangle.z <= 0.0 || transform.rectangle.w == 0.0) return false;
    vec2 ndc = clip.xy / clip.w;
    if (!textGridFinite(ndc)) return false;
    point = transform.rectangle.xy + (ndc + vec2(1.0)) * transform.rectangle.zw * 0.5;
    return textGridFinite(point) && all(lessThanEqual(abs(point), vec2(4096.0)));
}
float textGridCross(vec2 a, vec2 b) {
    return a.x * b.y - a.y * b.x;
}
bool textGridSampleUv(out vec2 sampleUv) {
    vec2 aWindow, bWindow, cWindow;
    if (!textGridWindowPoint(textGridClip[0], aWindow)
        || !textGridWindowPoint(textGridClip[1], bWindow)
        || !textGridWindowPoint(textGridClip[2], cWindow)) return false;
    vec2 a = roundEven(aWindow * 4096.0) / 4096.0;
    vec2 b = roundEven(bWindow * 4096.0) / 4096.0;
    vec2 c = roundEven(cWindow * 4096.0) / 4096.0;
    vec2 p = gl_FragCoord.xy;
    if (!textGridFinite(p) || any(greaterThan(abs(p), vec2(4096.0)))) return false;
    float area = textGridCross(b - a, c - a);
    // Integer-lattice admission separates the determinant from f32 roundoff; never divide if
    // actual post-VS arithmetic leaves that declared domain.
    if (isnan(area) || isinf(area) || area == 0.0) return false;
    vec3 barycentric = vec3(textGridCross(b - p, c - p),
                            textGridCross(c - p, a - p),
                            textGridCross(a - p, b - p)) / area;
    if (any(isnan(barycentric)) || any(isinf(barycentric))) return false;
    vec3 weights = barycentric / vec3(textGridClip[0].w, textGridClip[1].w, textGridClip[2].w);
    if (any(isnan(weights)) || any(isinf(weights))) return false;
    float sum = (weights.x + weights.y) + weights.z;
    if (isnan(sum) || isinf(sum) || sum <= 0.0) return false;
    sampleUv = ((weights.x * textGridUv[0] + weights.y * textGridUv[1])
        + weights.z * textGridUv[2]) / sum;
    return textGridFinite(sampleUv);
}
void main() {
#ifdef MARTENSITE_CLIPPING
    if (!textGridValidWorld(clippingWorldPosition)) discard;
    discardClippedFragment();
#endif
    vec2 sampleUv;
    if (!textGridSampleUv(sampleUv)) discard;
    outColor = vertexColor * texture(glyphs, sampleUv);
    if (outColor.a <= transform.cutoff) discard;
    nativePostAlpha(outColor.a);
}
