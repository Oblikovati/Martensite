// Exact binary32 homogeneous cross-products using only core 32-bit integer arithmetic.
// The branch is unreachable by covered ordinary-FILL samples of these exact degenerates.
uvec2 phongSignificandExponent(uint word) {
    uint exponent = (word >> 23u) & 255u;
    uint significand = word & 0x7fffffu;
    if (exponent != 0u) significand |= 0x800000u;
    return uvec2(significand, uint(exponent == 0u ? -149 : int(exponent) - 150));
}
uvec2 phongShiftProduct(uvec2 value, uint shift) {
    if (shift == 0u) return value;
    if (shift >= 32u) return uvec2(value.y >> (shift - 32u), 0u);
    return uvec2((value.x >> shift) | (value.y << (32u - shift)), value.y >> shift);
}
uvec4 phongExactProduct(float left, float right) {
    uint l = floatBitsToUint(left), r = floatBitsToUint(right);
    uvec2 a = phongSignificandExponent(l), b = phongSignificandExponent(r);
    if (a.x == 0u || b.x == 0u) return uvec4(0u);
    uint high, low;
    umulExtended(a.x, b.x, high, low);
    uint shift = low != 0u ? uint(findLSB(low)) : 32u + uint(findLSB(high));
    uvec2 product = phongShiftProduct(uvec2(low, high), shift);
    return uvec4(product, a.y + b.y + shift, (l ^ r) & 0x80000000u);
}
bool phongSameRatio(vec4 a, vec4 b, int axis) {
    return all(equal(phongExactProduct(a[axis], b.w), phongExactProduct(b[axis], a.w)));
}
bool phongInsideClip(vec4 clip) {
    if (any(isnan(clip)) || any(isinf(clip)) || clip.w <= 0.0) return false;
    return all(lessThanEqual(abs(clip.xyz), vec3(clip.w)));
}
bool phongExactProjectedAxis() {
    // Classify actual shader inputs before interpreting their binary32 exponents.
    if (!phongInsideClip(phongClip[0]) || !phongInsideClip(phongClip[1])
        || !phongInsideClip(phongClip[2])) return false;
    bool x = phongSameRatio(phongClip[0], phongClip[1], 0)
          && phongSameRatio(phongClip[0], phongClip[2], 0);
    bool y = phongSameRatio(phongClip[0], phongClip[1], 1)
          && phongSameRatio(phongClip[0], phongClip[2], 1);
    return x || y;
}
