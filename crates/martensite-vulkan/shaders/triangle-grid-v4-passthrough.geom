#version 450
#extension GL_GOOGLE_include_directive : require
layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;
layout(location=0) in vec4 inVertexColor[];
layout(location=1) in vec3 inWorldNormal[];
layout(location=2) in vec3 inViewDirection[];
#ifdef MARTENSITE_CLIPPING
layout(location=3) in vec4 inClippingWorldPosition[];
#endif
layout(location=4) in vec4 inWorldPosition[];
layout(location=5) in vec4 inFrontColor[];
layout(location=6) in vec4 inBackColor[];
layout(location=7) in vec4 inShadowPosition[][4];
layout(location=0) out vec4 vertexColor;
layout(location=1) out vec3 worldNormal;
layout(location=2) out vec3 viewDirection;
#ifdef MARTENSITE_CLIPPING
layout(location=3) out vec4 clippingWorldPosition;
#endif
layout(location=4) out vec4 worldPosition;
layout(location=5) out vec4 frontColor;
layout(location=6) out vec4 backColor;
layout(location=7) flat out vec4 v4PhongClip[3];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=10) flat out vec3 v4PhongNormal0;
layout(location=11) flat out vec3 v4PhongNormal1;
layout(location=12) flat out vec3 v4PhongNormal2;
#else
layout(location=10) flat out vec3 v4PhongNormal[3];
#endif
layout(location=15) out vec4 shadowPosition[4];
layout(location=19) flat out vec4 v4PhongPosition[3];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=22) flat out vec3 v4PhongView0;
layout(location=23) flat out vec3 v4PhongView1;
layout(location=24) flat out vec3 v4PhongView2;
#else
layout(location=22) flat out vec3 v4PhongView[3];
#endif
layout(location=25) flat out vec4 v4PhongColor[3];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
layout(location=13) flat out vec4 v4ClippingWorldPosition0;
layout(location=14) flat out vec4 v4ClippingWorldPosition1;
layout(location=10, component=3) flat out float v4ClippingWorldPosition2X;
layout(location=11, component=3) flat out float v4ClippingWorldPosition2Y;
layout(location=12, component=3) flat out float v4ClippingWorldPosition2Z;
layout(location=22, component=3) flat out float v4ClippingWorldPosition2W;
#else
layout(location=28) flat out vec4 v4ClippingWorldPosition[3];
#endif
#endif
void main() {
    for (int outputVertex = 0; outputVertex < 3; ++outputVertex) {
        vertexColor = inVertexColor[outputVertex];
        worldNormal = inWorldNormal[outputVertex];
        viewDirection = inViewDirection[outputVertex];
#ifdef MARTENSITE_CLIPPING
        clippingWorldPosition = inClippingWorldPosition[outputVertex];
#endif
        worldPosition = inWorldPosition[outputVertex];
        for (int shadow = 0; shadow < 4; ++shadow) {
            shadowPosition[shadow] = inShadowPosition[outputVertex][shadow];
        }
        frontColor = inFrontColor[outputVertex];
        backColor = inBackColor[outputVertex];
        for (int sourceVertex = 0; sourceVertex < 3; ++sourceVertex) {
            v4PhongClip[sourceVertex] = gl_in[sourceVertex].gl_Position;
#ifndef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
            v4PhongNormal[sourceVertex] = inWorldNormal[sourceVertex];
#endif
            v4PhongPosition[sourceVertex] = inWorldPosition[sourceVertex];
#ifndef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
            v4PhongView[sourceVertex] = inViewDirection[sourceVertex];
#endif
            v4PhongColor[sourceVertex] = inVertexColor[sourceVertex];
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
            if (sourceVertex == 0) v4ClippingWorldPosition0 = inClippingWorldPosition[sourceVertex];
            if (sourceVertex == 1) v4ClippingWorldPosition1 = inClippingWorldPosition[sourceVertex];
            if (sourceVertex == 2) {
                vec4 clipWorld = inClippingWorldPosition[sourceVertex];
                v4ClippingWorldPosition2X = clipWorld.x;
                v4ClippingWorldPosition2Y = clipWorld.y;
                v4ClippingWorldPosition2Z = clipWorld.z;
                v4ClippingWorldPosition2W = clipWorld.w;
            }
#else
            v4ClippingWorldPosition[sourceVertex] = inClippingWorldPosition[sourceVertex];
#endif
#endif
        }
#ifdef MARTENSITE_WORLD_CLIP_GRID12_V4_COMPACT
        v4PhongNormal0 = inWorldNormal[0];
        v4PhongNormal1 = inWorldNormal[1];
        v4PhongNormal2 = inWorldNormal[2];
        v4PhongView0 = inViewDirection[0];
        v4PhongView1 = inViewDirection[1];
        v4PhongView2 = inViewDirection[2];
#endif
        gl_Position = gl_in[outputVertex].gl_Position;
        gl_PrimitiveID = gl_PrimitiveIDIn;
        EmitVertex();
    }
    EndPrimitive();
}
