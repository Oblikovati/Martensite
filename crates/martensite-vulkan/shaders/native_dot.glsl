#ifndef MARTENSITE_NATIVE_DOT
#define MARTENSITE_NATIVE_DOT
#include "glsl_normalize.glsl"
// dot() of a vec3 as the AMD drivers compile the native program. NIR scalarizes fdot3 last
// component first, (z0*z1 + y0*y1) + x0*x1, and each product is contracted with the add consuming
// it. For the first add, whose operands are both products, radeonsi fuses its first operand and
// RADV its second (clip_position.glsl; both compile with ACO on this host, GL_RENDERER reports
// "radeonsi, strix_halo, ACO"), so the first two products are written swapped: RADV then computes
// fma(x0, x1, fma(z0, z1, y0*y1)), as radeonsi does with the native dot(). Without contraction
// (llvmpipe, lavapipe at SSE2) both orders round alike, so lavapipe images are unchanged.
// On RADV this moves bug23738 (320,163) and bug25264* (191,277) onto the native pixel.
float nativeDot3(vec3 a, vec3 b) {
    return (a.y * b.y + a.z * b.z) + a.x * b.x;
}
#endif
