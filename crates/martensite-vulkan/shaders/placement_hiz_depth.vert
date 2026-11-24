#version 450
#extension GL_GOOGLE_include_directive : require

#include "clip_position.glsl"
#include "draw_table.glsl"
layout(constant_id = 7) const bool kRemapClipDepth = true;
layout(location = 0) in vec4 position;
invariant gl_Position;

void main() {
    // The footprint must be the same as the plain lighting draw-table vertex stage.
    gl_Position = clipPosition(DRAW_CLIP_FROM_OBJECT, position);
    if (kRemapClipDepth) {
        gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
    }
}
