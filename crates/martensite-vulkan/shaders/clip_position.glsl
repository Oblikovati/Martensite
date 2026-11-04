// the reference implementation computes `occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex`;
// Mesa's GLSL front end lowers a matrix-vector product column by column, left to right
// (glsl/lower_mat_op_to_vec.cpp), while SPIR-V OpMatrixTimesVector is lowered last column
// first. The summation order is spelled out so that the rounded clip coordinates equal the
// native ones on the same driver; the matrix is the float product `(P * W) * M`.
// AMD drivers contract a product and the following add into one fused multiply-add. For an
// add of two products, radeonsi fuses its first operand and RADV its second (both drivers compile
// with ACO on this host; the operand choice is the drivers' own, see native_dot.glsl), so the
// first sum is written `y + x`: both then compute fma(c0, x, c1 * y). Without contraction
// (llvmpipe at SSE2) the two orders round alike.
vec4 clipPosition(mat4 clip, vec4 position) {
    return ((clip[1] * position.y + clip[0] * position.x) + clip[2] * position.z) + clip[3] * position.w;
}
