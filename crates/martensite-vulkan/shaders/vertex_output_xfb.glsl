// Diagnostic metadata exports existing outputs; the original clip/normal assignments stay intact.
#ifdef MARTENSITE_VERTEX_OUTPUT_CAPTURE
layout(xfb_buffer=0, xfb_stride=32) out gl_PerVertex {
    layout(xfb_offset=0) vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[];
    float gl_CullDistance[];
};
layout(location=11, xfb_buffer=0, xfb_offset=28, xfb_stride=32) flat out uint capturedVertexIndex;
#endif
