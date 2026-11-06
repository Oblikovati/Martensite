#ifndef MARTENSITE_CLIPPED_DEPTH_FAN_GLSL
#define MARTENSITE_CLIPPED_DEPTH_FAN_GLSL
#include "clipped_depth_polygon.glsl"
#include "window_depth_plane.glsl"
#include "clipped_depth_transport.glsl"
bool clippedDepthInitialPolygon(vec4 clip[3],out ClippedDepthVertex polygon[17]) {
    for(uint index=0u;index<3u;++index) {
        if(!clippedDepthFinite(clip[index])||clip[index].w<=0.0) return false;
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
        polygon[index].clip=clip[index];
        polygon[index].weights=vec3(index==0u?1:0,index==1u?1:0,index==2u?1:0);
        for(uint plane=0u;plane<8u;++plane) polygon[index].incidences[plane]=1.0;
#else
        polygon[index]=ClippedDepthVertex(clip[index],vec3(index==0u?1:0,index==1u?1:0,index==2u?1:0));
#endif
    }
    return true;
}
bool clippedDepthApplyPlanes(inout ClippedDepthVertex polygon[17],inout uint count,
                             vec4 planes[14],uint planeCount) {
    if(planeCount>14u||count>17u) return false;
    ClippedDepthVertex next[17];uint nextCount;
    for(uint plane=0u;plane<planeCount;++plane) {
        if(!clippedDepthHalfspace(polygon,count,planes[plane],next,nextCount)) return false;
        count=nextCount;
        for(uint vertex=0u;vertex<count;++vertex) polygon[vertex]=next[vertex];
        if(count==0u) break;
    }
    return true;
}
// This raw GL clip precedes kRemapClipDepth; viewport/sample positions use native framebuffer coordinates.
bool clippedDepthWindowFromRawGl(vec4 clip,vec4 viewport,out vec3 window) {
    if(!clippedDepthFinite(clip)||clip.w<=0.0||!clippedDepthFinite(viewport)) return false;
    precise float reciprocal=1.0/clip.w;
    if(isinf(reciprocal)) return false;
    precise vec3 normalized=clip.xyz*reciprocal;
    precise vec2 scale=viewport.zw*0.5;
    precise vec2 translation=viewport.xy+scale;
    precise vec2 position=normalized.xy*scale+translation;
    precise float depth=normalized.z*0.5+0.5;
    window=vec3(position,depth);
    return windowDepthFinite(window);
}
bool clippedDepthGridPoint(vec3 window,uint gridBits,out i64vec2 point);
bool clippedDepthWindowPolygon(ClippedDepthVertex polygon[17],uint count,vec4 viewport,
                               uint gridBits,out vec3 window[17]) {
    if(count>17u||(gridBits!=8u&&gridBits!=10u&&gridBits!=12u)) return false;
    for(uint index=0u;index<count;++index) {
        if(!clippedDepthWindowFromRawGl(polygon[index].clip,viewport,window[index])) return false;
        i64vec2 point;
        if(!clippedDepthGridPoint(window[index],gridBits,point)) return false;
    }
    return true;
}
bool clippedDepthGridPoint(vec3 window,uint gridBits,out i64vec2 point) {
    if(gridBits!=8u&&gridBits!=10u&&gridBits!=12u) return false;
    if(!windowDepthFinite(window)) return false;
    precise vec2 scaled=window.xy*float(1u<<gridBits);
    if(any(greaterThan(abs(scaled),vec2(16777215.0)))) return false;
    point=i64vec2(roundEven(scaled));
    return true;
}
bool clippedDepthGridArea(vec3 window[3],uint gridBits,out int64_t area) {
    i64vec2 a,b,c;
    if(!clippedDepthGridPoint(window[0],gridBits,a)) return false;
    if(!clippedDepthGridPoint(window[1],gridBits,b)) return false;
    if(!clippedDepthGridPoint(window[2],gridBits,c)) return false;
    area=(b.x-a.x)*(c.y-a.y)-(c.x-a.x)*(b.y-a.y);
    return true;
}
bool clippedDepthStoreWindowPlane(inout ClippedDepthFlatRecord record,uint fan,vec3 triangle[3],uint gridBits,
                                  inout uint emptyMask) {
    if(fan>=15u) return false;
    int64_t area;
    if(!clippedDepthGridArea(triangle,gridBits,area)) return false;
    if(area==int64_t(0)) {
        emptyMask|=1u<<fan;
        return clippedDepthTransportStorePlane(record,fan,WindowDepthPlane(vec3(0),vec2(0)),0.0);
    }
    if(area>int64_t(0)) {
        vec3 first=triangle[0];triangle[0]=triangle[1];triangle[1]=first;
    }
    WindowDepthPlane plane;float slope;
    if(!windowDepthFramebufferPrepare(triangle,plane,slope)) return false;
    return clippedDepthTransportStorePlane(record,fan,plane,slope);
}
bool clippedDepthStoreFanPlanes(inout ClippedDepthFlatRecord record,vec3 window[17],uint count,uint gridBits,bool changed,
                                out uint emptyMask) {
    emptyMask=0u;
    if(count>17u||(gridBits!=8u&&gridBits!=10u&&gridBits!=12u)) return false;
    for(uint index=0u;index+2u<count;++index) {
        vec3 triangle[3]=vec3[3](window[index+1u],window[index+2u],window[0]);
        if(!changed&&count==3u) triangle=vec3[3](window[0],window[1],window[2]);
        if(!clippedDepthStoreWindowPlane(record,index,triangle,gridBits,emptyMask)) return false;
    }
    return true;
}
#endif
