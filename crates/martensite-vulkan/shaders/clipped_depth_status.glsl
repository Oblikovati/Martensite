#ifndef MARTENSITE_CLIPPED_DEPTH_STATUS_GLSL
#define MARTENSITE_CLIPPED_DEPTH_STATUS_GLSL
#include "window_depth_status_storage.glsl"
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
const uint CLIPPED_DEPTH_SCHEMA=0x43464936u;
#define CLIPPED_DEPTH_SOURCE_MODE 2u
#else
const uint CLIPPED_DEPTH_SCHEMA=0x43464434u;
#define CLIPPED_DEPTH_SOURCE_MODE 1u
#endif
bool clippedDepthFault(uint reason) {
    atomicOr(windowDepthStatus.fault,reason);
    return false;
}
#ifndef MARTENSITE_FRAGMENT
bool clippedDepthBegin(out uint tail) {
    atomicAdd(windowDepthStatus.invocations,1u);
    uint count=windowDepthStatus.triangleCount;
    if(windowDepthStatus.schema!=WINDOW_DEPTH_SCHEMA||count>0x7fffffffu) return clippedDepthFault(2u);
    if(uint(gl_PrimitiveIDIn)>=count||count*2u>uint(windowDepthStatus.words.length())) return clippedDepthFault(1u);
    atomicAdd(windowDepthStatus.words[uint(gl_PrimitiveIDIn)*2u+1u],1u);
    tail=count*2u;
    return true;
}
bool clippedDepthComplete(uint reason) {
    atomicOr(windowDepthStatus.words[uint(gl_PrimitiveIDIn)*2u],WINDOW_DEPTH_DONE|reason);
    return reason==0u||reason==2u;
}
bool clippedDepthReferencePlanes(uint tail,out vec4 planes[14],out uint count) {
    uint length=uint(windowDepthStatus.words.length());
    if(tail>length||length-tail<4u) return clippedDepthFault(4u);
    count=windowDepthStatus.words[tail+1u];
    if(windowDepthStatus.words[tail]!=CLIPPED_DEPTH_SCHEMA||count==0u||count>8u) return clippedDepthFault(4u);
    uint stride=windowDepthStatus.words[tail+3u],triangles=windowDepthStatus.triangleCount;
    if(stride!=0u&&(stride!=64u||triangles>32768u)) return clippedDepthFault(4u);
    if(windowDepthStatus.words[tail+2u]!=CLIPPED_DEPTH_SOURCE_MODE||length-tail!=4u+count*4u+triangles*stride) return clippedDepthFault(4u);
    for(uint index=0u;index<count;++index) {
        uint offset=tail+4u+index*4u;
        planes[index]=uintBitsToFloat(uvec4(windowDepthStatus.words[offset],windowDepthStatus.words[offset+1u],windowDepthStatus.words[offset+2u],windowDepthStatus.words[offset+3u]));
        if(!clippedDepthFinite(planes[index])) return clippedDepthFault(4u);
    }
    return true;
}
#endif
#endif
