#version 450
#extension GL_GOOGLE_include_directive : require
#include "window_depth_plane.glsl"
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
#include "window_depth_status.glsl"
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
#include "window_depth_float64_status.glsl"
#endif
#endif
layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
layout(location=11) in vec4 inputRawGLClip[];
#endif
layout(location=0) in vec4 inputColor[];
layout(location=0) out vec4 vertexColor;
#ifdef MARTENSITE_LIGHTING
layout(location=1) in vec3 inputNormal[];
layout(location=2) in vec3 inputDirection[];
layout(location=4) in vec4 inputPosition[];
layout(location=5) in vec4 inputFront[];
layout(location=6) in vec4 inputBack[];
layout(location=7) in vec4 inputShadow[][4];
layout(location=1) out vec3 worldNormal;
layout(location=2) out vec3 viewDirection;
layout(location=4) out vec4 worldPosition;
layout(location=5) out vec4 frontColor;
layout(location=6) out vec4 backColor;
layout(location=15) out vec4 PosLightSpace[4];
#endif
layout(constant_id=20) const float viewportX=0.0;
layout(constant_id=21) const float viewportY=0.0;
layout(constant_id=22) const float viewportWidth=0.0;
layout(constant_id=23) const float viewportHeight=0.0;
layout(constant_id=24) const uint referenceBits=0u;
layout(constant_id=25) const uint rasterBits=0u;
layout(constant_id=26) const bool negativeOneToOne=false;
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
layout(location=7) flat out dvec3 depthPlaneOrigin64;
layout(location=9) flat out dvec2 depthPlaneGradient64;
layout(location=10) flat out uint depthPlaneEmpty;
#else
layout(location=7) flat out vec3 depthPlaneOrigin;
layout(location=8) flat out vec2 depthPlaneGradient;
layout(location=9) flat out uint depthPlaneEmpty;
#endif
#endif
void main() {
    vec4 clip[3]=vec4[3](gl_in[0].gl_Position,gl_in[1].gl_Position,gl_in[2].gl_Position);
    vec4 corrected[3];
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
    vec4 viewport=vec4(viewportX,viewportY,viewportWidth,viewportHeight);
    bool empty;
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
    WindowDepthPlane64 plane;
    vec4 rawGL[3]=vec4[3](inputRawGLClip[0],inputRawGLClip[1],inputRawGLClip[2]);
    if(!windowDepthPrepare64(clip,rawGL,viewport,negativeOneToOne,referenceBits,rasterBits,plane,empty)) return;
#else
    WindowDepthPlane plane;
    if(!windowDepthPrepareActual(clip,viewport,negativeOneToOne,referenceBits,rasterBits,plane,empty)) return;
#endif
    for(uint index=0u;index<3u;++index) corrected[index]=clip[index];
#else
    windowDepthCompensate(clip,vec4(viewportX,viewportY,viewportWidth,viewportHeight),
                          negativeOneToOne,referenceBits,rasterBits,corrected);
#endif
    for (uint index=0u;index<3u;++index) {
        gl_Position=corrected[index];
#ifdef MARTENSITE_WINDOW_DEPTH_FRAGMENT
#ifdef MARTENSITE_FLOAT64_WINDOW_DEPTH_V5
        depthPlaneOrigin64=plane.origin;depthPlaneGradient64=plane.gradient;depthPlaneEmpty=empty?1u:0u;
#else
        depthPlaneOrigin=plane.origin;depthPlaneGradient=plane.gradient;depthPlaneEmpty=empty?1u:0u;
#endif
#endif
        gl_PrimitiveID=gl_PrimitiveIDIn;
        vertexColor=inputColor[index];
#ifdef MARTENSITE_LIGHTING
        worldNormal=inputNormal[index];viewDirection=inputDirection[index];
        worldPosition=inputPosition[index];frontColor=inputFront[index];backColor=inputBack[index];
        for (uint shadow=0u;shadow<4u;++shadow) PosLightSpace[shadow]=inputShadow[index][shadow];
#endif
        EmitVertex();
    }
    EndPrimitive();
}
