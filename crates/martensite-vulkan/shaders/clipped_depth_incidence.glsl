#ifndef MARTENSITE_CLIPPED_DEPTH_INCIDENCE_GLSL
#define MARTENSITE_CLIPPED_DEPTH_INCIDENCE_GLSL
// Diagnostic records share the completion allocation, never a cross-stage data dependency.
const uint CLIPPED_INCIDENCE_STRIDE=64u;
bool clippedDepthIncidenceBase(uint primitive,out uint base) {
    uint count=windowDepthStatus.triangleCount,tail=count*2u;
    uint length=uint(windowDepthStatus.words.length());
    if(count>32768u||primitive>=count||tail>length||length-tail<4u) return false;
    if(windowDepthStatus.words[tail+3u]!=CLIPPED_INCIDENCE_STRIDE) return false;
    uint planes=windowDepthStatus.words[tail+1u];
    if(planes==0u||planes>8u) return false;
    uint start=tail+4u+planes*4u;
    if(start>length||count>(length-start)/CLIPPED_INCIDENCE_STRIDE) return false;
    base=start+primitive*CLIPPED_INCIDENCE_STRIDE;
    return true;
}
#ifdef MARTENSITE_FRAGMENT
bool clippedDepthIncidenceKey(out uint key) {
    vec2 pixel=floor(gl_FragCoord.xy),offset=gl_SamplePosition*16.0;
    if(any(isnan(pixel))||any(isinf(pixel))||any(lessThan(pixel,vec2(0)))||any(greaterThan(pixel,vec2(1023)))) return false;
    if(any(isnan(offset))||any(isinf(offset))||any(lessThan(offset,vec2(0)))||any(greaterThan(offset,vec2(15)))||any(notEqual(offset,roundEven(offset)))) return false;
    if(gl_SampleID<0||gl_SampleID>=8) return false;
    uvec2 p=uvec2(pixel),s=uvec2(offset);
    key=((((p.y*1024u+p.x)*16u+s.y)*16u+s.x)*8u)+uint(gl_SampleID);
    return true;
}
void clippedDepthObserveFailure() {
    uint base,key;
    if(!clippedDepthIncidenceBase(uint(gl_PrimitiveID),base)) return;
    if(!clippedDepthIncidenceKey(key)) {atomicOr(windowDepthStatus.fault,8192u);return;}
    atomicMin(windowDepthStatus.words[base],key);
}
#else
void clippedDepthObserveInputRow(uint base,uint vertex,vec4 clip,float actual[24],vec4 planes[14],uint count) {
    for(uint component=0u;component<4u;++component) atomicOr(windowDepthStatus.words[base+2u+vertex*4u+component],floatBitsToUint(clip[component]));
    for(uint plane=0u;plane<8u;++plane) {
        uint index=vertex*8u+plane;
        atomicOr(windowDepthStatus.words[base+14u+index],floatBitsToUint(actual[index]));
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
        float reference=plane<count?actual[index]:0.0;
#else
        float reference=plane<count?clippedDepthDistance(clip,planes[plane]):0.0;
#endif
        atomicOr(windowDepthStatus.words[base+38u+index],floatBitsToUint(reference));
    }
}
void clippedDepthObserveInputs(vec4 clip[3],float actual[24]) {
    uint base,tail=windowDepthStatus.triangleCount*2u;
    if(!clippedDepthIncidenceBase(uint(gl_PrimitiveIDIn),base)) return;
    vec4 planes[14];uint count;
    if(!clippedDepthReferencePlanes(tail,planes,count)) return;
    for(uint vertex=0u;vertex<3u;++vertex) clippedDepthObserveInputRow(base,vertex,clip[vertex],actual,planes,count);
    atomicOr(windowDepthStatus.words[base+1u],0x43494431u);
}
#endif
#endif
