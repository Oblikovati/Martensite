// The shared fresh status allocation/count/publication protocol also owns V5 completion.
#include "window_depth_float64_plane.glsl"
bool windowDepthBegin64() {
    atomicAdd(windowDepthStatus.invocations,1u);
    if(windowDepthStatus.schema!=WINDOW_DEPTH_SCHEMA) {atomicOr(windowDepthStatus.fault,2u);return false;}
    if(uint(gl_PrimitiveIDIn)>=windowDepthStatus.triangleCount) {atomicOr(windowDepthStatus.fault,1u);return false;}
    atomicAdd(windowDepthStatus.words[uint(gl_PrimitiveIDIn)*2u+1u],1u);
    return true;
}
uint windowDepthPositionGuard64(vec4 p,vec4 raster,vec4 viewport,uint bits,out dvec3 window) {
    if(!windowDepthFinite(p)||p.w<=0.0||any(greaterThan(abs(p.xyz),vec3(p.w)))) return 8u;
    if(!windowDepthFinite(raster)) return 8u;
    window=windowDepthPosition64(p,viewport);
    if(!windowDepthFinite64(dvec4(window,1.0lf))) return 16u;
    double scale=double(1u<<bits);
    if(any(greaterThan(abs(window.xy)*scale,dvec2(16777215.0lf)))) return 32u;
    window.xy=roundEven(window.xy*scale)/scale;
    return 0u;
}
bool windowDepthBiasGuard64(WindowDepthPlane64 plane,vec4 viewport) {
    vec2 effective=windowDepthEffectiveBias();
    if((windowDepthBias.operands.z!=0.0&&windowDepthBias.operands.z!=1.0)
       ||windowDepthBias.operands.w!=1.0/8388608.0||!windowDepthFinite(vec4(effective,0.0,0.0))) return false;
    for(uint corner=0u;corner<4u;++corner) {
        dvec2 samplePosition=dvec2(viewport.xy)+dvec2((corner&1u)!=0u?viewport.z:0.0,(corner&2u)!=0u?viewport.w:0.0);
        double value=windowDepthBiased64(plane,samplePosition,effective,windowDepthBias.operands.w);
        if(!windowDepthFinite64(dvec4(value))) return false;
    }
    return true;
}
bool windowDepthPrepare64(vec4 rasterClip[3],vec4 rawGL[3],vec4 viewport,bool negative,
                          uint referenceBits,uint rasterBits,out WindowDepthPlane64 plane,out bool empty) {
    if(!windowDepthBegin64()) return false;
    dvec3 window[3];
    for(uint i=0u;i<3u;++i) {
        uint failure=windowDepthPositionGuard64(rawGL[i],rasterClip[i],viewport,referenceBits,window[i]);
        if(failure!=0u) return windowDepthComplete(failure);
    }
    empty=windowDepthRasterEmpty(rasterClip,viewport,negative,rasterBits);
    plane=WindowDepthPlane64(dvec3(0.0lf),dvec2(0.0lf));
    if(empty) return windowDepthComplete(2u);
    plane=windowDepthReference64(window);
    if(!windowDepthFinite64(dvec4(plane.origin,0.0lf))||!windowDepthFinite64(dvec4(plane.gradient,0.0lf,0.0lf))) return windowDepthComplete(4u);
    return windowDepthComplete(windowDepthBiasGuard64(plane,viewport)?0u:64u);
}
