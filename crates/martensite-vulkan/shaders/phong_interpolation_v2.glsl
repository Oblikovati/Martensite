// Reuses 919bdaf12's unsnapped projective interpolation, without its observation targets.
// CPU admission screens finite positive-w inputs; it does not certify GPU intermediate rounding.
// No sentinel or hardware fallback substitutes a different interpolation policy.
#ifdef MARTENSITE_NATIVE_GRID12_V4
layout(location=7) flat in vec4 v4PhongClip[3];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=10) flat in vec3 v4PhongNormal0;
layout(location=11) flat in vec3 v4PhongNormal1;
layout(location=12) flat in vec3 v4PhongNormal2;
#define phongNormalAt(index) ((index) == 0 ? v4PhongNormal0 : (index) == 1 ? v4PhongNormal1 : v4PhongNormal2)
#else
layout(location=10) flat in vec3 v4PhongNormal[3];
#define phongNormalAt(index) v4PhongNormal[index]
#endif
layout(location=19) flat in vec4 v4PhongPosition[3];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=22) flat in vec3 v4PhongView0;
layout(location=23) flat in vec3 v4PhongView1;
layout(location=24) flat in vec3 v4PhongView2;
#define phongViewAt(index) ((index) == 0 ? v4PhongView0 : (index) == 1 ? v4PhongView1 : v4PhongView2)
#else
layout(location=22) flat in vec3 v4PhongView[3];
#define phongViewAt(index) v4PhongView[index]
#endif
layout(location=25) flat in vec4 v4PhongColor[3];
#define phongClip v4PhongClip
#define phongPosition v4PhongPosition
#define phongColor v4PhongColor
#else
layout(location=7) pervertexEXT in vec4 phongClip[];
layout(location=8) pervertexEXT in vec3 phongNormal[];
layout(location=9) pervertexEXT in vec4 phongPosition[];
layout(location=10) pervertexEXT in vec3 phongView[];
layout(location=12) pervertexEXT in vec4 phongColor[];
#define phongNormalAt(index) phongNormal[index]
#define phongViewAt(index) phongView[index]
#endif
layout(push_constant) uniform PhongViewport {
    layout(offset=64) vec4 rectangle;
#ifdef MARTENSITE_TRIANGLE_GRID_COVERAGE_V2
    layout(offset=80) uint coverageOrigin;
#ifdef MARTENSITE_NATIVE_GRID12_V4
    layout(offset=84) uint coverageFrontFaceCcw;
    layout(offset=88) uint coverageCullMode;
    layout(offset=92) uint depthNegativeOneToOne;
    layout(offset=96) float depthMinimum;
    layout(offset=100) float depthMaximum;
    layout(offset=104) float depthBiasFactor;
    layout(offset=108) float depthBiasUnits;
    layout(offset=112) float depthBiasUnitScale;
#endif
#endif
} phongViewport;
vec2 phongWindowPoint(vec4 clip) {
    return phongViewport.rectangle.xy + (clip.xy / clip.w + vec2(1.0)) * phongViewport.rectangle.zw * 0.5;
}
float phongArea(vec2 a, vec2 b) {
    return a.x * b.y - a.y * b.x;
}
vec3 phongProjectiveWeights() {
    vec2 a = phongWindowPoint(phongClip[0]);
    vec2 b = phongWindowPoint(phongClip[1]);
    vec2 c = phongWindowPoint(phongClip[2]);
    float area = phongArea(b - a, c - a);
    return vec3(phongArea(b - gl_FragCoord.xy, c - gl_FragCoord.xy),
                phongArea(c - gl_FragCoord.xy, a - gl_FragCoord.xy),
                phongArea(a - gl_FragCoord.xy, b - gl_FragCoord.xy)) / area
                / vec3(phongClip[0].w, phongClip[1].w, phongClip[2].w);
}
vec3 phongInterpolate3(vec3 a, vec3 b, vec3 c, vec3 weights) {
    if (all(equal(a, b)) && all(equal(a, c))) return a;
    return ((weights.x * a + weights.y * b) + weights.z * c) / ((weights.x + weights.y) + weights.z);
}
vec4 phongInterpolate4(vec4 a, vec4 b, vec4 c, vec3 weights) {
    if (all(equal(a, b)) && all(equal(a, c))) return a;
    return ((weights.x * a + weights.y * b) + weights.z * c) / ((weights.x + weights.y) + weights.z);
}
