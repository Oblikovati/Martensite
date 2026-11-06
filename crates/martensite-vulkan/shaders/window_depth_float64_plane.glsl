#ifndef MARTENSITE_WINDOW_DEPTH_FLOAT64_PLANE_GLSL
#define MARTENSITE_WINDOW_DEPTH_FLOAT64_PLANE_GLSL
// V5 owns ordered binary64 arithmetic on the actual pre-remap binary32 GL clip.
// It does not claim exact-real arithmetic, a float-control rounding mode, or hardware emulation.
struct WindowDepthPlane64 { dvec3 origin; dvec2 gradient; };
bool windowDepthFinite64(dvec4 value) { return !any(isnan(value))&&!any(isinf(value)); }
dvec3 windowDepthPosition64(vec4 rawGL,vec4 viewport) {
    precise dvec3 ndc=dvec3(rawGL.xyz)/double(rawGL.w);
    precise dvec2 normalized=ndc.xy*0.5lf+0.5lf;
    precise dvec2 window=normalized*dvec2(viewport.zw)+dvec2(viewport.xy);
    precise double depth=ndc.z*0.5lf+0.5lf;
    return dvec3(window,depth);
}
WindowDepthPlane64 windowDepthReference64(dvec3 p[3]) {
    precise dvec3 first=p[1]-p[0]; precise dvec3 second=p[2]-p[0];
    precise double determinant=first.x*second.y-second.x*first.y;
    precise dvec2 numerator=dvec2(first.z*second.y-second.z*first.y,
                                 first.x*second.z-second.x*first.z);
    precise dvec2 gradient=numerator/determinant;
    return WindowDepthPlane64(p[0],gradient);
}
double windowDepthSample64(WindowDepthPlane64 plane,dvec2 samplePosition) {
    precise dvec2 delta=samplePosition-plane.origin.xy;
    precise double terms=plane.gradient.x*delta.x+plane.gradient.y*delta.y;
    precise double depth=terms+plane.origin.z;
    return depth;
}
double windowDepthBiased64(WindowDepthPlane64 plane,dvec2 samplePosition,vec2 effective,float unitScale) {
    precise double depth=windowDepthSample64(plane,samplePosition);
    precise double slope=max(abs(plane.gradient.x),abs(plane.gradient.y));
    precise double factor=slope*double(effective.y);
    precise double units=double(effective.x)*double(unitScale);
    precise double bias=factor+units;
    precise double result=depth+bias;
    return result;
}
#endif
