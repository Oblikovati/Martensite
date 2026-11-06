#version 450
// the reference implementation's Graphic3dShaderManager: native projected triangle altitudes.
layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;
layout(location=0) in vec4 inputColor[];
layout(location=0) out vec4 vertexColor;
layout(location=7) in vec4 inputUV[];
layout(location=7) out vec4 surfaceUV;
layout(location=8) out vec3 edgeDistance;
// Homogeneous edge lines (a, b, c), |(a, b)| = 1, in viewport pixels; see emitHomogeneousEdgeLines.
layout(location=11) flat out vec3 edgeLineA;
layout(location=12) flat out vec3 edgeLineB;
layout(location=13) flat out vec3 edgeLineC;
layout(location=14) flat out uint edgeLinesActive;
#ifdef MARTENSITE_LIGHTING
layout(location=1) in vec3 inputNormal[];
layout(location=2) in vec3 inputDirection[];
layout(location=4) in vec4 inputPosition[];
layout(location=5) in vec4 inputFront[];
layout(location=6) in vec4 inputBack[];
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=9) flat in vec4 inputFixedFront[];
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=10) flat in vec4 inputFixedBack[];
#endif
layout(location=1) out vec3 worldNormal;
layout(location=2) out vec3 viewDirection;
layout(location=4) out vec4 worldPosition;
layout(location=5) out vec4 frontColor;
layout(location=6) out vec4 backColor;
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=9) flat out vec4 fixedFrontColor;
#endif
#ifdef MARTENSITE_FIXED_LIGHTING
layout(location=10) flat out vec4 fixedBackColor;
#endif
#endif
#ifdef MARTENSITE_CLIPPING
layout(location=3) in vec4 inputClipping[];
layout(location=3) out vec4 clippingWorldPosition;
#endif
layout(push_constant) uniform EdgeTransform {
    layout(offset=80) vec4 edgeColor;
    layout(offset=96) float edgeWidth;
    layout(offset=100) float edgeFeather;
    layout(offset=104) vec2 viewport;
    layout(offset=112) uint skipFirst;
} transform;
// Graphic3d cameras emit OpenGL clip depth; the vertex stage maps it to 0..1 unless the viewport
// transform does (VK_EXT_depth_clip_control, depth_equivalence.rs). The native ViewPortTransform maps
// depth too, and it enters the side lengths, so an unmapped depth moves the altitudes.
layout(constant_id=7) const bool kRemapClipDepth=true;
vec3 viewportPosition(vec4 position) {
    vec3 coordinate=position.xyz/position.w;
    return vec3((coordinate.xy*0.5+0.5)*transform.viewport,kRemapClipDepth?coordinate.z:coordinate.z*0.5+0.5);
}
// Martensite definition (the design notes, "Mesh edges at vertices at infinity"):
// the reference implementation divides by clip w, undefined for a w = 0 vertex. In homogeneous viewport coordinates
// S = ((xy*0.5 + 0.5*w)*viewport, w) a w = 0 vertex is a direction: the edge through S_j and S_l
// is the line S_j x S_l, so an edge towards a w = 0 vertex is a ray and an edge joining two
// w = 0 vertices lies at infinity. The fragment measures its distance to these lines directly;
// per-vertex distances interpolated through homogeneous clipping are not reliable on drivers.
const float INFINITE_EDGE_DISTANCE=1.0e6;
vec3 homogeneousViewport(vec4 position) {
    return vec3((position.xy*0.5+0.5*position.w)*transform.viewport,position.w);
}
vec3 homogeneousEdgeLine(vec4 first, vec4 second) {
    vec3 line=cross(homogeneousViewport(first),homogeneousViewport(second));
    float length=length(line.xy);
    return length>0.0?line/length:vec3(0.0,0.0,INFINITE_EDGE_DISTANCE);
}
void emitHomogeneousEdgeLines() {
    // Edge k is opposite vertex k, as in edgeDistance; skip-first hides edge v0-v1.
    edgeLineA=homogeneousEdgeLine(gl_in[1].gl_Position,gl_in[2].gl_Position);
    edgeLineB=homogeneousEdgeLine(gl_in[2].gl_Position,gl_in[0].gl_Position);
    edgeLineC=transform.skipFirst!=0u?vec3(0.0,0.0,INFINITE_EDGE_DISTANCE)
        :homogeneousEdgeLine(gl_in[0].gl_Position,gl_in[1].gl_Position);
    edgeLinesActive=1u;
}
void main() {
    bool infinite=gl_in[0].gl_Position.w==0.0||gl_in[1].gl_Position.w==0.0||gl_in[2].gl_Position.w==0.0;
    vec3 a=viewportPosition(gl_in[2].gl_Position)-viewportPosition(gl_in[1].gl_Position);
    vec3 b=viewportPosition(gl_in[2].gl_Position)-viewportPosition(gl_in[0].gl_Position);
    vec3 c=viewportPosition(gl_in[1].gl_Position)-viewportPosition(gl_in[0].gl_Position);
    float area=abs(b.x*c.y-b.y*c.x);
    vec3 height=max(vec3(area)/vec3(length(a),length(b),length(c)),vec3(10.0*transform.edgeWidth));
    float skipped=transform.skipFirst!=0u?transform.edgeWidth+1.0:0.0;
    for(int index=0;index<3;++index) {
        vertexColor=inputColor[index]; surfaceUV=inputUV[index];
        edgeDistance=index==0?vec3(height.x,0.0,skipped):index==1?vec3(0.0,height.y,skipped):vec3(0.0,0.0,height.z);
#ifdef MARTENSITE_LIGHTING
        worldNormal=inputNormal[index]; viewDirection=inputDirection[index]; worldPosition=inputPosition[index];
        frontColor=inputFront[index]; backColor=inputBack[index];
#ifdef MARTENSITE_FIXED_LIGHTING
        fixedFrontColor=inputFixedFront[0]; fixedBackColor=inputFixedBack[0];
#endif
#endif
#ifdef MARTENSITE_CLIPPING
        clippingWorldPosition=inputClipping[index];
#endif
        edgeLineA=edgeLineB=edgeLineC=vec3(0.0); edgeLinesActive=0u;
        if (infinite) emitHomogeneousEdgeLines();
        gl_Position=gl_in[index].gl_Position; EmitVertex();
    }
    EndPrimitive();
}
