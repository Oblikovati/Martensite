#ifndef MARTENSITE_CLIPPED_DEPTH_POLYGON_GLSL
#define MARTENSITE_CLIPPED_DEPTH_POLYGON_GLSL
// Explicit successor only. Intersection chains are admitted; union chains refuse reference.
const uint clippedDepthVertexLimit=17u;
struct ClippedDepthVertex {
    vec4 clip;
    vec3 weights;
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
    float incidences[8];
#endif
};
bool clippedDepthFinite(vec4 value) {
    bool finiteValue=!any(isnan(value))&&!any(isinf(value));
    return finiteValue;
}
float clippedDepthDistance(vec4 clip,vec4 plane) {
    precise vec4 products=clip*plane;
    precise float first=products.x+products.y;
    precise float second=first+products.z;
    precise float distance=second+products.w;
    return distance;
}
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
bool clippedDepthStoreIncidences(ClippedDepthVertex base,ClippedDepthVertex other,float t,
                                 vec4 clip,vec3 weights,out ClippedDepthVertex vertex) {
    vertex.clip=clip;vertex.weights=weights;
    for(uint plane=0u;plane<8u;++plane) {
        precise float difference=other.incidences[plane]-base.incidences[plane];
        precise float incidence=base.incidences[plane]+t*difference;
        if(isnan(incidence)||isinf(incidence)) return false;
        vertex.incidences[plane]=incidence;
    }
    return true;
}
#define CLIPPED_DEPTH_INTERSECTION(vertex,clip,weights,base,other,t) if(!clippedDepthStoreIncidences(base,other,t,clip,weights,vertex)) return false
#else
#define CLIPPED_DEPTH_INTERSECTION(vertex,clip,weights,base,other,t) vertex=ClippedDepthVertex(clip,weights)
#endif
bool clippedDepthIntersection(ClippedDepthVertex a,ClippedDepthVertex b,float da,float db,
                              out ClippedDepthVertex vertex) {
    precise float denominator=db-da;
    if(isnan(denominator)||isinf(denominator)||denominator==0.0) return false;
    bool useB=abs(db)<abs(da);
    ClippedDepthVertex base=useB?b:a,other=useB?a:b;
    precise float t=(useB?db:-da)/denominator;
    if(isnan(t)||isinf(t)||t<0.0||t>1.0) return false;
    precise vec4 delta=other.clip-base.clip;
    precise vec3 weightDelta=other.weights-base.weights;
    precise vec4 clip=base.clip+t*delta;
    precise vec3 weights=base.weights+t*weightDelta;
    CLIPPED_DEPTH_INTERSECTION(vertex,clip,weights,base,other,t);
    return clippedDepthFinite(clip)&&clippedDepthFinite(vec4(weights,1.0))&&clip.w>0.0;
}
bool clippedDepthAppend(inout ClippedDepthVertex polygon[17],inout uint count,
                        ClippedDepthVertex vertex) {
    if(count>=clippedDepthVertexLimit) return false;
    polygon[count]=vertex;
    count+=1u;
    return true;
}
bool clippedDepthAppendEdge(ClippedDepthVertex a,ClippedDepthVertex b,float da,float db,
                            inout ClippedDepthVertex polygon[17],inout uint count) {
    if(da>=0.0&&!clippedDepthAppend(polygon,count,a)) return false;
    if((da<0.0)==(db<0.0)) return true;
    ClippedDepthVertex vertex;
    if(!clippedDepthIntersection(a,b,da,db,vertex)) return false;
    return clippedDepthAppend(polygon,count,vertex);
}
bool clippedDepthHalfspace(ClippedDepthVertex inputPolygon[17],uint inputCount,vec4 plane,
                           out ClippedDepthVertex outputPolygon[17],out uint outputCount) {
    outputCount=0u;
    if(inputCount>clippedDepthVertexLimit||!clippedDepthFinite(plane)) return false;
    for(uint index=0u;index<inputCount;++index) {
        ClippedDepthVertex a=inputPolygon[index],b=inputPolygon[(index+1u)%inputCount];
        precise float da=clippedDepthDistance(a.clip,plane);
        precise float db=clippedDepthDistance(b.clip,plane);
        if(isnan(da)||isinf(da)||isnan(db)||isinf(db)) return false;
        if(!clippedDepthAppendEdge(a,b,da,db,outputPolygon,outputCount)) return false;
    }
    return true;
}
#endif
