#ifndef MARTENSITE_CLIPPED_DEPTH_SAMPLE_GLSL
#define MARTENSITE_CLIPPED_DEPTH_SAMPLE_GLSL
#include "window_depth_plane.glsl"
#include "clipped_depth_transport.glsl"
#include "clipped_depth_status.glsl"
#include "window_depth_bias.glsl"
#include "clipped_depth_incidence.glsl"
layout(constant_id=24) const uint referenceBits=0u;
bool clippedDepthSampleGrid(vec2 value,out i64vec2 grid) {
    if(referenceBits!=8u&&referenceBits!=10u&&referenceBits!=12u) return false;
    precise vec2 scaled=value*float(1u<<referenceBits);
    if(any(isnan(scaled))||any(isinf(scaled))||any(greaterThan(abs(scaled),vec2(16777215.0)))) return false;
    grid=i64vec2(roundEven(scaled));
    return true;
}
int64_t clippedDepthEdge(i64vec2 a,i64vec2 b,i64vec2 point) {
    i64vec2 edge=b-a,delta=point-a;
    return edge.x*delta.y-edge.y*delta.x;
}
bool clippedDepthSampleEdge(i64vec2 a,i64vec2 b,i64vec2 point,bool outer) {
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    // Geometry coverage is already known. Only the shared internal separator selects a plane.
    if(outer) return true;
#endif
    int64_t edge=clippedDepthEdge(a,b,point);
    i64vec2 delta=b-a;
    bool topLeft=delta.y<0||(delta.y==0&&delta.x<0);
    return edge>0||(edge==0&&(outer||topLeft));
}
bool clippedDepthSampleFan(uint fan,i64vec2 point,uvec2 range,uint emptyMask,out bool inside) {
    vec2 source[3];i64vec2 grid[3];
    if(!clippedDepthTransportPoint(0u,source[0])||!clippedDepthTransportPoint(fan+1u,source[1])||!clippedDepthTransportPoint(fan+2u,source[2])) return false;
    for(uint index=0u;index<3u;++index) if(!clippedDepthSampleGrid(source[index],grid[index])) return false;
    int64_t area=clippedDepthEdge(grid[0],grid[1],grid[2]);
    inside=false;
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    if((emptyMask&(1u<<fan))!=0u) return true;
    if(area==int64_t(0)) return false;
#else
    if((area==int64_t(0))!=((emptyMask&(1u<<fan))!=0u)) return false;
    if(area==int64_t(0)) return true;
#endif
    bool firstOuter=fan==range.x,lastOuter=fan==range.y;
    if(area<int64_t(0)) {i64vec2 first=grid[1];grid[1]=grid[2];grid[2]=first;bool outer=firstOuter;firstOuter=lastOuter;lastOuter=outer;}
    inside=clippedDepthSampleEdge(grid[0],grid[1],point,firstOuter)&&clippedDepthSampleEdge(grid[1],grid[2],point,true)&&clippedDepthSampleEdge(grid[2],grid[0],point,lastOuter);
    return true;
}
bool clippedDepthSelectFan(vec2 samplePosition,out uint selected) {
    uint count=clippedDepthTransportPolygonCount();i64vec2 point;
    if(count<3u||!clippedDepthSampleGrid(samplePosition,point)) return clippedDepthFault(512u);
    uvec2 range;uint emptyMask;
    if(!clippedDepthTransportFanRange(range,emptyMask)) return clippedDepthFault(512u);
    vec2 root;i64vec2 rootGrid;
    if(!clippedDepthTransportPoint(0u,root)||!clippedDepthSampleGrid(root,rootGrid)) return clippedDepthFault(1024u);
    if(all(equal(rootGrid,point))) {selected=range.x;return true;}
    bool found=false;
    for(uint fan=0u;fan+2u<count;++fan) {
        bool inside;
        if(!clippedDepthSampleFan(fan,point,range,emptyMask,inside)) return clippedDepthFault(1024u);
        if(inside) {if(found) return clippedDepthFault(2048u);selected=fan;found=true;}
    }
    if(!found) return clippedDepthFault(4096u);
    return true;
}
bool clippedDepthEvaluateSample(uint fan,vec2 point,out float result) {
    WindowDepthPlane plane;float slope;
    if(!clippedDepthTransportPlane(fan,plane,slope)) return false;
    precise float depth=windowDepthFramebufferSample(plane,point);
    vec2 effective=windowDepthEffectiveBias();
    precise float slopeTerm=slope*effective.y;
    precise float unitTerm=effective.x*windowDepthBias.operands.w;
    precise float bias=slopeTerm+unitTerm;
    precise float biased=depth+bias;
    if(!windowDepthFinite(vec3(slopeTerm,unitTerm,bias))||!windowDepthFinite(vec3(depth,biased,0.0))) return false;
    result=clamp(biased,0.0,1.0);
    return true;
}
bool clippedDepthWriteSample() {
    // SamplePosition forces sample-rate evaluation; ordinary colors below explicitly stay at pixel center.
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    vec2 lattice=gl_SamplePosition*16.0;
    if(any(isnan(lattice))||any(isinf(lattice))||any(lessThan(lattice,vec2(0)))||any(greaterThan(lattice,vec2(16)))||any(notEqual(lattice,roundEven(lattice)))) return clippedDepthFault(16384u);
#endif
    vec2 point=floor(gl_FragCoord.xy)+gl_SamplePosition;
    uint fan;float depth;
    if(!clippedDepthSelectFan(point,fan)||!clippedDepthEvaluateSample(fan,point,depth)) {
        clippedDepthObserveFailure();
        return clippedDepthFault(128u);
    }
    gl_FragDepth=depth;
    return true;
}
#endif
