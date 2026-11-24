#version 450
// Copies the retained main colour bytes and depth of an immediate frame into its new targets.
// Each fragment fetches its own texel, so the copy does not depend on the viewport orientation.
#extension GL_EXT_samplerless_texture_functions : require
layout(set = 0, binding = 0) uniform texture2D savedColor;
layout(set = 1, binding = 0) uniform texture2D savedDepth;
layout(location = 0) out vec4 color;
void main() {
    ivec2 texel = ivec2(gl_FragCoord.xy);
    color = texelFetch(savedColor, texel, 0);
    gl_FragDepth = texelFetch(savedDepth, texel, 0).r;
}
