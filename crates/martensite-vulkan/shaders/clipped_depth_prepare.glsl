#ifndef MARTENSITE_CLIPPED_DEPTH_PREPARE_GLSL
#define MARTENSITE_CLIPPED_DEPTH_PREPARE_GLSL
#include "clipped_depth_fan.glsl"
#include "clipped_depth_status.glsl"
#include "window_depth_bias.glsl"
bool clippedDepthCanonicalInterior(vec4 clip[3]) {
    for(uint index=0u;index<3u;++index) {
        vec4 p=clip[index];
        if(!clippedDepthFinite(p)||p.w<=0.0||any(greaterThan(abs(p.xyz),vec3(p.w)))) return false;
    }
    return true;
}
bool clippedDepthFiniteBiased(vec4 row,vec2 point) {
    WindowDepthPlane plane=WindowDepthPlane(vec3(0.0,0.0,row.x),row.yz);
    precise float depth=windowDepthFramebufferSample(plane,point);
    vec2 effective=windowDepthEffectiveBias();
    precise float slope=row.w*effective.y;
    precise float unit=effective.x*windowDepthBias.operands.w;
    precise float bias=slope+unit;
    precise float result=depth+bias;
    return clippedDepthFinite(vec4(slope,unit,bias,result));
}
bool clippedDepthValidateBias(ClippedDepthFlatRecord record,uint count,vec4 viewport) {
    if(!clippedDepthFinite(windowDepthBias.operands)||windowDepthBias.operands.w!=1.0/8388608.0) return false;
    if(windowDepthBias.operands.z!=0.0&&windowDepthBias.operands.z!=1.0) return false;
    for(uint fan=0u;fan+2u<count;++fan) {
        for(uint corner=0u;corner<4u;++corner) {
            vec2 point=viewport.xy+vec2((corner&1u)!=0u?viewport.z:0.0,(corner&2u)!=0u?viewport.w:0.0);
            if(!clippedDepthFiniteBiased(record.planeRows[fan],point)) return false;
        }
    }
    return true;
}
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
bool clippedDepthSharedPolygon(vec4 clip[3],uint tail,out ClippedDepthVertex polygon[17],out uint count);
bool clippedDepthViewportPlaneDomain(vec3 window[17],uint count,uint grid,out uint emptyMask);
bool clippedDepthStoreCertifiedPlanes(inout ClippedDepthFlatRecord record,vec3 window[17],uint count,uint grid,bool changed,uint emptyMask);
#define clippedDepthPreparePolygon clippedDepthSharedPolygon
#else
bool clippedDepthPreparePolygon(vec4 clip[3],uint tail,out ClippedDepthVertex polygon[17],out uint count) {
    if(!clippedDepthCanonicalInterior(clip)) return false;
    vec4 planes[14];uint planeCount;
    if(!clippedDepthReferencePlanes(tail,planes,planeCount)) return false;
    if(!clippedDepthInitialPolygon(clip,polygon)) return false;
    count=3u;
    return clippedDepthApplyPlanes(polygon,count,planes,planeCount);
}
#endif
bool clippedDepthPrepareActual(vec4 clip[3],vec4 viewport,uint grid,out ClippedDepthFlatRecord record) {
    uint tail;
    if(!clippedDepthBegin(tail)) return false;
    clippedDepthTransportInitialize(record);
    ClippedDepthVertex polygon[17];uint count;
    if(!clippedDepthPreparePolygon(clip,tail,polygon,count)) return clippedDepthComplete(8u);
    if(count<3u) return clippedDepthComplete(2u);
    vec3 window[17];uint emptyMask;
    if(!clippedDepthWindowPolygon(polygon,count,viewport,grid,window)) return clippedDepthComplete(32u);
    bool changed=count!=3u||any(notEqual(polygon[0].weights,vec3(1,0,0)))||any(notEqual(polygon[1].weights,vec3(0,1,0)))||any(notEqual(polygon[2].weights,vec3(0,0,1)));
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    if(!clippedDepthViewportPlaneDomain(window,count,grid,emptyMask)) return clippedDepthComplete(16u);
    if(!clippedDepthStoreCertifiedPlanes(record,window,count,grid,changed,emptyMask)) return clippedDepthComplete(16u);
#else
    if(!clippedDepthStoreFanPlanes(record,window,count,grid,changed,emptyMask)) return clippedDepthComplete(16u);
#endif
    for(uint index=0u;index<count;++index) if(!clippedDepthTransportStorePoint(record,index,window[index].xy)) return clippedDepthComplete(32u);
    if(!clippedDepthTransportComplete(record,count)||!clippedDepthValidateBias(record,count,viewport)) return clippedDepthComplete(64u);
    record.packedPoints[8].w=float(emptyMask);
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    return clippedDepthComplete(emptyMask==((1u<<(count-2u))-1u)?2u:0u);
#else
    return clippedDepthComplete(0u);
#endif
}
#endif
