#ifndef MARTENSITE_CLIPPED_DEPTH_TRANSPORT_GLSL
#define MARTENSITE_CLIPPED_DEPTH_TRANSPORT_GLSL
// All three emitted original vertices carry identical flat records. No SSBO stage handoff.
// Fifteen fan rows and all seventeen polygon points fit the fixed-Smooth3 interface.
// Packed point8.z stores polygon count; point8.zw never stores a geometric point.
#ifdef MARTENSITE_FRAGMENT
layout(location=4) flat in vec4 clippedDepthPlaneRows[15];
layout(location=19) flat in vec4 clippedDepthPackedPoints[9];
#else
layout(location=4) flat out vec4 clippedDepthPlaneRows[15];
layout(location=19) flat out vec4 clippedDepthPackedPoints[9];
#endif
struct ClippedDepthFlatRecord {
    vec4 planeRows[15];
    vec4 packedPoints[9];
};
#ifdef MARTENSITE_FRAGMENT
uint clippedDepthTransportPolygonCount() {
    float encoded=clippedDepthPackedPoints[8].z;
    if(isnan(encoded)||isinf(encoded)||encoded<3.0||encoded>17.0) return 0u;
    if(floor(encoded)!=encoded) return 0u;
    return uint(encoded);
}
bool clippedDepthTransportFanRange(out uvec2 range,out uint emptyMask) {
    uint count=clippedDepthTransportPolygonCount();float encoded=clippedDepthPackedPoints[8].w;
    if(count<3u||isnan(encoded)||isinf(encoded)||encoded<0.0||encoded>32767.0||floor(encoded)!=encoded) return false;
    uint mask=(1u<<(count-2u))-1u;emptyMask=uint(encoded);
    if((emptyMask&~mask)!=0u) return false;
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    if(count>4u) return false;
#endif
    uint nonemptyFans=(~emptyMask)&mask;
    if(nonemptyFans==0u) return false;
    range=uvec2(findLSB(nonemptyFans),findMSB(nonemptyFans));
    return true;
}
bool clippedDepthTransportPoint(uint index,out vec2 point) {
    uint count=clippedDepthTransportPolygonCount();
    if(count<3u||count>17u||index>=count) return false;
    vec4 pair=clippedDepthPackedPoints[index/2u];
    point=(index&1u)==0u?pair.xy:pair.zw;
    return !any(isnan(point))&&!any(isinf(point));
}
bool clippedDepthTransportPlane(uint index,out WindowDepthPlane plane,out float slope) {
    uint count=clippedDepthTransportPolygonCount();
    if(count<3u||count>17u||index>=count-2u) return false;
    vec4 row=clippedDepthPlaneRows[index];
    plane=WindowDepthPlane(vec3(0.0,0.0,row.x),row.yz);
    slope=row.w;
    return !any(isnan(row))&&!any(isinf(row))&&slope>=0.0;
}
#else
void clippedDepthTransportInitialize(out ClippedDepthFlatRecord record) {
    for(uint index=0u;index<15u;++index) record.planeRows[index]=vec4(0.0);
    for(uint index=0u;index<9u;++index) record.packedPoints[index]=vec4(0.0);
}
bool clippedDepthTransportStorePoint(inout ClippedDepthFlatRecord record,uint index,vec2 point) {
    if(index>=17u||any(isnan(point))||any(isinf(point))) return false;
    uint pair=index/2u;
    if((index&1u)==0u) record.packedPoints[pair].xy=point;
    else record.packedPoints[pair].zw=point;
    return true;
}
bool clippedDepthTransportStorePlane(inout ClippedDepthFlatRecord record,uint index,WindowDepthPlane plane,float slope) {
    if(index>=15u||slope<0.0) return false;
    vec4 row=vec4(plane.origin.z,plane.gradient,slope);
    if(any(isnan(row))||any(isinf(row))) return false;
    record.planeRows[index]=row;
    return true;
}
bool clippedDepthTransportComplete(inout ClippedDepthFlatRecord record,uint count) {
    if(count<3u||count>17u) return false;
    record.packedPoints[8].z=float(count);
    return true;
}
// EmitVertex invalidates outputs. Restore this complete local snapshot for every vertex.
void clippedDepthTransportPublish(ClippedDepthFlatRecord record) {
    for(uint index=0u;index<15u;++index) clippedDepthPlaneRows[index]=record.planeRows[index];
    for(uint index=0u;index<9u;++index) clippedDepthPackedPoints[index]=record.packedPoints[index];
}
#endif
#endif
