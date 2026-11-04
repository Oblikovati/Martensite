#ifndef MARTENSITE_CLIPPED_DEPTH_SHARED_INCIDENCE_GLSL
#define MARTENSITE_CLIPPED_DEPTH_SHARED_INCIDENCE_GLSL
// V6 shares the actual VS incidences; it never approximates a clip-space inverse plane.
bool clippedDepthInitialIncidences(inout ClippedDepthVertex polygon[17]) {
    for(uint vertex=0u;vertex<3u;++vertex) {
        for(uint plane=0u;plane<8u;++plane) {
            float incidence=gl_in[vertex].gl_ClipDistance[plane];
            if(isnan(incidence)||isinf(incidence)) return false;
            polygon[vertex].incidences[plane]=incidence;
        }
    }
    return true;
}
bool clippedDepthIncidenceHalfspace(ClippedDepthVertex inputPolygon[17],uint inputCount,uint plane,
                                    out ClippedDepthVertex outputPolygon[17],out uint outputCount) {
    outputCount=0u;
    if(inputCount>17u||plane>=8u) return false;
    for(uint index=0u;index<inputCount;++index) {
        ClippedDepthVertex a=inputPolygon[index],b=inputPolygon[(index+1u)%inputCount];
        if(!clippedDepthAppendEdge(a,b,a.incidences[plane],b.incidences[plane],outputPolygon,outputCount)) return false;
    }
    return true;
}
bool clippedDepthSharedPolygon(vec4 clip[3],uint tail,out ClippedDepthVertex polygon[17],out uint count) {
    if(referenceBits!=12u||rasterBits!=8u||!clippedDepthCanonicalInterior(clip)) return false;
    vec4 planes[14];uint planeCount;
    if(!clippedDepthReferencePlanes(tail,planes,planeCount)||!clippedDepthInitialPolygon(clip,polygon)) return false;
    if(!clippedDepthInitialIncidences(polygon)) return false;
    count=3u;ClippedDepthVertex next[17];uint nextCount;
    for(uint plane=0u;plane<planeCount;++plane) {
        if(!clippedDepthIncidenceHalfspace(polygon,count,plane,next,nextCount)) return false;
        count=nextCount;
        for(uint vertex=0u;vertex<count;++vertex) polygon[vertex]=next[vertex];
        if(count==0u) break;
    }
    return true;
}
bool clippedDepthCertifyMixedEmpty(vec3 first[3],vec3 second[3],out uint emptyMask) {
    emptyMask=0u;uint remaining=65536u;
    if(clippedDepthReferenceEmptyState(first,remaining)==0u) {emptyMask=1u;return true;}
    if(clippedDepthReferenceEmptyState(second,remaining)==0u) {emptyMask=2u;return true;}
    return false;
}
bool clippedDepthViewportPlaneDomain(vec3 window[17],uint count,uint grid,out uint emptyMask) {
    emptyMask=0u;
    if(count!=3u&&count!=4u) return false;
    vec3 first[3]=vec3[3](window[0],window[1],window[2]);int64_t firstArea;
    if(!clippedDepthGridArea(first,grid,firstArea)) return false;
    if(firstArea==int64_t(0)) emptyMask|=1u;
    if(count==3u) return true;
    vec3 second[3]=vec3[3](window[0],window[2],window[3]);int64_t secondArea;
    if(!clippedDepthGridArea(second,grid,secondArea)) return false;
    if(secondArea==int64_t(0)) emptyMask|=2u;
    if(emptyMask!=0u||(firstArea>int64_t(0))==(secondArea>int64_t(0))) return true;
    return clippedDepthCertifyMixedEmpty(first,second,emptyMask);
}
bool clippedDepthStoreCertifiedPlanes(inout ClippedDepthFlatRecord record,vec3 window[17],uint count,
                                      uint grid,bool changed,uint emptyMask) {
    if((count!=3u&&count!=4u)||grid!=12u||emptyMask>=(1u<<(count-2u))) return false;
    for(uint fan=0u;fan+2u<count;++fan) {
        if((emptyMask&(1u<<fan))!=0u) continue;
        vec3 triangle[3]=vec3[3](window[fan+1u],window[fan+2u],window[0]);
        if(!changed&&count==3u) triangle=vec3[3](window[0],window[1],window[2]);
        uint unexpectedEmpty=0u;
        if(!clippedDepthStoreWindowPlane(record,fan,triangle,grid,unexpectedEmpty)||unexpectedEmpty!=0u) return false;
    }
    return true;
}
#endif
