#ifndef MARTENSITE_CLIPPED_DEPTH_EMPTY_REFERENCE_GLSL
#define MARTENSITE_CLIPPED_DEPTH_EMPTY_REFERENCE_GLSL
// A closed all-sixteenth lattice proof is stronger than any supported sample-position subset.
int64_t clippedDepthLatticeCeiling(int64_t value) {
    int64_t result=value>=int64_t(0)?(value+int64_t(255))/int64_t(256):value/int64_t(256);
    return result;
}
int64_t clippedDepthLatticeFloor(int64_t value) {
    int64_t result=value>=int64_t(0)?value/int64_t(256):(value-int64_t(255))/int64_t(256);
    return result;
}
bool clippedDepthLatticeBounds(vec3 triangle[3],out i64vec2 low,out i64vec2 high) {
    i64vec2 point[3];
    for(uint vertex=0u;vertex<3u;++vertex) if(!clippedDepthGridPoint(triangle[vertex],12u,point[vertex])) return false;
    i64vec2 minimum=min(point[0],min(point[1],point[2]));
    i64vec2 maximum=max(point[0],max(point[1],point[2]));
    low=i64vec2(clippedDepthLatticeCeiling(minimum.x),clippedDepthLatticeCeiling(minimum.y));
    high=i64vec2(clippedDepthLatticeFloor(maximum.x),clippedDepthLatticeFloor(maximum.y));
    return true;
}
bool clippedDepthClosedReferenceHit(vec3 triangle[3],vec2 samplePoint,out bool hit) {
    int64_t signs[3];
    for(uint edge=0u;edge<3u;++edge) {
        vec3 polygon[3]=vec3[3](triangle[edge],triangle[(edge+1u)%3u],vec3(samplePoint,0));
        if(!clippedDepthGridArea(polygon,12u,signs[edge])) return false;
    }
    hit=(signs[0]>=int64_t(0)&&signs[1]>=int64_t(0)&&signs[2]>=int64_t(0))
        ||(signs[0]<=int64_t(0)&&signs[1]<=int64_t(0)&&signs[2]<=int64_t(0));
    return true;
}
uint clippedDepthReferenceRow(vec3 triangle[3],int64_t x,int64_t low,int64_t high) {
    if(x<int64_t(-65536)||x>int64_t(65536)||low<int64_t(-65536)||high>int64_t(65536)) return 2u;
    for(int64_t y=low;y<=high;++y) {
        bool hit;vec2 samplePoint=vec2(float(x),float(y))*(1.0/16.0);
        if(!clippedDepthClosedReferenceHit(triangle,samplePoint,hit)) return 2u;
        if(hit) return 1u;
    }
    return 0u;
}
uint clippedDepthReferenceScan(vec3 triangle[3],i64vec2 low,i64vec2 high) {
    if(any(lessThan(low,i64vec2(-65536)))||any(greaterThan(high,i64vec2(65536)))) return 2u;
    i64vec2 count=high-low+i64vec2(1);
    if(any(lessThanEqual(count,i64vec2(0)))) return 0u;
    if(count.x>int64_t(65536)/count.y) return 2u;
    for(int64_t x=low.x;x<=high.x;++x) {
        uint state=clippedDepthReferenceRow(triangle,x,low.y,high.y);
        if(state!=0u) return state;
    }
    return 0u;
}
uint clippedDepthReferenceEmptyState(vec3 triangle[3],inout uint remaining) {
    int64_t area;
    if(!clippedDepthGridArea(triangle,12u,area)) return 2u;
    if(area==int64_t(0)) return 0u;
    i64vec2 low,high;
    if(!clippedDepthLatticeBounds(triangle,low,high)) return 2u;
    if(any(greaterThan(low,high))) return 0u;
    i64vec2 count=high-low+i64vec2(1);
    if(count.x>int64_t(remaining)/count.y) return 2u;
    remaining-=uint(count.x*count.y);
    return clippedDepthReferenceScan(triangle,low,high);
}
#endif
