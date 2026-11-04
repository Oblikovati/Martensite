// Mesa's software GL path clips line endpoints before wide-line setup; its Vulkan path
// permits an XY guard band. User distances request the same post-vertex clipping stage.
#ifdef MARTENSITE_NATIVE_LINE_XY_CLIP
out gl_PerVertex {
    vec4 gl_Position;
    float gl_PointSize;
    float gl_ClipDistance[4];
};
void nativeLineViewportClip(vec4 position) {
    gl_ClipDistance[0] = position.x + position.w;
    gl_ClipDistance[1] = position.w - position.x;
    gl_ClipDistance[2] = position.y + position.w;
    gl_ClipDistance[3] = position.w - position.y;
}
#endif
