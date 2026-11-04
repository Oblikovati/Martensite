// NumericMaxZeroV2 defines only a NaN light-dot contribution, never a replacement normal.
// GLSL.std.450 FMax permits either NaN operand; the versioned kernel decision chooses zero.
// https://registry.khronos.org/SPIR-V/specs/unified1/GLSL.std.450.html#FMax
float numericLightDotMaxZero(float dotValue) {
    return isnan(dotValue) ? 0.0 : max(0.0, dotValue);
}
