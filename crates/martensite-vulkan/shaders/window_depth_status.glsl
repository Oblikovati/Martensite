// V3 completion is operation-local. No missing or failed primitive may publish a frame.
#include "window_depth_bias.glsl"
#include "window_depth_status_storage.glsl"
bool windowDepthFinite(vec4 value) {return !any(isnan(value))&&!any(isinf(value));}
bool windowDepthComplete(uint reason) {
    uint primitive=uint(gl_PrimitiveIDIn);
    if (primitive>=windowDepthStatus.triangleCount) {atomicOr(windowDepthStatus.fault,1u);return false;}
    atomicOr(windowDepthStatus.words[primitive*2u],WINDOW_DEPTH_DONE|reason);
    return reason==0u||reason==2u;
}
bool windowDepthFiniteSample(WindowDepthPlane plane,vec2 samplePosition) {
    precise vec2 delta=samplePosition-plane.origin.xy;
    precise vec2 terms=plane.gradient*delta;
    precise float sum=terms.x+terms.y;
    precise float depth=sum+plane.origin.z;
    if(!windowDepthFinite(vec4(delta,terms))||!windowDepthFinite(vec4(sum,depth,0.0,0.0))) return false;
    vec2 effective=windowDepthEffectiveBias();
    precise float slopeTerm=windowDepthMaximumSlope(plane)*effective.y;
    precise float unitTerm=effective.x*windowDepthBias.operands.w;
    precise float bias=slopeTerm+unitTerm;
    precise float biasedDepth=depth+bias;
    return windowDepthFinite(vec4(slopeTerm,unitTerm,bias,biasedDepth));
}
bool windowDepthPrepareActual(vec4 clip[3],vec4 viewport,bool negative,uint referenceBits,uint rasterBits,
                             out WindowDepthPlane plane,out bool empty) {
    atomicAdd(windowDepthStatus.invocations,1u);
    if(windowDepthStatus.schema!=WINDOW_DEPTH_SCHEMA) {atomicOr(windowDepthStatus.fault,2u);return false;}
    if(uint(gl_PrimitiveIDIn)>=windowDepthStatus.triangleCount) {atomicOr(windowDepthStatus.fault,1u);return false;}
    atomicAdd(windowDepthStatus.words[uint(gl_PrimitiveIDIn)*2u+1u],1u);
    vec3 window[3];
    for(uint index=0u;index<3u;++index) {
        vec4 p=clip[index];
        if(!windowDepthFinite(p)||p.w<=0.0||any(greaterThan(abs(p.xy),vec2(p.w)))||p.z>p.w||p.z<(negative?-p.w:0.0)) return windowDepthComplete(8u);
        window[index]=windowDepthPosition(p,viewport,negative);
        if(!windowDepthFinite(vec4(window[index],1.0))||any(greaterThan(abs(window[index].xy)*float(1u<<rasterBits),vec2(16777215.0)))) return windowDepthComplete(32u);
    }
    empty=windowDepthRasterEmpty(clip,viewport,negative,rasterBits);
    plane=WindowDepthPlane(vec3(0.0),vec2(0.0));
    if(empty) return windowDepthComplete(2u);
    vec2 referenceWindow[3];
    for(uint index=0u;index<3u;++index) {
        referenceWindow[index]=window[index].xy;
        if(any(greaterThan(abs(window[index].xy)*float(1u<<referenceBits),vec2(16777215.0)))) return windowDepthComplete(32u);
    }
    if(viewportGridTriangleEmpty(referenceWindow,referenceBits)) return windowDepthComplete(4u);
    plane=windowDepthReferencePlane(clip,viewport,negative,referenceBits);
    if(!windowDepthFinite(vec4(plane.origin,0.0))||!windowDepthFinite(vec4(plane.gradient,0.0,0.0))) return windowDepthComplete(16u);
    vec2 effective=windowDepthEffectiveBias();
    if((windowDepthBias.operands.z!=0.0&&windowDepthBias.operands.z!=1.0)
       ||windowDepthBias.operands.w!=1.0/8388608.0||!windowDepthFinite(vec4(effective,0.0,0.0))) return windowDepthComplete(64u);
    for(uint corner=0u;corner<4u;++corner) {
        vec2 samplePosition=viewport.xy+vec2((corner&1u)!=0u?viewport.z:0.0,(corner&2u)!=0u?viewport.w:0.0);
        if(!windowDepthFiniteSample(plane,samplePosition)) return windowDepthComplete(64u);
    }
    return windowDepthComplete(0u);
}
