#version 450
#extension GL_GOOGLE_include_directive : require
#extension GL_ARB_gpu_shader_int64 : require
#include "clipped_depth_prepare.glsl"
#include "clipped_depth_incidence.glsl"
layout(triangles) in;
in gl_PerVertex { vec4 gl_Position; float gl_PointSize; float gl_ClipDistance[8]; } gl_in[];
out gl_PerVertex { vec4 gl_Position; float gl_ClipDistance[8]; };
layout(triangle_strip,max_vertices=3) out;
layout(location=0) in vec4 inputColor[];
layout(location=1) in vec3 inputNormal[];
layout(location=2) in vec3 inputDirection[];
layout(location=3) in vec4 inputClipWorld[];
layout(location=4) in vec4 inputPosition[];
layout(location=5) in vec4 inputFront[];
layout(location=6) in vec4 inputBack[];
layout(location=9) flat in vec4 inputFixedFront[];
layout(location=10) flat in vec4 inputFixedBack[];
layout(location=12) in vec4 inputRawGl[];
layout(location=0) out vec4 vertexColor;
layout(location=1) out vec4 clippingWorldPosition;
layout(location=2) out vec4 frontColor;
layout(location=3) out vec4 backColor;
layout(constant_id=20) const float viewportX=0.0;
layout(constant_id=21) const float viewportY=0.0;
layout(constant_id=22) const float viewportWidth=0.0;
layout(constant_id=23) const float viewportHeight=0.0;
layout(constant_id=24) const uint referenceBits=0u;
layout(constant_id=25) const uint rasterBits=0u;
layout(constant_id=26) const bool negativeOneToOne=false;
#ifdef MARTENSITE_SHARED_CLIP_INCIDENCE_V6
#include "clipped_depth_empty_reference.glsl"
#include "clipped_depth_shared_incidence.glsl"
#endif
void main() {
    vec4 raw[3]=vec4[3](inputRawGl[0],inputRawGl[1],inputRawGl[2]);
    ClippedDepthFlatRecord record;
    if(!clippedDepthPrepareActual(raw,vec4(viewportX,viewportY,viewportWidth,viewportHeight),referenceBits,record)) return;
    float actualDistances[24];
    for(uint vertex=0u;vertex<3u;++vertex) for(uint plane=0u;plane<8u;++plane) actualDistances[vertex*8u+plane]=gl_in[vertex].gl_ClipDistance[plane];
    clippedDepthObserveInputs(raw,actualDistances);
    for(uint index=0u;index<3u;++index) {
        gl_Position=gl_in[index].gl_Position;
        for(uint plane=0u;plane<8u;++plane) gl_ClipDistance[plane]=gl_in[index].gl_ClipDistance[plane];
        gl_PrimitiveID=gl_PrimitiveIDIn;
        vertexColor=inputColor[index];clippingWorldPosition=inputClipWorld[index];
        frontColor=inputFront[index];backColor=inputBack[index];
        clippedDepthTransportPublish(record);
        EmitVertex();
    }
    EndPrimitive();
}
